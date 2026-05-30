//! TLS termination and mutual-TLS support for the A2A HTTP surface.
//!
//! When [`TlsConfig::enable`] is `true`, [`A2AServer::serve`] hands the
//! Axum router to [`axum_server`] backed by [`rustls`] instead of the
//! plaintext `axum::serve` path. When [`TlsConfig::client_ca_path`] is
//! also set, the server requires every TLS client to present a
//! certificate signed by one of the configured CAs - i.e. mutual TLS
//! ([`MutualTlsSecurityScheme`] in the A2A spec) - and the peer's leaf
//! certificate is exposed to downstream handlers as a
//! [`ClientCertPrincipal`] request extension.
//!
//! The TLS stack is [`rustls`] 0.23 with the [`ring`] crypto provider.
//! Rustls is widely deployed, has a pure-Rust audit trail, and gives us
//! direct access to peer certificates without going through an opaque
//! C-level TLS library - which is what makes the mTLS subject extraction
//! below tractable.
//!
//! [`MutualTlsSecurityScheme`]: crate::a2a_types::MutualTlsSecurityScheme
//! [`A2AServer::serve`]: crate::server::A2AServer::serve
//! [`ring`]: https://github.com/briansmith/ring

use std::future::Future;
use std::io;
use std::path::Path;
use std::pin::Pin;
use std::sync::Arc;

use anyhow::{Context, Result, anyhow};
use axum_server::accept::Accept;
use rustls::RootCertStore;
use rustls::ServerConfig as RustlsServerConfig;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls::server::WebPkiClientVerifier;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_rustls::TlsAcceptor;
use tokio_rustls::server::TlsStream;
use tower::Layer;
use tower_http::add_extension::{AddExtension, AddExtensionLayer};
use tracing::{debug, warn};

use crate::config::TlsConfig;

/// Subject information lifted from the leaf certificate of an mTLS
/// client. Plumbed through Axum request extensions so JSON-RPC handlers
/// can scope behaviour by certificate identity in the same way they
/// scope by [`AuthenticatedPrincipal`].
///
/// [`AuthenticatedPrincipal`]: crate::server::AuthenticatedPrincipal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientCertPrincipal {
    /// Subject DN as an RFC 4514 string, e.g. `CN=alice,O=Example,C=US`.
    /// Already validated against the configured trust roots before this
    /// struct is constructed.
    pub subject: String,
    /// The Common Name attribute lifted from the subject DN when one is
    /// present. May be `None` for certificates that only carry SANs.
    pub common_name: Option<String>,
    /// Issuer DN as an RFC 4514 string.
    pub issuer: String,
    /// Raw DER bytes of the leaf certificate, retained for callers that
    /// need to feed the cert into another verifier or log it for audit.
    #[serde(with = "der_bytes")]
    pub leaf_der: Vec<u8>,
}

/// Connection-level marker carrying the optional [`ClientCertPrincipal`]
/// produced during the TLS handshake. The acceptor inserts one of these
/// into every request's extensions; handlers extract via
/// `axum::Extension<PeerCert>` and inspect the inner `Option` because a
/// TLS-terminated listener without mTLS won't have a client certificate
/// to surface.
#[derive(Debug, Clone, Default)]
pub struct PeerCert(pub Option<ClientCertPrincipal>);

mod der_bytes {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_seq(bytes.iter())
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<u8>, D::Error> {
        Vec::<u8>::deserialize(deserializer)
    }
}

/// Read a PEM file from disk and return every certificate it contains.
pub(crate) fn load_certs(path: impl AsRef<Path>) -> Result<Vec<CertificateDer<'static>>> {
    let path = path.as_ref();
    let bytes = std::fs::read(path)
        .with_context(|| format!("failed to read TLS certificate file `{}`", path.display()))?;
    let mut slice = bytes.as_slice();
    let certs = rustls_pemfile::certs(&mut slice)
        .collect::<std::result::Result<Vec<_>, _>>()
        .with_context(|| format!("failed to parse PEM certificates in `{}`", path.display()))?;
    if certs.is_empty() {
        return Err(anyhow!("no certificates found in `{}`", path.display()));
    }
    Ok(certs)
}

/// Read a PEM file from disk and return the first private key it
/// contains (PKCS#1, PKCS#8, or SEC1).
pub(crate) fn load_private_key(path: impl AsRef<Path>) -> Result<PrivateKeyDer<'static>> {
    let path = path.as_ref();
    let bytes = std::fs::read(path)
        .with_context(|| format!("failed to read TLS key file `{}`", path.display()))?;
    let mut slice = bytes.as_slice();
    rustls_pemfile::private_key(&mut slice)
        .with_context(|| format!("failed to parse PEM private key in `{}`", path.display()))?
        .ok_or_else(|| anyhow!("no private key found in `{}`", path.display()))
}

/// Make sure a process-wide [`rustls::crypto::CryptoProvider`] is
/// installed before any [`RustlsServerConfig`] is constructed. We pick
/// `ring` because it builds across the platforms CI exercises without
/// pulling in `aws-lc-rs`'s C toolchain requirements.
pub(crate) fn install_default_crypto_provider() {
    // `install_default` returns `Err` if a provider is already installed
    // (e.g. when the host application also depends on rustls). Either
    // outcome is fine for us - we only need *some* provider to be the
    // process-wide default.
    let _ = rustls::crypto::ring::default_provider().install_default();
}

/// Construct a [`RustlsServerConfig`] from the on-disk certs/keys
/// referenced by [`TlsConfig`]. When `client_ca_path` is set, the
/// resulting config requires every TLS client to present a certificate
/// signed by one of the trusted CAs (i.e. mutual TLS).
pub(crate) fn build_server_config(tls: &TlsConfig) -> Result<Arc<RustlsServerConfig>> {
    install_default_crypto_provider();

    if tls.cert_path.trim().is_empty() {
        return Err(anyhow!(
            "SERVER_TLS_CERT_PATH is required when SERVER_TLS_ENABLE=true"
        ));
    }
    if tls.key_path.trim().is_empty() {
        return Err(anyhow!(
            "SERVER_TLS_KEY_PATH is required when SERVER_TLS_ENABLE=true"
        ));
    }

    let cert_chain = load_certs(&tls.cert_path)?;
    let key = load_private_key(&tls.key_path)?;

    let builder = RustlsServerConfig::builder();

    let server_config = if let Some(ca_path) = tls.client_ca_path.as_ref() {
        let mut roots = RootCertStore::empty();
        for cert in load_certs(ca_path)? {
            roots.add(cert).with_context(|| {
                format!("failed to add CA certificate from `{ca_path}` to trust store")
            })?;
        }
        let verifier = WebPkiClientVerifier::builder(Arc::new(roots))
            .build()
            .context("failed to build mTLS client certificate verifier")?;
        builder
            .with_client_cert_verifier(verifier)
            .with_single_cert(cert_chain, key)
            .context("failed to build rustls server config (mTLS)")?
    } else {
        builder
            .with_no_client_auth()
            .with_single_cert(cert_chain, key)
            .context("failed to build rustls server config")?
    };

    Ok(Arc::new(server_config))
}

/// Best-effort extraction of a [`ClientCertPrincipal`] from a leaf
/// certificate. `None` when the DER refuses to parse - the surrounding
/// acceptor logs and downgrades to "anonymous mTLS" rather than failing
/// the connection, because rustls has already validated the certificate
/// chain by the time we get here.
fn principal_from_leaf(leaf: &CertificateDer<'_>) -> Option<ClientCertPrincipal> {
    match x509_parser::parse_x509_certificate(leaf.as_ref()) {
        Ok((_, parsed)) => {
            let subject = parsed.subject().to_string();
            let issuer = parsed.issuer().to_string();
            let common_name = parsed
                .subject()
                .iter_common_name()
                .next()
                .and_then(|cn| cn.as_str().ok())
                .map(|s| s.to_string());
            Some(ClientCertPrincipal {
                subject,
                common_name,
                issuer,
                leaf_der: leaf.as_ref().to_vec(),
            })
        }
        Err(e) => {
            warn!("failed to parse client certificate as X.509: {e}");
            None
        }
    }
}

/// `axum_server::Accept` implementation that performs the TLS handshake
/// via [`tokio_rustls::TlsAcceptor`] and injects a [`PeerCert`]
/// extension into every request served over the resulting connection.
///
/// We re-implement the handshake here (rather than reuse
/// [`axum_server::tls_rustls::RustlsAcceptor`]) because we need
/// post-handshake access to the negotiated [`rustls::ServerConnection`]
/// to pull the peer's leaf certificate. The vendored acceptor does not
/// expose that without taking ownership of the stream.
#[derive(Clone)]
pub(crate) struct MtlsAcceptor {
    inner: TlsAcceptor,
}

impl MtlsAcceptor {
    pub fn new(config: Arc<RustlsServerConfig>) -> Self {
        Self {
            inner: TlsAcceptor::from(config),
        }
    }
}

impl std::fmt::Debug for MtlsAcceptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MtlsAcceptor").finish_non_exhaustive()
    }
}

impl<I, S> Accept<I, S> for MtlsAcceptor
where
    I: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    S: Send + 'static,
{
    type Stream = TlsStream<I>;
    type Service = AddExtension<S, PeerCert>;
    type Future = Pin<Box<dyn Future<Output = io::Result<(Self::Stream, Self::Service)>> + Send>>;

    fn accept(&self, stream: I, service: S) -> Self::Future {
        let acceptor = self.inner.clone();
        Box::pin(async move {
            let tls_stream = acceptor.accept(stream).await?;
            let principal = {
                let (_, conn) = tls_stream.get_ref();
                conn.peer_certificates()
                    .and_then(|chain| chain.first())
                    .and_then(principal_from_leaf)
            };
            if let Some(ref p) = principal {
                debug!(
                    subject = %p.subject,
                    common_name = ?p.common_name,
                    issuer = %p.issuer,
                    "TLS handshake completed with client certificate",
                );
            }
            let service = AddExtensionLayer::new(PeerCert(principal)).layer(service);
            Ok((tls_stream, service))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tempdir_path(name: &str) -> std::path::PathBuf {
        let mut p = std::env::temp_dir();
        p.push(format!(
            "rust-adk-tls-{}-{}",
            std::process::id(),
            name.replace('/', "_")
        ));
        p
    }

    #[test]
    fn load_certs_errors_on_missing_file() {
        let err = load_certs("/nonexistent/path.pem").expect_err("must fail");
        assert!(err.to_string().contains("failed to read TLS certificate"));
    }

    #[test]
    fn load_private_key_errors_on_missing_file() {
        let err = load_private_key("/nonexistent/path.pem").expect_err("must fail");
        assert!(err.to_string().contains("failed to read TLS key file"));
    }

    #[test]
    fn load_certs_errors_when_pem_is_empty() {
        let tmp = tempdir_path("empty.pem");
        std::fs::write(&tmp, "").expect("write empty pem");
        let err = load_certs(&tmp).expect_err("must fail");
        assert!(err.to_string().contains("no certificates found"));
        let _ = std::fs::remove_file(&tmp);
    }

    #[test]
    fn principal_from_leaf_handles_garbage() {
        let der = CertificateDer::from(vec![0u8; 8]);
        assert!(principal_from_leaf(&der).is_none());
    }

    #[test]
    fn install_default_crypto_provider_is_idempotent() {
        install_default_crypto_provider();
        install_default_crypto_provider();
    }

    #[test]
    fn build_server_config_requires_cert_path() {
        let cfg = TlsConfig {
            enable: true,
            cert_path: String::new(),
            key_path: "/tmp/key.pem".to_string(),
            client_ca_path: None,
        };
        let err = build_server_config(&cfg).expect_err("must fail");
        assert!(err.to_string().contains("SERVER_TLS_CERT_PATH"));
    }

    #[test]
    fn build_server_config_requires_key_path() {
        let cfg = TlsConfig {
            enable: true,
            cert_path: "/tmp/cert.pem".to_string(),
            key_path: String::new(),
            client_ca_path: None,
        };
        let err = build_server_config(&cfg).expect_err("must fail");
        assert!(err.to_string().contains("SERVER_TLS_KEY_PATH"));
    }
}

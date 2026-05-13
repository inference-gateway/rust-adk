//! Integration tests for the TLS / mTLS plumbing in `A2AServer`.
//!
//! The suite spins up a real `A2AServer` on a TLS listener, performs a
//! TLS handshake from the test process using `tokio_rustls`, and asserts
//! that the round-trip produces the expected behaviour:
//!
//! - `tls_terminates_on_https_listener` proves that `SERVER_TLS_ENABLE=true`
//!   with a cert/key pair actually serves traffic over TLS (no client
//!   cert required) and that `/health` is reachable.
//! - `mtls_handshake_with_valid_client_cert_succeeds` proves that
//!   `SERVER_TLS_CLIENT_CA_PATH` flips the listener into mTLS mode and
//!   that a client presenting a cert signed by the trusted CA can
//!   complete the handshake.
//! - `mtls_handshake_without_client_cert_is_rejected` proves that the
//!   same mTLS listener aborts the handshake when the client presents
//!   no certificate, so unauthenticated callers cannot bypass mTLS.
//!
//! Certificates and keys are generated per-test with `rcgen` and
//! materialised to a temp directory; nothing is shared on disk between
//! tests.

use std::io;
use std::net::{SocketAddr, TcpListener as StdTcpListener};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use inference_gateway_adk::{
    A2AServerBuilder, Config, ServerConfig, TaskHandler, TlsConfig, a2a_types,
};
use rcgen::{
    BasicConstraints, CertificateParams, DistinguishedName, DnType, IsCa, Issuer, KeyPair,
    KeyUsagePurpose,
};
use rustls::RootCertStore;
use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer, ServerName};
use serde_json::json;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_rustls::TlsConnector;

#[derive(Debug)]
struct EchoTaskHandler;

#[async_trait]
impl TaskHandler for EchoTaskHandler {
    async fn handle_task(
        &self,
        task: a2a_types::Task,
        _message: Option<a2a_types::Message>,
    ) -> anyhow::Result<a2a_types::Task> {
        Ok(task)
    }
}

fn allocate_port() -> u16 {
    let listener = StdTcpListener::bind("127.0.0.1:0").expect("bind ephemeral port");
    listener.local_addr().expect("local addr").port()
}

fn temp_path(name: &str) -> PathBuf {
    let mut p = std::env::temp_dir();
    p.push(format!(
        "rust-adk-tls-{}-{}-{name}",
        std::process::id(),
        uuid_like()
    ));
    p
}

/// A unique-enough suffix for test artefacts without pulling in `uuid`
/// as a dev-dependency.
fn uuid_like() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static N: AtomicU64 = AtomicU64::new(0);
    let n = N.fetch_add(1, Ordering::Relaxed);
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    format!("{ts:x}-{n:x}")
}

struct GeneratedCa {
    cert_pem: String,
    issuer: Issuer<'static, KeyPair>,
}

fn generate_ca(common_name: &str) -> GeneratedCa {
    let mut params = CertificateParams::default();
    params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
    params.key_usages = vec![
        KeyUsagePurpose::KeyCertSign,
        KeyUsagePurpose::CrlSign,
        KeyUsagePurpose::DigitalSignature,
    ];
    let mut dn = DistinguishedName::new();
    dn.push(DnType::CommonName, common_name);
    dn.push(DnType::OrganizationName, "rust-adk-tests");
    params.distinguished_name = dn;
    let keypair = KeyPair::generate().expect("generate CA keypair");
    let cert = params.self_signed(&keypair).expect("self-sign CA");
    let cert_pem = cert.pem();
    let issuer = Issuer::new(params, keypair);
    GeneratedCa { cert_pem, issuer }
}

struct GeneratedLeaf {
    cert_pem: String,
    key_pem: String,
}

fn issue_leaf(
    ca: &GeneratedCa,
    common_name: &str,
    sans: Vec<String>,
    server_auth: bool,
) -> GeneratedLeaf {
    let mut params = CertificateParams::new(sans).expect("leaf params");
    let mut dn = DistinguishedName::new();
    dn.push(DnType::CommonName, common_name);
    dn.push(DnType::OrganizationName, "rust-adk-tests");
    params.distinguished_name = dn;
    params.key_usages = vec![
        KeyUsagePurpose::DigitalSignature,
        KeyUsagePurpose::KeyEncipherment,
    ];
    params.extended_key_usages = if server_auth {
        vec![rcgen::ExtendedKeyUsagePurpose::ServerAuth]
    } else {
        vec![rcgen::ExtendedKeyUsagePurpose::ClientAuth]
    };
    let keypair = KeyPair::generate().expect("generate leaf keypair");
    let signed = params
        .signed_by(&keypair, &ca.issuer)
        .expect("sign leaf");
    GeneratedLeaf {
        cert_pem: signed.pem(),
        key_pem: keypair.serialize_pem(),
    }
}

fn write_temp(suffix: &str, contents: &str) -> PathBuf {
    let path = temp_path(suffix);
    std::fs::write(&path, contents).expect("write pem to temp dir");
    path
}

fn build_test_config(tls: TlsConfig, port: u16) -> Config {
    Config {
        port,
        tls_config: Some(tls.clone()),
        server_config: ServerConfig {
            host: "127.0.0.1".to_string(),
            port,
            tls_enable: true,
            tls_cert_path: Some(tls.cert_path.clone()),
            tls_key_path: Some(tls.key_path.clone()),
            tls_client_ca_path: tls.client_ca_path.clone(),
        },
        ..Config::default()
    }
}

fn agent_card(addr: SocketAddr) -> a2a_types::AgentCard {
    serde_json::from_value(json!({
        "name": "TLS Test Agent",
        "description": "Used by tls_test.rs",
        "version": "1.0.0",
        "protocolVersion": "0.2.6",
        "url": format!("https://{addr}/a2a"),
        "preferredTransport": "JSONRPC",
        "capabilities": {
            "streaming": false,
            "pushNotifications": false,
            "stateTransitionHistory": false
        },
        "defaultInputModes": ["text/plain"],
        "defaultOutputModes": ["text/plain"],
        "skills": [
            {"id": "x", "name": "x", "description": "x", "tags": ["x"]}
        ]
    }))
    .expect("agent card")
}

fn spawn_server(tls: TlsConfig, addr: SocketAddr) {
    let (ready_tx, ready_rx) = std::sync::mpsc::channel();
    let cfg = build_test_config(tls, addr.port());
    let card = agent_card(addr);
    std::thread::Builder::new()
        .name(format!("rust-adk-tls-test-{}", addr.port()))
        .spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("runtime builds");
            rt.block_on(async move {
                let server = A2AServerBuilder::new()
                    .with_config(cfg)
                    .with_agent_card(card)
                    .with_background_task_handler(EchoTaskHandler)
                    .build()
                    .await
                    .expect("server builds");
                let _ = ready_tx.send(());
                if let Err(e) = server.serve(addr).await {
                    eprintln!("tls test server stopped: {e}");
                }
            });
        })
        .expect("spawn server");

    ready_rx
        .recv_timeout(Duration::from_secs(5))
        .expect("server became ready");

    // Wait for the TLS listener to actually accept connections.
    for _ in 0..200 {
        if std::net::TcpStream::connect_timeout(&addr, Duration::from_millis(50)).is_ok() {
            return;
        }
        std::thread::sleep(Duration::from_millis(25));
    }
    panic!("tls server did not become ready at {addr}");
}

fn root_store(ca_pem: &str) -> RootCertStore {
    let mut store = RootCertStore::empty();
    let mut slice = ca_pem.as_bytes();
    for cert in rustls_pemfile::certs(&mut slice) {
        let cert = cert.expect("parse CA");
        store.add(cert).expect("trust CA");
    }
    store
}

fn leaf_chain(cert_pem: &str) -> Vec<CertificateDer<'static>> {
    let mut slice = cert_pem.as_bytes();
    rustls_pemfile::certs(&mut slice)
        .map(|c| c.expect("parse leaf"))
        .collect()
}

fn leaf_key(key_pem: &str) -> PrivateKeyDer<'static> {
    let mut slice = key_pem.as_bytes();
    let der = rustls_pemfile::pkcs8_private_keys(&mut slice)
        .next()
        .expect("rcgen emits pkcs8")
        .expect("parse pkcs8");
    PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(der.secret_pkcs8_der().to_vec()))
}

fn ensure_default_provider() {
    let _ = rustls::crypto::ring::default_provider().install_default();
}

async fn https_get_health(addr: SocketAddr, ca_pem: &str) -> io::Result<u16> {
    ensure_default_provider();
    let root_store = root_store(ca_pem);
    let client_config = rustls::ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();
    let connector = TlsConnector::from(Arc::new(client_config));
    let stream = TcpStream::connect(addr).await?;
    let domain = ServerName::try_from("localhost")
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    let mut tls = connector.connect(domain, stream).await?;

    let req = "GET /health HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n";
    tls.write_all(req.as_bytes()).await?;
    tls.flush().await?;

    let mut buf = Vec::with_capacity(256);
    let mut chunk = [0u8; 256];
    loop {
        match tls.read(&mut chunk).await {
            Ok(0) => break,
            Ok(n) => {
                buf.extend_from_slice(&chunk[..n]);
                if buf.len() > 1024 {
                    break;
                }
            }
            Err(_) => break,
        }
    }
    let head = std::str::from_utf8(&buf).unwrap_or_default();
    let status = head
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|s| s.parse::<u16>().ok())
        .ok_or_else(|| io::Error::other(format!("no status in response: {head:?}")))?;
    Ok(status)
}

async fn mtls_handshake(
    addr: SocketAddr,
    ca_pem: &str,
    client_cert_pem: Option<&str>,
    client_key_pem: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    ensure_default_provider();
    let root_store = root_store(ca_pem);
    let builder = rustls::ClientConfig::builder().with_root_certificates(root_store);
    let client_config = match (client_cert_pem, client_key_pem) {
        (Some(c), Some(k)) => builder.with_client_auth_cert(leaf_chain(c), leaf_key(k))?,
        _ => builder.with_no_client_auth(),
    };
    let connector = TlsConnector::from(Arc::new(client_config));
    let stream = TcpStream::connect(addr).await?;
    let domain = ServerName::try_from("localhost")?;
    let mut tls = connector.connect(domain, stream).await?;
    // Drive the handshake by attempting a tiny write/read. With rustls,
    // the certificate-required alert from the server can arrive *after*
    // the client-side `connect()` resolves, so propagating the
    // post-handshake I/O error is what surfaces the rejection.
    tls.write_all(b"GET /health HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n")
        .await?;
    let mut buf = [0u8; 64];
    let n = tls.read(&mut buf).await?;
    if n == 0 {
        return Err("server closed mTLS connection without sending any bytes".into());
    }
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn tls_terminates_on_https_listener() {
    let ca = generate_ca("rust-adk-tls-test-ca");
    let server_leaf = issue_leaf(
        &ca,
        "localhost",
        vec!["localhost".to_string(), "127.0.0.1".to_string()],
        true,
    );

    let cert_path = write_temp("server.crt", &server_leaf.cert_pem);
    let key_path = write_temp("server.key", &server_leaf.key_pem);

    let tls = TlsConfig {
        enable: true,
        cert_path: cert_path.to_string_lossy().into_owned(),
        key_path: key_path.to_string_lossy().into_owned(),
        client_ca_path: None,
    };

    let port = allocate_port();
    let addr: SocketAddr = format!("127.0.0.1:{port}").parse().unwrap();
    spawn_server(tls, addr);

    let status = tokio::time::timeout(
        Duration::from_secs(10),
        https_get_health(addr, &ca.cert_pem),
    )
    .await
    .expect("HTTPS request did not time out")
    .expect("HTTPS handshake + GET /health succeeds");
    assert_eq!(status, 200, "expected HTTP 200 OK from /health");

    let _ = std::fs::remove_file(cert_path);
    let _ = std::fs::remove_file(key_path);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn mtls_handshake_with_valid_client_cert_succeeds() {
    let ca = generate_ca("rust-adk-mtls-test-ca");
    let server_leaf = issue_leaf(
        &ca,
        "localhost",
        vec!["localhost".to_string(), "127.0.0.1".to_string()],
        true,
    );
    let client_leaf = issue_leaf(&ca, "test-client", vec!["test-client".to_string()], false);

    let cert_path = write_temp("mtls-server.crt", &server_leaf.cert_pem);
    let key_path = write_temp("mtls-server.key", &server_leaf.key_pem);
    let ca_path = write_temp("mtls-ca.crt", &ca.cert_pem);

    let tls = TlsConfig {
        enable: true,
        cert_path: cert_path.to_string_lossy().into_owned(),
        key_path: key_path.to_string_lossy().into_owned(),
        client_ca_path: Some(ca_path.to_string_lossy().into_owned()),
    };

    let port = allocate_port();
    let addr: SocketAddr = format!("127.0.0.1:{port}").parse().unwrap();
    spawn_server(tls, addr);

    let result = tokio::time::timeout(
        Duration::from_secs(10),
        mtls_handshake(
            addr,
            &ca.cert_pem,
            Some(&client_leaf.cert_pem),
            Some(&client_leaf.key_pem),
        ),
    )
    .await
    .expect("mTLS handshake did not time out");
    assert!(
        result.is_ok(),
        "mTLS handshake with valid client cert failed: {:?}",
        result.err()
    );

    let _ = std::fs::remove_file(cert_path);
    let _ = std::fs::remove_file(key_path);
    let _ = std::fs::remove_file(ca_path);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn mtls_handshake_without_client_cert_is_rejected() {
    let ca = generate_ca("rust-adk-mtls-test-ca-2");
    let server_leaf = issue_leaf(
        &ca,
        "localhost",
        vec!["localhost".to_string(), "127.0.0.1".to_string()],
        true,
    );

    let cert_path = write_temp("mtls-server2.crt", &server_leaf.cert_pem);
    let key_path = write_temp("mtls-server2.key", &server_leaf.key_pem);
    let ca_path = write_temp("mtls-ca2.crt", &ca.cert_pem);

    let tls = TlsConfig {
        enable: true,
        cert_path: cert_path.to_string_lossy().into_owned(),
        key_path: key_path.to_string_lossy().into_owned(),
        client_ca_path: Some(ca_path.to_string_lossy().into_owned()),
    };

    let port = allocate_port();
    let addr: SocketAddr = format!("127.0.0.1:{port}").parse().unwrap();
    spawn_server(tls, addr);

    // No client cert: rustls server-side enforcement should kill the
    // handshake with a TLS alert. We accept either an explicit error
    // from `mtls_handshake` or a non-zero `write/read` failure surfaced
    // as a transport error.
    let result = tokio::time::timeout(
        Duration::from_secs(10),
        mtls_handshake(addr, &ca.cert_pem, None, None),
    )
    .await
    .expect("mTLS reject did not time out");
    assert!(
        result.is_err(),
        "mTLS handshake without client cert was accepted - expected rejection",
    );

    let _ = std::fs::remove_file(cert_path);
    let _ = std::fs::remove_file(key_path);
    let _ = std::fs::remove_file(ca_path);
}

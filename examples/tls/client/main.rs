//! TLS / mTLS-enabled A2A client.
//!
//! Calls `GET /health` and `GET /.well-known/agent.json` over HTTPS
//! against the example server, trusting only the example CA and (when
//! mTLS is on) presenting the example client certificate.
//!
//! Env vars:
//!
//! - `SERVER_URL` (default `https://localhost:8443`)
//! - `TLS_CA_PATH` (default `examples/tls/certs/ca.crt`) - the CA the
//!   client trusts. Must match the CA that signed the server cert.
//! - `TLS_CLIENT_CERT_PATH` / `TLS_CLIENT_KEY_PATH` - optional; when
//!   both are set the client presents an mTLS identity. Use these
//!   against a server started with `SERVER_TLS_CLIENT_CA_PATH=…`.
//!
//! The client uses `reqwest` with the `rustls-tls` feature (configured
//! at runtime via `reqwest::ClientBuilder::add_root_certificate(...)`
//! and `.identity(...)`). The crate's default `reqwest` features wire
//! `native-tls`; this example builds its own minimal HTTPS client to
//! keep the dependency surface unchanged.

use std::env;
use std::fs;
use std::time::Duration;

use anyhow::{Context, Result};
use reqwest::Certificate;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();

    let server_url =
        env::var("SERVER_URL").unwrap_or_else(|_| "https://localhost:8443".to_string());
    let ca_path =
        env::var("TLS_CA_PATH").unwrap_or_else(|_| "examples/tls/certs/ca.crt".to_string());

    info!("TLS A2A client targeting {server_url}");
    info!("Trusting CA at {ca_path}");

    let ca_bytes = fs::read(&ca_path).with_context(|| format!("read CA from {ca_path}"))?;
    let ca = Certificate::from_pem(&ca_bytes).context("parse CA PEM")?;

    let mut builder = reqwest::Client::builder()
        .add_root_certificate(ca)
        .timeout(Duration::from_secs(10));

    let client_cert = env::var("TLS_CLIENT_CERT_PATH").ok();
    let client_key = env::var("TLS_CLIENT_KEY_PATH").ok();
    let mtls = match (client_cert.as_deref(), client_key.as_deref()) {
        (Some(cert), Some(key)) => {
            info!("Presenting mTLS identity: cert={cert}, key={key}");
            let cert_pem = fs::read(cert).with_context(|| format!("read client cert {cert}"))?;
            let key_pem = fs::read(key).with_context(|| format!("read client key {key}"))?;
            // reqwest's Identity::from_pem expects the cert + key
            // concatenated in a single buffer.
            let mut identity_pem = cert_pem.clone();
            identity_pem.extend_from_slice(b"\n");
            identity_pem.extend_from_slice(&key_pem);
            let identity =
                reqwest::Identity::from_pem(&identity_pem).context("build mTLS Identity")?;
            builder = builder.identity(identity);
            true
        }
        _ => false,
    };

    let client = builder.build().context("build reqwest client")?;

    // GET /health
    let health_url = format!("{}/health", server_url.trim_end_matches('/'));
    match client.get(&health_url).send().await {
        Ok(response) => {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            info!("GET /health → HTTP {status}: {body}");
        }
        Err(e) => {
            error!("GET /health failed: {e}");
            if !mtls {
                error!(
                    "Hint: if the server is in mTLS mode, set TLS_CLIENT_CERT_PATH + TLS_CLIENT_KEY_PATH",
                );
            }
            return Err(e.into());
        }
    }

    // GET /.well-known/agent.json
    let card_url = format!(
        "{}/.well-known/agent.json",
        server_url.trim_end_matches('/')
    );
    match client.get(&card_url).send().await {
        Ok(response) => {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            info!("GET /.well-known/agent.json → HTTP {status}: {body}");
        }
        Err(e) => {
            error!("GET /.well-known/agent.json failed: {e}");
            return Err(e.into());
        }
    }

    Ok(())
}

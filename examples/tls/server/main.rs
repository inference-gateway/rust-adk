//! TLS / mTLS-enabled A2A server.
//!
//! Demonstrates wiring `TlsConfig` through `A2AServerBuilder` so the A2A
//! endpoint terminates TLS and (optionally) requires every client to
//! present a certificate signed by a trusted CA (mutual TLS).
//!
//! Two modes share this binary:
//!
//! 1. **TLS only** (the default for `cargo run`): the server listens on
//!    HTTPS using the cert/key pair under `examples/tls/certs/`. Any
//!    client that trusts the example CA can connect; no client
//!    certificate is required.
//!
//! 2. **Mutual TLS**: set `SERVER_TLS_CLIENT_CA_PATH=…/ca.crt` (the same
//!    file the server uses for its own chain in this example) before
//!    starting the binary. The handshake now requires the client to
//!    present a certificate signed by that CA, and the client-cert
//!    subject is plumbed through to handlers as a
//!    [`PeerCert`] / [`ClientCertPrincipal`] extension.
//!
//! All TLS knobs are picked up via `envy::prefixed("A2A_").from_env::<Config>()`:
//!
//! - `A2A_SERVER_TLS_ENABLE=true`
//! - `A2A_SERVER_TLS_CERT_PATH=/path/to/server.crt`
//! - `A2A_SERVER_TLS_KEY_PATH=/path/to/server.key`
//! - `A2A_SERVER_TLS_CLIENT_CA_PATH=/path/to/ca.crt` (optional, mTLS only)
//!
//! Generate the example PKI by running `make-certs.sh` next to this
//! file, which uses `openssl` to mint a CA, a server leaf with
//! `subjectAltName=DNS:localhost,IP:127.0.0.1`, and a client leaf with
//! `CN=demo-client`.

use std::net::SocketAddr;

use inference_gateway_adk::{A2AServerBuilder, Config, a2a_types::AgentCard};
use serde_json::json;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();
    dotenvy::dotenv().ok();

    let config: Config = envy::prefixed("A2A_").from_env()?;
    if !config.tls_config.enable {
        return Err("A2A_SERVER_TLS_ENABLE=true is required to run the tls example".into());
    }
    let tls = &config.tls_config;

    let port = config.server_config.port;
    let addr: SocketAddr = format!("0.0.0.0:{port}").parse()?;

    let agent_card: AgentCard = serde_json::from_value(json!({
        "name": "TLS-Gated Rust A2A Agent",
        "description": "Example showing TlsConfig + (optional) mTLS enforcement",
        "version": "0.1.0",
        "protocolVersion": "0.2.6",
        "url": format!("https://localhost:{port}/a2a"),
        "preferredTransport": "JSONRPC",
        "capabilities": {
            "streaming": true,
            "pushNotifications": false,
            "stateTransitionHistory": false
        },
        "defaultInputModes": ["text/plain"],
        "defaultOutputModes": ["text/plain"],
        "skills": [
            {
                "id": "echo",
                "name": "echo",
                "description": "Echo back user messages",
                "tags": ["echo", "tls"]
            }
        ]
    }))?;

    let mtls = tls.client_ca_path.is_some();

    let server = A2AServerBuilder::new()
        .with_config(config.clone())
        .with_agent_card(agent_card)
        .with_default_task_handlers()
        .build()
        .await?;

    info!(
        cert = %tls.cert_path,
        key = %tls.key_path,
        client_ca = ?tls.client_ca_path,
        "TLS-enabled A2A server listening on https://localhost:{port} (mtls={mtls})"
    );
    info!("Try:");
    info!("  curl --cacert examples/tls/certs/ca.crt https://localhost:{port}/health");
    if mtls {
        info!(
            "  curl --cacert examples/tls/certs/ca.crt --cert examples/tls/certs/client.crt --key examples/tls/certs/client.key https://localhost:{port}/.well-known/agent.json"
        );
    } else {
        info!(
            "  curl --cacert examples/tls/certs/ca.crt https://localhost:{port}/.well-known/agent.json"
        );
    }

    if let Err(e) = server.serve(addr).await {
        error!("Server failed to start: {}", e);
    }

    Ok(())
}

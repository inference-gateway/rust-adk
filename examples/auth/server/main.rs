//! Auth-enabled A2A server.
//!
//! Demonstrates how to gate `POST /a2a` behind a bearer-token verifier
//! while keeping `GET /health` and `GET /.well-known/agent.json` public.
//!
//! Two modes share this binary:
//!
//! 1. **Static-token quick path** (default for `cargo run`): a tiny
//!    in-process [`AuthVerifier`] accepts one hard-coded token (set
//!    via `EXAMPLE_BEARER_TOKEN`). Zero external dependencies.
//! 2. **OIDC mode** (used by `docker-compose.yaml` against a Keycloak
//!    realm): set `A2A_AUTH_ENABLE=true`, `A2A_AUTH_ISSUER_URL=...`, and
//!    `A2A_AUTH_CLIENT_ID=...`. The example loads those through
//!    `envy::prefixed("A2A_").from_env::<Config>()` and the builder
//!    instantiates the bundled [`OidcJwtVerifier`] which does OIDC
//!    discovery + JWKS validation. No code changes needed - the env
//!    vars are enough.

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use inference_gateway_adk::{
    A2AServerBuilder, AuthError, AuthVerifier, AuthenticatedPrincipal, Config, a2a_types::AgentCard,
};
use serde_json::{Value, json};
use tracing::{error, info};

/// Static-token verifier for the zero-deps `cargo run` demo.
#[derive(Debug)]
struct StaticTokenVerifier {
    expected: String,
}

#[async_trait]
impl AuthVerifier for StaticTokenVerifier {
    async fn verify(&self, token: &str) -> Result<AuthenticatedPrincipal, AuthError> {
        if token == self.expected {
            let mut claims = HashMap::new();
            claims.insert("sub".to_string(), Value::String("demo-user".to_string()));
            claims.insert(
                "tenant".to_string(),
                Value::String("demo-tenant".to_string()),
            );
            Ok(AuthenticatedPrincipal {
                subject: "demo-user".to_string(),
                tenant: "demo-tenant".to_string(),
                issuer: "https://example.test".to_string(),
                claims,
            })
        } else {
            Err(AuthError::InvalidToken("token did not match".to_string()))
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    let config: Config = envy::prefixed("A2A_").from_env()?;
    let auth_enabled = config.auth_config.enable;
    let port = config.server_config.port;

    let agent_card: AgentCard = serde_json::from_value(json!({
        "name": "Auth-Gated Rust A2A Agent",
        "description": "Example showing AuthConfig + bearer token enforcement",
        "version": "0.1.0",
        "protocolVersion": "0.2.6",
        "url": format!("http://localhost:{port}/a2a"),
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
                "tags": ["echo", "auth"]
            }
        ],
        // Required for agent/getAuthenticatedExtendedCard to return the
        // card rather than -32601 METHOD_NOT_FOUND.
        "supportsExtendedAgentCard": true
    }))?;

    let mut builder = A2AServerBuilder::new()
        .with_agent_card(agent_card)
        .with_default_task_handlers();

    if auth_enabled {
        // OIDC mode: hand the builder the Config we already loaded; it
        // constructs OidcJwtVerifier from `auth_config.issuer_url` /
        // `auth_config.client_id`.
        info!(
            "A2A_AUTH_ENABLE=true → using OidcJwtVerifier (issuer={})",
            config.auth_config.issuer_url
        );
        builder = builder.with_config(config);
    } else {
        // EXAMPLE_BEARER_TOKEN is an example-specific demo knob — it's not
        // part of the ADK's config surface, so we read it directly.
        let bearer_token =
            std::env::var("EXAMPLE_BEARER_TOKEN").unwrap_or_else(|_| "demo-token-123".to_string());
        info!(
            "A2A_AUTH_ENABLE not set → using StaticTokenVerifier (expected token: {bearer_token})"
        );
        let verifier: Arc<dyn AuthVerifier> = Arc::new(StaticTokenVerifier {
            expected: bearer_token,
        });
        builder = builder.with_auth_verifier(verifier);
    }

    let server = builder.build().await?;

    let addr = format!("0.0.0.0:{port}").parse()?;
    info!("Auth-gated A2A server listening on port {port}");
    info!("Try:");
    info!("  curl http://localhost:{port}/health                               # public");
    info!("  curl http://localhost:{port}/.well-known/agent.json               # public");
    if auth_enabled {
        info!(
            "  curl -H 'Authorization: Bearer <jwt>' http://localhost:{port}/a2a -d '...'  # protected (JWT from Keycloak)"
        );
    } else {
        info!(
            "  curl -H 'Authorization: Bearer <token>' http://localhost:{port}/a2a -d '...'  # protected (static token)"
        );
    }

    if let Err(e) = server.serve(addr).await {
        error!("Server failed to start: {}", e);
    }
    Ok(())
}

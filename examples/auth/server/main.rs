//! Auth-enabled A2A server.
//!
//! Demonstrates how to gate `POST /a2a` behind a bearer-token verifier
//! while keeping `GET /health` and `GET /.well-known/agent.json` public.
//!
//! The example uses a tiny in-process [`AuthVerifier`] that accepts one
//! hard-coded token (set via `EXAMPLE_BEARER_TOKEN`) so the demo runs
//! without spinning up an OIDC issuer. In production you would either:
//!
//! 1. Set `AUTH_ENABLE=true` + `AUTH_ISSUER_URL=...` and let
//!    [`OidcJwtVerifier`] do OIDC discovery + JWKS validation
//!    automatically (no code changes needed - the builder picks it up
//!    from `Config::from_env()`).
//! 2. Or implement [`AuthVerifier`] yourself for a custom backend
//!    (static keys, HMAC tokens, internal identity service, ...) and
//!    plug it in via `with_auth_verifier(...)` as this example does.

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use inference_gateway_adk::{
    A2AServerBuilder, AuthError, AuthVerifier, AuthenticatedPrincipal, a2a_types::AgentCard,
};
use serde_json::{Value, json};
use tracing::{error, info};

/// Static-token verifier - swap for `OidcJwtVerifier::from_config(&cfg)`
/// in real deployments.
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

    let bearer_token =
        std::env::var("EXAMPLE_BEARER_TOKEN").unwrap_or_else(|_| "demo-token-123".to_string());

    let agent_card: AgentCard = serde_json::from_value(json!({
        "name": "Auth-Gated Rust A2A Agent",
        "description": "Example showing AuthConfig + bearer token enforcement",
        "version": "0.1.0",
        "protocolVersion": "0.2.6",
        "url": "http://localhost:8081/a2a",
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

    let verifier: Arc<dyn AuthVerifier> = Arc::new(StaticTokenVerifier {
        expected: bearer_token.clone(),
    });

    let server = A2AServerBuilder::new()
        .with_agent_card(agent_card)
        .with_gateway_url("http://localhost:8080/v1")
        .with_default_task_handlers()
        .with_auth_verifier(verifier)
        .build()
        .await?;

    let addr = "0.0.0.0:8081".parse()?;
    info!("Auth-gated A2A server listening on port 8081");
    info!("Expected bearer token: {bearer_token}");
    info!("Try:");
    info!("  curl http://localhost:8081/health                               # public");
    info!("  curl http://localhost:8081/.well-known/agent.json               # public");
    info!(
        "  curl -H 'Authorization: Bearer {bearer_token}' http://localhost:8081/a2a -d '...'  # protected"
    );

    if let Err(e) = server.serve(addr).await {
        error!("Server failed to start: {}", e);
    }
    Ok(())
}

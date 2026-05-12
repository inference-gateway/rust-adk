//! Client for the auth-enabled example server.
//!
//! Demonstrates that `GET /health` and `GET /.well-known/agent.json` are
//! reachable without a token, while `POST /a2a` requires a bearer token.
//! The `agent/getAuthenticatedExtendedCard` call is executed both with
//! and without a token to show the contrast.
//!
//! Two modes share this binary:
//!
//! 1. **Static-token quick path** (default): reuses `EXAMPLE_BEARER_TOKEN`
//!    so it can talk to a server running with the static-token verifier.
//! 2. **OIDC mode** (`AUTH_MODE=oidc`): performs an OAuth2
//!    `client_credentials` exchange against `AUTH_TOKEN_URL` using
//!    `AUTH_CLIENT_ID` / `AUTH_CLIENT_SECRET`, then uses the returned
//!    JWT to call the protected endpoint. This is what
//!    `docker-compose.yaml` wires up against Keycloak.

use std::env;

use inference_gateway_adk::A2AClient;
use serde::Deserialize;
use serde_json::{Value, json};
use tracing::{error, info};

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
}

async fn fetch_oidc_token(http: &reqwest::Client) -> Result<String, Box<dyn std::error::Error>> {
    let token_url =
        env::var("AUTH_TOKEN_URL").map_err(|_| "AUTH_TOKEN_URL is required when AUTH_MODE=oidc")?;
    let client_id =
        env::var("AUTH_CLIENT_ID").map_err(|_| "AUTH_CLIENT_ID is required when AUTH_MODE=oidc")?;
    let client_secret = env::var("AUTH_CLIENT_SECRET")
        .map_err(|_| "AUTH_CLIENT_SECRET is required when AUTH_MODE=oidc")?;

    info!("→ fetching client_credentials JWT from {token_url}");
    let response = http
        .post(&token_url)
        .form(&[
            ("grant_type", "client_credentials"),
            ("client_id", client_id.as_str()),
            ("client_secret", client_secret.as_str()),
        ])
        .send()
        .await?
        .error_for_status()?;
    let token: TokenResponse = response.json().await?;
    info!(
        "← received access_token ({} chars)",
        token.access_token.len()
    );
    Ok(token.access_token)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    let server_url = env::var("SERVER_URL").unwrap_or_else(|_| "http://localhost:8081".to_string());
    let auth_mode = env::var("AUTH_MODE").unwrap_or_default().to_lowercase();

    info!("Auth-example client connecting to {server_url} (mode={auth_mode:?})");

    // --- Public endpoints: no token needed -----------------------------

    let public_client = A2AClient::new(&server_url)?;
    match public_client.get_health().await {
        Ok(health) => info!("Health (public, no token): {}", health.status),
        Err(e) => error!("Health check failed: {e}"),
    }
    match public_client.get_agent_card().await {
        Ok(card) => info!(
            "Agent card (public, no token): {} v{}",
            card.name, card.version
        ),
        Err(e) => error!("Agent card fetch failed: {e}"),
    }

    // --- Acquire a bearer token -----------------------------------------

    let http = reqwest::Client::new();
    let bearer_token = if auth_mode == "oidc" {
        fetch_oidc_token(&http).await?
    } else {
        env::var("EXAMPLE_BEARER_TOKEN").unwrap_or_else(|_| "demo-token-123".to_string())
    };

    // --- Protected endpoint: no token → expect HTTP 401 ---------------

    let rpc_url = format!("{server_url}/a2a");
    let rpc_body = json!({
        "jsonrpc": "2.0",
        "id": "example-1",
        "method": "agent/getAuthenticatedExtendedCard",
        "params": { "tenant": "demo-tenant" }
    });

    info!("→ calling agent/getAuthenticatedExtendedCard WITHOUT a token");
    let no_token_response = http.post(&rpc_url).json(&rpc_body).send().await?;
    info!(
        "← server replied {} (expected 401)",
        no_token_response.status()
    );

    // --- Protected endpoint: valid token → expect 200 + the card ------

    info!("→ calling agent/getAuthenticatedExtendedCard WITH bearer token");
    let response = http
        .post(&rpc_url)
        .bearer_auth(&bearer_token)
        .json(&rpc_body)
        .send()
        .await?;
    let status = response.status();
    let body: Value = response.json().await?;
    info!("← server replied {status}");
    if let Some(result) = body.get("result") {
        info!(
            "extended agent card: name={} version={}",
            result.get("name").and_then(|v| v.as_str()).unwrap_or("?"),
            result
                .get("version")
                .and_then(|v| v.as_str())
                .unwrap_or("?"),
        );
    } else {
        error!("expected `result` in JSON-RPC response, got: {body}");
    }

    Ok(())
}

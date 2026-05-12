//! Client for the auth-enabled example server.
//!
//! Demonstrates that `GET /health` and `GET /.well-known/agent.json` are
//! reachable without a token, while `POST /a2a` requires the bearer
//! token configured on the server. The `agent/getAuthenticatedExtendedCard`
//! call is executed both with and without a token to show the contrast.

use std::env;

use inference_gateway_adk::A2AClient;
use serde_json::{Value, json};
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    let server_url = env::var("SERVER_URL").unwrap_or_else(|_| "http://localhost:8081".to_string());
    let bearer_token =
        env::var("EXAMPLE_BEARER_TOKEN").unwrap_or_else(|_| "demo-token-123".to_string());

    info!("Auth-example client connecting to {server_url}");

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

    // --- Protected endpoint: no token → expect HTTP 401 ---------------

    let http = reqwest::Client::new();
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

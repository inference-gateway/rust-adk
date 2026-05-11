//! Shared server for the `a2a-methods` example collection.
//!
//! Hosts the JSON-RPC dispatch surface backed by an in-memory storage, with
//! no LLM agent configured. Replies to `message/send` and `message/stream`
//! fall back to a deterministic echo (because no agent is wired up) which
//! keeps the per-method client examples reproducible without external
//! dependencies.

use inference_gateway_adk::A2AServerBuilder;
use inference_gateway_adk::a2a_types::AgentCard;
use serde_json::json;
use std::env;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    let port: u16 = env::var("SERVER_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8085);

    let agent_card: AgentCard = serde_json::from_value(json!({
        "name": "A2A Methods Example Agent",
        "description": "Reference server used by the per-method A2A client examples",
        "version": "0.1.0",
        "protocolVersion": "0.2.6",
        "url": format!("http://localhost:{port}/a2a"),
        "preferredTransport": "JSONRPC",
        "capabilities": {
            "streaming": true,
            "pushNotifications": true,
            "stateTransitionHistory": false
        },
        "defaultInputModes": ["text/plain"],
        "defaultOutputModes": ["text/plain"],
        "skills": [
            {
                "id": "echo",
                "name": "echo",
                "description": "Echoes user input — sufficient for exercising every JSON-RPC method.",
                "tags": ["echo", "examples"]
            }
        ]
    }))?;

    let server = A2AServerBuilder::new()
        .with_agent_card(agent_card)
        // The gateway URL is required by the builder but is intentionally
        // unreachable: the per-method examples never trigger an LLM call,
        // so the server's offline echo path handles `message/send` and
        // `message/stream` end-to-end.
        .with_gateway_url("http://127.0.0.1:1/v1")
        .build()
        .await?;

    let addr = format!("0.0.0.0:{port}").parse()?;
    info!("a2a-methods example server listening on port {port}");
    info!("JSON-RPC endpoint: http://localhost:{port}/a2a");

    if let Err(e) = server.serve(addr).await {
        error!("server failed: {e}");
    }

    Ok(())
}

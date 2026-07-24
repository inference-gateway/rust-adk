use inference_gateway_adk::A2AServerBuilder;
use inference_gateway_adk::a2a_types::AgentCard;
use inference_gateway_adk::{Config, telemetry};
use serde_json::json;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = envy::prefixed("A2A_")
        .from_env::<Config>()
        .unwrap_or_default();
    // Bound with `let _guard` so batched spans flush on SIGINT (needs
    // `--features telemetry` on the ADK crate to actually export).
    let _guard = telemetry::init(
        &config.telemetry_config,
        "minimal-server",
        env!("CARGO_PKG_VERSION"),
    )?;

    let agent_card: AgentCard = serde_json::from_value(json!({
        "name": "Minimal Rust A2A Agent",
        "description": "A minimal A2A server built with the Rust ADK",
        "version": "0.1.0",
        "protocolVersion": "0.2.6",
        "url": "http://server:8080",
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
                "tags": ["echo", "minimal"]
            }
        ]
    }))?;

    let server = A2AServerBuilder::new()
        .with_agent_card(agent_card)
        .with_gateway_url("http://gateway:8080/v1")
        .with_default_task_handlers()
        .build()
        .await?;

    let addr = "0.0.0.0:8080".parse()?;
    info!("Minimal A2A server with Inference Gateway SDK running on port 8080");
    info!("Using Inference Gateway at: http://gateway:8080/v1");

    if let Err(e) = server.serve(addr).await {
        error!("Server failed to start: {}", e);
    }

    Ok(())
}

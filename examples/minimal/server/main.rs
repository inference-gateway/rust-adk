use inference_gateway_adk::A2AServerBuilder;
use inference_gateway_adk::a2a_types::AgentCard;
use serde_json::json;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    let agent_card: AgentCard = serde_json::from_value(json!({
        "name": "Minimal Rust A2A Agent",
        "description": "A minimal A2A server built with the Rust ADK",
        "version": "0.1.0",
        "protocolVersion": "0.2.6",
        "url": "http://localhost:8081",
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
        .with_gateway_url("http://localhost:8080/v1")
        .with_default_task_handlers()
        .build()
        .await?;

    let addr = "0.0.0.0:8081".parse()?;
    info!("Minimal A2A server with Inference Gateway SDK running on port 8081");
    info!("Using Inference Gateway at: http://localhost:8080/v1");

    if let Err(e) = server.serve(addr).await {
        error!("Server failed to start: {}", e);
    }

    Ok(())
}

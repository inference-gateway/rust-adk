use inference_gateway_adk::A2AClient;
use serde_json::json;
use std::env;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    let server_url =
        env::var("SERVER_URL").unwrap_or_else(|_| "http://localhost:8081".to_string());
    let client = A2AClient::new(&server_url)?;

    info!("Static-agent-card A2A client connecting to {}", server_url);

    match client.get_health().await {
        Ok(health) => {
            info!("Health check successful: {}", health.status);
            if let Some(details) = &health.details {
                info!("Health details: {}", serde_json::to_string_pretty(details)?);
            }
        }
        Err(e) => {
            error!("Health check failed: {}", e);
            return Ok(());
        }
    }

    match client.get_agent_card().await {
        Ok(agent_card) => {
            info!("Agent card retrieved successfully");
            info!("Agent: {} - {}", agent_card.name, agent_card.description);
            info!(
                "Protocol version: {}, Transport: {:?}",
                agent_card.protocol_version, agent_card.preferred_transport
            );
        }
        Err(e) => {
            error!("Failed to get agent card: {}", e);
        }
    }

    let task_params = json!({
        "jsonrpc": "2.0",
        "id": "static-card-001",
        "method": "generate_content",
        "params": {
            "messages": [
                {
                    "role": "user",
                    "content": "Hello! I'm testing the A2A server with a JSON-loaded agent card."
                }
            ]
        }
    });

    match client.send_task(task_params).await {
        Ok(response) => {
            info!("A2A task sent successfully");
            info!("Response: {}", serde_json::to_string_pretty(&response)?);
        }
        Err(e) => {
            error!("Failed to send A2A task: {}", e);
        }
    }

    Ok(())
}

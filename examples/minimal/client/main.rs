use inference_gateway_adk::A2AClient;
use serde_json::json;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    let client = A2AClient::new("http://localhost:8081")?;

    info!("Minimal A2A client connecting to http://localhost:8081");

    match client.get_health().await {
        Ok(health) => {
            info!("Health check successful: {}", health.status);
            info!("Server timestamp: {}", health.timestamp);
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
        }
        Err(e) => {
            error!("Failed to get agent card: {}", e);
        }
    }

    let task_params = json!({
        "jsonrpc": "2.0",
        "id": "minimal-001",
        "method": "generate_content",
        "params": {
            "messages": [
                {
                    "role": "user",
                    "content": "Hello! Testing the minimal A2A server."
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

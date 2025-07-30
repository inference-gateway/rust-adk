use inference_gateway_adk::A2AClient;
use serde_json::json;
use tokio::time::{Duration, sleep};
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt().init();

    // Create A2A client (connects to A2A server that uses SDK internally)
    let client = A2AClient::new("http://localhost:8081")?; // A2A server port

    info!("A2A Client starting with SDK integration...");
    info!("Connecting to A2A server at: http://localhost:8081");
    info!("A2A server uses Inference Gateway SDK at: http://localhost:8080/v1");

    // Test health check
    match client.get_health().await {
        Ok(health) => {
            info!("Health check successful: {}", health.status);
            info!("Server timestamp: {}", health.timestamp);
            if let Some(details) = &health.details {
                info!("Health details: {}", serde_json::to_string_pretty(details)?);
            }
        }
        Err(e) => {
            error!("Health check failed: {}", e);
            return Ok(());
        }
    }

    // Test agent card retrieval
    match client.get_agent_card().await {
        Ok(agent_card) => {
            info!("Agent card retrieved successfully");
            info!("Agent: {} - {}", agent_card.name, agent_card.description);
            info!(
                "Protocol version: {}, Transport: {}",
                agent_card.protocol_version, agent_card.preferred_transport
            );
        }
        Err(e) => {
            error!("Failed to get agent card: {}", e);
        }
    }

    // Test A2A task sending with JSON-RPC format
    let task_params = json!({
        "jsonrpc": "2.0",
        "id": "test-001",
        "method": "generate_content",
        "params": {
            "messages": [
                {
                    "role": "user",
                    "content": "Hello! I'm testing the A2A server with Inference Gateway SDK integration. Can you respond?"
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

    // Test streaming task
    let streaming_params = json!({
        "jsonrpc": "2.0",
        "id": "stream-001",
        "method": "generate_content",
        "params": {
            "messages": [
                {
                    "role": "user",
                    "content": "Tell me about the benefits of using the Inference Gateway SDK for A2A communication."
                }
            ]
        }
    });

    match client
        .send_task_streaming(streaming_params, |event| {
            info!(
                "Streaming event received: {}",
                serde_json::to_string_pretty(&event)?
            );
            Ok(())
        })
        .await
    {
        Ok(_) => {
            info!("Streaming task completed successfully");
        }
        Err(e) => {
            error!("Streaming task failed: {}", e);
        }
    }

    // Periodic health monitoring example
    info!("Starting periodic health monitoring (3 checks)...");

    for i in 1..=3 {
        sleep(Duration::from_secs(3)).await;

        match client.get_health().await {
            Ok(health) => {
                info!(
                    "[Check {}] A2A Server status: {} at {}",
                    i,
                    health.status,
                    health.timestamp.format("%H:%M:%S")
                );

                if let Some(details) = &health.details {
                    if let Some(gateway_healthy) = details.get("gateway_healthy") {
                        info!(
                            "[Check {}] Inference Gateway healthy: {}",
                            i, gateway_healthy
                        );
                    }
                }
            }
            Err(e) => {
                error!("[Check {}] Health check failed: {}", i, e);
            }
        }
    }

    info!("A2A Client with SDK integration demo completed");
    Ok(())
}

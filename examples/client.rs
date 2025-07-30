use inference_gateway_adk::A2AClient;
use serde_json::json;
use tokio::time::{sleep, Duration};
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt().init();

    // Create client
    let client = A2AClient::new("http://localhost:8080")?;

    info!("A2A Client starting...");

    // Test health check
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

    // Test agent card retrieval
    match client.get_agent_card().await {
        Ok(_agent_card) => {
            info!("Agent card retrieved successfully");
            // Note: AgentCard is a complex generated type, so we'll just log success
        }
        Err(e) => {
            error!("Failed to get agent card: {}", e);
        }
    }

    // Test task sending
    let task_params = json!({
        "message": "Hello from A2A client!",
        "type": "test"
    });

    match client.send_task(task_params).await {
        Ok(response) => {
            info!("Task sent successfully");
            info!("Response: {}", serde_json::to_string_pretty(&response)?);
        }
        Err(e) => {
            error!("Failed to send task: {}", e);
        }
    }

    // Test streaming task (with simple event handler)
    let streaming_params = json!({
        "message": "Hello streaming!",
        "type": "streaming_test"
    });

    match client.send_task_streaming(streaming_params, |event| {
        info!("Streaming event received: {}", serde_json::to_string_pretty(&event)?);
        Ok(())
    }).await {
        Ok(_) => {
            info!("Streaming task completed successfully");
        }
        Err(e) => {
            error!("Streaming task failed: {}", e);
        }
    }

    // Periodic health monitoring example
    info!("Starting periodic health monitoring (press Ctrl+C to stop)...");
    
    for i in 1..=3 {
        sleep(Duration::from_secs(5)).await;
        
        match client.get_health().await {
            Ok(health) => {
                info!("[Check {}] Agent status: {} at {}", i, health.status, health.timestamp.format("%H:%M:%S"));
            }
            Err(e) => {
                error!("[Check {}] Health check failed: {}", i, e);
            }
        }
    }

    info!("Client demo completed");
    Ok(())
}
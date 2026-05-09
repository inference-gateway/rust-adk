use inference_gateway_adk::A2AClient;
use serde_json::json;
use std::env;
use tokio::time::{Duration, sleep};
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    let server_url = env::var("SERVER_URL").unwrap_or_else(|_| "http://localhost:8082".to_string());
    let client = A2AClient::new(&server_url)?;

    info!("Toolbox A2A client connecting to {}", server_url);

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
        "id": "toolbox-001",
        "method": "generate_content",
        "params": {
            "messages": [
                {
                    "role": "user",
                    "content": "What's the weather in San Francisco, and what is 12 * 7?"
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

    let streaming_params = json!({
        "jsonrpc": "2.0",
        "id": "toolbox-stream-001",
        "method": "generate_content",
        "params": {
            "messages": [
                {
                    "role": "user",
                    "content": "Search the web for 'Rust async programming' and summarise the top results."
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

                if let Some(details) = &health.details
                    && let Some(gateway_healthy) = details.get("gateway_healthy")
                {
                    info!(
                        "[Check {}] Inference Gateway healthy: {}",
                        i, gateway_healthy
                    );
                }
            }
            Err(e) => {
                error!("[Check {}] Health check failed: {}", i, e);
            }
        }
    }

    info!("Toolbox client demo completed");
    Ok(())
}

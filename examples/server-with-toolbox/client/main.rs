use futures::StreamExt;
use inference_gateway_adk::A2AClient;
use inference_gateway_adk::a2a_types::{Message, Part, Role, SendMessageRequest};
use std::env;
use tokio::time::{Duration, sleep};
use tracing::{error, info};
use uuid::Uuid;

fn user_message(text: &str) -> SendMessageRequest {
    SendMessageRequest {
        configuration: None,
        message: Some(Message {
            context_id: None,
            extensions: vec![],
            message_id: Uuid::new_v4().to_string(),
            metadata: None,
            parts: vec![Part {
                data: None,
                file: None,
                metadata: None,
                text: Some(text.to_string()),
            }],
            reference_task_ids: vec![],
            role: Role::RoleUser,
            task_id: None,
        }),
        metadata: None,
        tenant: "server-with-toolbox".to_string(),
    }
}

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

    // message/send — dispatch a single tool-use prompt and print the reply.
    match client
        .send_message(user_message(
            "What's the weather in San Francisco, and what is 12 * 7?",
        ))
        .await
    {
        Ok(response) => {
            info!("message/send dispatched");
            if let Some(task) = response.task {
                info!("→ task {} in state {:?}", task.id, task.status.state);
                if let Some(msg) = task.status.message {
                    let text = msg
                        .parts
                        .iter()
                        .filter_map(|p| p.text.clone())
                        .collect::<Vec<_>>()
                        .join("");
                    info!("→ agent reply: {}", text);
                }
            }
        }
        Err(e) => {
            error!("Failed to dispatch message/send: {}", e);
        }
    }

    // message/stream — observe state transitions and incremental artifacts.
    match client
        .stream_message(user_message(
            "Search the web for 'Rust async programming' and summarise the top results.",
        ))
        .await
    {
        Ok(stream) => {
            let mut stream = Box::pin(stream);
            let mut event_index = 0usize;
            while let Some(item) = stream.next().await {
                event_index += 1;
                match item {
                    Ok(event) => {
                        if let Some(task) = event.task {
                            info!(
                                "[{event_index}] message/stream → task {} created (state {:?})",
                                task.id, task.status.state
                            );
                        }
                        if let Some(update) = event.status_update {
                            let suffix = if update.final_ { " (final)" } else { "" };
                            info!(
                                "[{event_index}] message/stream → status update: {:?}{suffix}",
                                update.status.state
                            );
                        }
                        if let Some(artifact) = event.artifact_update {
                            let text = artifact
                                .artifact
                                .parts
                                .iter()
                                .filter_map(|p| p.text.clone())
                                .collect::<Vec<_>>()
                                .join("");
                            info!("[{event_index}] message/stream → artifact: {:?}", text);
                        }
                    }
                    Err(e) => {
                        error!("[{event_index}] message/stream event failed: {}", e);
                        break;
                    }
                }
            }
            info!("message/stream closed after {event_index} events");
        }
        Err(e) => {
            error!("Failed to open message/stream: {}", e);
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

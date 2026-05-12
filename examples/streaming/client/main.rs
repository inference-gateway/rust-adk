use futures::StreamExt;
use inference_gateway_adk::A2AClient;
use inference_gateway_adk::a2a_types::{Message, Part, Role, SendMessageRequest};
use std::env;
use std::time::Instant;
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
        tenant: "streaming".to_string(),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    let server_url = env::var("SERVER_URL").unwrap_or_else(|_| "http://localhost:8086".to_string());
    let client = A2AClient::new(&server_url)?;

    info!("streaming client → {server_url}");

    let health = client.get_health().await?;
    info!("server health: {}", health.status);

    let stream = client
        .stream_message(user_message("stream me a pangram"))
        .await?;

    let start = Instant::now();
    let mut stream = Box::pin(stream);
    let mut event_index = 0usize;
    while let Some(item) = stream.next().await {
        event_index += 1;
        match item {
            Ok(event) => {
                let elapsed = start.elapsed().as_secs_f64();
                if let Some(task) = event.task {
                    info!(
                        "[{elapsed:.2}s] #{event_index} → task {} created (state {:?})",
                        task.id, task.status.state
                    );
                }
                if let Some(update) = event.status_update {
                    let suffix = if update.final_ { " (final)" } else { "" };
                    info!(
                        "[{elapsed:.2}s] #{event_index} → status: {:?}{suffix}",
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
                    let last = artifact.last_chunk.unwrap_or(false);
                    let marker = if last { " (last chunk)" } else { "" };
                    info!("[{elapsed:.2}s] #{event_index} → chunk: {text:?}{marker}");
                }
            }
            Err(e) => {
                error!("[#{event_index}] stream error: {e}");
                break;
            }
        }
    }

    info!(
        "stream closed after {event_index} events ({:.2}s total)",
        start.elapsed().as_secs_f64()
    );
    Ok(())
}

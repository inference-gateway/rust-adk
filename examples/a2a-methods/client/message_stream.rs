//! `message/stream` — open the SSE stream and observe each event live.
//!
//! The example server registers an explicit
//! [`EchoStreamHandler`](crate) that drives the task through
//! `Submitted` → `Working` → echo artifact → `Completed`. This client
//! prints each event as it arrives.
//!
//! ```bash
//! cargo run --example a2a-methods-server
//! cargo run --example a2a-methods-message-stream
//! ```

use futures::StreamExt;
use inference_gateway_adk::A2AClient;
use inference_gateway_adk::a2a_types::{Message, Part, Role, SendMessageRequest};
use std::env;
use tracing::info;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    let server_url = env::var("SERVER_URL").unwrap_or_else(|_| "http://localhost:8085".to_string());
    let client = A2AClient::new(&server_url)?;

    let request = SendMessageRequest {
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
                text: Some("Hello via message/stream".to_string()),
            }],
            reference_task_ids: vec![],
            role: Role::RoleUser,
            task_id: None,
        }),
        metadata: None,
        tenant: "example".to_string(),
    };

    let mut stream = Box::pin(client.stream_message(request).await?);

    let mut event_index = 0usize;
    while let Some(event) = stream.next().await {
        let response = event?;
        event_index += 1;

        if let Some(task) = response.task {
            info!(
                "[{event_index}] message/stream → task {} created (state {:?})",
                task.id, task.status.state
            );
        }

        if let Some(update) = response.status_update {
            let suffix = if update.final_ { " (final)" } else { "" };
            info!(
                "[{event_index}] message/stream → status update: {:?}{suffix}",
                update.status.state
            );
        }

        if let Some(artifact_event) = response.artifact_update {
            let text = artifact_event
                .artifact
                .parts
                .iter()
                .filter_map(|p| p.text.clone())
                .collect::<Vec<_>>()
                .join("");
            info!("[{event_index}] message/stream → artifact: {:?}", text);
        }
    }

    info!("message/stream → stream closed after {event_index} events");
    Ok(())
}

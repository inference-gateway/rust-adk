//! Client for the filesystem artifacts example.
//!
//! Streams a task against the artifacts example server, collects the
//! emitted file artifact's URI, then downloads the artifact directly
//! from the artifacts HTTP server and prints its contents.

use futures::StreamExt;
use inference_gateway_adk::A2AClient;
use inference_gateway_adk::a2a_types::{Message, Part, Role, SendMessageRequest};
use std::env;
use tracing::{error, info, warn};
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
        tenant: "artifacts-demo".to_string(),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    let server_url = env::var("SERVER_URL").unwrap_or_else(|_| "http://localhost:8087".to_string());
    let client = A2AClient::new(&server_url)?;

    info!("artifacts client → {server_url}");

    let stream = client
        .stream_message(user_message("generate a report"))
        .await?;
    let mut stream = Box::pin(stream);

    let mut artifact_uri: Option<String> = None;
    let mut artifact_name: Option<String> = None;
    while let Some(item) = stream.next().await {
        match item {
            Ok(event) => {
                if let Some(task) = event.task {
                    info!("task {} created (state {:?})", task.id, task.status.state);
                }
                if let Some(update) = event.status_update {
                    let suffix = if update.final_ { " (final)" } else { "" };
                    info!("status: {:?}{suffix}", update.status.state);
                }
                if let Some(update) = event.artifact_update {
                    for part in &update.artifact.parts {
                        if let Some(file_part) = part.file.as_ref() {
                            artifact_uri = file_part.file_with_uri.clone();
                            artifact_name = Some(file_part.name.clone());
                            info!(
                                "received file artifact `{}` (mime={}) at {:?}",
                                file_part.name, file_part.media_type, artifact_uri,
                            );
                        }
                    }
                }
            }
            Err(e) => {
                error!("stream error: {e}");
                break;
            }
        }
    }

    let Some(uri) = artifact_uri else {
        warn!("server did not emit a file artifact - nothing to download");
        return Ok(());
    };

    info!("downloading {uri}");
    let response = reqwest::get(&uri).await?;
    if !response.status().is_success() {
        error!("download failed: status={}", response.status());
        return Ok(());
    }
    let body = response.text().await?;
    info!(
        "downloaded `{}` ({} bytes):\n---\n{}\n---",
        artifact_name.unwrap_or_else(|| "<unnamed>".to_string()),
        body.len(),
        body,
    );

    Ok(())
}

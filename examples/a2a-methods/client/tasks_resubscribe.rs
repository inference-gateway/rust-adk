//! `tasks/resubscribe` - re-attach to an existing task and stream its
//! remaining state transitions over SSE.
//!
//! The example seeds a task via `message/send` (the offline echo handler
//! drives it straight to `Completed`), then calls `tasks/resubscribe` to
//! demonstrate the recovery path: the server replays a snapshot of the
//! task's current state followed by a `TaskStatusUpdateEvent` with
//! `final: true`, and the stream closes cleanly.
//!
//! This mirrors the canonical use of `tasks/resubscribe` after a transport
//! disconnect - the client knows the `tasks/{task_id}` resource name and
//! wants to resume observing the task without re-issuing
//! `message/stream`.
//!
//! ```bash
//! cargo run --example a2a-methods-server
//! cargo run --example a2a-methods-tasks-resubscribe
//! ```
use futures::StreamExt;
use inference_gateway_adk::A2AClient;
use inference_gateway_adk::a2a_types::{
    Message, Part, Role, SendMessageRequest, SubscribeToTaskRequest,
};
use std::env;
use tracing::info;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    let server_url = env::var("SERVER_URL").unwrap_or_else(|_| "http://localhost:8085".to_string());
    let client = A2AClient::new(&server_url)?;

    // 1. Seed a task we can later resubscribe to. The example server's
    //    EchoBackgroundTaskHandler drives the task through to `Completed`
    //    synchronously, so the resubscribe below will see a terminal task.
    let seed = client
        .send_message(SendMessageRequest {
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
                    text: Some("seed for tasks/resubscribe".to_string()),
                }],
                reference_task_ids: vec![],
                role: Role::RoleUser,
                task_id: None,
            }),
            metadata: None,
            tenant: "example".to_string(),
        })
        .await?;
    let seeded_task = seed.task.ok_or("server did not return a task")?;
    let task_resource = format!("tasks/{}", seeded_task.id);

    info!(
        "seeded task {} (state {:?}); resubscribing...",
        seeded_task.id, seeded_task.status.state
    );

    // 2. Resubscribe. The server replays a snapshot of the task followed
    //    by a final TaskStatusUpdateEvent and then closes the stream.
    let mut stream = Box::pin(
        client
            .resubscribe_task(SubscribeToTaskRequest {
                name: task_resource.clone(),
                tenant: "example".to_string(),
            })
            .await?,
    );

    let mut event_index = 0usize;
    while let Some(event) = stream.next().await {
        let response = event?;
        event_index += 1;

        if let Some(task) = response.task {
            info!(
                "[{event_index}] tasks/resubscribe → snapshot: id={} state={:?}",
                task.id, task.status.state
            );
        }

        if let Some(update) = response.status_update {
            let suffix = if update.final_ { " (final)" } else { "" };
            info!(
                "[{event_index}] tasks/resubscribe → status update: {:?}{suffix}",
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
            info!("[{event_index}] tasks/resubscribe → artifact: {:?}", text);
        }
    }

    info!("tasks/resubscribe → stream closed after {event_index} events");
    Ok(())
}

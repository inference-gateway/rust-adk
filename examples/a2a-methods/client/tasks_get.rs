//! `tasks/get` — fetch a stored task by its resource name (`tasks/{task_id}`).
//!
//! Creates a task via `message/send` first so the example is self-contained.
//!
//! ```bash
//! cargo run --example a2a-methods-server
//! cargo run --example a2a-methods-tasks-get
//! ```

use inference_gateway_adk::A2AClient;
use inference_gateway_adk::a2a_types::{GetTaskRequest, Message, Part, Role, SendMessageRequest};
use std::env;
use tracing::info;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    let server_url = env::var("SERVER_URL").unwrap_or_else(|_| "http://localhost:8085".to_string());
    let client = A2AClient::new(&server_url)?;

    // 1. seed a task we can later look up.
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
                    text: Some("seed for tasks/get".to_string()),
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

    // 2. fetch it via tasks/get.
    let fetched = client
        .get_task(GetTaskRequest {
            history_length: None,
            name: format!("tasks/{}", seeded_task.id),
            tenant: Some("example".to_string()),
        })
        .await?;

    info!(
        "tasks/get → id={} state={:?} history_len={}",
        fetched.id,
        fetched.status.state,
        fetched.history.len()
    );

    Ok(())
}

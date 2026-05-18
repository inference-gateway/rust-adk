//! `tasks/list` - page through stored tasks for a tenant.
//!
//! Seeds a couple of tasks first so the response is non-empty.
//!
//! ```bash
//! cargo run -p a2a-methods-server
//! cargo run -p a2a-methods-tasks-list
//! ```

use inference_gateway_adk::A2AClient;
use inference_gateway_adk::a2a_types::{
    ListTasksRequest, Message, Part, Role, SendMessageRequest, TaskState,
};
use std::env;
use tracing::info;
use uuid::Uuid;

async fn seed_task(client: &A2AClient, text: &str) -> anyhow::Result<()> {
    client
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
                    text: Some(text.to_string()),
                }],
                reference_task_ids: vec![],
                role: Role::RoleUser,
                task_id: None,
            }),
            metadata: None,
            tenant: "example".to_string(),
        })
        .await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    let server_url = env::var("SERVER_URL").unwrap_or_else(|_| "http://localhost:8085".to_string());
    let client = A2AClient::new(&server_url)?;

    seed_task(&client, "list-seed-1").await?;
    seed_task(&client, "list-seed-2").await?;

    let listed = client
        .list_tasks(ListTasksRequest {
            context_id: String::new(),
            history_length: None,
            include_artifacts: None,
            last_updated_after: 0,
            page_size: Some(10),
            page_token: String::new(),
            status: TaskState::TaskStateUnspecified,
            tenant: "example".to_string(),
        })
        .await?;

    info!(
        "tasks/list → returned {} tasks (total_size={}, next_page_token={:?})",
        listed.tasks.len(),
        listed.total_size,
        listed.next_page_token
    );

    for t in &listed.tasks {
        info!("  · {} ({:?})", t.id, t.status.state);
    }

    Ok(())
}

//! `CreateTaskPushNotificationConfig` - store a push notification
//! configuration on a task.
//!
//! The server persists the config in storage; an actual webhook sender is
//! tracked in a separate ticket, so no HTTP delivery happens yet - but the
//! set/get/list/delete control plane is fully wired up.
//!
//! ```bash
//! cargo run -p a2a-methods-server
//! cargo run -p a2a-methods-client --bin push-config-set
//! ```

use inference_gateway_adk::A2AClient;
use inference_gateway_adk::a2a_types::{
    Message, Part, Role, SendMessageRequest, TaskPushNotificationConfig,
};
use std::env;
use tracing::info;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    let server_url = env::var("SERVER_URL").unwrap_or_else(|_| "http://localhost:8085".to_string());
    let client = A2AClient::new(&server_url)?;

    let seed = client
        .send_message(SendMessageRequest {
            configuration: None,
            message: Message {
                context_id: None,
                extensions: vec![],
                message_id: Uuid::new_v4().to_string(),
                metadata: None,
                parts: vec![Part {
                    text: Some("seed for CreateTaskPushNotificationConfig".to_string()),
                    ..Default::default()
                }],
                reference_task_ids: vec![],
                role: Role::RoleUser,
                task_id: None,
            },
            metadata: None,
            tenant: Some("example".to_string()),
        })
        .await?;
    let task = seed.task.ok_or("server did not return a task")?;
    let stored = client
        .set_task_push_notification_config(TaskPushNotificationConfig {
            authentication: None,
            id: Some("primary".to_string()),
            task_id: Some(task.id.clone()),
            tenant: Some("example".to_string()),
            token: Some("example-shared-secret".to_string()),
            url: "https://your-app.example/webhooks/a2a".to_string(),
        })
        .await?;

    info!(
        "CreateTaskPushNotificationConfig → stored {:?}/{:?} → {}",
        stored.task_id, stored.id, stored.url
    );

    Ok(())
}

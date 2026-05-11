//! `tasks/pushNotificationConfig/set` — store a push notification
//! configuration on a task.
//!
//! The server persists the config in storage; an actual webhook sender is
//! tracked in a separate ticket, so no HTTP delivery happens yet — but the
//! set/get/list/delete control plane is fully wired up.
//!
//! ```bash
//! cargo run --example a2a-methods-server
//! cargo run --example a2a-methods-push-config-set
//! ```

use inference_gateway_adk::A2AClient;
use inference_gateway_adk::a2a_types::{
    Message, Part, PushNotificationConfig, Role, SendMessageRequest,
    SetTaskPushNotificationConfigRequest, TaskPushNotificationConfig,
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
            message: Some(Message {
                context_id: None,
                extensions: vec![],
                message_id: Uuid::new_v4().to_string(),
                metadata: None,
                parts: vec![Part {
                    data: None,
                    file: None,
                    metadata: None,
                    text: Some("seed for pushNotificationConfig/set".to_string()),
                }],
                reference_task_ids: vec![],
                role: Role::RoleUser,
                task_id: None,
            }),
            metadata: None,
            tenant: "example".to_string(),
        })
        .await?;
    let task = seed.task.ok_or("server did not return a task")?;
    let parent = format!("tasks/{}", task.id);
    let config_id = "primary";
    let name = format!("{parent}/pushNotificationConfigs/{config_id}");

    let stored = client
        .set_task_push_notification_config(SetTaskPushNotificationConfigRequest {
            parent: parent.clone(),
            config_id: config_id.to_string(),
            tenant: Some("example".to_string()),
            config: TaskPushNotificationConfig {
                name: name.clone(),
                push_notification_config: PushNotificationConfig {
                    authentication: None,
                    id: None,
                    token: Some("example-shared-secret".to_string()),
                    url: "https://your-app.example/webhooks/a2a".to_string(),
                },
            },
        })
        .await?;

    info!(
        "tasks/pushNotificationConfig/set → stored {} → {}",
        stored.name, stored.push_notification_config.url
    );

    Ok(())
}

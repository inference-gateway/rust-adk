//! `tasks/pushNotificationConfig/get` - read back a stored push notification
//! configuration.
//!
//! ```bash
//! cargo run --example a2a-methods-server
//! cargo run --example a2a-methods-push-config-get
//! ```

use inference_gateway_adk::A2AClient;
use inference_gateway_adk::a2a_types::{
    GetTaskPushNotificationConfigRequest, Message, Part, PushNotificationConfig, Role,
    SendMessageRequest, SetTaskPushNotificationConfigRequest, TaskPushNotificationConfig,
};
use std::env;
use tracing::info;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    let server_url = env::var("SERVER_URL").unwrap_or_else(|_| "http://localhost:8085".to_string());
    let client = A2AClient::new(&server_url)?;

    // Seed a task and an attached push config so this example stands alone.
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
                    text: Some("seed for pushNotificationConfig/get".to_string()),
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

    client
        .set_task_push_notification_config(SetTaskPushNotificationConfigRequest {
            parent: parent.clone(),
            config_id: config_id.to_string(),
            tenant: Some("example".to_string()),
            config: TaskPushNotificationConfig {
                name: name.clone(),
                push_notification_config: PushNotificationConfig {
                    authentication: None,
                    id: None,
                    token: None,
                    url: "https://your-app.example/webhooks/a2a".to_string(),
                },
            },
        })
        .await?;

    let fetched = client
        .get_task_push_notification_config(GetTaskPushNotificationConfigRequest {
            name: name.clone(),
            tenant: "example".to_string(),
        })
        .await?;

    info!(
        "tasks/pushNotificationConfig/get → name={} url={}",
        fetched.name, fetched.push_notification_config.url
    );

    Ok(())
}

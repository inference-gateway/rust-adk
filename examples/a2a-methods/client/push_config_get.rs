//! `GetTaskPushNotificationConfig` - read back a stored push notification
//! configuration.
//!
//! ```bash
//! cargo run -p a2a-methods-server
//! cargo run -p a2a-methods-client --bin push-config-get
//! ```

use inference_gateway_adk::A2AClient;
use inference_gateway_adk::a2a_types::{
    GetTaskPushNotificationConfigRequest, Message, Part, Role, SendMessageRequest,
    TaskPushNotificationConfig,
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
            message: Message {
                context_id: None,
                extensions: vec![],
                message_id: Uuid::new_v4().to_string(),
                metadata: None,
                parts: vec![Part {
                    text: Some("seed for GetTaskPushNotificationConfig".to_string()),
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
    let config_id = "primary";

    client
        .set_task_push_notification_config(TaskPushNotificationConfig {
            authentication: None,
            id: Some(config_id.to_string()),
            task_id: Some(task.id.clone()),
            tenant: Some("example".to_string()),
            token: None,
            url: "https://your-app.example/webhooks/a2a".to_string(),
        })
        .await?;

    let fetched = client
        .get_task_push_notification_config(GetTaskPushNotificationConfigRequest {
            id: config_id.to_string(),
            task_id: task.id.clone(),
            tenant: Some("example".to_string()),
        })
        .await?;

    info!(
        "GetTaskPushNotificationConfig → id={:?} url={}",
        fetched.id, fetched.url
    );

    Ok(())
}

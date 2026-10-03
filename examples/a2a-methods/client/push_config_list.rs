//! `ListTaskPushNotificationConfigs` - list the push notification
//! configurations belonging to a task.
//!
//! ```bash
//! cargo run -p a2a-methods-server
//! cargo run -p a2a-methods-client --bin push-config-list
//! ```

use inference_gateway_adk::A2AClient;
use inference_gateway_adk::a2a_types::{
    ListTaskPushNotificationConfigsRequest, Message, Part, Role, SendMessageRequest,
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

    let seed = client
        .send_message(SendMessageRequest {
            configuration: None,
            message: Message {
                context_id: None,
                extensions: vec![],
                message_id: Uuid::new_v4().to_string(),
                metadata: None,
                parts: vec![Part {
                    text: Some("seed for ListTaskPushNotificationConfigs".to_string()),
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
    // Seed two configs so the list result is non-trivial.
    for (idx, url) in [
        "https://your-app.example/webhooks/primary",
        "https://your-app.example/webhooks/secondary",
    ]
    .iter()
    .enumerate()
    {
        client
            .set_task_push_notification_config(TaskPushNotificationConfig {
                authentication: None,
                id: Some(format!("cfg-{idx}")),
                task_id: Some(task.id.clone()),
                tenant: Some("example".to_string()),
                token: None,
                url: (*url).to_string(),
            })
            .await?;
    }

    let listed = client
        .list_task_push_notification_configs(ListTaskPushNotificationConfigsRequest {
            page_size: Some(10),
            page_token: Some(String::new()),
            task_id: task.id.clone(),
            tenant: Some("example".to_string()),
        })
        .await?;

    info!(
        "ListTaskPushNotificationConfigs → {} configs (next_page_token={:?})",
        listed.configs.len(),
        listed.next_page_token
    );
    for c in &listed.configs {
        info!("  · {:?} → {}", c.id, c.url);
    }

    Ok(())
}

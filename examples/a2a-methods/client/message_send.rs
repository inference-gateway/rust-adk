//! `message/send` - dispatch a message to the agent and read the resulting
//! task off the response.
//!
//! Run alongside `a2a-methods-server`:
//!
//! ```bash
//! cargo run --example a2a-methods-server
//! cargo run --example a2a-methods-message-send
//! ```

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
                text: Some("Hello via message/send".to_string()),
            }],
            reference_task_ids: vec![],
            role: Role::RoleUser,
            task_id: None,
        }),
        metadata: None,
        tenant: "example".to_string(),
    };

    let response = client.send_message(request).await?;

    if let Some(task) = response.task {
        info!(
            "message/send → task {} in state {:?}",
            task.id, task.status.state
        );
    } else if let Some(msg) = response.message {
        info!("message/send → message {}", msg.message_id);
    }

    Ok(())
}

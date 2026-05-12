use inference_gateway_adk::A2AClient;
use inference_gateway_adk::a2a_types::{Message, Part, Role, SendMessageRequest};
use std::env;
use tracing::{error, info};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    let server_url = env::var("SERVER_URL").unwrap_or_else(|_| "http://localhost:8081".to_string());
    let client = A2AClient::new(&server_url)?;

    info!("Minimal A2A client connecting to {}", server_url);

    match client.get_health().await {
        Ok(health) => {
            info!("Health check successful: {}", health.status);
            info!("Server timestamp: {}", health.timestamp);
        }
        Err(e) => {
            error!("Health check failed: {}", e);
            return Ok(());
        }
    }

    match client.get_agent_card().await {
        Ok(agent_card) => {
            info!("Agent card retrieved successfully");
            info!("Agent: {} - {}", agent_card.name, agent_card.description);
        }
        Err(e) => {
            error!("Failed to get agent card: {}", e);
        }
    }

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
                text: Some("Hello! Testing the minimal A2A server.".to_string()),
            }],
            reference_task_ids: vec![],
            role: Role::RoleUser,
            task_id: None,
        }),
        metadata: None,
        tenant: "minimal".to_string(),
    };

    match client.send_message(request).await {
        Ok(response) => {
            info!("message/send dispatched");
            if let Some(task) = response.task {
                info!("→ task {} in state {:?}", task.id, task.status.state);
                if let Some(msg) = task.status.message {
                    let text = msg
                        .parts
                        .iter()
                        .filter_map(|p| p.text.clone())
                        .collect::<Vec<_>>()
                        .join("");
                    info!("→ agent reply: {}", text);
                }
            }
        }
        Err(e) => {
            error!("Failed to dispatch message/send: {}", e);
        }
    }

    Ok(())
}

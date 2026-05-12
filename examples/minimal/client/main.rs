use inference_gateway_adk::A2AClient;
use inference_gateway_adk::a2a_types::{
    GetTaskRequest, Message, Part, Role, SendMessageRequest, Task, TaskState,
};
use std::env;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{error, info};
use uuid::Uuid;

async fn poll_until_terminal(
    client: &A2AClient,
    task_id: &str,
) -> Result<Task, Box<dyn std::error::Error>> {
    for _ in 0..150 {
        let task = client
            .get_task(GetTaskRequest {
                history_length: None,
                name: format!("tasks/{task_id}"),
                tenant: Some("minimal".to_string()),
            })
            .await?;
        if matches!(
            task.status.state,
            TaskState::TaskStateCompleted
                | TaskState::TaskStateFailed
                | TaskState::TaskStateCancelled
                | TaskState::TaskStateRejected
        ) {
            return Ok(task);
        }
        sleep(Duration::from_millis(200)).await;
    }
    Err(format!("task {task_id} did not reach a terminal state in time").into())
}

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
                info!(
                    "→ task {} accepted in state {:?}",
                    task.id, task.status.state
                );
                match poll_until_terminal(&client, &task.id).await {
                    Ok(final_task) => {
                        info!(
                            "→ task {} reached state {:?}",
                            final_task.id, final_task.status.state
                        );
                        if let Some(msg) = final_task.status.message {
                            let text = msg
                                .parts
                                .iter()
                                .filter_map(|p| p.text.clone())
                                .collect::<Vec<_>>()
                                .join("");
                            info!("→ agent reply: {}", text);
                        }
                    }
                    Err(e) => error!("Failed to poll task to terminal state: {}", e),
                }
            }
        }
        Err(e) => {
            error!("Failed to dispatch message/send: {}", e);
        }
    }

    Ok(())
}

use inference_gateway_adk::A2AClient;
use inference_gateway_adk::a2a_types::{
    GetTaskRequest, Message, Part, Role, SendMessageRequest, Task, TaskState,
};
use std::env;
use tokio::time::{Duration, sleep};
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
                tenant: Some("ai-powered".to_string()),
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

fn user_message(text: &str) -> SendMessageRequest {
    SendMessageRequest {
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
        tenant: "ai-powered".to_string(),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    let server_url = env::var("SERVER_URL").unwrap_or_else(|_| "http://localhost:8082".to_string());
    let client = A2AClient::new(&server_url)?;

    info!("AI-powered A2A client connecting to {server_url}");

    let health = client.get_health().await?;
    info!("server health: {}", health.status);

    let agent_card = client.get_agent_card().await?;
    info!(
        "agent: {} v{} ({})",
        agent_card.name, agent_card.version, agent_card.description
    );

    let prompts = [
        "What's the weather in San Francisco, and what is 12 * 7?",
        "Search the web for 'Rust async programming' and summarise the top result.",
    ];

    for prompt in prompts {
        info!("→ sending: {prompt}");
        let response = client.send_message(user_message(prompt)).await?;
        let Some(task) = response.task else {
            error!("server returned no task for message/send");
            continue;
        };
        info!(
            "  task {} accepted in state {:?}",
            task.id, task.status.state
        );

        let final_task = poll_until_terminal(&client, &task.id).await?;
        info!(
            "  task {} reached state {:?}",
            final_task.id, final_task.status.state
        );
        if let Some(msg) = final_task.status.message {
            let text = msg
                .parts
                .iter()
                .filter_map(|p| p.text.clone())
                .collect::<Vec<_>>()
                .join("");
            info!("  agent reply: {text}");
        }
    }

    Ok(())
}

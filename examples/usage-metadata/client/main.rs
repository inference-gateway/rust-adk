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
                tenant: Some("usage-metadata".to_string()),
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
        tenant: "usage-metadata".to_string(),
    }
}

/// Render the `usage` and `execution_stats` blocks the server attaches to a
/// terminal task's `metadata` when usage metadata is enabled.
fn render_usage_metadata(task: &Task) {
    let Some(meta) = task.metadata.as_ref() else {
        info!(
            "  no usage metadata on this task - the server has \
             A2A_AGENT_CLIENT_ENABLE_USAGE_METADATA=false"
        );
        return;
    };

    if let Some(usage) = meta.0.get("usage") {
        info!(
            "  token usage: prompt={}, completion={}, total={}",
            usage
                .get("prompt_tokens")
                .and_then(|v| v.as_i64())
                .unwrap_or(0),
            usage
                .get("completion_tokens")
                .and_then(|v| v.as_i64())
                .unwrap_or(0),
            usage
                .get("total_tokens")
                .and_then(|v| v.as_i64())
                .unwrap_or(0),
        );
    }

    if let Some(stats) = meta.0.get("execution_stats") {
        info!(
            "  execution stats: iterations={}, messages={}, tool_calls={}, failed_tools={}",
            stats
                .get("iterations")
                .and_then(|v| v.as_u64())
                .unwrap_or(0),
            stats.get("messages").and_then(|v| v.as_u64()).unwrap_or(0),
            stats
                .get("tool_calls")
                .and_then(|v| v.as_u64())
                .unwrap_or(0),
            stats
                .get("failed_tools")
                .and_then(|v| v.as_u64())
                .unwrap_or(0),
        );
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    let server_url = env::var("SERVER_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
    let client = A2AClient::new(&server_url)?;

    info!("usage-metadata A2A client connecting to {server_url}");

    let health = client.get_health().await?;
    info!("server health: {}", health.status);

    let prompt = "Please add these numbers for me: 12, 30, and 168.";
    info!("-> sending: {prompt}");

    let response = client.send_message(user_message(prompt)).await?;
    let Some(task) = response.task else {
        error!("server returned no task for message/send");
        return Ok(());
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
    if let Some(msg) = final_task.status.message.as_ref() {
        let text = msg
            .parts
            .iter()
            .filter_map(|p| p.text.clone())
            .collect::<Vec<_>>()
            .join("");
        info!("  agent reply: {text}");
    }

    render_usage_metadata(&final_task);

    Ok(())
}

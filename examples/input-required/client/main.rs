use inference_gateway_adk::A2AClient;
use inference_gateway_adk::a2a_types::{
    GetTaskRequest, Message, Part, Role, SendMessageRequest, Task, TaskState,
};
use std::env;
use tokio::time::{Duration, sleep};
use tracing::{error, info};
use uuid::Uuid;

async fn poll_until_settled(
    client: &A2AClient,
    task_id: &str,
) -> Result<Task, Box<dyn std::error::Error>> {
    // We include `InputRequired` as a settled state for the purposes of
    // this demo: the handler has decided it cannot proceed without more
    // input, and the task will sit in this state until the protocol
    // ships a resume path.
    for _ in 0..50 {
        let task = client
            .get_task(GetTaskRequest {
                history_length: None,
                name: format!("tasks/{task_id}"),
                tenant: Some("input-required".to_string()),
            })
            .await?;
        if matches!(
            task.status.state,
            TaskState::TaskStateCompleted
                | TaskState::TaskStateFailed
                | TaskState::TaskStateCancelled
                | TaskState::TaskStateRejected
                | TaskState::TaskStateInputRequired
        ) {
            return Ok(task);
        }
        sleep(Duration::from_millis(200)).await;
    }
    Err(format!("task {task_id} did not settle in time").into())
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
        tenant: "input-required".to_string(),
    }
}

async fn dispatch(
    client: &A2AClient,
    label: &str,
    text: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("→ [{label}] sending: {text}");
    let response = client.send_message(user_message(text)).await?;
    let Some(task) = response.task else {
        error!("[{label}] server returned no task for message/send");
        return Ok(());
    };
    info!(
        "  [{label}] task {} accepted in state {:?}",
        task.id, task.status.state
    );

    let settled = poll_until_settled(client, &task.id).await?;
    info!(
        "  [{label}] task {} settled in state {:?}",
        settled.id, settled.status.state
    );
    if let Some(msg) = settled.status.message {
        let body = msg
            .parts
            .iter()
            .filter_map(|p| p.text.clone())
            .collect::<Vec<_>>()
            .join("");
        info!("  [{label}] agent says: {body}");
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    let server_url = env::var("SERVER_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());
    let client = A2AClient::new(&server_url)?;

    info!("input-required client → {server_url}");

    let health = client.get_health().await?;
    info!("server health: {}", health.status);

    // Branch A: message contains a known city — handler completes.
    dispatch(
        &client,
        "with-city",
        "What's the weather in London right now?",
    )
    .await?;

    // Branch B: no city mentioned — handler pauses in InputRequired.
    dispatch(&client, "no-city", "What's the weather?").await?;

    info!("done — observe the two branches above: TaskStateCompleted vs TaskStateInputRequired");
    Ok(())
}

use inference_gateway_adk::A2AClient;
use inference_gateway_adk::a2a_types::{
    GetTaskRequest, Message, Part, Role, SendMessageRequest, Task, TaskState,
};
use std::env;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use tracing::{error, info};
use uuid::Uuid;

fn make_user_message(text: &str) -> SendMessageRequest {
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
        tenant: "queue-storage".to_string(),
    }
}

async fn poll_until_terminal(
    client: &A2AClient,
    task_id: &str,
) -> Result<Task, Box<dyn std::error::Error>> {
    // Generous deadline: with 1 worker + 5 tasks × 2s, the *last* task
    // doesn't finish for ~10s wall time.
    for _ in 0..200 {
        let task = client
            .get_task(GetTaskRequest {
                history_length: None,
                name: format!("tasks/{task_id}"),
                tenant: Some("queue-storage".to_string()),
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

fn reply_text(task: &Task) -> String {
    task.status
        .message
        .as_ref()
        .map(|m| {
            m.parts
                .iter()
                .filter_map(|p| p.text.clone())
                .collect::<Vec<_>>()
                .join("")
        })
        .unwrap_or_default()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    let server_url = env::var("SERVER_URL").unwrap_or_else(|_| "http://localhost:8083".to_string());
    let n: usize = env::var("EXAMPLE_TASKS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(5);

    info!("queue-storage client → {server_url} (will dispatch {n} tasks)");

    let client = A2AClient::new(&server_url)?;

    // Wait until the server is up. Compose handles this via depends_on,
    // but running outside compose needs a small retry loop.
    for attempt in 0..30 {
        match client.get_health().await {
            Ok(h) => {
                info!("server healthy ({})", h.status);
                break;
            }
            Err(e) if attempt == 29 => {
                error!("server never became healthy: {e}");
                return Err(e.into());
            }
            Err(_) => sleep(Duration::from_millis(500)).await,
        }
    }

    let start = Instant::now();
    let mut task_ids = Vec::with_capacity(n);

    info!(
        "[{:.2}s] enqueuing {n} tasks via message/send …",
        start.elapsed().as_secs_f64()
    );
    for i in 0..n {
        let task = client
            .send_message(make_user_message(&format!("hello #{i}")))
            .await?
            .task
            .expect("server returns task on message/send");
        info!(
            "[{:.2}s] enqueued task #{i} → id={} state={:?}",
            start.elapsed().as_secs_f64(),
            task.id,
            task.status.state,
        );
        task_ids.push(task.id);
    }

    info!(
        "[{:.2}s] polling all {n} tasks to terminal state …",
        start.elapsed().as_secs_f64()
    );
    for (i, id) in task_ids.iter().enumerate() {
        let final_task = poll_until_terminal(&client, id).await?;
        info!(
            "[{:.2}s] task #{i} → state={:?} reply={:?}",
            start.elapsed().as_secs_f64(),
            final_task.status.state,
            reply_text(&final_task),
        );
    }

    info!(
        "done - total wall time {:.2}s for {} tasks",
        start.elapsed().as_secs_f64(),
        n
    );

    Ok(())
}

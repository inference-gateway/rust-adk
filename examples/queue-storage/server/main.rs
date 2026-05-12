use inference_gateway_adk::A2AServerBuilder;
use inference_gateway_adk::a2a_types::{
    Message, Part, Role, Task, TaskState, TaskStatus, Timestamp,
};
use inference_gateway_adk::{Config, TaskHandler};
use std::env;
use std::time::Duration;
use tracing::{error, info};

/// Deliberately slow `TaskHandler` so worker concurrency becomes visible
/// in the client log. Sleeps for `delay`, then completes the task with
/// `echo: <input text>`.
///
/// Configure via `EXAMPLE_DELAY_MS` (default 2000).
#[derive(Debug)]
struct SleepEchoHandler {
    delay: Duration,
}

#[async_trait::async_trait]
impl TaskHandler for SleepEchoHandler {
    async fn handle_task(&self, mut task: Task, message: Option<Message>) -> anyhow::Result<Task> {
        let input = message
            .as_ref()
            .and_then(|m| m.parts.first())
            .and_then(|p| p.text.clone())
            .unwrap_or_default();

        info!(
            task_id = %task.id,
            delay_ms = self.delay.as_millis() as u64,
            "worker dequeued task, sleeping before reply",
        );
        tokio::time::sleep(self.delay).await;

        let reply = Message {
            context_id: Some(task.context_id.clone()),
            extensions: vec![],
            message_id: uuid::Uuid::new_v4().to_string(),
            metadata: None,
            parts: vec![Part {
                data: None,
                file: None,
                metadata: None,
                text: Some(format!("echo: {input}")),
            }],
            reference_task_ids: vec![],
            role: Role::RoleAgent,
            task_id: Some(task.id.clone()),
        };

        task.history.push(reply.clone());
        task.status = TaskStatus {
            message: Some(reply),
            state: TaskState::TaskStateCompleted,
            timestamp: Some(Timestamp(chrono::Utc::now())),
        };
        Ok(task)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    // `Config::from_env()` parses A2A_QUEUE_PROVIDER / A2A_QUEUE_URL /
    // A2A_QUEUE_NAMESPACE / A2A_QUEUE_WORKERS / PORT into
    // `config.queue_config` and `config.port`.
    let config = Config::from_env()?;

    let delay_ms = env::var("EXAMPLE_DELAY_MS")
        .ok()
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(2000);

    let port = env::var("PORT")
        .ok()
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or(8083);

    info!(
        "starting queue-storage example server (provider={:?}, workers={}, delay_ms={}, port={})",
        config.queue_config.provider, config.queue_config.workers, delay_ms, port,
    );
    if config.queue_config.provider == inference_gateway_adk::QueueProvider::Redis {
        info!("redis URL: {:?}", config.queue_config.url);
    }

    let server = A2AServerBuilder::new()
        .with_config(config)
        .with_agent_card_from_file(".well-known/agent.json", None)
        .with_background_task_handler(SleepEchoHandler {
            delay: Duration::from_millis(delay_ms),
        })
        .with_default_streaming_task_handler()
        .build()
        .await?;

    let addr: std::net::SocketAddr = format!("0.0.0.0:{port}").parse()?;
    info!("queue-storage server listening on {addr}");

    if let Err(e) = server.serve(addr).await {
        error!("server stopped: {e}");
    }

    Ok(())
}

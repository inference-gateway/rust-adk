use inference_gateway_adk::A2AServerBuilder;
use inference_gateway_adk::a2a_types::{
    Message, Part, Role, Task, TaskState, TaskStatus, Timestamp,
};
use inference_gateway_adk::{Config, TaskHandler};
use serde::Deserialize;
use std::time::Duration;
use tracing::{error, info};
use tracing_subscriber::{EnvFilter, fmt};

/// App-specific config, kept under its own `APP_*` prefix so it stays
/// clearly separate from the ADK's `A2A_*` namespace. Loaded with a
/// second `envy::prefixed(...)` call in `main` - the ADK's `Config` is
/// loaded independently with `envy::prefixed("A2A_")`.
///
/// This is the recommended layout for clients that want their own
/// configuration alongside the ADK's: define your struct with whatever
/// prefix you like (or no prefix at all), load it separately, and pass
/// only the ADK [`Config`] into [`A2AServerBuilder::with_config`].
///
/// `log_level` and `log_format` here drive the `tracing` subscriber
/// installed in `main`. The ADK never installs a subscriber itself -
/// the consumer owns logging, the ADK just emits events.
#[derive(Debug, Deserialize)]
#[serde(default)]
struct AppConfig {
    /// Anything accepted by `EnvFilter`. Examples: `"info"`, `"debug"`,
    /// or fine-grained directives like
    /// `"info,tower_http=debug,inference_gateway_adk=trace"`.
    log_level: String,
    /// `"pretty"` (default) or `"json"`. Swap for whatever formatter /
    /// Subscriber composition your stack needs.
    log_format: String,
    delay_ms: u64,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            log_level: "info,tower_http=debug".to_string(),
            log_format: "pretty".to_string(),
            delay_ms: 2000,
        }
    }
}

/// Deliberately slow `TaskHandler` so worker concurrency becomes visible
/// in the client log. Sleeps for `delay`, then completes the task with
/// `echo: <input text>`.
///
/// Configure via `APP_DELAY_MS` (default 2000) - see `AppConfig`.
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
    // Two independent envy calls - each owns its own prefix. The ADK's
    // `Config` reads `A2A_*`; this example's `AppConfig` reads `APP_*`.
    // Clients can use any prefix (or none) for their own config; the ADK
    // only cares that you hand it a `Config`.
    let config: Config = envy::prefixed("A2A_").from_env()?;
    let app: AppConfig = envy::prefixed("APP_").from_env()?;

    // ─────────────────────────────────────────────────────────────────
    // Subscriber installation - fully owned by the consumer.
    //
    // The ADK does NOT install a tracing subscriber. Every `info!()` /
    // `debug!()` inside the ADK dispatches through whatever global
    // subscriber is registered here. To swap in OpenTelemetry, a JSON
    // exporter for Loki/Datadog/Honeycomb, or a custom `Subscriber`
    // impl - change this block. The ADK doesn't need to know.
    // ─────────────────────────────────────────────────────────────────
    let filter = EnvFilter::try_new(&app.log_level)
        .unwrap_or_else(|_| EnvFilter::new("info,tower_http=debug"));

    match app.log_format.as_str() {
        "json" => fmt().with_env_filter(filter).json().init(),
        _ => fmt().with_env_filter(filter).init(),
    }

    let port = config.server_config.port;
    info!(
        log_level = %app.log_level,
        log_format = %app.log_format,
        provider = ?config.queue_config.provider,
        workers = config.queue_config.workers,
        delay_ms = app.delay_ms,
        port = port,
        "starting queue-storage example server",
    );
    if config.queue_config.provider == inference_gateway_adk::QueueProvider::Redis {
        info!("redis URL: {:?}", config.queue_config.url);
    }

    let server = A2AServerBuilder::new()
        .with_config(config)
        .with_agent_card_from_file(".well-known/agent.json", None)
        .with_background_task_handler(SleepEchoHandler {
            delay: Duration::from_millis(app.delay_ms),
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

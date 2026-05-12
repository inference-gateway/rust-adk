//! Filesystem-backed artifacts example server.
//!
//! Spawns the A2A server on `:8087` and the artifacts server on `:8088`.
//! The streaming task handler produces a small text report as a file
//! artifact (`report.txt`) whose URI clients can fetch directly over
//! HTTP from the artifacts server.
//!
//! Run with:
//!
//! ```bash
//! cd examples/artifacts-filesystem/server
//! cargo run --example artifacts-filesystem-server
//! ```

use inference_gateway_adk::a2a_types::{Message as A2AMessage, Task, TaskState};
use inference_gateway_adk::{
    A2AServerBuilder, ArtifactsConfig, ArtifactsServerConfig, ArtifactsStorageConfig, Config,
    StreamEmitter, StreamableTaskHandler,
};
use std::time::Duration;
use tracing::{error, info};

/// Streaming handler that emits a fixed text report as a downloadable
/// file artifact. Demonstrates the `emit_file_artifact` helper.
#[derive(Debug)]
struct ReportHandler;

#[async_trait::async_trait]
impl StreamableTaskHandler for ReportHandler {
    async fn handle_streaming_task(
        &self,
        task: Task,
        _message: Option<A2AMessage>,
        emitter: StreamEmitter,
    ) -> anyhow::Result<()> {
        emitter
            .emit_status(
                &task.id,
                &task.context_id,
                TaskState::TaskStateWorking,
                None,
                false,
            )
            .await?;

        let report = format!(
            "# Generated Report\n\nTask id: {}\nContext id: {}\nGenerated at: {}\n",
            task.id,
            task.context_id,
            chrono::Utc::now().to_rfc3339(),
        );
        emitter
            .emit_file_artifact(
                &task.id,
                &task.context_id,
                "report.txt",
                report.into_bytes(),
                Some("text/plain"),
                true,
            )
            .await?;

        emitter
            .emit_status(
                &task.id,
                &task.context_id,
                TaskState::TaskStateCompleted,
                None,
                true,
            )
            .await
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    // Wire the artifacts subsystem in code so the example is
    // self-contained. Production deployments would typically populate
    // these knobs via the `ARTIFACTS_*` env vars instead.
    let config = Config {
        artifacts_config: ArtifactsConfig {
            enable: true,
            server: ArtifactsServerConfig {
                host: "0.0.0.0".to_string(),
                port: 8088,
                read_timeout: Duration::from_secs(30),
                write_timeout: Duration::from_secs(30),
                tls: None,
            },
            storage: ArtifactsStorageConfig {
                base_path: "./artifacts-data".to_string(),
                base_url: "http://localhost:8088".to_string(),
                ..ArtifactsStorageConfig::default()
            },
            retention: Default::default(),
        },
        ..Config::default()
    };

    let server = A2AServerBuilder::new()
        .with_config(config)
        .with_agent_card_from_file(".well-known/agent.json", None)
        .with_streaming_task_handler(ReportHandler)
        .with_default_background_task_handler()
        .build()
        .await?;

    let addr = "0.0.0.0:8087".parse()?;
    info!("artifacts A2A server listening on {addr}");
    info!("artifacts HTTP server listening on http://localhost:8088");

    if let Err(e) = server.serve(addr).await {
        error!("server stopped: {e}");
    }

    Ok(())
}

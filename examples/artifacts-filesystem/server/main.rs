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
    A2AServerBuilder, ArtifactsConfig, Config, StreamEmitter, StreamableTaskHandler,
};
use std::env;
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

    // The artifacts subsystem loads from the `ARTIFACTS_*` env surface
    // (see docker-compose.yaml). Running the example directly without
    // those vars falls back to local-friendly defaults so it still
    // serves artifacts on :8088.
    let mut artifacts_config = envy::prefixed("ARTIFACTS_")
        .from_env::<ArtifactsConfig>()
        .map_err(|e| format!("failed to load ARTIFACTS_* config: {e}"))?;
    if env::var_os("ARTIFACTS_ENABLE").is_none() {
        artifacts_config.enable = true;
        artifacts_config.server.port = 8088;
        artifacts_config.storage.base_path = "./artifacts-data".to_string();
        artifacts_config.storage.base_url = "http://localhost:8088".to_string();
    }

    let base_url = artifacts_config.storage.base_url.clone();
    let config = Config {
        artifacts_config,
        ..Config::default()
    };

    let server = A2AServerBuilder::new()
        .with_config(config)
        .with_agent_card_from_file(".well-known/agent.json", None)
        .with_streaming_task_handler(ReportHandler)
        .build()
        .await?;

    let addr = "0.0.0.0:8087".parse()?;
    info!("artifacts A2A server listening on {addr}");
    info!("artifacts HTTP server listening on {base_url}");

    if let Err(e) = server.serve(addr).await {
        error!("server stopped: {e}");
    }

    Ok(())
}

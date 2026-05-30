//! MinIO-backed artifacts example server.
//!
//! Spawns the A2A server on `:8089` and the artifacts server on `:8090`.
//! The streaming task handler produces a small text report as a file
//! artifact (`report.txt`) whose URI clients can fetch directly from
//! MinIO over HTTP (the bucket is configured with anonymous read in the
//! compose `createbucket` init step).
//!
//! Run with:
//!
//! ```bash
//! cd examples/artifacts-minio/server
//! cargo run --features minio --example artifacts-minio-server
//! ```
//!
//! Requires a running MinIO instance — see `docker-compose.yaml` in the
//! example root.

use inference_gateway_adk::a2a_types::{Message as A2AMessage, Task, TaskState};
use inference_gateway_adk::{
    A2AServerBuilder, ArtifactsConfig, ArtifactsStorageProvider, Config, StreamEmitter,
    StreamableTaskHandler,
};
use std::env;
use tracing::{error, info};

/// Streaming handler that emits a fixed text report as a downloadable
/// file artifact. Demonstrates the `emit_file_artifact` helper backed by
/// MinIO storage.
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
            "# Generated Report\n\nTask id: {}\nContext id: {}\nGenerated at: {}\nBackend: MinIO\n",
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

    // MinIO connection settings load from the `ARTIFACTS_STORAGE_*` env
    // surface (see docker-compose.yaml). This example always runs the
    // MinIO backend on :8090; anything the env surface leaves unset falls
    // back to a local-friendly default so `cargo run` works out of the box.
    let mut artifacts_config = envy::prefixed("ARTIFACTS_")
        .from_env::<ArtifactsConfig>()
        .map_err(|e| format!("failed to load ARTIFACTS_* config: {e}"))?;
    artifacts_config.enable = true;
    artifacts_config.server.port = 8090;
    artifacts_config.storage.provider = ArtifactsStorageProvider::Minio;

    let storage = &mut artifacts_config.storage;
    let endpoint = storage
        .endpoint
        .get_or_insert_with(|| "http://localhost:9000".to_string())
        .clone();
    storage
        .access_key
        .get_or_insert_with(|| "minioadmin".to_string());
    storage
        .secret_key
        .get_or_insert_with(|| "minioadmin".to_string());
    let bucket_name = storage
        .bucket_name
        .get_or_insert_with(|| "artifacts".to_string())
        .clone();
    storage
        .region
        .get_or_insert_with(|| "us-east-1".to_string());
    if env::var_os("ARTIFACTS_STORAGE_BASE_URL").is_none() {
        storage.base_url = "http://localhost:9000".to_string();
    }
    storage.use_ssl = endpoint.starts_with("https://");

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

    let addr = "0.0.0.0:8089".parse()?;
    info!("artifacts-minio A2A server listening on {addr}");
    info!("artifacts-minio HTTP server listening on http://localhost:8090");
    info!(
        bucket = %bucket_name,
        endpoint = %endpoint,
        "MinIO storage configured",
    );

    if let Err(e) = server.serve(addr).await {
        error!("server stopped: {e}");
    }

    Ok(())
}

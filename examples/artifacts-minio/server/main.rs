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
    A2AServerBuilder, ArtifactsConfig, ArtifactsServerConfig, ArtifactsStorageConfig,
    ArtifactsStorageProvider, Config, StreamEmitter, StreamableTaskHandler,
};
use std::env;
use std::time::Duration;
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

    let endpoint = env::var("ARTIFACTS_STORAGE_ENDPOINT")
        .unwrap_or_else(|_| "http://localhost:9000".to_string());
    let access_key =
        env::var("ARTIFACTS_STORAGE_ACCESS_KEY").unwrap_or_else(|_| "minioadmin".to_string());
    let secret_key =
        env::var("ARTIFACTS_STORAGE_SECRET_KEY").unwrap_or_else(|_| "minioadmin".to_string());
    let bucket_name =
        env::var("ARTIFACTS_STORAGE_BUCKET_NAME").unwrap_or_else(|_| "artifacts".to_string());
    let base_url = env::var("ARTIFACTS_STORAGE_BASE_URL")
        .unwrap_or_else(|_| "http://localhost:9000".to_string());

    let config = Config {
        artifacts_config: ArtifactsConfig {
            enable: true,
            server: ArtifactsServerConfig {
                host: "0.0.0.0".to_string(),
                port: 8090,
                read_timeout: Duration::from_secs(30),
                write_timeout: Duration::from_secs(30),
                tls: None,
            },
            storage: ArtifactsStorageConfig {
                provider: ArtifactsStorageProvider::Minio,
                base_path: String::new(),
                base_url,
                endpoint: Some(endpoint.clone()),
                access_key: Some(access_key),
                secret_key: Some(secret_key),
                bucket_name: Some(bucket_name.clone()),
                region: Some("us-east-1".to_string()),
                use_ssl: endpoint.starts_with("https://"),
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

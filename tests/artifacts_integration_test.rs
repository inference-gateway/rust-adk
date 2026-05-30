//! End-to-end integration test for the artifacts subsystem.
//!
//! Stands up a full [`A2AServer`] with the artifacts server enabled and
//! a `StreamableTaskHandler` that mints a file artifact mid-stream. The
//! test then:
//!
//! 1. Opens `message/stream` and collects every event.
//! 2. Asserts one of the events carried a `FilePart` with `fileWithUri`
//!    pointing at the artifacts server.
//! 3. Fetches that URL over HTTP and verifies the round-tripped bytes.

use futures_util::StreamExt;
use inference_gateway_adk::a2a_types::{AgentCard, Message, Part, Role, SendMessageRequest, Task};
use inference_gateway_adk::{
    A2AClient, A2AServerBuilder, ArtifactsConfig, ArtifactsServerConfig, ArtifactsStorageConfig,
    Config, StreamEmitter, StreamableTaskHandler, a2a_types,
};
use std::net::TcpListener as StdTcpListener;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::timeout;

/// Streaming handler that emits a single text status, then a single
/// binary file artifact via [`StreamEmitter::emit_file_artifact`], then
/// transitions to `Completed`.
#[derive(Debug)]
struct ReportProducingHandler {
    payload: Vec<u8>,
}

#[async_trait::async_trait]
impl StreamableTaskHandler for ReportProducingHandler {
    async fn handle_streaming_task(
        &self,
        task: Task,
        _message: Option<Message>,
        emitter: StreamEmitter,
    ) -> anyhow::Result<()> {
        emitter
            .emit_status(
                &task.id,
                &task.context_id,
                a2a_types::TaskState::TaskStateWorking,
                None,
                false,
            )
            .await?;

        emitter
            .emit_file_artifact(
                &task.id,
                &task.context_id,
                "report.txt",
                self.payload.clone(),
                Some("text/plain"),
                true,
            )
            .await?;

        emitter
            .emit_status(
                &task.id,
                &task.context_id,
                a2a_types::TaskState::TaskStateCompleted,
                None,
                true,
            )
            .await
    }
}

fn agent_card() -> AgentCard {
    serde_json::from_value(serde_json::json!({
        "name": "Artifacts E2E Agent",
        "description": "End-to-end artifacts test",
        "version": "0.0.0",
        "protocolVersion": "0.2.6",
        "url": "http://localhost/a2a",
        "preferredTransport": "JSONRPC",
        "capabilities": {
            "streaming": true,
            "pushNotifications": false,
            "stateTransitionHistory": false
        },
        "defaultInputModes": ["text/plain"],
        "defaultOutputModes": ["text/plain"],
        "skills": [
            {"id": "report", "name": "report", "description": "report", "tags": ["report"]}
        ]
    }))
    .expect("agent card")
}

fn allocate_port() -> u16 {
    let listener = StdTcpListener::bind("127.0.0.1:0").expect("bind ephemeral port");
    listener.local_addr().expect("local_addr").port()
}

#[tokio::test]
async fn message_stream_emits_file_artifact_resolvable_via_artifacts_server() {
    let payload = b"end-to-end artifact body".to_vec();
    let artifacts_port = allocate_port();
    let a2a_port = allocate_port();

    let artifacts_root = std::env::temp_dir().join(format!(
        "rust-adk-artifacts-e2e-{}-{}",
        std::process::id(),
        uuid::Uuid::new_v4()
    ));
    let _ = std::fs::create_dir_all(&artifacts_root);

    let config = Config {
        artifacts_config: ArtifactsConfig {
            enable: true,
            server: ArtifactsServerConfig {
                host: "127.0.0.1".to_string(),
                port: artifacts_port,
                read_timeout: Duration::from_secs(5),
                write_timeout: Duration::from_secs(5),
                tls: None,
            },
            storage: ArtifactsStorageConfig {
                base_path: artifacts_root.to_string_lossy().to_string(),
                base_url: format!("http://127.0.0.1:{artifacts_port}"),
                ..ArtifactsStorageConfig::default()
            },
            retention: Default::default(),
        },
        ..Config::default()
    };

    let server = A2AServerBuilder::new()
        .with_config(config)
        .with_agent_card(agent_card())
        .with_streaming_task_handler(ReportProducingHandler {
            payload: payload.clone(),
        })
        .build()
        .await
        .expect("server builds");

    let a2a_addr: std::net::SocketAddr = format!("127.0.0.1:{a2a_port}").parse().expect("a2a addr");
    let server_handle = tokio::spawn(async move {
        let _ = server.serve(a2a_addr).await;
    });

    // Give the listeners a moment to come up (both A2A and artifacts).
    for _ in 0..50 {
        let probe = reqwest::get(format!("http://127.0.0.1:{artifacts_port}/health")).await;
        if probe.is_ok() {
            break;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }

    let client = A2AClient::new(format!("http://127.0.0.1:{a2a_port}")).expect("client");

    let request = SendMessageRequest {
        configuration: None,
        message: Some(Message {
            context_id: None,
            extensions: vec![],
            message_id: "msg-artifacts-e2e".to_string(),
            metadata: None,
            parts: vec![Part {
                data: None,
                file: None,
                metadata: None,
                text: Some("please produce a report".to_string()),
            }],
            reference_task_ids: vec![],
            role: Role::RoleUser,
            task_id: None,
        }),
        metadata: None,
        tenant: "tests".to_string(),
    };

    let mut stream = Box::pin(client.stream_message(request).await.expect("stream"));
    let mut file_uri: Option<String> = None;
    let mut filename: Option<String> = None;

    while let Some(item) = timeout(Duration::from_secs(5), stream.next())
        .await
        .expect("stream did not stall")
    {
        let event = item.expect("event");
        if let Some(update) = event.artifact_update.as_ref() {
            for part in &update.artifact.parts {
                if let Some(file_part) = part.file.as_ref()
                    && let Some(uri) = file_part.file_with_uri.as_ref()
                {
                    file_uri = Some(uri.clone());
                    filename = Some(file_part.name.clone());
                }
            }
        }
    }

    let uri = file_uri.expect("expected at least one file_with_uri artifact part");
    assert_eq!(filename.as_deref(), Some("report.txt"));
    assert!(
        uri.starts_with(&format!("http://127.0.0.1:{artifacts_port}/artifacts/")),
        "uri should reference artifacts server, got: {uri}",
    );

    let download = reqwest::get(&uri)
        .await
        .expect("download")
        .bytes()
        .await
        .expect("body");
    assert_eq!(download.as_ref(), payload.as_slice());

    let head = reqwest::Client::new()
        .head(&uri)
        .send()
        .await
        .expect("head");
    assert_eq!(head.status(), reqwest::StatusCode::OK);

    Arc::new(()); // tidy
    server_handle.abort();
    let _ = std::fs::remove_dir_all(&artifacts_root);
}

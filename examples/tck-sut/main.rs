use anyhow::anyhow;
use inference_gateway_adk::a2a_types::{
    AgentCard, Artifact, Message, Part, PartRaw, Role, StreamResponse, Task,
    TaskArtifactUpdateEvent, TaskState, TaskStatus, Timestamp,
};
use inference_gateway_adk::{A2AServerBuilder, StreamEmitter, StreamableTaskHandler, TaskHandler};
use serde_json::json;
use std::time::Duration;
use tracing::{error, info};

const PORT: u16 = 9999;
const STREAMING_TIMEOUT: Duration = Duration::from_secs(2);

const STREAMED_TEXT: [(&str, &str); 4] = [
    ("tck-stream-001", "Stream hello from TCK"),
    ("tck-stream-003", "Stream task lifecycle"),
    ("tck-stream-ordering-001", "Ordered output"),
    ("tck-stream-artifact-text", "Streamed text content"),
];

/// Implements the a2aproject/a2a-tck scenarios (scenarios/*.feature), selecting the
/// behaviour by the messageId prefix the TCK sends.
#[derive(Debug)]
struct TckHandler;

#[async_trait::async_trait]
impl TaskHandler for TckHandler {
    async fn handle_task(&self, task: Task, message: Option<Message>) -> anyhow::Result<Task> {
        run_core_scenario(task, &message_id(&message))
    }

    async fn handle_message(&self, message: &Message) -> anyhow::Result<Option<Message>> {
        if !message.message_id.starts_with("tck-message-response") {
            return Ok(None);
        }
        Ok(Some(Message {
            context_id: message.context_id.clone(),
            extensions: vec![],
            message_id: uuid::Uuid::new_v4().to_string(),
            metadata: None,
            parts: vec![text_part("Direct message response")],
            reference_task_ids: vec![],
            role: Role::RoleAgent,
            task_id: None,
        }))
    }
}

#[async_trait::async_trait]
impl StreamableTaskHandler for TckHandler {
    async fn handle_streaming_task(
        &self,
        task: Task,
        message: Option<Message>,
        emitter: StreamEmitter,
    ) -> anyhow::Result<()> {
        let id = message_id(&message);
        match id.as_str() {
            id if id.starts_with("tck-stream-002") => {}
            id if id.starts_with("test-resubscribe-message-id") => {
                emit_state(&emitter, &task, TaskState::TaskStateWorking).await?;
                tokio::time::sleep(2 * STREAMING_TIMEOUT).await;
            }
            id if id.starts_with("tck-stream-artifact-file") => {
                emit_state(&emitter, &task, TaskState::TaskStateWorking).await?;
                emit_artifact(&emitter, &task, new_artifact(file_part()), false, true).await?;
            }
            id if id.starts_with("tck-stream-artifact-chunked") => {
                emit_state(&emitter, &task, TaskState::TaskStateWorking).await?;
                let mut chunk = new_artifact(text_part("chunk-1 "));
                emit_artifact(&emitter, &task, chunk.clone(), false, false).await?;
                chunk.parts = vec![text_part("chunk-2")];
                emit_artifact(&emitter, &task, chunk, true, true).await?;
            }
            id if streamed_text(id).is_some() => {
                emit_state(&emitter, &task, TaskState::TaskStateWorking).await?;
                let text = streamed_text(id).unwrap_or_default();
                emit_artifact(&emitter, &task, new_artifact(text_part(text)), false, true).await?;
            }
            id => {
                let state = run_core_scenario(task.clone(), id)
                    .map_or(TaskState::TaskStateFailed, |t| t.status.state);
                return emit_state(&emitter, &task, state).await;
            }
        }
        emit_state(&emitter, &task, TaskState::TaskStateCompleted).await
    }
}

/// Runs the core_operations.feature scenario selected by the messageId prefix.
fn run_core_scenario(mut task: Task, id: &str) -> anyhow::Result<Task> {
    match id {
        id if id.starts_with("tck-reject-task") => return Err(anyhow!("rejected")),
        id if id.starts_with("tck-input-required") => {
            task.status = status(TaskState::TaskStateInputRequired, None);
            return Ok(task);
        }
        id if id.starts_with("tck-complete-task") => return Ok(complete(task, "Hello from TCK")),
        id if id.starts_with("tck-artifact-text") => {
            add_artifact(&mut task, text_part("Generated text content"))
        }
        id if id.starts_with("tck-artifact-file-url") => {
            add_artifact(&mut task, file_url_part("https://example.com/output.txt"))
        }
        id if id.starts_with("tck-artifact-file") => add_artifact(&mut task, file_part()),
        id if id.starts_with("tck-artifact-data") => add_artifact(
            &mut task,
            Part {
                data: Some(json!({"key": "value", "count": 42}).into()),
                ..Default::default()
            },
        ),
        id => return Ok(complete(task, &format!("Unhandled messageId prefix: {id}"))),
    }
    Ok(complete(task, ""))
}

fn message_id(message: &Option<Message>) -> String {
    message
        .as_ref()
        .map(|m| m.message_id.clone())
        .unwrap_or_default()
}

fn streamed_text(message_id: &str) -> Option<&'static str> {
    STREAMED_TEXT
        .iter()
        .find(|(prefix, _)| message_id.starts_with(prefix))
        .map(|(_, text)| *text)
}

fn status(state: TaskState, message: Option<Message>) -> TaskStatus {
    TaskStatus {
        message,
        state,
        timestamp: Some(Timestamp(chrono::Utc::now())),
    }
}

/// Marks the task completed, replying with `text` unless it is empty.
fn complete(mut task: Task, text: &str) -> Task {
    if text.is_empty() {
        task.status = status(TaskState::TaskStateCompleted, None);
        return task;
    }
    let reply = Message {
        context_id: task.context_id.clone(),
        extensions: vec![],
        message_id: uuid::Uuid::new_v4().to_string(),
        metadata: None,
        parts: vec![text_part(text)],
        reference_task_ids: vec![],
        role: Role::RoleAgent,
        task_id: Some(task.id.clone()),
    };
    task.history.push(reply.clone());
    task.status = status(TaskState::TaskStateCompleted, Some(reply));
    task
}

fn text_part(text: &str) -> Part {
    Part {
        text: Some(text.to_string()),
        ..Default::default()
    }
}

fn file_part() -> Part {
    Part {
        filename: Some("output.txt".to_string()),
        media_type: Some("text/plain".to_string()),
        raw: PartRaw::try_from("dGNr").ok(),
        ..Default::default()
    }
}

fn file_url_part(url: &str) -> Part {
    Part {
        filename: Some("output.txt".to_string()),
        media_type: Some("text/plain".to_string()),
        url: Some(url.to_string()),
        ..Default::default()
    }
}

fn new_artifact(part: Part) -> Artifact {
    Artifact {
        artifact_id: uuid::Uuid::new_v4().to_string(),
        description: None,
        extensions: vec![],
        metadata: None,
        name: None,
        parts: vec![part],
    }
}

fn add_artifact(task: &mut Task, part: Part) {
    task.artifacts.push(new_artifact(part));
}

async fn emit_state(emitter: &StreamEmitter, task: &Task, state: TaskState) -> anyhow::Result<()> {
    emitter
        .emit_status(&task.id, task.context_id_str(), state, None)
        .await
}

async fn emit_artifact(
    emitter: &StreamEmitter,
    task: &Task,
    artifact: Artifact,
    append: bool,
    last_chunk: bool,
) -> anyhow::Result<()> {
    emitter
        .emit(StreamResponse {
            artifact_update: Some(TaskArtifactUpdateEvent {
                append: Some(append),
                artifact,
                context_id: task.context_id_str().to_string(),
                last_chunk: Some(last_chunk),
                metadata: None,
                task_id: task.id.clone(),
            }),
            message: None,
            status_update: None,
            task: None,
        })
        .await
}

fn agent_card() -> serde_json::Result<AgentCard> {
    serde_json::from_value(json!({
        "name": "tck-sut",
        "description": "System under test for the A2A TCK",
        "version": "1.0.0",
        "supportedInterfaces": [{
            "url": format!("http://localhost:{PORT}/a2a"),
            "protocolBinding": "JSONRPC",
            "protocolVersion": "1.0"
        }],
        "capabilities": {"streaming": true, "pushNotifications": true},
        "defaultInputModes": ["text"],
        "defaultOutputModes": ["text"],
        "skills": [{
            "id": "tck",
            "name": "TCK Conformance",
            "description": "Handles TCK conformance test messages",
            "tags": ["tck"]
        }]
    }))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    let card = agent_card()?;
    let server = A2AServerBuilder::new()
        .with_agent_card(card.clone())
        .with_extended_agent_card(card)
        .with_background_task_handler(TckHandler)
        .with_streaming_task_handler(TckHandler)
        .build()
        .await?;

    let addr = format!("0.0.0.0:{PORT}").parse()?;
    info!("tck system under test listening on {addr}");

    if let Err(e) = server.serve(addr).await {
        error!("server stopped: {e}");
    }

    Ok(())
}

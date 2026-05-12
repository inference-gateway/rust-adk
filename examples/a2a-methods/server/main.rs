//! Shared server for the `a2a-methods` example collection.
//!
//! Hosts the JSON-RPC dispatch surface backed by an in-memory storage, with
//! no LLM agent configured. Both handler paths are explicit so the
//! request/response flow is visible in this file:
//!
//! * [`EchoBackgroundTaskHandler`] drives `message/send` (returns an echo
//!   reply on a Completed task).
//! * [`EchoStreamHandler`] drives `message/stream` (Submitted → Working →
//!   echo artifact → Completed, with short delays between transitions).
//!
//! No external dependencies are required.

use inference_gateway_adk::A2AServerBuilder;
use inference_gateway_adk::a2a_types::{
    AgentCard, Message, Part, Role, Task, TaskState, TaskStatus, Timestamp,
};
use inference_gateway_adk::{StreamEmitter, StreamableTaskHandler, TaskHandler};
use serde_json::json;
use std::env;
use std::time::Duration;
use tracing::{error, info};

fn echo_text(message: &Option<Message>) -> String {
    let user_text = message
        .as_ref()
        .map(|m| {
            m.parts
                .iter()
                .filter_map(|p| p.text.clone())
                .collect::<Vec<_>>()
                .join("\n")
        })
        .unwrap_or_default();
    format!("Echo: {user_text}")
}

fn build_agent_message(task: &Task, text: &str) -> Message {
    Message {
        context_id: Some(task.context_id.clone()),
        extensions: vec![],
        message_id: uuid::Uuid::new_v4().to_string(),
        metadata: None,
        parts: vec![Part {
            data: None,
            file: None,
            metadata: None,
            text: Some(text.to_string()),
        }],
        reference_task_ids: vec![],
        role: Role::RoleAgent,
        task_id: Some(task.id.clone()),
    }
}

/// Explicit `message/send` handler. Returns the task in `Completed` with an
/// echo of the user input attached as the final agent message.
#[derive(Debug, Default)]
struct EchoBackgroundTaskHandler;

#[async_trait::async_trait]
impl TaskHandler for EchoBackgroundTaskHandler {
    async fn handle_task(&self, mut task: Task, message: Option<Message>) -> anyhow::Result<Task> {
        let reply = build_agent_message(&task, &echo_text(&message));
        task.history.push(reply.clone());
        task.status = TaskStatus {
            message: Some(reply),
            state: TaskState::TaskStateCompleted,
            timestamp: Some(Timestamp(chrono::Utc::now())),
        };
        Ok(task)
    }
}

/// Explicit `message/stream` handler used by the example server.
///
/// The handler walks the freshly-created task through three observable
/// states so the example demonstrates streaming end-to-end:
///
///   1. `Working`            (after a short delay so the transition is visible)
///   2. echo `TaskArtifactUpdateEvent`
///   3. `Completed` with `final: true`
#[derive(Debug, Default)]
struct EchoStreamHandler;

#[async_trait::async_trait]
impl StreamableTaskHandler for EchoStreamHandler {
    async fn handle_streaming_task(
        &self,
        task: Task,
        message: Option<Message>,
        emitter: StreamEmitter,
    ) -> anyhow::Result<()> {
        // 1) Submitted → Working
        tokio::time::sleep(Duration::from_millis(250)).await;
        emitter
            .emit_status(
                &task.id,
                &task.context_id,
                TaskState::TaskStateWorking,
                None,
                false,
            )
            .await?;

        // 2) Produce + emit the echo reply as an artifact
        tokio::time::sleep(Duration::from_millis(400)).await;
        let reply_text = echo_text(&message);
        emitter
            .emit_text_artifact(&task.id, &task.context_id, reply_text.clone(), true)
            .await?;

        // 3) Final status: Completed (final = true)
        let reply_message = build_agent_message(&task, &reply_text);
        emitter
            .emit_status(
                &task.id,
                &task.context_id,
                TaskState::TaskStateCompleted,
                Some(reply_message),
                true,
            )
            .await
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    let port: u16 = env::var("SERVER_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8085);

    let agent_card: AgentCard = serde_json::from_value(json!({
        "name": "A2A Methods Example Agent",
        "description": "Reference server used by the per-method A2A client examples",
        "version": "0.1.0",
        "protocolVersion": "0.2.6",
        "url": format!("http://localhost:{port}/a2a"),
        "preferredTransport": "JSONRPC",
        "capabilities": {
            "streaming": true,
            "pushNotifications": true,
            "stateTransitionHistory": false
        },
        // Opt the example in to `agent/getAuthenticatedExtendedCard` so the
        // dedicated client example returns a card instead of METHOD_NOT_FOUND.
        "supportsExtendedAgentCard": true,
        "defaultInputModes": ["text/plain"],
        "defaultOutputModes": ["text/plain"],
        "skills": [
            {
                "id": "echo",
                "name": "echo",
                "description": "Echoes user input - sufficient for exercising every JSON-RPC method.",
                "tags": ["echo", "examples"]
            }
        ]
    }))?;

    let server = A2AServerBuilder::new()
        .with_agent_card(agent_card)
        // The gateway URL is required by the builder but is intentionally
        // unreachable: the per-method examples never trigger an LLM call.
        // Both handlers below are explicit so the request/response flow is
        // fully visible to readers of this example.
        .with_gateway_url("http://127.0.0.1:1/v1")
        .with_background_task_handler(EchoBackgroundTaskHandler)
        .with_streaming_task_handler(EchoStreamHandler)
        .build()
        .await?;

    let addr = format!("0.0.0.0:{port}").parse()?;
    info!("a2a-methods example server listening on port {port}");
    info!("JSON-RPC endpoint: http://localhost:{port}/a2a");

    if let Err(e) = server.serve(addr).await {
        error!("server failed: {e}");
    }

    Ok(())
}

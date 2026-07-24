use super::agent::Agent;
use super::artifact_service::ArtifactService;
use super::storage::Storage;
use super::usage_tracker::UsageTracker;
use crate::a2a_types::{
    Artifact, Message as A2AMessage, Part, Role, StreamResponse, Struct, Task,
    TaskArtifactUpdateEvent, TaskState, TaskStatus, TaskStatusUpdateEvent, Timestamp,
};
use anyhow::{Result, anyhow};
use futures_util::stream::StreamExt;
use inference_gateway_sdk::{CompletionUsage, Message, MessageContent, MessageRole};
use serde_json::{Map, Value};
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{debug, warn};

/// Handler invoked by the server for `message/send` requests.
///
/// Implementations receive a freshly-built task (already in
/// `TaskStateSubmitted`) plus the incoming user message, run the business
/// logic, and return the final task - typically with `state == Completed`
/// and an agent reply attached to `status.message`.
#[async_trait::async_trait]
pub trait TaskHandler: Send + Sync + std::fmt::Debug {
    async fn handle_task(&self, task: Task, message: Option<A2AMessage>) -> Result<Task>;
}

/// Handler invoked by the server for `message/stream` requests.
///
/// The server is responsible for parsing the request, persisting the initial
/// `Submitted` task, and emitting the first event (the `Task` wrapper). The
/// handler then drives the task to a terminal state by emitting
/// `StreamResponse` events via [`StreamEmitter`]. The last emitted event
/// **must** carry a `TaskStatusUpdateEvent` with `final: true`; otherwise
/// callers will treat the stream as unterminated.
#[async_trait::async_trait]
pub trait StreamableTaskHandler: Send + Sync + std::fmt::Debug {
    /// Drive a `message/stream` interaction.
    ///
    /// `task` is the freshly-built task already persisted in storage at
    /// `TaskStateSubmitted`. The handler should emit subsequent events
    /// (typically `Working` → optional artifact(s) → `Completed`).
    async fn handle_streaming_task(
        &self,
        task: Task,
        message: Option<A2AMessage>,
        emitter: StreamEmitter,
    ) -> Result<()>;
}

/// Emits `StreamResponse` events into an active `message/stream` response and
/// keeps the stored task in sync with the latest status.
#[derive(Clone)]
pub struct StreamEmitter {
    tx: mpsc::Sender<StreamResponse>,
    storage: Arc<dyn Storage>,
    artifact_service: Option<Arc<dyn ArtifactService>>,
}

impl std::fmt::Debug for StreamEmitter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StreamEmitter").finish_non_exhaustive()
    }
}

impl StreamEmitter {
    pub(super) fn new(tx: mpsc::Sender<StreamResponse>, storage: Arc<dyn Storage>) -> Self {
        Self {
            tx,
            storage,
            artifact_service: None,
        }
    }

    /// Attach an [`ArtifactService`] so the emitter can mint URI-bearing
    /// file/data artifacts (via [`emit_file_artifact`](Self::emit_file_artifact)
    /// / [`emit_data_artifact`](Self::emit_data_artifact)).
    pub(super) fn with_artifact_service(
        mut self,
        artifact_service: Option<Arc<dyn ArtifactService>>,
    ) -> Self {
        self.artifact_service = artifact_service;
        self
    }

    /// Access the underlying artifact service, when one is wired up.
    /// Streaming handlers can fall back to building their own artifacts
    /// when this returns `None`.
    pub fn artifact_service(&self) -> Option<Arc<dyn ArtifactService>> {
        self.artifact_service.clone()
    }

    /// Send a raw `StreamResponse` to the connected client.
    pub async fn emit(&self, response: StreamResponse) -> Result<()> {
        self.tx
            .send(response)
            .await
            .map_err(|_| anyhow!("stream receiver dropped before handler finished"))
    }

    /// Convenience helper that updates the stored task to `state` (attaching
    /// `message` to the task status if provided), then emits a
    /// `TaskStatusUpdateEvent` describing the new state.
    pub async fn emit_status(
        &self,
        task_id: &str,
        context_id: &str,
        state: TaskState,
        message: Option<A2AMessage>,
        final_: bool,
    ) -> Result<()> {
        let now = Timestamp(chrono::Utc::now());
        let new_status = TaskStatus {
            message: message.clone(),
            state,
            timestamp: Some(now),
        };

        if let Some(mut task) = self.storage.get_task(task_id).await {
            task.status = new_status.clone();
            if let Some(ref msg) = message {
                task.history.push(msg.clone());
            }
            self.storage.put_task(task).await;
        }

        let event = TaskStatusUpdateEvent {
            context_id: context_id.to_string(),
            final_,
            metadata: None,
            status: new_status,
            task_id: task_id.to_string(),
        };

        self.emit(StreamResponse {
            artifact_update: None,
            message: None,
            status_update: Some(event),
            task: None,
        })
        .await
    }

    /// Convenience helper that appends a text artifact to the stored task and
    /// emits a `TaskArtifactUpdateEvent` describing it.
    pub async fn emit_text_artifact(
        &self,
        task_id: &str,
        context_id: &str,
        text: impl Into<String>,
        last_chunk: bool,
    ) -> Result<()> {
        let artifact_id = uuid::Uuid::new_v4().to_string();
        let text = text.into();
        let artifact = Artifact {
            artifact_id: artifact_id.clone(),
            description: None,
            extensions: vec![],
            metadata: None,
            name: None,
            parts: vec![Part {
                data: None,
                file: None,
                metadata: None,
                text: Some(text),
            }],
        };

        if let Some(mut task) = self.storage.get_task(task_id).await {
            task.artifacts.push(artifact.clone());
            self.storage.put_task(task).await;
        }

        let event = TaskArtifactUpdateEvent {
            append: None,
            artifact,
            context_id: context_id.to_string(),
            last_chunk: Some(last_chunk),
            metadata: None,
            task_id: task_id.to_string(),
        };

        self.emit(StreamResponse {
            artifact_update: Some(event),
            message: None,
            status_update: None,
            task: None,
        })
        .await
    }

    /// Persist `data` through the configured [`ArtifactService`] and
    /// emit a [`TaskArtifactUpdateEvent`] whose [`FilePart`] carries a
    /// URI rather than inline bytes.
    ///
    /// Falls back to a [`FilePart`] with `fileWithBytes` when no
    /// [`ArtifactService`] is configured.
    ///
    /// [`FilePart`]: crate::a2a_types::FilePart
    pub async fn emit_file_artifact(
        &self,
        task_id: &str,
        context_id: &str,
        filename: &str,
        data: Vec<u8>,
        mime: Option<&str>,
        last_chunk: bool,
    ) -> Result<()> {
        let svc = match self.artifact_service.as_ref() {
            Some(s) => Arc::clone(s),
            None => Arc::new(super::artifact_service::DefaultArtifactService::without_storage())
                as Arc<dyn ArtifactService>,
        };
        let artifact = svc
            .create_file_artifact(filename, "", filename, data, mime)
            .await?;

        if let Some(mut task) = self.storage.get_task(task_id).await {
            task.artifacts.push(artifact.clone());
            self.storage.put_task(task).await;
        }

        let event = TaskArtifactUpdateEvent {
            append: None,
            artifact,
            context_id: context_id.to_string(),
            last_chunk: Some(last_chunk),
            metadata: None,
            task_id: task_id.to_string(),
        };
        self.emit(StreamResponse {
            artifact_update: Some(event),
            message: None,
            status_update: None,
            task: None,
        })
        .await
    }

    /// Build a structured-data artifact via the configured
    /// [`ArtifactService`] (or a service-less default) and emit a
    /// [`TaskArtifactUpdateEvent`] describing it.
    pub async fn emit_data_artifact(
        &self,
        task_id: &str,
        context_id: &str,
        name: &str,
        description: &str,
        data: serde_json::Value,
        last_chunk: bool,
    ) -> Result<()> {
        let svc: Arc<dyn ArtifactService> = match self.artifact_service.as_ref() {
            Some(s) => Arc::clone(s),
            None => Arc::new(super::artifact_service::DefaultArtifactService::without_storage()),
        };
        let artifact = svc.create_data_artifact(name, description, data);

        if let Some(mut task) = self.storage.get_task(task_id).await {
            task.artifacts.push(artifact.clone());
            self.storage.put_task(task).await;
        }

        let event = TaskArtifactUpdateEvent {
            append: None,
            artifact,
            context_id: context_id.to_string(),
            last_chunk: Some(last_chunk),
            metadata: None,
            task_id: task_id.to_string(),
        };
        self.emit(StreamResponse {
            artifact_update: Some(event),
            message: None,
            status_update: None,
            task: None,
        })
        .await
    }

    /// Merge the accumulated usage/execution statistics from `tracker` into the
    /// stored task's `metadata` field. Streaming handlers call this once, just
    /// before emitting the terminal status update, so the persisted task
    /// (returned by later `tasks/get` calls) carries the same `usage` /
    /// `execution_stats` blocks the background handler attaches.
    pub async fn populate_usage_metadata(&self, task_id: &str, tracker: &UsageTracker) {
        let usage = tracker.metadata();
        if usage.is_empty() {
            return;
        }
        if let Some(mut task) = self.storage.get_task(task_id).await {
            merge_usage_metadata(&mut task, usage);
            self.storage.put_task(task).await;
        }
    }
}

/// Merge a usage-metadata map into `task.metadata`, preserving any existing
/// keys. Used by both default handlers so the emitted `usage` /
/// `execution_stats` blocks are identical across the streaming and background
/// paths.
fn merge_usage_metadata(task: &mut Task, usage: Map<String, Value>) {
    if usage.is_empty() {
        return;
    }
    match task.metadata.as_mut() {
        Some(existing) => {
            for (key, value) in usage {
                existing.0.insert(key, value);
            }
        }
        None => task.metadata = Some(Struct(usage)),
    }
}

pub(super) fn build_agent_text_message(task: &Task, text: &str) -> A2AMessage {
    A2AMessage {
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

fn message_content_to_string(content: &MessageContent) -> String {
    match content {
        MessageContent::String(s) => s.clone(),
        MessageContent::Array(parts) => serde_json::to_string(parts).unwrap_or_default(),
    }
}

/// Translate the task history into the SDK message format expected by the
/// agent's [`LLMClient`]. Optionally prepends the agent's system prompt.
///
/// [`LLMClient`]: super::agent_llm_client::LLMClient
fn build_sdk_messages(agent: &Agent, task: &Task) -> Vec<Message> {
    let mut messages: Vec<Message> = Vec::new();
    if let Some(prompt) = agent.system_prompt.clone() {
        messages.push(Message {
            role: MessageRole::System,
            content: MessageContent::String(prompt),
            reasoning: None,
            reasoning_content: None,
            tool_call_id: None,
            tool_calls: Vec::new(),
        });
    }
    let max_history = agent.max_conversation_history as usize;
    let history = if max_history > 0 && task.history.len() > max_history {
        &task.history[task.history.len() - max_history..]
    } else {
        &task.history[..]
    };
    for a2a_msg in history {
        let text = a2a_msg
            .parts
            .iter()
            .filter_map(|p| p.text.clone())
            .collect::<Vec<_>>()
            .join("");
        if text.is_empty() {
            continue;
        }
        let role = match a2a_msg.role {
            Role::RoleAgent => MessageRole::Assistant,
            _ => MessageRole::User,
        };
        messages.push(Message {
            role,
            content: MessageContent::String(text),
            reasoning: None,
            reasoning_content: None,
            tool_call_id: None,
            tool_calls: Vec::new(),
        });
    }
    messages
}

/// Static message returned by the default handlers when no agent is
/// configured.
const NO_AGENT_REPLY: &str = "I received your message. I'm a default task handler without AI capabilities. \
     To enable AI responses, configure an OpenAI-compatible agent via \
     `A2AServerBuilder::with_agent(...)`.";

/// Outcome of [`run_tool_loop`]. Carries the conversation buffer (with all
/// assistant tool-call messages + tool result messages appended in order)
/// plus the final assistant text the model returned once it stopped
/// invoking tools, and a flag indicating whether the loop hit the iteration
/// cap.
struct ToolLoopOutcome {
    messages: Vec<Message>,
    final_text: String,
    exhausted: bool,
}

/// Drive a non-streaming "model call → execute tool_calls → feed results
/// back" loop up to `agent.max_chat_completion()` iterations. The default
/// task handlers use this to bridge the gap between the inference gateway
/// (which only emits raw OpenAI-style tool_calls) and the registered
/// [`ToolHandler`] implementations on the agent.
///
/// Tool activity is silent at the wire level - which
/// only debug-logs tool lifecycle events from inside its
/// `DefaultBackgroundTaskHandler` instead of forwarding them as A2A
/// `TaskStatusUpdate` events (the A2A spec has no tool-event variant).
async fn run_tool_loop(
    agent: &Agent,
    mut messages: Vec<Message>,
    tracker: &UsageTracker,
) -> Result<ToolLoopOutcome> {
    let llm = agent.llm_client();
    let tools = agent.toolbox.clone();
    let max_iterations = agent.max_chat_completion().max(1) as usize;

    for _ in 0..max_iterations {
        tracker.increment_iteration();

        let response = llm
            .create_chat_completion(messages.clone(), tools.clone())
            .await
            .map_err(|e| anyhow!("llm call failed: {e}"))?;

        if let Some(usage) = response.usage.as_ref() {
            tracker.add_token_usage(usage);
        }

        let Some(choice) = response.choices.into_iter().next() else {
            return Ok(ToolLoopOutcome {
                messages,
                final_text: String::new(),
                exhausted: false,
            });
        };

        let assistant_text = message_content_to_string(&choice.message.content);
        let tool_calls = choice.message.tool_calls.clone();
        let reasoning = choice.message.reasoning.clone();
        let reasoning_content = choice.message.reasoning_content.clone();

        messages.push(Message {
            role: MessageRole::Assistant,
            content: MessageContent::String(assistant_text.clone()),
            reasoning,
            reasoning_content,
            tool_call_id: None,
            tool_calls: tool_calls.clone(),
        });

        if tool_calls.is_empty() {
            return Ok(ToolLoopOutcome {
                messages,
                final_text: assistant_text,
                exhausted: false,
            });
        }

        let num_tool_calls = tool_calls.len();
        for tool_call in tool_calls {
            let tool_name = tool_call.function.name.clone();
            let args: Value = serde_json::from_str(&tool_call.function.arguments)
                .unwrap_or_else(|_| Value::String(tool_call.function.arguments.clone()));

            debug!("tool dispatch: {tool_name}");
            tracker.increment_tool_calls();

            let tool_result = match agent.tool_handler(&tool_name) {
                Some(handler) => match handler.handle(args).await {
                    Ok(value) => value,
                    Err(e) => {
                        tracker.increment_failed_tools();
                        format!("tool `{tool_name}` failed: {e}")
                    }
                },
                None => {
                    tracker.increment_failed_tools();
                    format!("no handler registered for tool `{tool_name}`")
                }
            };

            messages.push(Message {
                role: MessageRole::Tool,
                content: MessageContent::String(tool_result),
                reasoning: None,
                reasoning_content: None,
                tool_call_id: Some(tool_call.id.clone()),
                tool_calls: Vec::new(),
            });
        }
        tracker.add_messages(num_tool_calls);
    }

    Ok(ToolLoopOutcome {
        messages,
        final_text: String::new(),
        exhausted: true,
    })
}

/// Opt-in default `message/send` handler wired up by
/// [`A2AServerBuilder::with_default_background_task_handler`] /
/// [`A2AServerBuilder::with_default_task_handlers`].
///
/// When an [`Agent`] is configured, delegates to the inference gateway via a
/// single non-streaming `generate_content` call and returns the resulting
/// task with `state == Completed` and the reply attached. Without an agent,
/// returns the static [`NO_AGENT_REPLY`] message - `processWithoutAgentBackground`.
#[derive(Debug)]
pub struct DefaultBackgroundTaskHandler {
    agent: Option<Arc<Agent>>,
    enable_usage_metadata: bool,
}

impl DefaultBackgroundTaskHandler {
    pub fn new(agent: Option<Arc<Agent>>) -> Self {
        let enable_usage_metadata = agent
            .as_ref()
            .map(|a| a.usage_metadata_enabled())
            .unwrap_or(true);
        Self {
            agent,
            enable_usage_metadata,
        }
    }

    /// Override whether terminal tasks carry `usage` / `execution_stats`
    /// metadata. [`A2AServerBuilder`](crate::A2AServerBuilder) calls this to
    /// honour [`AgentConfig::enable_usage_metadata`](crate::AgentConfig).
    pub fn set_enable_usage_metadata(&mut self, enable: bool) {
        self.enable_usage_metadata = enable;
    }

    /// Whether terminal tasks will carry usage/execution metadata.
    pub fn is_usage_metadata_enabled(&self) -> bool {
        self.enable_usage_metadata
    }
}

#[async_trait::async_trait]
impl TaskHandler for DefaultBackgroundTaskHandler {
    async fn handle_task(&self, mut task: Task, _message: Option<A2AMessage>) -> Result<Task> {
        let tracker = UsageTracker::new();
        let (reply_text, terminal_state) = match self.agent.as_ref() {
            Some(agent) => {
                let messages = build_sdk_messages(agent, &task);
                match run_tool_loop(agent, messages, &tracker).await {
                    Ok(outcome) if outcome.exhausted => {
                        warn!(
                            "default background handler: tool loop exhausted \
                             after {} iterations without a final answer",
                            agent.max_chat_completion()
                        );
                        (
                            "Tool loop exhausted before the model produced a \
                             final answer."
                                .to_string(),
                            TaskState::TaskStateFailed,
                        )
                    }
                    Ok(outcome) => {
                        let text = if outcome.final_text.is_empty() {
                            "Task completed".to_string()
                        } else {
                            outcome.final_text
                        };
                        (text, TaskState::TaskStateCompleted)
                    }
                    Err(e) => {
                        warn!("default background handler: agent call failed: {e}");
                        (
                            format!("Agent call failed: {e}"),
                            TaskState::TaskStateFailed,
                        )
                    }
                }
            }
            None => (NO_AGENT_REPLY.to_string(), TaskState::TaskStateCompleted),
        };

        let reply = build_agent_text_message(&task, &reply_text);
        task.history.push(reply.clone());
        task.status = TaskStatus {
            message: Some(reply),
            state: terminal_state,
            timestamp: Some(Timestamp(chrono::Utc::now())),
        };

        if self.enable_usage_metadata && tracker.has_usage() {
            merge_usage_metadata(&mut task, tracker.metadata());
        }

        Ok(task)
    }
}

/// Opt-in default `message/stream` handler wired up by
/// [`A2AServerBuilder::with_default_streaming_task_handler`] /
/// [`A2AServerBuilder::with_default_task_handlers`].
///
/// When an [`Agent`] is configured, the handler iterates `generate_content_stream`
/// from the inference gateway, parses each OpenAI-style delta, and emits a
/// [`TaskArtifactUpdateEvent`] per non-empty content chunk (`append: true`,
/// shared `artifact_id`) - clients see the reply build up in real time. The
/// stream terminates with a final `last_chunk: true` artifact + a
/// `Completed` status update.
///
/// Without an agent, emits a single instructional artifact + `Completed`
/// so the bundled defaults remain usable for examples and tests.
#[derive(Debug)]
pub struct DefaultStreamingTaskHandler {
    agent: Option<Arc<Agent>>,
    enable_usage_metadata: bool,
}

impl DefaultStreamingTaskHandler {
    pub fn new(agent: Option<Arc<Agent>>) -> Self {
        let enable_usage_metadata = agent
            .as_ref()
            .map(|a| a.usage_metadata_enabled())
            .unwrap_or(true);
        Self {
            agent,
            enable_usage_metadata,
        }
    }

    /// Override whether terminal tasks carry `usage` / `execution_stats`
    /// metadata. [`A2AServerBuilder`](crate::A2AServerBuilder) calls this to
    /// honour [`AgentConfig::enable_usage_metadata`](crate::AgentConfig).
    pub fn set_enable_usage_metadata(&mut self, enable: bool) {
        self.enable_usage_metadata = enable;
    }

    /// Whether terminal tasks will carry usage/execution metadata.
    pub fn is_usage_metadata_enabled(&self) -> bool {
        self.enable_usage_metadata
    }
}

#[async_trait::async_trait]
impl StreamableTaskHandler for DefaultStreamingTaskHandler {
    async fn handle_streaming_task(
        &self,
        task: Task,
        _message: Option<A2AMessage>,
        emitter: StreamEmitter,
    ) -> Result<()> {
        emitter
            .emit_status(
                &task.id,
                &task.context_id,
                TaskState::TaskStateWorking,
                None,
                false,
            )
            .await?;

        let tracker = UsageTracker::new();
        let final_text = match self.agent.as_ref() {
            Some(agent) => stream_agent_deltas(agent, &task, &emitter, &tracker).await?,
            None => {
                emitter
                    .emit_text_artifact(&task.id, &task.context_id, NO_AGENT_REPLY, true)
                    .await?;
                NO_AGENT_REPLY.to_string()
            }
        };

        if self.enable_usage_metadata && tracker.has_usage() {
            emitter.populate_usage_metadata(&task.id, &tracker).await;
        }

        let reply_message = build_agent_text_message(&task, &final_text);
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

/// Drive `generate_content_stream` and forward each delta chunk to
/// `emitter` as an incremental [`TaskArtifactUpdateEvent`] sharing a single
/// `artifact_id`. Returns the accumulated reply text on success. On gateway
/// failure, the helper falls back to a one-shot error artifact so the
/// stream still terminates cleanly.
///
/// When the agent advertises tools, this helper first runs a non-streaming
/// [`run_tool_loop`] preflight so any `tool_calls` the model emits get
/// dispatched to registered [`ToolHandler`] implementations. Once the model
/// stops requesting tools (or the iteration cap is hit), the final answer
/// is fetched via `generate_content_stream` and delivered as deltas.
async fn stream_agent_deltas(
    agent: &Agent,
    task: &Task,
    emitter: &StreamEmitter,
    tracker: &UsageTracker,
) -> Result<String> {
    let base_messages = build_sdk_messages(agent, task);

    let messages = if agent.toolbox().is_some() {
        match run_tool_loop(agent, base_messages, tracker).await {
            Ok(outcome) if outcome.exhausted => {
                let msg = "Tool loop exhausted before the model produced a \
                           final answer."
                    .to_string();
                emitter
                    .emit_text_artifact(&task.id, &task.context_id, &msg, true)
                    .await?;
                return Ok(msg);
            }
            Ok(outcome) => {
                if !outcome.final_text.is_empty()
                    && outcome
                        .messages
                        .last()
                        .map(|m| m.tool_calls.is_empty())
                        .unwrap_or(true)
                {
                    emitter
                        .emit_text_artifact(&task.id, &task.context_id, &outcome.final_text, true)
                        .await?;
                    return Ok(outcome.final_text);
                }
                outcome.messages
            }
            Err(e) => {
                warn!("default streaming handler: tool loop failed: {e}");
                let msg = format!("Agent stream failed: {e}");
                emitter
                    .emit_text_artifact(&task.id, &task.context_id, &msg, true)
                    .await?;
                return Ok(msg);
            }
        }
    } else {
        base_messages
    };

    let llm = agent.llm_client();
    let tools = agent.toolbox.clone();
    // The streaming tail is one more model round-trip; count it so a turn that
    // never entered the tool loop (e.g. an agent without tools) still reports
    // at least one iteration, matching the Go ADK.
    tracker.increment_iteration();
    let mut stream = llm.create_streaming_chat_completion(messages, tools);

    let artifact_id = uuid::Uuid::new_v4().to_string();
    let mut buffer = String::new();

    while let Some(item) = stream.next().await {
        let event = match item {
            Ok(e) => e,
            Err(e) => {
                warn!("default streaming handler: gateway error: {e}");
                let msg = format!("Agent stream failed: {e}");
                emitter
                    .emit_text_artifact(&task.id, &task.context_id, &msg, true)
                    .await?;
                return Ok(msg);
            }
        };

        let data = event.data.trim();
        if data.is_empty() || data == "[DONE]" {
            if data == "[DONE]" {
                break;
            }
            continue;
        }

        let parsed: serde_json::Value = match serde_json::from_str(data) {
            Ok(v) => v,
            Err(_) => continue,
        };

        if let Some(usage_value) = parsed.get("usage").filter(|v| !v.is_null())
            && let Ok(usage) = serde_json::from_value::<CompletionUsage>(usage_value.clone())
        {
            tracker.add_token_usage(&usage);
        }

        let Some(text) = parsed
            .get("choices")
            .and_then(|c| c.as_array())
            .and_then(|arr| arr.first())
            .and_then(|c| c.get("delta"))
            .and_then(|d| d.get("content"))
            .and_then(|t| t.as_str())
        else {
            continue;
        };
        if text.is_empty() {
            continue;
        }
        buffer.push_str(text);

        let chunk_event = TaskArtifactUpdateEvent {
            append: Some(true),
            artifact: Artifact {
                artifact_id: artifact_id.clone(),
                description: None,
                extensions: vec![],
                metadata: None,
                name: None,
                parts: vec![Part {
                    data: None,
                    file: None,
                    metadata: None,
                    text: Some(text.to_string()),
                }],
            },
            context_id: task.context_id.clone(),
            last_chunk: Some(false),
            metadata: None,
            task_id: task.id.clone(),
        };
        emitter
            .emit(StreamResponse {
                artifact_update: Some(chunk_event),
                message: None,
                status_update: None,
                task: None,
            })
            .await?;
    }

    let final_event = TaskArtifactUpdateEvent {
        append: Some(true),
        artifact: Artifact {
            artifact_id,
            description: None,
            extensions: vec![],
            metadata: None,
            name: None,
            parts: vec![],
        },
        context_id: task.context_id.clone(),
        last_chunk: Some(true),
        metadata: None,
        task_id: task.id.clone(),
    };
    emitter
        .emit(StreamResponse {
            artifact_update: Some(final_event),
            message: None,
            status_update: None,
            task: None,
        })
        .await?;

    Ok(buffer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::a2a_types::{AgentCard, Role, SendMessageRequest};
    use crate::server::agent_builder::AgentBuilder;
    use crate::server::protocol::{AppState, a2a_handler};
    use crate::server::server_builder::A2AServerBuilder;
    use axum::Router;
    use axum::extract::State;
    use axum::response::Json;
    use axum::routing::post;
    use inference_gateway_sdk::{
        ChatCompletionTool, ChatCompletionToolType, FunctionObject, FunctionParameters,
    };
    use tokio::net::TcpListener;

    fn agent_card_with_streaming(streaming: bool) -> AgentCard {
        serde_json::from_value(serde_json::json!({
            "name": "Validation Agent",
            "description": "Builder validation tests",
            "version": "0.0.0",
            "protocolVersion": "0.2.6",
            "url": "http://localhost/a2a",
            "preferredTransport": "JSONRPC",
            "capabilities": {
                "streaming": streaming,
                "pushNotifications": false,
                "stateTransitionHistory": false
            },
            "defaultInputModes": ["text/plain"],
            "defaultOutputModes": ["text/plain"],
            "skills": [
                {"id": "x", "name": "x", "description": "x", "tags": ["x"]}
            ]
        }))
        .unwrap()
    }

    /// Drive the `DefaultStreamingTaskHandler` against a mock OpenAI-compatible
    /// gateway and verify the handler iterates the delta stream, emitting an
    /// incremental artifact event per non-empty content chunk (all sharing a
    /// single artifact_id with `append: true`), terminating with `last_chunk:
    /// true` and a `Completed` status whose message carries the accumulated
    /// reply.
    #[tokio::test]
    async fn default_streaming_handler_iterates_gateway_deltas() {
        use crate::A2AClient;
        use crate::a2a_types::Message as A2AMessage;
        use crate::config::AgentConfig;
        use axum::response::sse::{Event as SseEvent, KeepAlive as SseKeepAlive, Sse as SseResp};
        use futures_util::StreamExt as _;

        // ----- Mock OpenAI-compatible gateway --------------------------------
        async fn chat_completions() -> SseResp<
            impl futures_util::Stream<Item = std::result::Result<SseEvent, std::convert::Infallible>>,
        > {
            let deltas = [
                serde_json::json!({"choices":[{"delta":{"content":"Hel"}}]}).to_string(),
                serde_json::json!({"choices":[{"delta":{"content":"lo "}}]}).to_string(),
                serde_json::json!({"choices":[{"delta":{"content":"world"}}]}).to_string(),
                "[DONE]".to_string(),
            ];
            let stream = futures_util::stream::iter(
                deltas
                    .into_iter()
                    .map(|d| Ok::<_, std::convert::Infallible>(SseEvent::default().data(d))),
            );
            SseResp::new(stream).keep_alive(SseKeepAlive::default())
        }

        let gateway_listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind gateway");
        let gateway_addr = gateway_listener.local_addr().expect("addr");
        let gateway_app = Router::new().route("/chat/completions", post(chat_completions));
        tokio::spawn(async move {
            axum::serve(gateway_listener, gateway_app).await.ok();
        });

        // ----- A2A server using DefaultStreamingTaskHandler ------------------
        let agent_card = agent_card_with_streaming(true);
        let agent_config = AgentConfig {
            provider: "openai".to_string(),
            model: "test-model".to_string(),
            base_url: Some(format!("http://{gateway_addr}")),
            ..AgentConfig::default()
        };
        let agent = AgentBuilder::new()
            .with_config(&agent_config)
            .build()
            .await
            .expect("agent builds");

        let server = A2AServerBuilder::new()
            .with_agent_card(agent_card)
            .with_agent(agent)
            .with_default_task_handlers()
            .build()
            .await
            .expect("server builds");

        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind a2a");
        let addr = listener.local_addr().expect("addr");
        let app = Router::new()
            .route("/a2a", post(a2a_handler))
            .with_state(Arc::new(AppState::new(server)));
        tokio::spawn(async move {
            axum::serve(listener, app).await.ok();
        });

        let client = A2AClient::new(format!("http://{addr}")).expect("client");

        let request = SendMessageRequest {
            configuration: None,
            message: Some(A2AMessage {
                context_id: None,
                extensions: vec![],
                message_id: "msg-default-stream".to_string(),
                metadata: None,
                parts: vec![Part {
                    data: None,
                    file: None,
                    metadata: None,
                    text: Some("hi".to_string()),
                }],
                reference_task_ids: vec![],
                role: Role::RoleUser,
                task_id: None,
            }),
            metadata: None,
            tenant: "tests".to_string(),
        };

        let mut stream = Box::pin(client.stream_message(request).await.expect("stream"));
        let mut events: Vec<StreamResponse> = Vec::new();
        while let Some(item) = stream.next().await {
            events.push(item.expect("event"));
        }

        assert_eq!(
            events.len(),
            7,
            "unexpected event count {}: {:?}",
            events.len(),
            events
        );

        assert!(events[0].task.is_some(), "first event carries task");
        let working = events[1]
            .status_update
            .as_ref()
            .expect("second event is status update");
        assert_eq!(working.status.state, TaskState::TaskStateWorking);
        assert!(!working.final_);

        let mut artifact_ids = std::collections::HashSet::new();
        let chunks: Vec<String> = (2..=4)
            .map(|i| {
                let upd = events[i]
                    .artifact_update
                    .as_ref()
                    .unwrap_or_else(|| panic!("event[{i}] should be an artifact update"));
                assert_eq!(upd.append, Some(true), "deltas must have append=true");
                assert_eq!(upd.last_chunk, Some(false));
                artifact_ids.insert(upd.artifact.artifact_id.clone());
                upd.artifact
                    .parts
                    .iter()
                    .filter_map(|p| p.text.clone())
                    .collect::<String>()
            })
            .collect();
        assert_eq!(chunks, vec!["Hel", "lo ", "world"]);
        assert_eq!(
            artifact_ids.len(),
            1,
            "all deltas must share a single artifact_id"
        );

        let terminal_artifact = events[5]
            .artifact_update
            .as_ref()
            .expect("event[5] should be the terminal artifact chunk");
        assert_eq!(terminal_artifact.last_chunk, Some(true));
        assert!(
            terminal_artifact.artifact.parts.is_empty(),
            "terminal chunk should have empty parts"
        );
        assert_eq!(
            artifact_ids.iter().next().unwrap(),
            &terminal_artifact.artifact.artifact_id,
            "terminal chunk must share artifact_id with deltas"
        );

        let completed = events[6]
            .status_update
            .as_ref()
            .expect("event[6] should be the Completed status");
        assert_eq!(completed.status.state, TaskState::TaskStateCompleted);
        assert!(completed.final_);
        let assembled = completed
            .status
            .message
            .as_ref()
            .expect("completed status carries the final message")
            .parts
            .iter()
            .filter_map(|p| p.text.clone())
            .collect::<String>();
        assert_eq!(assembled, "Hello world");
    }

    // ----- tool-dispatch coverage -------------------------------------------

    #[derive(Clone, Default)]
    struct ToolMockState {
        non_streaming_calls: std::sync::Arc<std::sync::atomic::AtomicUsize>,
        captured_tool_results: std::sync::Arc<std::sync::Mutex<Vec<String>>>,
    }

    fn tool_call_response_json() -> serde_json::Value {
        serde_json::json!({
            "id": "chatcmpl-tool",
            "object": "chat.completion",
            "created": 0,
            "model": "test-model",
            "choices": [{
                "index": 0,
                "finish_reason": "tool_calls",
                "message": {
                    "role": "assistant",
                    "content": "",
                    "tool_calls": [{
                        "id": "call_1",
                        "type": "function",
                        "function": {
                            "name": "echo_arg",
                            "arguments": "{\"text\":\"hi\"}",
                        }
                    }],
                },
            }],
        })
    }

    fn final_answer_response_json(text: &str) -> serde_json::Value {
        serde_json::json!({
            "id": "chatcmpl-final",
            "object": "chat.completion",
            "created": 0,
            "model": "test-model",
            "choices": [{
                "index": 0,
                "finish_reason": "stop",
                "message": {
                    "role": "assistant",
                    "content": text,
                    "tool_calls": [],
                },
            }],
        })
    }

    async fn mock_non_streaming(
        State(state): State<std::sync::Arc<ToolMockState>>,
        body: Value,
    ) -> Json<Value> {
        if let Some(msgs) = body.get("messages").and_then(|v| v.as_array()) {
            for m in msgs {
                if m.get("role").and_then(|v| v.as_str()) == Some("tool")
                    && let Some(text) = m.get("content").and_then(|v| v.as_str())
                {
                    state
                        .captured_tool_results
                        .lock()
                        .expect("mutex poisoned")
                        .push(text.to_string());
                }
            }
        }
        let call_index = state
            .non_streaming_calls
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        if call_index == 0 {
            Json(tool_call_response_json())
        } else {
            Json(final_answer_response_json("12 is the tool result"))
        }
    }

    /// Single dispatcher: the tool loop is non-streaming end-to-end (since
    /// the inference gateway's OpenAI surface only exposes
    /// `CreateChatCompletion`), so this mock only needs to serve the two
    /// non-streaming responses in order.
    async fn mock_chat_completions(
        State(state): State<std::sync::Arc<ToolMockState>>,
        body: axum::body::Bytes,
    ) -> Json<Value> {
        let parsed: Value = serde_json::from_slice(&body).expect("valid JSON");
        mock_non_streaming(State(state), parsed).await
    }

    async fn build_echo_agent_with_recorder(
        gateway_url: String,
    ) -> (Agent, std::sync::Arc<std::sync::Mutex<Vec<String>>>) {
        use crate::config::AgentConfig;

        let recorded = std::sync::Arc::new(std::sync::Mutex::new(Vec::<String>::new()));
        let recorded_clone = std::sync::Arc::clone(&recorded);

        let echo_tool = ChatCompletionTool {
            type_: ChatCompletionToolType::Function,
            function: FunctionObject {
                name: "echo_arg".to_string(),
                description: Some("echo back the text arg".to_string()),
                parameters: Some(FunctionParameters(
                    serde_json::json!({
                        "type": "object",
                        "properties": {"text": {"type": "string"}},
                        "required": ["text"],
                    })
                    .as_object()
                    .unwrap()
                    .clone(),
                )),
                strict: false,
            },
        };

        let agent_cfg = AgentConfig {
            provider: "openai".to_string(),
            model: "test-model".to_string(),
            base_url: Some(gateway_url),
            ..AgentConfig::default()
        };

        let agent = AgentBuilder::new()
            .with_config(&agent_cfg)
            .with_toolbox(vec![echo_tool])
            .with_async_function_tool("echo_arg".to_string(), move |args: Value| {
                let recorded = std::sync::Arc::clone(&recorded_clone);
                async move {
                    let text = args
                        .get("text")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    recorded.lock().expect("mutex poisoned").push(text.clone());
                    Ok(format!("echoed: {text}"))
                }
            })
            .build()
            .await
            .expect("agent builds");
        (agent, recorded)
    }

    #[tokio::test]
    async fn default_background_handler_dispatches_tool_calls() {
        use crate::A2AClient;
        use crate::a2a_types::Message as A2AMessage;

        let mock_state = std::sync::Arc::new(ToolMockState::default());
        let gateway_listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let gateway_addr = gateway_listener.local_addr().expect("addr");
        let gateway_app = Router::new()
            .route("/chat/completions", post(mock_chat_completions))
            .with_state(std::sync::Arc::clone(&mock_state));
        tokio::spawn(async move {
            axum::serve(gateway_listener, gateway_app).await.ok();
        });

        let (agent, recorded) =
            build_echo_agent_with_recorder(format!("http://{gateway_addr}")).await;
        let card = agent_card_with_streaming(false);

        let mut server = A2AServerBuilder::new()
            .with_agent_card(card)
            .with_agent(agent)
            .with_default_background_task_handler()
            .build()
            .await
            .expect("server builds");

        let runner = server
            .task_manager
            .take()
            .expect("task manager configured for background handler")
            .start();

        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind a2a");
        let addr = listener.local_addr().expect("a2a addr");
        let app = Router::new()
            .route("/a2a", post(a2a_handler))
            .with_state(Arc::new(AppState::new(server)));
        tokio::spawn(async move {
            axum::serve(listener, app).await.ok();
        });

        let client = A2AClient::new(format!("http://{addr}")).expect("client");
        let response = client
            .send_message(SendMessageRequest {
                configuration: None,
                message: Some(A2AMessage {
                    context_id: None,
                    extensions: vec![],
                    message_id: "msg-bg-tool".to_string(),
                    metadata: None,
                    parts: vec![Part {
                        data: None,
                        file: None,
                        metadata: None,
                        text: Some("ask".to_string()),
                    }],
                    reference_task_ids: vec![],
                    role: Role::RoleUser,
                    task_id: None,
                }),
                metadata: None,
                tenant: "tests".to_string(),
            })
            .await
            .expect("message/send");

        let submitted = response.task.expect("task in response");
        assert_eq!(submitted.status.state, TaskState::TaskStateSubmitted);

        let final_task = poll_until_terminal(&client, &submitted.id).await;
        assert_eq!(final_task.status.state, TaskState::TaskStateCompleted);
        let final_text = final_task
            .status
            .message
            .expect("final agent message")
            .parts
            .iter()
            .filter_map(|p| p.text.clone())
            .collect::<String>();
        assert_eq!(final_text, "12 is the tool result");

        assert_eq!(
            recorded.lock().expect("mutex poisoned").clone(),
            vec!["hi".to_string()],
            "echo_arg should fire exactly once with the model-supplied argument",
        );
        assert_eq!(
            mock_state
                .captured_tool_results
                .lock()
                .expect("mutex poisoned")
                .clone(),
            vec!["echoed: hi".to_string()],
            "second gateway call should include the tool result as a Tool-role message",
        );

        runner.shutdown().await;
    }

    /// Poll `tasks/get` until the task reaches a terminal state, with a
    /// per-test timeout. Used by the queue-driven `message/send` tests
    /// that need to wait for the background worker to complete.
    async fn poll_until_terminal(client: &crate::A2AClient, task_id: &str) -> Task {
        for _ in 0..100 {
            let fetched = client
                .get_task(crate::a2a_types::GetTaskRequest {
                    history_length: None,
                    name: format!("tasks/{task_id}"),
                    tenant: Some("tests".to_string()),
                })
                .await
                .expect("tasks/get");
            if matches!(
                fetched.status.state,
                TaskState::TaskStateCompleted
                    | TaskState::TaskStateFailed
                    | TaskState::TaskStateCancelled
                    | TaskState::TaskStateRejected
            ) {
                return fetched;
            }
            tokio::time::sleep(std::time::Duration::from_millis(20)).await;
        }
        panic!("task {task_id} never reached terminal state within 2s");
    }

    #[tokio::test]
    async fn default_streaming_handler_dispatches_tool_calls() {
        use crate::A2AClient;
        use crate::a2a_types::Message as A2AMessage;
        use futures_util::StreamExt;

        let mock_state = std::sync::Arc::new(ToolMockState::default());
        let gateway_listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let gateway_addr = gateway_listener.local_addr().expect("addr");
        let gateway_app = Router::new()
            .route("/chat/completions", post(mock_chat_completions))
            .with_state(std::sync::Arc::clone(&mock_state));
        tokio::spawn(async move {
            axum::serve(gateway_listener, gateway_app).await.ok();
        });

        let (agent, recorded) =
            build_echo_agent_with_recorder(format!("http://{gateway_addr}")).await;
        let card = agent_card_with_streaming(true);

        let server = A2AServerBuilder::new()
            .with_agent_card(card)
            .with_agent(agent)
            .with_default_streaming_task_handler()
            .build()
            .await
            .expect("server builds");

        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind a2a");
        let addr = listener.local_addr().expect("a2a addr");
        let app = Router::new()
            .route("/a2a", post(a2a_handler))
            .with_state(Arc::new(AppState::new(server)));
        tokio::spawn(async move {
            axum::serve(listener, app).await.ok();
        });

        let client = A2AClient::new(format!("http://{addr}")).expect("client");
        let request = SendMessageRequest {
            configuration: None,
            message: Some(A2AMessage {
                context_id: None,
                extensions: vec![],
                message_id: "msg-stream-tool".to_string(),
                metadata: None,
                parts: vec![Part {
                    data: None,
                    file: None,
                    metadata: None,
                    text: Some("ask".to_string()),
                }],
                reference_task_ids: vec![],
                role: Role::RoleUser,
                task_id: None,
            }),
            metadata: None,
            tenant: "tests".to_string(),
        };

        let mut stream = Box::pin(client.stream_message(request).await.expect("stream"));
        let mut events: Vec<StreamResponse> = Vec::new();
        while let Some(item) = stream.next().await {
            events.push(item.expect("event"));
        }

        assert_eq!(
            recorded.lock().expect("mutex poisoned").clone(),
            vec!["hi".to_string()],
            "echo_arg should fire once during the tool-loop preflight"
        );

        let saw_tool_status = events.iter().any(|e| {
            e.status_update
                .as_ref()
                .and_then(|u| u.status.message.as_ref())
                .map(|m| {
                    m.parts
                        .iter()
                        .filter_map(|p| p.text.clone())
                        .any(|t| t.contains("calling tool"))
                })
                .unwrap_or(false)
        });
        assert!(
            !saw_tool_status,
            "stream should NOT carry tool-lifecycle status updates",
        );

        let accumulated: String = events
            .iter()
            .filter_map(|e| e.artifact_update.as_ref())
            .flat_map(|a| {
                a.artifact
                    .parts
                    .iter()
                    .filter_map(|p| p.text.clone())
                    .collect::<Vec<_>>()
            })
            .collect::<String>();
        assert_eq!(accumulated, "12 is the tool result");

        let last = events.last().expect("at least one event");
        let last_status = last
            .status_update
            .as_ref()
            .expect("last event is a status update");
        assert_eq!(last_status.status.state, TaskState::TaskStateCompleted);
        assert!(last_status.final_);
    }

    // ----- usage-metadata coverage ------------------------------------------

    fn submitted_usage_task(text: &str) -> Task {
        Task {
            artifacts: vec![],
            context_id: "ctx-usage".to_string(),
            history: vec![A2AMessage {
                context_id: Some("ctx-usage".to_string()),
                extensions: vec![],
                message_id: "u-usage".to_string(),
                metadata: None,
                parts: vec![Part {
                    data: None,
                    file: None,
                    metadata: None,
                    text: Some(text.to_string()),
                }],
                reference_task_ids: vec![],
                role: Role::RoleUser,
                task_id: Some("task-usage".to_string()),
            }],
            id: "task-usage".to_string(),
            metadata: None,
            status: TaskStatus {
                message: None,
                state: TaskState::TaskStateSubmitted,
                timestamp: None,
            },
        }
    }

    async fn build_toolless_agent(gateway_url: String) -> Agent {
        use crate::config::AgentConfig;
        let cfg = AgentConfig {
            provider: "openai".to_string(),
            model: "test-model".to_string(),
            base_url: Some(gateway_url),
            ..AgentConfig::default()
        };
        AgentBuilder::new()
            .with_config(&cfg)
            .build()
            .await
            .expect("agent builds")
    }

    /// Mock non-streaming `/chat/completions` returning a final answer that
    /// carries a `usage` block, so the tool loop records token counts.
    async fn mock_final_with_usage() -> Json<Value> {
        Json(serde_json::json!({
            "id": "chatcmpl-usage",
            "object": "chat.completion",
            "created": 0,
            "model": "test-model",
            "choices": [{
                "index": 0,
                "finish_reason": "stop",
                "message": {"role": "assistant", "content": "All done", "tool_calls": []},
            }],
            "usage": {"prompt_tokens": 11, "completion_tokens": 4, "total_tokens": 15},
        }))
    }

    /// Mock streaming `/chat/completions` that emits two content deltas, then a
    /// trailing usage-only chunk (empty `choices`), then `[DONE]`.
    async fn mock_stream_with_usage() -> axum::response::sse::Sse<
        impl futures_util::Stream<
            Item = std::result::Result<axum::response::sse::Event, std::convert::Infallible>,
        >,
    > {
        use axum::response::sse::{Event as SseEvent, KeepAlive as SseKeepAlive, Sse as SseResp};
        let chunks = [
            serde_json::json!({"choices":[{"delta":{"content":"Hi "}}]}).to_string(),
            serde_json::json!({"choices":[{"delta":{"content":"there"}}]}).to_string(),
            serde_json::json!({
                "choices": [],
                "usage": {"prompt_tokens": 7, "completion_tokens": 2, "total_tokens": 9},
            })
            .to_string(),
            "[DONE]".to_string(),
        ];
        let stream = futures_util::stream::iter(
            chunks
                .into_iter()
                .map(|d| Ok::<_, std::convert::Infallible>(SseEvent::default().data(d))),
        );
        SseResp::new(stream).keep_alive(SseKeepAlive::default())
    }

    #[tokio::test]
    async fn default_background_handler_attaches_usage_metadata() {
        let gateway_listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let gateway_addr = gateway_listener.local_addr().expect("addr");
        let gateway_app = Router::new().route("/chat/completions", post(mock_final_with_usage));
        tokio::spawn(async move {
            axum::serve(gateway_listener, gateway_app).await.ok();
        });

        let agent = build_toolless_agent(format!("http://{gateway_addr}")).await;
        let handler = DefaultBackgroundTaskHandler::new(Some(Arc::new(agent)));
        assert!(
            handler.is_usage_metadata_enabled(),
            "usage metadata defaults on"
        );

        let task = handler
            .handle_task(submitted_usage_task("hi"), None)
            .await
            .expect("handle_task");
        assert_eq!(task.status.state, TaskState::TaskStateCompleted);

        let meta = task
            .metadata
            .expect("usage metadata attached on completion");
        assert_eq!(meta.0["usage"]["prompt_tokens"], 11);
        assert_eq!(meta.0["usage"]["completion_tokens"], 4);
        assert_eq!(meta.0["usage"]["total_tokens"], 15);
        let stats = &meta.0["execution_stats"];
        assert_eq!(stats["iterations"], 1);
        assert_eq!(stats["messages"], 0);
        assert_eq!(stats["tool_calls"], 0);
        assert_eq!(stats["failed_tools"], 0);
    }

    #[tokio::test]
    async fn default_background_handler_omits_usage_metadata_when_disabled() {
        let gateway_listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let gateway_addr = gateway_listener.local_addr().expect("addr");
        let gateway_app = Router::new().route("/chat/completions", post(mock_final_with_usage));
        tokio::spawn(async move {
            axum::serve(gateway_listener, gateway_app).await.ok();
        });

        let agent = build_toolless_agent(format!("http://{gateway_addr}")).await;
        let mut handler = DefaultBackgroundTaskHandler::new(Some(Arc::new(agent)));
        handler.set_enable_usage_metadata(false);
        assert!(!handler.is_usage_metadata_enabled());

        let task = handler
            .handle_task(submitted_usage_task("hi"), None)
            .await
            .expect("handle_task");
        assert_eq!(task.status.state, TaskState::TaskStateCompleted);
        assert!(
            task.metadata.is_none(),
            "metadata must be absent when usage metadata is disabled"
        );
    }

    /// Drive the streaming handler directly against a mock that returns a usage
    /// chunk, then return the task as persisted in storage (where the handler
    /// writes the merged metadata before the terminal status update).
    async fn run_streaming_usage_case(enable: bool) -> Task {
        use crate::{InMemoryStorage, Storage};

        let gateway_listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let gateway_addr = gateway_listener.local_addr().expect("addr");
        let gateway_app = Router::new().route("/chat/completions", post(mock_stream_with_usage));
        tokio::spawn(async move {
            axum::serve(gateway_listener, gateway_app).await.ok();
        });

        let agent = build_toolless_agent(format!("http://{gateway_addr}")).await;
        let storage: Arc<dyn Storage> = Arc::new(InMemoryStorage::new());
        let task = submitted_usage_task("hi");
        storage.put_task(task.clone()).await;

        let (tx, mut rx) = tokio::sync::mpsc::channel::<StreamResponse>(64);
        let drain = tokio::spawn(async move { while rx.recv().await.is_some() {} });
        let emitter = StreamEmitter::new(tx, Arc::clone(&storage));

        let mut handler = DefaultStreamingTaskHandler::new(Some(Arc::new(agent)));
        handler.set_enable_usage_metadata(enable);
        handler
            .handle_streaming_task(task.clone(), None, emitter)
            .await
            .expect("handle_streaming_task");
        drain.await.ok();

        storage.get_task(&task.id).await.expect("task persisted")
    }

    #[tokio::test]
    async fn default_streaming_handler_attaches_usage_metadata() {
        let stored = run_streaming_usage_case(true).await;
        let meta = stored
            .metadata
            .expect("usage metadata attached to the stored task");
        assert_eq!(meta.0["usage"]["prompt_tokens"], 7);
        assert_eq!(meta.0["usage"]["completion_tokens"], 2);
        assert_eq!(meta.0["usage"]["total_tokens"], 9);
        // The streaming tail counts as one iteration even without a tool loop.
        assert_eq!(meta.0["execution_stats"]["iterations"], 1);
    }

    #[tokio::test]
    async fn default_streaming_handler_omits_usage_metadata_when_disabled() {
        let stored = run_streaming_usage_case(false).await;
        assert!(
            stored.metadata.is_none(),
            "metadata must be absent when usage metadata is disabled"
        );
    }

    #[tokio::test]
    async fn build_sdk_messages_trims_to_max_conversation_history() {
        let mut task = submitted_usage_task("m0");
        for i in 1..5 {
            let mut msg = task.history[0].clone();
            msg.parts[0].text = Some(format!("m{i}"));
            task.history.push(msg);
        }

        let agent = AgentBuilder::new()
            .with_config(&crate::config::AgentConfig {
                provider: "openai".to_string(),
                model: "test-model".to_string(),
                ..Default::default()
            })
            .with_max_conversation_history(2)
            .build()
            .await
            .expect("agent builds");

        let messages = build_sdk_messages(&agent, &task);
        let texts: Vec<String> = messages
            .iter()
            .map(|m| match &m.content {
                MessageContent::String(s) => s.clone(),
                _ => String::new(),
            })
            .collect();
        // No system prompt configured, so only the last 2 history messages survive.
        assert_eq!(texts, vec!["m3".to_string(), "m4".to_string()]);
    }
}

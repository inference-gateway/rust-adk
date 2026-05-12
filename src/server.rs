use crate::a2a_types::{
    AgentCard, Artifact, CancelTaskRequest, DeleteTaskPushNotificationConfigRequest,
    GetTaskPushNotificationConfigRequest, GetTaskRequest, ListTaskPushNotificationConfigRequest,
    ListTaskPushNotificationConfigResponse, ListTasksRequest, ListTasksResponse,
    Message as A2AMessage, Part, Role, SendMessageRequest, SendMessageResponse,
    SetTaskPushNotificationConfigRequest, StreamResponse, Task, TaskArtifactUpdateEvent, TaskState,
    TaskStatus, TaskStatusUpdateEvent, Timestamp,
};
use crate::client::HealthStatus;
use crate::config::{AgentConfig, Config};
use crate::storage::{InMemoryStorage, parse_task_name};
use anyhow::{Result, anyhow};
use axum::{
    Router,
    extract::State,
    http::StatusCode,
    response::{
        IntoResponse, Json, Response,
        sse::{Event, KeepAlive, Sse},
    },
    routing::{get, post},
};
use futures_util::stream::{Stream, StreamExt};
use inference_gateway_sdk::{
    ChatCompletionTool, InferenceGatewayAPI, InferenceGatewayClient, Message, MessageContent,
    MessageRole, Provider,
};
use serde_json::Value;
use std::collections::HashMap;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{debug, error, info, warn};

/// Agent card field overrides
#[derive(Debug, Clone, Default)]
pub struct AgentCardOverrides {
    pub name: Option<String>,
    pub description: Option<String>,
    pub version: Option<String>,
    pub url: Option<String>,
}

impl AgentCardOverrides {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
}

/// Trait for handling tool calls
#[async_trait::async_trait]
pub trait ToolHandler: Send + Sync {
    /// Handle a tool call with the given arguments and return the result
    async fn handle(&self, args: Value) -> Result<String>;
}

/// A simple function-based tool handler
pub struct FunctionToolHandler<F>
where
    F: Fn(Value) -> Result<String> + Send + Sync,
{
    handler: F,
}

impl<F> FunctionToolHandler<F>
where
    F: Fn(Value) -> Result<String> + Send + Sync,
{
    pub fn new(handler: F) -> Self {
        Self { handler }
    }
}

#[async_trait::async_trait]
impl<F> ToolHandler for FunctionToolHandler<F>
where
    F: Fn(Value) -> Result<String> + Send + Sync,
{
    async fn handle(&self, args: Value) -> Result<String> {
        (self.handler)(args)
    }
}

/// An async function-based tool handler
pub struct AsyncFunctionToolHandler<F, Fut>
where
    F: Fn(Value) -> Fut + Send + Sync,
    Fut: std::future::Future<Output = Result<String>> + Send,
{
    handler: F,
}

impl<F, Fut> AsyncFunctionToolHandler<F, Fut>
where
    F: Fn(Value) -> Fut + Send + Sync,
    Fut: std::future::Future<Output = Result<String>> + Send,
{
    pub fn new(handler: F) -> Self {
        Self { handler }
    }
}

#[async_trait::async_trait]
impl<F, Fut> ToolHandler for AsyncFunctionToolHandler<F, Fut>
where
    F: Fn(Value) -> Fut + Send + Sync,
    Fut: std::future::Future<Output = Result<String>> + Send,
{
    async fn handle(&self, args: Value) -> Result<String> {
        (self.handler)(args).await
    }
}

/// Handler invoked by the server for `message/send` requests.
///
/// Implementations receive a freshly-built task (already in
/// `TaskStateSubmitted`) plus the incoming user message, run the business
/// logic, and return the final task — typically with `state == Completed`
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
    storage: Arc<InMemoryStorage>,
}

impl std::fmt::Debug for StreamEmitter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StreamEmitter").finish_non_exhaustive()
    }
}

impl StreamEmitter {
    fn new(tx: mpsc::Sender<StreamResponse>, storage: Arc<InMemoryStorage>) -> Self {
        Self { tx, storage }
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

        self.storage.update_task(task_id, |task| {
            task.status = new_status.clone();
            if let Some(ref msg) = message {
                task.history.push(msg.clone());
            }
        });

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

        self.storage.update_task(task_id, |task| {
            task.artifacts.push(artifact.clone());
        });

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
}

fn build_agent_text_message(task: &Task, text: &str) -> A2AMessage {
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

/// Translate the task history into the SDK message format expected by
/// [`InferenceGatewayClient`]. Optionally prepends the agent's system
/// prompt.
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
    for a2a_msg in &task.history {
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

/// Construct an [`InferenceGatewayClient`] preconfigured with any tools the
/// agent advertises.
fn agent_gateway_client(agent: &Agent, gateway_url: &str) -> InferenceGatewayClient {
    let client = InferenceGatewayClient::new(gateway_url);
    match agent.toolbox.clone() {
        Some(tools) => client.with_tools(Some(tools)),
        None => client,
    }
}

/// Static message returned by the default handlers when no agent is
/// configured. Mirrors the Go ADK's instructional fallback.
const NO_AGENT_REPLY: &str = "I received your message. I'm a default task handler without AI capabilities. \
     To enable AI responses, configure an OpenAI-compatible agent via \
     `A2AServerBuilder::with_agent(...)`.";

/// Opt-in default `message/send` handler wired up by
/// [`A2AServerBuilder::with_default_background_task_handler`] /
/// [`A2AServerBuilder::with_default_task_handlers`].
///
/// When an [`Agent`] is configured, delegates to the inference gateway via a
/// single non-streaming `generate_content` call and returns the resulting
/// task with `state == Completed` and the reply attached. Without an agent,
/// returns the static [`NO_AGENT_REPLY`] message — mirroring the Go ADK's
/// `processWithoutAgentBackground`.
#[derive(Debug)]
pub struct DefaultBackgroundTaskHandler {
    agent: Option<Arc<Agent>>,
    gateway_url: String,
}

impl DefaultBackgroundTaskHandler {
    pub fn new(agent: Option<Arc<Agent>>, gateway_url: String) -> Self {
        Self { agent, gateway_url }
    }
}

#[async_trait::async_trait]
impl TaskHandler for DefaultBackgroundTaskHandler {
    async fn handle_task(&self, mut task: Task, _message: Option<A2AMessage>) -> Result<Task> {
        let (reply_text, terminal_state) = match self.agent.as_ref() {
            Some(agent) => {
                let messages = build_sdk_messages(agent, &task);
                let client = agent_gateway_client(agent, &self.gateway_url);
                match client
                    .generate_content(agent.provider, &agent.model, messages)
                    .await
                {
                    Ok(response) => {
                        let text = response
                            .choices
                            .first()
                            .map(|c| message_content_to_string(&c.message.content))
                            .unwrap_or_default();
                        let text = if text.is_empty() {
                            "Task completed".to_string()
                        } else {
                            text
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
/// shared `artifact_id`) — clients see the reply build up in real time. The
/// stream terminates with a final `last_chunk: true` artifact + a
/// `Completed` status update.
///
/// Without an agent, emits a single instructional artifact + `Completed`
/// so the bundled defaults remain usable for examples and tests.
#[derive(Debug)]
pub struct DefaultStreamingTaskHandler {
    agent: Option<Arc<Agent>>,
    gateway_url: String,
}

impl DefaultStreamingTaskHandler {
    pub fn new(agent: Option<Arc<Agent>>, gateway_url: String) -> Self {
        Self { agent, gateway_url }
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

        let final_text = match self.agent.as_ref() {
            Some(agent) => stream_agent_deltas(agent, &self.gateway_url, &task, &emitter).await?,
            None => {
                emitter
                    .emit_text_artifact(&task.id, &task.context_id, NO_AGENT_REPLY, true)
                    .await?;
                NO_AGENT_REPLY.to_string()
            }
        };

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
async fn stream_agent_deltas(
    agent: &Agent,
    gateway_url: &str,
    task: &Task,
    emitter: &StreamEmitter,
) -> Result<String> {
    let messages = build_sdk_messages(agent, task);
    let client = agent_gateway_client(agent, gateway_url);
    let stream = client.generate_content_stream(agent.provider, &agent.model, messages);
    let mut stream = Box::pin(stream);

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

        // OpenAI streaming chunk: {"choices":[{"delta":{"content":"..."},...}],...}
        let parsed: serde_json::Value = match serde_json::from_str(data) {
            Ok(v) => v,
            Err(_) => continue,
        };
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

    // Terminal chunk signals the artifact is complete; the buffered text is
    // delivered as the final agent message via the Completed status event.
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

fn parse_provider(provider_str: &str) -> Result<Provider> {
    match provider_str.to_lowercase().as_str() {
        "groq" => Ok(Provider::Groq),
        "google" => Ok(Provider::Google),
        "openai" => Ok(Provider::Openai),
        "anthropic" => Ok(Provider::Anthropic),
        "cohere" => Ok(Provider::Cohere),
        "cloudflare" => Ok(Provider::Cloudflare),
        "deepseek" => Ok(Provider::Deepseek),
        "ollama" => Ok(Provider::Ollama),
        _ => Err(anyhow!(
            "Unsupported provider: {}. Supported providers: groq, google, openai, anthropic, cohere, cloudflare, ollama",
            provider_str
        )),
    }
}

#[derive(Debug)]
pub struct A2AServer {
    #[allow(dead_code)]
    config: Config,
    agent_card: Option<AgentCard>,
    agent: Option<Arc<Agent>>,
    gateway_url: String,
    storage: Arc<InMemoryStorage>,
    background_task_handler: Option<Arc<dyn TaskHandler>>,
    streaming_task_handler: Option<Arc<dyn StreamableTaskHandler>>,
}

pub struct Agent {
    #[allow(dead_code)]
    config: AgentConfig,
    system_prompt: Option<String>,
    provider: Provider,
    model: String,
    #[allow(dead_code)]
    max_chat_completion: u32,
    #[allow(dead_code)]
    max_conversation_history: u32,
    #[allow(dead_code)]
    toolbox: Option<Vec<ChatCompletionTool>>,
    tool_handlers: HashMap<String, Box<dyn ToolHandler>>,
}

impl std::fmt::Debug for Agent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Agent")
            .field("config", &self.config)
            .field("system_prompt", &self.system_prompt)
            .field("provider", &self.provider)
            .field("model", &self.model)
            .field("max_chat_completion", &self.max_chat_completion)
            .field("max_conversation_history", &self.max_conversation_history)
            .field("toolbox", &self.toolbox)
            .field(
                "tool_handlers",
                &format!("{} handlers", self.tool_handlers.len()),
            )
            .finish()
    }
}

pub struct A2AServerBuilder {
    config: Option<Config>,
    agent_card: Option<AgentCard>,
    agent_card_path: Option<String>,
    agent_card_overrides: Option<AgentCardOverrides>,
    agent: Option<Arc<Agent>>,
    gateway_url: Option<String>,
    storage: Option<Arc<InMemoryStorage>>,
    background_task_handler: Option<Arc<dyn TaskHandler>>,
    streaming_task_handler: Option<Arc<dyn StreamableTaskHandler>>,
    use_default_background_task_handler: bool,
    use_default_streaming_task_handler: bool,
}

pub struct AgentBuilder {
    config: Option<AgentConfig>,
    system_prompt: Option<String>,
    max_chat_completion: u32,
    max_conversation_history: u32,
    toolbox: Option<Vec<ChatCompletionTool>>,
    tool_handlers: HashMap<String, Box<dyn ToolHandler>>,
}

#[derive(Debug)]
struct AppState {
    server: A2AServer,
}

impl A2AServerBuilder {
    pub fn new() -> Self {
        Self {
            config: None,
            agent_card: None,
            agent_card_path: None,
            agent_card_overrides: None,
            agent: None,
            gateway_url: None,
            storage: None,
            background_task_handler: None,
            streaming_task_handler: None,
            use_default_background_task_handler: false,
            use_default_streaming_task_handler: false,
        }
    }

    pub fn with_config(mut self, config: Config) -> Self {
        self.config = Some(config);
        self
    }

    pub fn with_agent_card(mut self, agent_card: AgentCard) -> Self {
        self.agent_card = Some(agent_card);
        self
    }

    pub fn with_agent_card_from_file(
        mut self,
        path: impl Into<String>,
        overrides: Option<AgentCardOverrides>,
    ) -> Self {
        self.agent_card_path = Some(path.into());
        self.agent_card_overrides = overrides;
        self
    }

    pub fn with_agent(mut self, agent: Agent) -> Self {
        self.agent = Some(Arc::new(agent));
        self
    }

    pub fn with_gateway_url(mut self, url: impl Into<String>) -> Self {
        self.gateway_url = Some(url.into());
        self
    }

    /// Inject an external in-memory storage. Mostly useful for tests and for
    /// sharing state across multiple `A2AServer` instances.
    pub fn with_storage(mut self, storage: Arc<InMemoryStorage>) -> Self {
        self.storage = Some(storage);
        self
    }

    /// Register a handler that drives `message/send` requests (the
    /// background/HTTP path).
    pub fn with_background_task_handler<H: TaskHandler + 'static>(mut self, handler: H) -> Self {
        self.background_task_handler = Some(Arc::new(handler));
        self
    }

    /// Register a handler that drives `message/stream` requests (the SSE
    /// path).
    pub fn with_streaming_task_handler<H: StreamableTaskHandler + 'static>(
        mut self,
        handler: H,
    ) -> Self {
        self.streaming_task_handler = Some(Arc::new(handler));
        self
    }

    /// Opt in to the bundled [`DefaultBackgroundTaskHandler`] so
    /// `message/send` works without custom code. If an [`Agent`] is also
    /// registered via [`with_agent`], the default handler delegates to it
    /// via the configured inference gateway; otherwise it returns an echo
    /// reply. Default construction is deferred to [`build`] so this method
    /// can be called before or after [`with_agent`].
    pub fn with_default_background_task_handler(mut self) -> Self {
        self.use_default_background_task_handler = true;
        self
    }

    /// Opt in to the bundled [`DefaultStreamingTaskHandler`] so
    /// `message/stream` works without custom code (Submitted → Working →
    /// reply artifact → Completed). Uses the registered [`Agent`] when
    /// present, otherwise falls back to echo. Default construction is
    /// deferred to [`build`].
    pub fn with_default_streaming_task_handler(mut self) -> Self {
        self.use_default_streaming_task_handler = true;
        self
    }

    /// Opt in to both [`DefaultBackgroundTaskHandler`] and
    /// [`DefaultStreamingTaskHandler`].
    pub fn with_default_task_handlers(self) -> Self {
        self.with_default_background_task_handler()
            .with_default_streaming_task_handler()
    }

    pub async fn build(self) -> Result<A2AServer> {
        let config = self.config.unwrap_or_default();

        let mut agent_card = if let Some(path) = self.agent_card_path {
            match tokio::fs::read_to_string(&path).await {
                Ok(content) => match serde_json::from_str::<AgentCard>(&content) {
                    Ok(card) => {
                        info!("Loaded agent card from: {}", path);
                        Some(card)
                    }
                    Err(e) => {
                        return Err(anyhow!("Failed to parse agent card from {}: {}", path, e));
                    }
                },
                Err(e) => {
                    return Err(anyhow!("Could not load agent card from {}: {}", path, e));
                }
            }
        } else {
            self.agent_card
        };

        if agent_card.is_none() {
            return Err(anyhow!(
                "Agent card is required. Use with_agent_card() or with_agent_card_from_file() to configure the server."
            ));
        }

        #[allow(clippy::collapsible_if)]
        if let Some(ref mut card) = agent_card {
            if let Some(overrides) = self.agent_card_overrides {
                if let Some(name) = overrides.name {
                    info!("Overriding agent card name: {} -> {}", card.name, name);
                    card.name = name;
                }
                if let Some(description) = overrides.description {
                    info!("Overriding agent card description");
                    card.description = description;
                }
                if let Some(version) = overrides.version {
                    info!(
                        "Overriding agent card version: {} -> {}",
                        card.version, version
                    );
                    card.version = version;
                }
                if let Some(url) = overrides.url {
                    info!("Overriding agent card URL: {:?} -> {}", card.url, url);
                    card.url = Some(url);
                }
            }
        }

        let gateway_url = self
            .gateway_url
            .unwrap_or_else(|| "http://localhost:8080/v1".to_string());

        let streaming_enabled = agent_card
            .as_ref()
            .and_then(|c| c.capabilities.streaming)
            .unwrap_or(false);

        // Construct opt-in defaults now so the agent + gateway URL captured
        // here are visible to the handlers, regardless of the order in which
        // `with_agent` / `with_default_*` were called.
        let background_task_handler = match self.background_task_handler {
            Some(h) => Some(h),
            None if self.use_default_background_task_handler => Some(Arc::new(
                DefaultBackgroundTaskHandler::new(self.agent.clone(), gateway_url.clone()),
            )
                as Arc<dyn TaskHandler>),
            None => None,
        };
        let streaming_task_handler = match self.streaming_task_handler {
            Some(h) => Some(h),
            None if self.use_default_streaming_task_handler => Some(Arc::new(
                DefaultStreamingTaskHandler::new(self.agent.clone(), gateway_url.clone()),
            )
                as Arc<dyn StreamableTaskHandler>),
            None => None,
        };

        match (
            background_task_handler.is_some(),
            streaming_task_handler.is_some(),
        ) {
            (false, false) => {
                return Err(anyhow!(
                    "at least one task handler must be configured — use \
                     A2AServerBuilder::with_background_task_handler()/\
                     with_streaming_task_handler(), or with_default_task_handlers() \
                     for both"
                ));
            }
            (false, _) if !streaming_enabled => {
                return Err(anyhow!(
                    "background task handler is required when streaming is not enabled \
                     in the agent card — use with_background_task_handler() or \
                     with_default_background_task_handler()"
                ));
            }
            (_, false) if streaming_enabled => {
                return Err(anyhow!(
                    "streaming task handler is required when streaming is enabled in \
                     the agent card — use with_streaming_task_handler() or \
                     with_default_streaming_task_handler()"
                ));
            }
            _ => {}
        }

        Ok(A2AServer {
            config,
            agent_card,
            agent: self.agent,
            gateway_url,
            storage: self
                .storage
                .unwrap_or_else(|| Arc::new(InMemoryStorage::new())),
            background_task_handler,
            streaming_task_handler,
        })
    }
}

impl Default for A2AServerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl AgentBuilder {
    pub fn new() -> Self {
        Self {
            config: None,
            system_prompt: None,
            max_chat_completion: 10,
            max_conversation_history: 20,
            toolbox: None,
            tool_handlers: HashMap::new(),
        }
    }

    pub fn with_config(mut self, config: &AgentConfig) -> Self {
        self.config = Some(config.clone());
        self
    }

    pub fn with_system_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.system_prompt = Some(prompt.into());
        self
    }

    pub fn with_max_chat_completion(mut self, max: u32) -> Self {
        self.max_chat_completion = max;
        self
    }

    pub fn with_max_conversation_history(mut self, max: u32) -> Self {
        self.max_conversation_history = max;
        self
    }

    pub fn with_toolbox(mut self, tools: Vec<ChatCompletionTool>) -> Self {
        self.toolbox = Some(tools);
        self
    }

    pub fn with_tool_handler<H: ToolHandler + 'static>(mut self, name: String, handler: H) -> Self {
        self.tool_handlers.insert(name, Box::new(handler));
        self
    }

    pub fn with_function_tool<F>(mut self, name: String, handler: F) -> Self
    where
        F: Fn(Value) -> Result<String> + Send + Sync + 'static,
    {
        self.tool_handlers
            .insert(name, Box::new(FunctionToolHandler::new(handler)));
        self
    }

    pub fn with_async_function_tool<F, Fut>(mut self, name: String, handler: F) -> Self
    where
        F: Fn(Value) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<String>> + Send + 'static,
    {
        self.tool_handlers
            .insert(name, Box::new(AsyncFunctionToolHandler::new(handler)));
        self
    }

    pub async fn build(self) -> Result<Agent> {
        let config = self.config.unwrap_or_default();

        let provider = parse_provider(&config.provider)?;
        let model = config.model.clone();

        if model.is_empty() {
            return Err(anyhow!(
                "Model cannot be empty. Please configure a model in the agent config"
            ));
        }

        Ok(Agent {
            config,
            system_prompt: self.system_prompt,
            provider,
            model,
            max_chat_completion: self.max_chat_completion,
            max_conversation_history: self.max_conversation_history,
            toolbox: self.toolbox,
            tool_handlers: self.tool_handlers,
        })
    }
}

impl Default for AgentBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Agent {
    pub fn toolbox(&self) -> Option<&Vec<ChatCompletionTool>> {
        self.toolbox.as_ref()
    }
}

impl A2AServer {
    /// Access the in-memory storage backing this server. Useful for tests
    /// and callers that need to inspect or pre-populate state.
    pub fn storage(&self) -> Arc<InMemoryStorage> {
        Arc::clone(&self.storage)
    }

    pub async fn serve(self, addr: SocketAddr) -> Result<()> {
        let state = AppState { server: self };

        let app = Router::new()
            .route("/health", get(health_handler))
            .route("/.well-known/agent.json", get(agent_card_handler))
            .route("/a2a", post(a2a_handler))
            .layer(
                ServiceBuilder::new()
                    .layer(TraceLayer::new_for_http())
                    .layer(CorsLayer::permissive()),
            )
            .with_state(Arc::new(state));

        info!("A2A Server starting on {}", addr);

        let listener = TcpListener::bind(addr)
            .await
            .map_err(|e| anyhow!("Failed to bind to address {}: {}", addr, e))?;

        axum::serve(listener, app)
            .await
            .map_err(|e| anyhow!("Server error: {}", e))?;

        Ok(())
    }
}

async fn health_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<HealthStatus>, StatusCode> {
    debug!("Health check requested");

    let gateway_client = InferenceGatewayClient::new(&state.server.gateway_url);
    let gateway_healthy = gateway_client.health_check().await.unwrap_or(false);

    let status = if gateway_healthy && state.server.agent.is_some() {
        "healthy"
    } else if state.server.agent.is_some() {
        "degraded"
    } else {
        "healthy"
    };

    let health = HealthStatus {
        status: status.to_string(),
        timestamp: chrono::Utc::now(),
        details: Some(serde_json::json!({
            "has_agent": state.server.agent.is_some(),
            "gateway_healthy": gateway_healthy,
            "version": env!("CARGO_PKG_VERSION"),
            "sdk_version": "0.13.3"
        })),
    };

    debug!("Health status: {}", health.status);
    Ok(Json(health))
}

async fn agent_card_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<AgentCard>, StatusCode> {
    debug!("Agent card requested");

    if let Some(ref agent_card) = state.server.agent_card {
        debug!("Returning configured agent card");
        return Ok(Json(agent_card.clone()));
    }

    error!("No agent card configured - server should not have started without one");
    Err(StatusCode::INTERNAL_SERVER_ERROR)
}

// =========================================================================
// JSON-RPC dispatch
// =========================================================================

/// JSON-RPC standard error codes plus A2A-specific extensions used by the
/// Go ADK. The values are kept aligned with `inference-gateway/adk`.
mod jsonrpc_errors {
    /// Server received invalid JSON. The default axum extractor rejects
    /// malformed JSON before it reaches our handler, but the constant is
    /// kept for parity with the spec and for future custom parsers.
    #[allow(dead_code)]
    pub const PARSE_ERROR: i64 = -32700;
    pub const INVALID_REQUEST: i64 = -32600;
    pub const METHOD_NOT_FOUND: i64 = -32601;
    pub const INVALID_PARAMS: i64 = -32602;
    pub const INTERNAL_ERROR: i64 = -32603;

    /// Task not found.
    pub const TASK_NOT_FOUND: i64 = -32001;
    /// Task cannot be cancelled in its current state.
    pub const TASK_NOT_CANCELABLE: i64 = -32002;
    /// Push notifications are not supported by this agent.
    #[allow(dead_code)]
    pub const PUSH_NOTIFICATION_NOT_SUPPORTED: i64 = -32003;
}

fn json_rpc_success(id: Value, result: Value) -> Json<Value> {
    Json(serde_json::json!({
        "jsonrpc": "2.0",
        "id": id,
        "result": result,
    }))
}

fn json_rpc_error(id: Value, code: i64, message: &str, data: Option<Value>) -> Json<Value> {
    let mut err = serde_json::json!({
        "code": code,
        "message": message,
    });
    if let Some(d) = data {
        err["data"] = d;
    }
    Json(serde_json::json!({
        "jsonrpc": "2.0",
        "id": id,
        "error": err,
    }))
}

async fn a2a_handler(State(state): State<Arc<AppState>>, Json(payload): Json<Value>) -> Response {
    debug!("A2A request received: {payload:?}");

    let id = payload.get("id").cloned().unwrap_or(Value::Null);

    let jsonrpc = payload.get("jsonrpc").and_then(|v| v.as_str());
    if jsonrpc != Some("2.0") {
        return json_rpc_error(
            id,
            jsonrpc_errors::INVALID_REQUEST,
            "Invalid Request",
            Some(Value::String(
                "Missing or invalid \"jsonrpc\" field; must be \"2.0\"".to_string(),
            )),
        )
        .into_response();
    }

    let method = match payload.get("method").and_then(|v| v.as_str()) {
        Some(m) => m.to_string(),
        None => {
            return json_rpc_error(
                id,
                jsonrpc_errors::INVALID_REQUEST,
                "Invalid Request",
                Some(Value::String("Missing \"method\" field".to_string())),
            )
            .into_response();
        }
    };

    let params = payload.get("params").cloned().unwrap_or(Value::Null);

    match method.as_str() {
        "message/send" => handle_message_send(&state, id, params)
            .await
            .into_response(),
        "message/stream" => handle_message_stream(state.clone(), id, params).await,
        "tasks/get" => handle_tasks_get(&state, id, params).into_response(),
        "tasks/list" => handle_tasks_list(&state, id, params).into_response(),
        "tasks/cancel" => handle_tasks_cancel(&state, id, params).into_response(),
        "tasks/pushNotificationConfig/set" => {
            handle_set_push_config(&state, id, params).into_response()
        }
        "tasks/pushNotificationConfig/get" => {
            handle_get_push_config(&state, id, params).into_response()
        }
        "tasks/pushNotificationConfig/list" => {
            handle_list_push_configs(&state, id, params).into_response()
        }
        "tasks/pushNotificationConfig/delete" => {
            handle_delete_push_config(&state, id, params).into_response()
        }
        other => {
            warn!("Unknown JSON-RPC method requested: {other}");
            json_rpc_error(
                id,
                jsonrpc_errors::METHOD_NOT_FOUND,
                "Method not found",
                Some(Value::String(other.to_string())),
            )
            .into_response()
        }
    }
}

fn invalid_params(id: Value, e: serde_json::Error) -> Json<Value> {
    json_rpc_error(
        id,
        jsonrpc_errors::INVALID_PARAMS,
        "Invalid params",
        Some(Value::String(e.to_string())),
    )
}

fn invalid_params_message(id: Value, detail: impl Into<String>) -> Json<Value> {
    json_rpc_error(
        id,
        jsonrpc_errors::INVALID_PARAMS,
        "Invalid params",
        Some(Value::String(detail.into())),
    )
}

/// Validate the A2A-spec-required content of a `message/send` /
/// `message/stream` request. Returns an error suitable for surfacing as the
/// `data` field of a JSON-RPC `-32602` response.
fn validate_send_message_request(req: &SendMessageRequest) -> Result<(), String> {
    let Some(msg) = req.message.as_ref() else {
        return Err("`message` is required".to_string());
    };
    if msg.message_id.is_empty() {
        return Err(
            "`message.messageId` must be a non-empty string — per the A2A spec the message \
             creator owns this identifier (used by the server for duplicate detection)"
                .to_string(),
        );
    }
    if msg.parts.is_empty() {
        return Err("`message.parts` must contain at least one part".to_string());
    }
    Ok(())
}

fn build_task_from_request(req: &SendMessageRequest) -> Task {
    let task_id = uuid::Uuid::new_v4().to_string();
    let context_id = req
        .message
        .as_ref()
        .and_then(|m| m.context_id.clone())
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

    let mut history = Vec::new();
    if let Some(mut msg) = req.message.clone() {
        if msg.context_id.is_none() {
            msg.context_id = Some(context_id.clone());
        }
        if msg.task_id.is_none() {
            msg.task_id = Some(task_id.clone());
        }
        history.push(msg);
    }

    Task {
        artifacts: vec![],
        context_id,
        history,
        id: task_id,
        metadata: None,
        status: TaskStatus {
            message: None,
            state: TaskState::TaskStateSubmitted,
            timestamp: Some(Timestamp(chrono::Utc::now())),
        },
    }
}

async fn handle_message_send(state: &Arc<AppState>, id: Value, params: Value) -> Json<Value> {
    let request: SendMessageRequest = match serde_json::from_value(params) {
        Ok(r) => r,
        Err(e) => return invalid_params(id, e),
    };

    if let Err(detail) = validate_send_message_request(&request) {
        return invalid_params_message(id, detail);
    }

    let Some(handler) = state.server.background_task_handler.as_ref().cloned() else {
        return json_rpc_error(
            id,
            jsonrpc_errors::METHOD_NOT_FOUND,
            "Method not found",
            Some(Value::String(
                "message/send is not supported: no background task handler is configured"
                    .to_string(),
            )),
        );
    };

    let initial_task = build_task_from_request(&request);
    state.server.storage.put_task(initial_task.clone());

    let task = match handler.handle_task(initial_task, request.message).await {
        Ok(t) => t,
        Err(e) => {
            error!("background task handler failed: {e}");
            return json_rpc_error(
                id,
                jsonrpc_errors::INTERNAL_ERROR,
                "Internal error",
                Some(Value::String(e.to_string())),
            );
        }
    };

    state.server.storage.put_task(task.clone());

    let reply = task.status.message.clone();
    let response = SendMessageResponse {
        message: reply,
        task: Some(task),
    };

    match serde_json::to_value(response) {
        Ok(v) => json_rpc_success(id, v),
        Err(e) => json_rpc_error(
            id,
            jsonrpc_errors::INTERNAL_ERROR,
            "Internal error",
            Some(Value::String(e.to_string())),
        ),
    }
}

async fn handle_message_stream(state: Arc<AppState>, id: Value, params: Value) -> Response {
    let request: SendMessageRequest = match serde_json::from_value(params) {
        Ok(r) => r,
        Err(e) => return invalid_params(id, e).into_response(),
    };

    if let Err(detail) = validate_send_message_request(&request) {
        return invalid_params_message(id, detail).into_response();
    }

    let Some(handler) = state.server.streaming_task_handler.as_ref().cloned() else {
        return json_rpc_error(
            id,
            jsonrpc_errors::METHOD_NOT_FOUND,
            "Method not found",
            Some(Value::String(
                "message/stream is not supported: no streaming task handler is configured"
                    .to_string(),
            )),
        )
        .into_response();
    };

    let task = build_task_from_request(&request);
    state.server.storage.put_task(task.clone());

    let (tx, rx) = mpsc::channel::<StreamResponse>(32);

    // Initial event: the freshly-created task in `Submitted`.
    let initial = StreamResponse {
        artifact_update: None,
        message: None,
        status_update: None,
        task: Some(task.clone()),
    };
    if tx.send(initial).await.is_err() {
        // Receiver dropped before we could send — return an internal error
        // synchronously rather than an empty stream.
        return json_rpc_error(
            id,
            jsonrpc_errors::INTERNAL_ERROR,
            "Internal error",
            Some(Value::String(
                "stream receiver closed before initial event".to_string(),
            )),
        )
        .into_response();
    }

    let emitter = StreamEmitter::new(tx, Arc::clone(&state.server.storage));
    let task_id = task.id.clone();
    let message = request.message;
    tokio::spawn(async move {
        if let Err(e) = handler.handle_streaming_task(task, message, emitter).await {
            error!("streaming task handler for task {task_id} failed: {e}");
        }
    });

    let envelope_id = id.clone();
    let stream = ReceiverStream::new(rx).map(move |response| {
        let envelope = serde_json::json!({
            "jsonrpc": "2.0",
            "id": envelope_id.clone(),
            "result": response,
        });
        Ok::<_, Infallible>(
            Event::default()
                .json_data(envelope)
                .unwrap_or_else(|e| Event::default().data(format!("serialization error: {e}"))),
        )
    });

    let stream: Box<dyn Stream<Item = Result<Event, Infallible>> + Send + Unpin> =
        Box::new(Box::pin(stream));

    Sse::new(stream)
        .keep_alive(KeepAlive::default())
        .into_response()
}

fn handle_tasks_get(state: &Arc<AppState>, id: Value, params: Value) -> Json<Value> {
    let request: GetTaskRequest = match serde_json::from_value(params) {
        Ok(r) => r,
        Err(e) => return invalid_params(id, e),
    };

    let task_id = match parse_task_name(&request.name) {
        Some(parsed) => parsed,
        None => {
            return json_rpc_error(
                id,
                jsonrpc_errors::INVALID_PARAMS,
                "Invalid params",
                Some(Value::String(format!(
                    "`name` must be of the form tasks/{{task_id}} (got {:?})",
                    request.name
                ))),
            );
        }
    };

    match state.server.storage.get_task(task_id) {
        Some(mut task) => {
            if let Some(limit) = request.history_length {
                let limit = limit.max(0) as usize;
                if task.history.len() > limit {
                    let skip = task.history.len() - limit;
                    task.history = task.history.split_off(skip);
                }
            }
            match serde_json::to_value(task) {
                Ok(v) => json_rpc_success(id, v),
                Err(e) => json_rpc_error(
                    id,
                    jsonrpc_errors::INTERNAL_ERROR,
                    "Internal error",
                    Some(Value::String(e.to_string())),
                ),
            }
        }
        None => json_rpc_error(
            id,
            jsonrpc_errors::TASK_NOT_FOUND,
            "Task not found",
            Some(Value::String(request.name)),
        ),
    }
}

fn handle_tasks_list(state: &Arc<AppState>, id: Value, params: Value) -> Json<Value> {
    let request: ListTasksRequest = match serde_json::from_value(params) {
        Ok(r) => r,
        Err(e) => return invalid_params(id, e),
    };

    let mut tasks = state.server.storage.list_tasks();

    if !request.context_id.is_empty() {
        tasks.retain(|t| t.context_id == request.context_id);
    }
    if !matches!(request.status, TaskState::TaskStateUnspecified) {
        tasks.retain(|t| t.status.state == request.status);
    }

    let total_size = tasks.len() as i32;
    let page_size = request.page_size.unwrap_or(50).clamp(1, 100);
    if tasks.len() > page_size as usize {
        tasks.truncate(page_size as usize);
    }

    let response = ListTasksResponse {
        next_page_token: String::new(),
        page_size,
        tasks,
        total_size,
    };

    match serde_json::to_value(response) {
        Ok(v) => json_rpc_success(id, v),
        Err(e) => json_rpc_error(
            id,
            jsonrpc_errors::INTERNAL_ERROR,
            "Internal error",
            Some(Value::String(e.to_string())),
        ),
    }
}

fn handle_tasks_cancel(state: &Arc<AppState>, id: Value, params: Value) -> Json<Value> {
    let request: CancelTaskRequest = match serde_json::from_value(params) {
        Ok(r) => r,
        Err(e) => return invalid_params(id, e),
    };

    let task_id = match parse_task_name(&request.name) {
        Some(t) => t.to_string(),
        None => {
            return json_rpc_error(
                id,
                jsonrpc_errors::INVALID_PARAMS,
                "Invalid params",
                Some(Value::String(format!(
                    "`name` must be of the form tasks/{{task_id}} (got {:?})",
                    request.name
                ))),
            );
        }
    };

    let existing = match state.server.storage.get_task(&task_id) {
        Some(t) => t,
        None => {
            return json_rpc_error(
                id,
                jsonrpc_errors::TASK_NOT_FOUND,
                "Task not found",
                Some(Value::String(request.name)),
            );
        }
    };

    if matches!(
        existing.status.state,
        TaskState::TaskStateCompleted
            | TaskState::TaskStateFailed
            | TaskState::TaskStateCancelled
            | TaskState::TaskStateRejected
    ) {
        return json_rpc_error(
            id,
            jsonrpc_errors::TASK_NOT_CANCELABLE,
            "Task cannot be cancelled in its current state",
            Some(Value::String(format!(
                "task {:?} is in terminal state {:?}",
                task_id, existing.status.state
            ))),
        );
    }

    let updated = state
        .server
        .storage
        .update_task(&task_id, |t| {
            t.status = TaskStatus {
                message: None,
                state: TaskState::TaskStateCancelled,
                timestamp: Some(Timestamp(chrono::Utc::now())),
            };
        })
        .expect("task disappeared between get and update");

    match serde_json::to_value(updated) {
        Ok(v) => json_rpc_success(id, v),
        Err(e) => json_rpc_error(
            id,
            jsonrpc_errors::INTERNAL_ERROR,
            "Internal error",
            Some(Value::String(e.to_string())),
        ),
    }
}

fn handle_set_push_config(state: &Arc<AppState>, id: Value, params: Value) -> Json<Value> {
    let request: SetTaskPushNotificationConfigRequest = match serde_json::from_value(params) {
        Ok(r) => r,
        Err(e) => return invalid_params(id, e),
    };

    let canonical_name = format!(
        "{}/pushNotificationConfigs/{}",
        request.parent, request.config_id
    );

    let mut config = request.config;
    if config.name.is_empty() {
        config.name = canonical_name.clone();
    }

    state
        .server
        .storage
        .put_push_notification_config(config.clone());

    match serde_json::to_value(config) {
        Ok(v) => json_rpc_success(id, v),
        Err(e) => json_rpc_error(
            id,
            jsonrpc_errors::INTERNAL_ERROR,
            "Internal error",
            Some(Value::String(e.to_string())),
        ),
    }
}

fn handle_get_push_config(state: &Arc<AppState>, id: Value, params: Value) -> Json<Value> {
    let request: GetTaskPushNotificationConfigRequest = match serde_json::from_value(params) {
        Ok(r) => r,
        Err(e) => return invalid_params(id, e),
    };

    match state
        .server
        .storage
        .get_push_notification_config(&request.name)
    {
        Some(config) => match serde_json::to_value(config) {
            Ok(v) => json_rpc_success(id, v),
            Err(e) => json_rpc_error(
                id,
                jsonrpc_errors::INTERNAL_ERROR,
                "Internal error",
                Some(Value::String(e.to_string())),
            ),
        },
        None => json_rpc_error(
            id,
            jsonrpc_errors::TASK_NOT_FOUND,
            "Push notification config not found",
            Some(Value::String(request.name)),
        ),
    }
}

fn handle_list_push_configs(state: &Arc<AppState>, id: Value, params: Value) -> Json<Value> {
    let request: ListTaskPushNotificationConfigRequest = match serde_json::from_value(params) {
        Ok(r) => r,
        Err(e) => return invalid_params(id, e),
    };

    let configs = state
        .server
        .storage
        .list_push_notification_configs(&request.parent);

    let response = ListTaskPushNotificationConfigResponse {
        configs,
        next_page_token: String::new(),
    };

    match serde_json::to_value(response) {
        Ok(v) => json_rpc_success(id, v),
        Err(e) => json_rpc_error(
            id,
            jsonrpc_errors::INTERNAL_ERROR,
            "Internal error",
            Some(Value::String(e.to_string())),
        ),
    }
}

fn handle_delete_push_config(state: &Arc<AppState>, id: Value, params: Value) -> Json<Value> {
    let request: DeleteTaskPushNotificationConfigRequest = match serde_json::from_value(params) {
        Ok(r) => r,
        Err(e) => return invalid_params(id, e),
    };

    let removed = state
        .server
        .storage
        .delete_push_notification_config(&request.name);

    if !removed {
        return json_rpc_error(
            id,
            jsonrpc_errors::TASK_NOT_FOUND,
            "Push notification config not found",
            Some(Value::String(request.name)),
        );
    }

    // The A2A spec leaves the response empty for delete operations; we mirror
    // the Go ADK and return an empty object.
    json_rpc_success(id, serde_json::json!({}))
}

#[cfg(test)]
mod tests {
    use super::*;
    use inference_gateway_sdk::{ChatCompletionToolType, FunctionObject, FunctionParameters};

    #[derive(Debug)]
    struct TestCase {
        name: &'static str,
        #[allow(dead_code)]
        description: &'static str,
    }

    #[tokio::test]
    async fn test_server_builder() {
        let test_cases = vec![
            TestCase {
                name: "default_builder",
                description: "Should create server with default configuration",
            },
            TestCase {
                name: "with_config",
                description: "Should create server with custom configuration",
            },
        ];

        for test_case in test_cases {
            match test_case.name {
                "default_builder" => {
                    let agent_card_json = serde_json::json!({
                        "name": "Test Agent",
                        "description": "A test agent for unit testing",
                        "version": "1.0.0",
                        "protocolVersion": "0.2.6",
                        "url": "http://localhost:8080/a2a",
                        "preferredTransport": "JSONRPC",
                        "capabilities": {
                            "streaming": true,
                            "pushNotifications": false,
                            "stateTransitionHistory": false
                        },
                        "defaultInputModes": ["text/plain"],
                        "defaultOutputModes": ["text/plain"],
                        "skills": [
                            {
                                "id": "test-skill",
                                "name": "Test Skill",
                                "description": "A test skill",
                                "tags": ["test"]
                            }
                        ]
                    });
                    let agent_card: AgentCard = serde_json::from_value(agent_card_json).unwrap();

                    let server = A2AServerBuilder::new()
                        .with_agent_card(agent_card)
                        .with_default_task_handlers()
                        .build()
                        .await;
                    assert!(server.is_ok(), "Default builder should succeed");
                }
                "with_config" => {
                    let agent_card_json = serde_json::json!({
                        "name": "Test Agent",
                        "description": "A test agent for unit testing",
                        "version": "1.0.0",
                        "protocolVersion": "0.2.6",
                        "url": "http://localhost:8080/a2a",
                        "preferredTransport": "JSONRPC",
                        "capabilities": {
                            "streaming": true,
                            "pushNotifications": false,
                            "stateTransitionHistory": false
                        },
                        "defaultInputModes": ["text/plain"],
                        "defaultOutputModes": ["text/plain"],
                        "skills": [
                            {
                                "id": "test-skill",
                                "name": "Test Skill",
                                "description": "A test skill",
                                "tags": ["test"]
                            }
                        ]
                    });
                    let agent_card: AgentCard = serde_json::from_value(agent_card_json).unwrap();

                    let config = Config::default();
                    let server = A2AServerBuilder::new()
                        .with_config(config)
                        .with_agent_card(agent_card)
                        .with_default_task_handlers()
                        .build()
                        .await;
                    assert!(server.is_ok(), "Builder with config should succeed");
                }
                _ => {}
            }
        }
    }

    #[tokio::test]
    async fn test_agent_builder() {
        let test_cases = vec![
            TestCase {
                name: "default_agent",
                description: "Should create agent with default configuration",
            },
            TestCase {
                name: "with_system_prompt",
                description: "Should create agent with custom system prompt",
            },
            TestCase {
                name: "with_toolbox",
                description: "Should create agent with toolbox",
            },
            TestCase {
                name: "with_tool_handlers",
                description: "Should create agent with tool handlers",
            },
            TestCase {
                name: "empty_model_error",
                description: "Should fail when model is empty",
            },
            TestCase {
                name: "invalid_provider_error",
                description: "Should fail when provider is invalid",
            },
        ];

        for test_case in test_cases {
            match test_case.name {
                "default_agent" => {
                    let agent = AgentBuilder::new().build().await;
                    assert!(agent.is_ok(), "Default agent builder should succeed");
                }
                "with_system_prompt" => {
                    let agent = AgentBuilder::new()
                        .with_system_prompt("You are a helpful assistant")
                        .build()
                        .await;
                    assert!(
                        agent.is_ok(),
                        "Agent builder with system prompt should succeed"
                    );
                    let agent = agent.unwrap();
                    assert_eq!(
                        agent.system_prompt,
                        Some("You are a helpful assistant".to_string())
                    );
                }
                "with_toolbox" => {
                    let tool = ChatCompletionTool {
                        type_: ChatCompletionToolType::Function,
                        function: FunctionObject {
                            name: "test_tool".to_string(),
                            description: Some("A test tool".to_string()),
                            parameters: Some(FunctionParameters(
                                serde_json::json!({
                                    "type": "object",
                                    "properties": {
                                        "input": {
                                            "type": "string",
                                            "description": "Test input"
                                        }
                                    },
                                    "required": ["input"]
                                })
                                .as_object()
                                .unwrap()
                                .clone(),
                            )),
                            strict: false,
                        },
                    };
                    let tools = vec![tool];
                    let agent = AgentBuilder::new()
                        .with_toolbox(tools.clone())
                        .build()
                        .await;
                    assert!(agent.is_ok(), "Agent builder with toolbox should succeed");
                    let agent = agent.unwrap();
                    assert!(agent.toolbox.is_some(), "Agent should have toolbox");
                    assert_eq!(
                        agent.toolbox.as_ref().unwrap().len(),
                        1,
                        "Toolbox should have one tool"
                    );
                }
                "with_tool_handlers" => {
                    let tool = ChatCompletionTool {
                        type_: ChatCompletionToolType::Function,
                        function: FunctionObject {
                            name: "test_handler".to_string(),
                            description: Some("A test tool with handler".to_string()),
                            parameters: Some(FunctionParameters(
                                serde_json::json!({
                                    "type": "object",
                                    "properties": {
                                        "input": {
                                            "type": "string",
                                            "description": "Test input"
                                        }
                                    },
                                    "required": ["input"]
                                })
                                .as_object()
                                .unwrap()
                                .clone(),
                            )),
                            strict: false,
                        },
                    };
                    let tools = vec![tool];
                    let agent = AgentBuilder::new()
                        .with_toolbox(tools.clone())
                        .with_function_tool("test_handler".to_string(), |args| {
                            let input = args["input"].as_str().unwrap_or("default");
                            Ok(format!("Processed: {input}"))
                        })
                        .build()
                        .await;
                    assert!(
                        agent.is_ok(),
                        "Agent builder with tool handlers should succeed"
                    );
                    let agent = agent.unwrap();
                    assert!(agent.toolbox.is_some(), "Agent should have toolbox");
                    assert_eq!(
                        agent.tool_handlers.len(),
                        1,
                        "Agent should have one tool handler"
                    );
                    assert!(
                        agent.tool_handlers.contains_key("test_handler"),
                        "Agent should have the test_handler"
                    );
                }
                "empty_model_error" => {
                    let config = AgentConfig {
                        model: "".to_string(),
                        ..Default::default()
                    };
                    let agent = AgentBuilder::new().with_config(&config).build().await;
                    assert!(agent.is_err(), "Agent builder should fail with empty model");
                    assert!(
                        agent
                            .unwrap_err()
                            .to_string()
                            .contains("Model cannot be empty")
                    );
                }
                "invalid_provider_error" => {
                    let config = AgentConfig {
                        provider: "invalid_provider".to_string(),
                        ..Default::default()
                    };
                    let agent = AgentBuilder::new().with_config(&config).build().await;
                    assert!(
                        agent.is_err(),
                        "Agent builder should fail with invalid provider"
                    );
                    assert!(
                        agent
                            .unwrap_err()
                            .to_string()
                            .contains("Unsupported provider")
                    );
                }
                _ => {}
            }
        }
    }

    #[tokio::test]
    async fn message_stream_emits_state_transitions_end_to_end() {
        use crate::A2AClient;
        use crate::a2a_types::{Message as A2AMessage, Role, SendMessageRequest};
        use futures_util::StreamExt;

        #[derive(Debug)]
        struct EchoStream;

        #[async_trait::async_trait]
        impl StreamableTaskHandler for EchoStream {
            async fn handle_streaming_task(
                &self,
                task: Task,
                message: Option<A2AMessage>,
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
                let user_text = message
                    .as_ref()
                    .map(|m| {
                        m.parts
                            .iter()
                            .filter_map(|p| p.text.clone())
                            .collect::<Vec<_>>()
                            .join("")
                    })
                    .unwrap_or_default();
                let reply_text = format!("Echo: {user_text}");
                emitter
                    .emit_text_artifact(&task.id, &task.context_id, reply_text.clone(), true)
                    .await?;
                let reply_message = build_agent_text_message(&task, &reply_text);
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

        let agent_card: AgentCard = serde_json::from_value(serde_json::json!({
            "name": "Test Stream Agent",
            "description": "Streaming SSE end-to-end test",
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
                {
                    "id": "echo",
                    "name": "echo",
                    "description": "echo",
                    "tags": ["echo"]
                }
            ]
        }))
        .unwrap();

        let server = A2AServerBuilder::new()
            .with_agent_card(agent_card)
            .with_streaming_task_handler(EchoStream)
            .build()
            .await
            .expect("server builds");

        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("local addr");
        let app = Router::new()
            .route("/a2a", post(a2a_handler))
            .with_state(Arc::new(AppState { server }));
        tokio::spawn(async move {
            axum::serve(listener, app).await.ok();
        });

        let client = A2AClient::new(format!("http://{addr}")).expect("client");

        let request = SendMessageRequest {
            configuration: None,
            message: Some(A2AMessage {
                context_id: None,
                extensions: vec![],
                message_id: "msg-1".to_string(),
                metadata: None,
                parts: vec![Part {
                    data: None,
                    file: None,
                    metadata: None,
                    text: Some("ping".to_string()),
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

        // Expect exactly four events in order:
        // 1) task wrapper (Submitted)
        // 2) status update → Working (final=false)
        // 3) artifact update with echo text
        // 4) status update → Completed (final=true)
        assert_eq!(
            events.len(),
            4,
            "expected 4 events, got {}: {:?}",
            events.len(),
            events
        );

        let initial_task = events[0]
            .task
            .as_ref()
            .expect("first event carries the task");
        assert_eq!(initial_task.status.state, TaskState::TaskStateSubmitted);

        let working = events[1]
            .status_update
            .as_ref()
            .expect("second event is a status update");
        assert_eq!(working.status.state, TaskState::TaskStateWorking);
        assert!(!working.final_);

        let artifact = events[2]
            .artifact_update
            .as_ref()
            .expect("third event is an artifact update");
        let text = artifact
            .artifact
            .parts
            .iter()
            .filter_map(|p| p.text.clone())
            .collect::<String>();
        assert_eq!(text, "Echo: ping");

        let completed = events[3]
            .status_update
            .as_ref()
            .expect("fourth event is a status update");
        assert_eq!(completed.status.state, TaskState::TaskStateCompleted);
        assert!(completed.final_);
        let final_message_text = completed
            .status
            .message
            .as_ref()
            .expect("completed status carries the final agent message")
            .parts
            .iter()
            .filter_map(|p| p.text.clone())
            .collect::<String>();
        assert_eq!(final_message_text, "Echo: ping");
    }

    #[tokio::test]
    async fn message_stream_uses_custom_handler() {
        use crate::A2AClient;
        use crate::a2a_types::{Message as A2AMessage, Role, SendMessageRequest};
        use futures_util::StreamExt;

        #[derive(Debug)]
        struct TwoStateHandler;

        #[async_trait::async_trait]
        impl StreamableTaskHandler for TwoStateHandler {
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
                        TaskState::TaskStateFailed,
                        None,
                        true,
                    )
                    .await
            }
        }

        let agent_card: AgentCard = serde_json::from_value(serde_json::json!({
            "name": "Custom Handler Test",
            "description": "Verifies with_streaming_task_handler is used",
            "version": "0.0.0",
            "protocolVersion": "0.2.6",
            "url": "http://localhost/a2a",
            "preferredTransport": "JSONRPC",
            "capabilities": {"streaming": true, "pushNotifications": false, "stateTransitionHistory": false},
            "defaultInputModes": ["text/plain"],
            "defaultOutputModes": ["text/plain"],
            "skills": [{"id": "x", "name": "x", "description": "x", "tags": ["x"]}]
        }))
        .unwrap();

        let server = A2AServerBuilder::new()
            .with_agent_card(agent_card)
            .with_streaming_task_handler(TwoStateHandler)
            .build()
            .await
            .expect("server");

        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");
        let app = Router::new()
            .route("/a2a", post(a2a_handler))
            .with_state(Arc::new(AppState { server }));
        tokio::spawn(async move {
            axum::serve(listener, app).await.ok();
        });

        let client = A2AClient::new(format!("http://{addr}")).expect("client");

        let request = SendMessageRequest {
            configuration: None,
            message: Some(A2AMessage {
                context_id: None,
                extensions: vec![],
                message_id: "msg-2".to_string(),
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

        assert_eq!(events.len(), 2);
        assert!(events[0].task.is_some());
        let final_update = events[1].status_update.as_ref().expect("status update");
        assert_eq!(final_update.status.state, TaskState::TaskStateFailed);
        assert!(final_update.final_);
    }

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

    #[tokio::test]
    async fn build_fails_when_no_handler_configured() {
        let err = A2AServerBuilder::new()
            .with_agent_card(agent_card_with_streaming(true))
            .build()
            .await
            .expect_err("build should reject empty handler configuration");
        let message = err.to_string();
        assert!(
            message.contains("at least one task handler"),
            "unexpected error: {message}"
        );
    }

    #[tokio::test]
    async fn build_requires_streaming_handler_when_streaming_enabled() {
        let err = A2AServerBuilder::new()
            .with_agent_card(agent_card_with_streaming(true))
            .with_default_background_task_handler()
            .build()
            .await
            .expect_err("streaming-enabled card without streaming handler should fail");
        let message = err.to_string();
        assert!(
            message.contains("streaming task handler is required"),
            "unexpected error: {message}"
        );
    }

    #[tokio::test]
    async fn build_requires_background_handler_when_streaming_disabled() {
        let err = A2AServerBuilder::new()
            .with_agent_card(agent_card_with_streaming(false))
            .with_default_streaming_task_handler()
            .build()
            .await
            .expect_err("streaming-disabled card without background handler should fail");
        let message = err.to_string();
        assert!(
            message.contains("background task handler is required"),
            "unexpected error: {message}"
        );
    }

    #[tokio::test]
    async fn build_succeeds_with_default_task_handlers() {
        let server = A2AServerBuilder::new()
            .with_agent_card(agent_card_with_streaming(true))
            .with_default_task_handlers()
            .build()
            .await;
        assert!(
            server.is_ok(),
            "with_default_task_handlers should satisfy validation"
        );
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
        use crate::a2a_types::{Message as A2AMessage, Role, SendMessageRequest};
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
            ..AgentConfig::default()
        };
        let agent = AgentBuilder::new()
            .with_config(&agent_config)
            .build()
            .await
            .expect("agent builds");

        let server = A2AServerBuilder::new()
            .with_agent_card(agent_card)
            .with_gateway_url(format!("http://{gateway_addr}"))
            .with_agent(agent)
            .with_default_task_handlers()
            .build()
            .await
            .expect("server builds");

        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind a2a");
        let addr = listener.local_addr().expect("addr");
        let app = Router::new()
            .route("/a2a", post(a2a_handler))
            .with_state(Arc::new(AppState { server }));
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

        // Expected sequence:
        //   [0] task wrapper (Submitted)
        //   [1] status Working
        //   [2..=4] three artifact deltas — texts "Hel", "lo ", "world"
        //   [5] final artifact chunk (last_chunk=true, empty parts)
        //   [6] status Completed (final=true) with accumulated message
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

        // Deltas: collect artifact_ids and texts across events 2, 3, 4.
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
}

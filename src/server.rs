use crate::a2a_types::{
    AgentCard, CancelTaskRequest, DeleteTaskPushNotificationConfigRequest,
    GetTaskPushNotificationConfigRequest, GetTaskRequest, ListTaskPushNotificationConfigRequest,
    ListTaskPushNotificationConfigResponse, ListTasksRequest, ListTasksResponse,
    Message as A2AMessage, Part, Role, SendMessageRequest, SendMessageResponse,
    SetTaskPushNotificationConfigRequest, Task, TaskPushNotificationConfig, TaskState, TaskStatus,
    Timestamp,
};
use crate::client::HealthStatus;
use crate::config::{AgentConfig, Config};
use crate::storage::{InMemoryStorage, parse_task_name};
use anyhow::{Result, anyhow};
use axum::{
    Router,
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
};
use inference_gateway_sdk::{
    ChatCompletionTool, InferenceGatewayAPI, InferenceGatewayClient, Message, MessageContent,
    MessageRole, Provider,
};
use serde_json::Value;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
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

fn message_content_to_string(content: &MessageContent) -> String {
    match content {
        MessageContent::String(s) => s.clone(),
        MessageContent::Array(parts) => serde_json::to_string(parts).unwrap_or_default(),
    }
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
    agent: Option<Agent>,
    gateway_url: String,
    storage: Arc<InMemoryStorage>,
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
    agent: Option<Agent>,
    gateway_url: Option<String>,
    storage: Option<Arc<InMemoryStorage>>,
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
        self.agent = Some(agent);
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

        Ok(A2AServer {
            config,
            agent_card,
            agent: self.agent,
            gateway_url,
            storage: self.storage.unwrap_or_else(|| Arc::new(InMemoryStorage::new())),
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

async fn a2a_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Json<Value> {
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
        );
    }

    let method = match payload.get("method").and_then(|v| v.as_str()) {
        Some(m) => m.to_string(),
        None => {
            return json_rpc_error(
                id,
                jsonrpc_errors::INVALID_REQUEST,
                "Invalid Request",
                Some(Value::String("Missing \"method\" field".to_string())),
            );
        }
    };

    let params = payload.get("params").cloned().unwrap_or(Value::Null);

    match method.as_str() {
        "message/send" => handle_message_send(&state, id, params).await,
        "message/stream" => handle_message_stream(&state, id, params).await,
        "tasks/get" => handle_tasks_get(&state, id, params),
        "tasks/list" => handle_tasks_list(&state, id, params),
        "tasks/cancel" => handle_tasks_cancel(&state, id, params),
        "tasks/pushNotificationConfig/set" => handle_set_push_config(&state, id, params),
        "tasks/pushNotificationConfig/get" => handle_get_push_config(&state, id, params),
        "tasks/pushNotificationConfig/list" => handle_list_push_configs(&state, id, params),
        "tasks/pushNotificationConfig/delete" => handle_delete_push_config(&state, id, params),
        other => {
            warn!("Unknown JSON-RPC method requested: {other}");
            json_rpc_error(
                id,
                jsonrpc_errors::METHOD_NOT_FOUND,
                "Method not found",
                Some(Value::String(other.to_string())),
            )
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

fn build_task_from_request(req: &SendMessageRequest) -> Task {
    let task_id = uuid::Uuid::new_v4().to_string();
    let context_id = req
        .message
        .as_ref()
        .and_then(|m| m.context_id.clone())
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

    let mut history = Vec::new();
    if let Some(msg) = req.message.clone() {
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

/// Generate a reply via the configured inference gateway. Only invoked when
/// the caller asked for a blocking `message/send`; otherwise the task is
/// returned in the `submitted` state so it can later be cancelled or polled
/// via `tasks/get`. When no agent is configured (or the gateway is
/// unreachable) the helper falls back to an echo so offline integration
/// tests can still complete the path end-to-end.
async fn synthesize_agent_reply(
    server: &A2AServer,
    incoming: &Option<A2AMessage>,
) -> Option<A2AMessage> {
    let context_id = incoming.as_ref().and_then(|m| m.context_id.clone());

    let user_text = incoming
        .as_ref()
        .map(|m| {
            m.parts
                .iter()
                .filter_map(|p| p.text.clone())
                .collect::<Vec<_>>()
                .join("\n")
        })
        .unwrap_or_default();

    let echo_reply = || A2AMessage {
        context_id: context_id.clone(),
        extensions: vec![],
        message_id: uuid::Uuid::new_v4().to_string(),
        metadata: None,
        parts: vec![Part {
            data: None,
            file: None,
            metadata: None,
            text: Some(format!("Echo: {user_text}")),
        }],
        reference_task_ids: vec![],
        role: Role::RoleAgent,
        task_id: None,
    };

    let Some(agent) = server.agent.as_ref() else {
        return Some(echo_reply());
    };

    let mut messages = Vec::new();
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
    if !user_text.is_empty() {
        messages.push(Message {
            role: MessageRole::User,
            content: MessageContent::String(user_text.clone()),
            reasoning: None,
            reasoning_content: None,
            tool_call_id: None,
            tool_calls: Vec::new(),
        });
    }

    let gateway_client = InferenceGatewayClient::new(&server.gateway_url);
    let gateway_client = if let Some(tools) = agent.toolbox.clone() {
        gateway_client.with_tools(Some(tools))
    } else {
        gateway_client
    };

    match gateway_client
        .generate_content(agent.provider, &agent.model, messages)
        .await
    {
        Ok(response) => response
            .choices
            .first()
            .map(|choice| A2AMessage {
                context_id: context_id.clone(),
                extensions: vec![],
                message_id: uuid::Uuid::new_v4().to_string(),
                metadata: None,
                parts: vec![Part {
                    data: None,
                    file: None,
                    metadata: None,
                    text: Some(message_content_to_string(&choice.message.content)),
                }],
                reference_task_ids: vec![],
                role: Role::RoleAgent,
                task_id: None,
            })
            .or_else(|| Some(echo_reply())),
        Err(e) => {
            warn!("Inference gateway unavailable, returning echo reply: {e}");
            Some(echo_reply())
        }
    }
}

async fn handle_message_send(state: &Arc<AppState>, id: Value, params: Value) -> Json<Value> {
    let request: SendMessageRequest = match serde_json::from_value(params) {
        Ok(r) => r,
        Err(e) => return invalid_params(id, e),
    };

    let blocking = request
        .configuration
        .as_ref()
        .map(|c| c.blocking)
        .unwrap_or(false);

    let mut task = build_task_from_request(&request);

    let reply = if blocking {
        let reply = synthesize_agent_reply(&state.server, &request.message).await;
        if let Some(ref msg) = reply {
            task.history.push(msg.clone());
            task.status = TaskStatus {
                message: Some(msg.clone()),
                state: TaskState::TaskStateCompleted,
                timestamp: Some(Timestamp(chrono::Utc::now())),
            };
        }
        reply
    } else {
        None
    };

    state.server.storage.put_task(task.clone());

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

async fn handle_message_stream(state: &Arc<AppState>, id: Value, params: Value) -> Json<Value> {
    // True SSE streaming is tracked separately; here we surface a synchronous
    // SendMessageResponse so callers can dispatch the method without
    // failing. The response payload mirrors `message/send` so the wire shape
    // is parity-compatible with the Go ADK's first stream chunk.
    handle_message_send(state, id, params).await
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
}

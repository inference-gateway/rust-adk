use crate::a2a_types::{
    AgentCard, DeleteTaskPushNotificationConfigParams, ListTaskPushNotificationConfigParams,
    Message as A2aMessage, MessageRole as A2aMessageRole, MessageSendParams, Part, Task,
    TaskIdParams, TaskPushNotificationConfig, TaskQueryParams, TaskState, TaskStatus,
    TaskStatusUpdateEvent, TextPart,
};
use crate::client::HealthStatus;
use crate::config::{AgentConfig, Config};
use anyhow::{Result, anyhow};
use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::{
        IntoResponse, Response,
        sse::{Event, KeepAlive, Sse},
    },
    routing::{get, post},
};
use futures::stream;
use inference_gateway_sdk::{
    InferenceGatewayAPI, InferenceGatewayClient, Message, MessageRole, Provider, Tool,
};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{debug, error, info, warn};
use uuid::Uuid;

/// JSON-RPC error code: invalid request envelope.
const JSONRPC_ERR_INVALID_REQUEST: i64 = -32600;
/// JSON-RPC error code: method not found.
const JSONRPC_ERR_METHOD_NOT_FOUND: i64 = -32601;
/// JSON-RPC error code: invalid parameters.
const JSONRPC_ERR_INVALID_PARAMS: i64 = -32602;
/// JSON-RPC error code: internal server error.
const JSONRPC_ERR_INTERNAL: i64 = -32603;
/// A2A error code: task not found.
const A2A_ERR_TASK_NOT_FOUND: i64 = -32001;
/// A2A error code: task not cancelable.
const A2A_ERR_TASK_NOT_CANCELABLE: i64 = -32002;

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

fn parse_provider(provider_str: &str) -> Result<Provider> {
    match provider_str.to_lowercase().as_str() {
        "groq" => Ok(Provider::Groq),
        "google" => Ok(Provider::Google),
        "openai" => Ok(Provider::OpenAI),
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
    toolbox: Option<Vec<Tool>>,
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
}

pub struct AgentBuilder {
    config: Option<AgentConfig>,
    system_prompt: Option<String>,
    max_chat_completion: u32,
    max_conversation_history: u32,
    toolbox: Option<Vec<Tool>>,
    tool_handlers: HashMap<String, Box<dyn ToolHandler>>,
}

/// Per-server in-memory storage shared across handlers.
#[derive(Debug, Default)]
struct TaskStore {
    /// task_id -> Task
    tasks: HashMap<String, Task>,
    /// (task_id, push_notification_config_id) -> TaskPushNotificationConfig
    push_configs: HashMap<(String, String), TaskPushNotificationConfig>,
}

#[derive(Debug)]
struct AppState {
    server: A2AServer,
    store: Arc<RwLock<TaskStore>>,
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
                    info!("Overriding agent card URL: {} -> {}", card.url, url);
                    card.url = url;
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

    pub fn with_toolbox(mut self, tools: Vec<Tool>) -> Self {
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
    pub fn toolbox(&self) -> Option<&Vec<Tool>> {
        self.toolbox.as_ref()
    }
}

impl A2AServer {
    pub async fn serve(self, addr: SocketAddr) -> Result<()> {
        let state = AppState {
            server: self,
            store: Arc::new(RwLock::new(TaskStore::default())),
        };

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
            "sdk_version": "0.11.0"
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

/// Build a JSON-RPC error response value.
fn jsonrpc_error(id: Value, code: i64, message: &str, data: Option<Value>) -> Value {
    let mut error = json!({
        "code": code,
        "message": message,
    });
    if let Some(d) = data {
        error["data"] = d;
    }
    json!({
        "jsonrpc": "2.0",
        "id": id,
        "error": error,
    })
}

/// Build a JSON-RPC success response value.
fn jsonrpc_result(id: Value, result: Value) -> Value {
    json!({
        "jsonrpc": "2.0",
        "id": id,
        "result": result,
    })
}

/// Top-level dispatcher for the `/a2a` endpoint. Reads the JSON-RPC envelope and
/// routes to the appropriate per-method handler based on the `method` field.
async fn a2a_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Response {
    debug!("A2A request received: {:?}", payload);

    // Extract the `id` so error responses always echo it back, even when validation fails.
    let id = payload.get("id").cloned().unwrap_or(Value::Null);

    // Validate JSON-RPC envelope.
    let jsonrpc = payload.get("jsonrpc").and_then(Value::as_str);
    if jsonrpc != Some("2.0") {
        warn!("Rejecting request: invalid or missing jsonrpc field");
        return Json(jsonrpc_error(
            id,
            JSONRPC_ERR_INVALID_REQUEST,
            "Invalid Request",
            Some(json!("jsonrpc must be \"2.0\"")),
        ))
        .into_response();
    }

    let method = match payload.get("method").and_then(Value::as_str) {
        Some(m) => m.to_string(),
        None => {
            warn!("Rejecting request: missing method field");
            return Json(jsonrpc_error(
                id,
                JSONRPC_ERR_INVALID_REQUEST,
                "Invalid Request",
                Some(json!("method field is required")),
            ))
            .into_response();
        }
    };

    let params = payload.get("params").cloned().unwrap_or(Value::Null);

    match method.as_str() {
        "message/send" => handle_message_send(state, id, params).await,
        "message/stream" => handle_message_stream(state, id, params).await,
        "tasks/get" => handle_tasks_get(state, id, params).await,
        "tasks/cancel" => handle_tasks_cancel(state, id, params).await,
        "tasks/pushNotificationConfig/set" => handle_push_set(state, id, params).await,
        "tasks/pushNotificationConfig/get" => handle_push_get(state, id, params).await,
        "tasks/pushNotificationConfig/list" => handle_push_list(state, id, params).await,
        "tasks/pushNotificationConfig/delete" => handle_push_delete(state, id, params).await,
        "tasks/resubscribe" => handle_resubscribe(state, id, params).await,
        other => {
            warn!("Method not found: {}", other);
            Json(jsonrpc_error(
                id,
                JSONRPC_ERR_METHOD_NOT_FOUND,
                "Method not found",
                Some(json!({ "method": other })),
            ))
            .into_response()
        }
    }
}

/// Try to typed-parse params; on failure, return a JSON-RPC -32602 error response.
fn parse_params<T: serde::de::DeserializeOwned>(
    id: &Value,
    params: Value,
    method: &str,
) -> Result<T, Response> {
    match serde_json::from_value::<T>(params) {
        Ok(v) => Ok(v),
        Err(e) => {
            warn!("Invalid params for {}: {}", method, e);
            Err(Json(jsonrpc_error(
                id.clone(),
                JSONRPC_ERR_INVALID_PARAMS,
                "Invalid params",
                Some(json!({ "method": method, "reason": e.to_string() })),
            ))
            .into_response())
        }
    }
}

/// Convert a typed `MessageSendParams` payload to inference-gateway SDK messages,
/// using the configured agent's system prompt as a prefix if present.
fn build_messages_for_send(
    agent: Option<&Agent>,
    params: &MessageSendParams,
) -> Vec<Message> {
    let mut messages = Vec::new();
    if let Some(agent) = agent
        && let Some(sp) = &agent.system_prompt
    {
        messages.push(Message {
            role: MessageRole::System,
            content: sp.clone(),
            ..Default::default()
        });
    }

    let role = match params.message.role {
        A2aMessageRole::User => MessageRole::User,
        A2aMessageRole::Agent => MessageRole::Assistant,
    };
    let content = collect_text_from_parts(&params.message.parts);
    messages.push(Message {
        role,
        content,
        ..Default::default()
    });
    messages
}

fn collect_text_from_parts(parts: &[Part]) -> String {
    parts
        .iter()
        .filter_map(|p| match p {
            Part::TextPart(t) => Some(t.text.clone()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn synthesize_agent_message(text: String) -> A2aMessage {
    A2aMessage {
        context_id: None,
        extensions: Vec::new(),
        kind: "message".to_string(),
        message_id: Uuid::new_v4().to_string(),
        metadata: Default::default(),
        parts: vec![Part::TextPart(TextPart {
            kind: "text".to_string(),
            metadata: Default::default(),
            text,
        })],
        reference_task_ids: Vec::new(),
        role: A2aMessageRole::Agent,
        task_id: None,
    }
}

fn synthesize_task(history: Vec<A2aMessage>, agent_response: A2aMessage) -> Task {
    let task_id = Uuid::new_v4().to_string();
    let context_id = Uuid::new_v4().to_string();
    let mut full_history = history;
    full_history.push(agent_response.clone());
    Task {
        artifacts: Vec::new(),
        context_id,
        history: full_history,
        id: task_id,
        kind: "task".to_string(),
        metadata: Default::default(),
        status: TaskStatus {
            message: Some(agent_response),
            state: TaskState::Completed,
            timestamp: Some(chrono::Utc::now().to_rfc3339()),
        },
    }
}

/// Generate the agent's textual reply, falling back to a deterministic stub when
/// no agent is configured or the gateway is unreachable. The stub keeps unit and
/// integration tests deterministic without a live LLM.
async fn produce_agent_text(state: &AppState, params: &MessageSendParams) -> String {
    let Some(agent) = state.server.agent.as_ref() else {
        debug!("No agent configured — returning stub agent text");
        let user_text = collect_text_from_parts(&params.message.parts);
        return if user_text.is_empty() {
            "Acknowledged.".to_string()
        } else {
            format!("Echo: {user_text}")
        };
    };

    let messages = build_messages_for_send(Some(agent), params);
    let client = InferenceGatewayClient::new(&state.server.gateway_url);
    let client_with_tools = if let Some(tools) = &agent.toolbox {
        client.with_tools(Some(tools.clone()))
    } else {
        client
    };

    match client_with_tools
        .generate_content(agent.provider, &agent.model, messages)
        .await
    {
        Ok(response) => response
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .unwrap_or_else(|| "No response generated".to_string()),
        Err(e) => {
            error!("Inference gateway error: {}", e);
            format!("Error contacting inference gateway: {e}")
        }
    }
}

/// `message/send` — synchronous reply: stores a Task and returns it.
async fn handle_message_send(state: Arc<AppState>, id: Value, params: Value) -> Response {
    let send_params = match parse_params::<MessageSendParams>(&id, params, "message/send") {
        Ok(p) => p,
        Err(resp) => return resp,
    };

    let agent_text = produce_agent_text(&state, &send_params).await;
    let agent_message = synthesize_agent_message(agent_text);
    let task = synthesize_task(vec![send_params.message.clone()], agent_message);

    state
        .store
        .write()
        .await
        .tasks
        .insert(task.id.clone(), task.clone());

    let result = match serde_json::to_value(&task) {
        Ok(v) => v,
        Err(e) => {
            error!("Failed to serialize task: {}", e);
            return Json(jsonrpc_error(
                id,
                JSONRPC_ERR_INTERNAL,
                "Internal error",
                Some(json!(e.to_string())),
            ))
            .into_response();
        }
    };

    Json(jsonrpc_result(id, result)).into_response()
}

/// `message/stream` — emits an SSE stream of `SendStreamingMessageSuccessResponse`
/// events terminated by a final status-update event.
async fn handle_message_stream(state: Arc<AppState>, id: Value, params: Value) -> Response {
    let send_params = match parse_params::<MessageSendParams>(&id, params, "message/stream") {
        Ok(p) => p,
        Err(resp) => return resp,
    };

    let agent_text = produce_agent_text(&state, &send_params).await;
    let agent_message = synthesize_agent_message(agent_text);
    let task = synthesize_task(vec![send_params.message.clone()], agent_message.clone());

    state
        .store
        .write()
        .await
        .tasks
        .insert(task.id.clone(), task.clone());

    // Emit two SSE events: an initial Task envelope, then a final status-update.
    let task_event = jsonrpc_result(
        id.clone(),
        serde_json::to_value(&task).unwrap_or(Value::Null),
    );
    let final_event = jsonrpc_result(
        id.clone(),
        serde_json::to_value(TaskStatusUpdateEvent {
            context_id: task.context_id.clone(),
            final_: true,
            kind: "status-update".to_string(),
            metadata: Default::default(),
            status: task.status.clone(),
            task_id: task.id.clone(),
        })
        .unwrap_or(Value::Null),
    );

    sse_response(vec![task_event, final_event])
}

/// `tasks/get` — looks up a stored task by id.
async fn handle_tasks_get(state: Arc<AppState>, id: Value, params: Value) -> Response {
    let query: TaskQueryParams = match parse_params(&id, params, "tasks/get") {
        Ok(p) => p,
        Err(resp) => return resp,
    };

    let store = state.store.read().await;
    match store.tasks.get(&query.id) {
        Some(task) => {
            let mut task = task.clone();
            if let Some(limit) = query.history_length {
                let limit = limit.max(0) as usize;
                if task.history.len() > limit {
                    let start = task.history.len() - limit;
                    task.history = task.history[start..].to_vec();
                }
            }
            let result = serde_json::to_value(&task).unwrap_or(Value::Null);
            Json(jsonrpc_result(id, result)).into_response()
        }
        None => Json(jsonrpc_error(
            id,
            A2A_ERR_TASK_NOT_FOUND,
            "Task not found",
            Some(json!({ "id": query.id })),
        ))
        .into_response(),
    }
}

/// `tasks/cancel` — marks a stored task as canceled if it is still in flight.
async fn handle_tasks_cancel(state: Arc<AppState>, id: Value, params: Value) -> Response {
    let task_params: TaskIdParams = match parse_params(&id, params, "tasks/cancel") {
        Ok(p) => p,
        Err(resp) => return resp,
    };

    let mut store = state.store.write().await;
    let task = match store.tasks.get_mut(&task_params.id) {
        Some(t) => t,
        None => {
            return Json(jsonrpc_error(
                id,
                A2A_ERR_TASK_NOT_FOUND,
                "Task not found",
                Some(json!({ "id": task_params.id })),
            ))
            .into_response();
        }
    };

    if matches!(
        task.status.state,
        TaskState::Completed | TaskState::Canceled | TaskState::Failed | TaskState::Rejected
    ) {
        return Json(jsonrpc_error(
            id,
            A2A_ERR_TASK_NOT_CANCELABLE,
            "Task cannot be canceled",
            Some(json!({ "id": task_params.id, "state": task.status.state.to_string() })),
        ))
        .into_response();
    }

    task.status = TaskStatus {
        message: task.status.message.clone(),
        state: TaskState::Canceled,
        timestamp: Some(chrono::Utc::now().to_rfc3339()),
    };
    let task = task.clone();
    drop(store);

    let result = serde_json::to_value(&task).unwrap_or(Value::Null);
    Json(jsonrpc_result(id, result)).into_response()
}

/// `tasks/pushNotificationConfig/set` — stores a push notification config under
/// a synthesised id (or the caller-provided id, if any), keyed by task id.
async fn handle_push_set(state: Arc<AppState>, id: Value, params: Value) -> Response {
    let mut config: TaskPushNotificationConfig = match parse_params(
        &id,
        params,
        "tasks/pushNotificationConfig/set",
    ) {
        Ok(p) => p,
        Err(resp) => return resp,
    };

    // The spec lets the caller omit `id` on the inner PushNotificationConfig;
    // we synthesise one so future get/list/delete calls have a stable handle.
    let config_id = config
        .push_notification_config
        .id
        .clone()
        .unwrap_or_else(|| Uuid::new_v4().to_string());
    config.push_notification_config.id = Some(config_id.clone());

    state.store.write().await.push_configs.insert(
        (config.task_id.clone(), config_id.clone()),
        config.clone(),
    );

    let result = serde_json::to_value(&config).unwrap_or(Value::Null);
    Json(jsonrpc_result(id, result)).into_response()
}

/// `tasks/pushNotificationConfig/get` — accepts either `TaskIdParams` or the
/// extended `GetTaskPushNotificationConfigParams`, returning the matching
/// stored config (the first one found if no `pushNotificationConfigId` is
/// supplied).
async fn handle_push_get(state: Arc<AppState>, id: Value, params: Value) -> Response {
    // The schema allows two shapes: TaskIdParams or GetTaskPushNotificationConfigParams.
    // Both have a required `id` field, so we extract it manually and treat
    // `pushNotificationConfigId` as optional regardless of which shape was sent.
    let task_id = match params.get("id").and_then(Value::as_str) {
        Some(s) => s.to_string(),
        None => {
            return Json(jsonrpc_error(
                id,
                JSONRPC_ERR_INVALID_PARAMS,
                "Invalid params",
                Some(json!("expected TaskIdParams or GetTaskPushNotificationConfigParams")),
            ))
            .into_response();
        }
    };
    let config_id_opt = params
        .get("pushNotificationConfigId")
        .and_then(Value::as_str)
        .map(str::to_string);

    let store = state.store.read().await;
    let matched = store
        .push_configs
        .iter()
        .find(|((t, c), _)| t == &task_id && config_id_opt.as_ref().is_none_or(|cid| cid == c))
        .map(|(_, v)| v.clone());

    match matched {
        Some(cfg) => {
            let result = serde_json::to_value(&cfg).unwrap_or(Value::Null);
            Json(jsonrpc_result(id, result)).into_response()
        }
        None => Json(jsonrpc_error(
            id,
            A2A_ERR_TASK_NOT_FOUND,
            "Task push notification config not found",
            Some(json!({ "taskId": task_id, "pushNotificationConfigId": config_id_opt })),
        ))
        .into_response(),
    }
}

/// `tasks/pushNotificationConfig/list` — returns all configs for a task id.
async fn handle_push_list(state: Arc<AppState>, id: Value, params: Value) -> Response {
    let list_params: ListTaskPushNotificationConfigParams = match parse_params(
        &id,
        params,
        "tasks/pushNotificationConfig/list",
    ) {
        Ok(p) => p,
        Err(resp) => return resp,
    };

    let store = state.store.read().await;
    let configs: Vec<TaskPushNotificationConfig> = store
        .push_configs
        .iter()
        .filter_map(|((t, _), v)| {
            if t == &list_params.id {
                Some(v.clone())
            } else {
                None
            }
        })
        .collect();

    let result = serde_json::to_value(&configs).unwrap_or(Value::Null);
    Json(jsonrpc_result(id, result)).into_response()
}

/// `tasks/pushNotificationConfig/delete` — removes a single config by
/// (task_id, push_notification_config_id) if present.
async fn handle_push_delete(state: Arc<AppState>, id: Value, params: Value) -> Response {
    let del_params: DeleteTaskPushNotificationConfigParams = match parse_params(
        &id,
        params,
        "tasks/pushNotificationConfig/delete",
    ) {
        Ok(p) => p,
        Err(resp) => return resp,
    };

    let mut store = state.store.write().await;
    store
        .push_configs
        .remove(&(del_params.id.clone(), del_params.push_notification_config_id.clone()));

    Json(jsonrpc_result(id, Value::Null)).into_response()
}

/// `tasks/resubscribe` — opens an SSE stream for an existing task, emitting
/// the current task state as a final status-update event. If the task does
/// not exist, returns a JSON-RPC error response (not an SSE stream).
async fn handle_resubscribe(state: Arc<AppState>, id: Value, params: Value) -> Response {
    let task_params: TaskIdParams = match parse_params(&id, params, "tasks/resubscribe") {
        Ok(p) => p,
        Err(resp) => return resp,
    };

    let store = state.store.read().await;
    let task = match store.tasks.get(&task_params.id).cloned() {
        Some(t) => t,
        None => {
            return Json(jsonrpc_error(
                id,
                A2A_ERR_TASK_NOT_FOUND,
                "Task not found",
                Some(json!({ "id": task_params.id })),
            ))
            .into_response();
        }
    };
    drop(store);

    let event = jsonrpc_result(
        id.clone(),
        serde_json::to_value(TaskStatusUpdateEvent {
            context_id: task.context_id.clone(),
            final_: true,
            kind: "status-update".to_string(),
            metadata: Default::default(),
            status: task.status.clone(),
            task_id: task.id.clone(),
        })
        .unwrap_or(Value::Null),
    );

    sse_response(vec![event])
}

/// Helper that turns a list of JSON values into an axum SSE response, one
/// event per value, terminated by the connection closing naturally.
fn sse_response(events: Vec<Value>) -> Response {
    let stream = stream::iter(events.into_iter().map(|v| {
        let payload = serde_json::to_string(&v).unwrap_or_else(|_| "{}".to_string());
        Ok::<Event, Infallible>(Event::default().data(payload))
    }));
    Sse::new(stream)
        .keep_alive(KeepAlive::default())
        .into_response()
}

#[cfg(test)]
mod tests {
    use super::*;
    use inference_gateway_sdk::{FunctionObject, ToolType};

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
                    let tool = Tool {
                        r#type: ToolType::Function,
                        function: FunctionObject {
                            name: "test_tool".to_string(),
                            description: "A test tool".to_string(),
                            parameters: serde_json::json!({
                                "type": "object",
                                "properties": {
                                    "input": {
                                        "type": "string",
                                        "description": "Test input"
                                    }
                                },
                                "required": ["input"]
                            }),
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
                    let tool = Tool {
                        r#type: ToolType::Function,
                        function: FunctionObject {
                            name: "test_handler".to_string(),
                            description: "A test tool with handler".to_string(),
                            parameters: serde_json::json!({
                                "type": "object",
                                "properties": {
                                    "input": {
                                        "type": "string",
                                        "description": "Test input"
                                    }
                                },
                                "required": ["input"]
                            }),
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

    /// Compile-time check that the canonical `*Request` type names (one per
    /// JSON-RPC method in the spec) remain re-exportable from
    /// `inference_gateway_adk::a2a_types`.
    #[test]
    fn schema_request_types_are_in_scope() {
        use crate::a2a_types::{
            CancelTaskRequest, DeleteTaskPushNotificationConfigRequest,
            GetTaskPushNotificationConfigRequest, GetTaskRequest,
            ListTaskPushNotificationConfigRequest, PushNotificationConfig, SendMessageRequest,
            SendStreamingMessageRequest, SetTaskPushNotificationConfigRequest,
            TaskResubscriptionRequest,
        };
        fn _assert<T>() {}
        _assert::<SendMessageRequest>();
        _assert::<SendStreamingMessageRequest>();
        _assert::<GetTaskRequest>();
        _assert::<CancelTaskRequest>();
        _assert::<SetTaskPushNotificationConfigRequest>();
        _assert::<GetTaskPushNotificationConfigRequest>();
        _assert::<ListTaskPushNotificationConfigRequest>();
        _assert::<DeleteTaskPushNotificationConfigRequest>();
        _assert::<TaskResubscriptionRequest>();
        _assert::<PushNotificationConfig>();
    }
}

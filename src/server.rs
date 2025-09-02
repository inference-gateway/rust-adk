use crate::a2a_types::AgentCard;
use crate::client::HealthStatus;
use crate::config::{AgentConfig, Config};
use anyhow::{Result, anyhow};
use axum::{
    Router,
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
};
use inference_gateway_sdk::{
    InferenceGatewayAPI, InferenceGatewayClient, Message, MessageRole, Provider, Tool,
};
use serde_json::Value;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{debug, error, info};

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

async fn a2a_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    debug!("A2A request received: {:?}", payload);

    let messages = if let Some(params) = payload.get("params") {
        if let Some(messages_array) = params.get("messages") {
            serde_json::from_value(messages_array.clone()).unwrap_or_else(|_| {
                vec![Message {
                    role: MessageRole::User,
                    content: "Hello from A2A!".to_string(),
                    ..Default::default()
                }]
            })
        } else {
            vec![Message {
                role: MessageRole::User,
                content: "Hello from A2A!".to_string(),
                ..Default::default()
            }]
        }
    } else {
        vec![Message {
            role: MessageRole::User,
            content: "Hello from A2A!".to_string(),
            ..Default::default()
        }]
    };

    let mut final_messages = Vec::new();
    #[allow(clippy::collapsible_if)]
    if let Some(ref agent) = state.server.agent {
        if let Some(ref system_prompt) = agent.system_prompt {
            final_messages.push(Message {
                role: MessageRole::System,
                content: system_prompt.clone(),
                ..Default::default()
            });
        }
    }
    final_messages.extend(messages);

    let gateway_client = InferenceGatewayClient::new(&state.server.gateway_url);

    let (provider, model, toolbox) = match &state.server.agent {
        Some(agent) => (agent.provider, agent.model.clone(), agent.toolbox.clone()),
        None => {
            error!("No agent configured for A2A request");
            let error_response = serde_json::json!({
                "jsonrpc": "2.0",
                "id": payload.get("id"),
                "error": {
                    "code": -32603,
                    "message": "Internal error",
                    "data": "No agent configured. Agent with provider and model must be configured before handling A2A requests."
                }
            });
            return Ok(Json(error_response));
        }
    };

    let client_with_tools = if let Some(tools) = toolbox {
        gateway_client.with_tools(Some(tools))
    } else {
        gateway_client
    };

    match client_with_tools
        .generate_content(provider, &model, final_messages.clone())
        .await
    {
        Ok(response) => {
            let choice = match response.choices.first() {
                Some(choice) => choice,
                None => {
                    debug!("No choice returned from LLM");
                    let error_response = serde_json::json!({
                        "jsonrpc": "2.0",
                        "id": payload.get("id"),
                        "error": {
                            "code": -32603,
                            "message": "Internal error",
                            "data": "No response generated from LLM"
                        }
                    });
                    return Ok(Json(error_response));
                }
            };

            if let Some(tool_calls) = &choice.message.tool_calls {
                debug!("Processing {} tool calls", tool_calls.len());

                let agent = state.server.agent.as_ref().unwrap();

                let mut follow_up_convo = final_messages.clone();

                follow_up_convo.push(Message {
                    role: MessageRole::Assistant,
                    content: choice.message.content.clone(),
                    tool_calls: choice.message.tool_calls.clone(),
                    ..Default::default()
                });

                for tool_call in tool_calls {
                    debug!("Processing tool call: {}", tool_call.function.name);

                    if let Some(handler) = agent.tool_handlers.get(&tool_call.function.name) {
                        match tool_call.function.parse_arguments() {
                            Ok(args) => match handler.handle(args).await {
                                Ok(result) => {
                                    debug!(
                                        "Tool call '{}' completed successfully",
                                        tool_call.function.name
                                    );

                                    follow_up_convo.push(Message {
                                        role: MessageRole::Tool,
                                        content: result,
                                        tool_call_id: Some(tool_call.id.clone()),
                                        ..Default::default()
                                    });
                                }
                                Err(e) => {
                                    error!("Tool call '{}' failed: {}", tool_call.function.name, e);

                                    follow_up_convo.push(Message {
                                        role: MessageRole::Tool,
                                        content: format!("Error: {e}"),
                                        tool_call_id: Some(tool_call.id.clone()),
                                        ..Default::default()
                                    });
                                }
                            },
                            Err(e) => {
                                error!(
                                    "Failed to parse arguments for tool '{}': {}",
                                    tool_call.function.name, e
                                );

                                follow_up_convo.push(Message {
                                    role: MessageRole::Tool,
                                    content: format!("Error parsing arguments: {e}"),
                                    tool_call_id: Some(tool_call.id.clone()),
                                    ..Default::default()
                                });
                            }
                        }
                    } else {
                        error!("No handler found for tool: {}", tool_call.function.name);

                        follow_up_convo.push(Message {
                            role: MessageRole::Tool,
                            content: format!(
                                "Error: No handler found for tool '{}'",
                                tool_call.function.name
                            ),
                            tool_call_id: Some(tool_call.id.clone()),
                            ..Default::default()
                        });
                    }
                }

                debug!("Sending follow-up request with tool results");

                let follow_up_client = InferenceGatewayClient::new(&state.server.gateway_url);
                let follow_up_client_with_tools =
                    if let Some(tools) = &state.server.agent.as_ref().unwrap().toolbox {
                        follow_up_client.with_tools(Some(tools.clone()))
                    } else {
                        follow_up_client
                    };

                match follow_up_client_with_tools
                    .generate_content(provider, &model, follow_up_convo)
                    .await
                {
                    Ok(follow_up_response) => {
                        let final_content = follow_up_response
                            .choices
                            .first()
                            .map(|c| c.message.content.clone())
                            .unwrap_or_else(|| "No final response generated".to_string());

                        let a2a_response = serde_json::json!({
                            "jsonrpc": "2.0",
                            "id": payload.get("id"),
                            "result": {
                                "status": "completed",
                                "message": {
                                    "role": "assistant",
                                    "parts": [{
                                        "kind": "text",
                                        "content": final_content
                                    }]
                                },
                                "timestamp": chrono::Utc::now().to_rfc3339()
                            }
                        });

                        debug!("A2A response generated via SDK with tool calls");
                        Ok(Json(a2a_response))
                    }
                    Err(e) => {
                        error!("Follow-up request failed: {}", e);

                        let error_response = serde_json::json!({
                            "jsonrpc": "2.0",
                            "id": payload.get("id"),
                            "error": {
                                "code": -32603,
                                "message": "Internal error",
                                "data": format!("Follow-up request failed: {}", e)
                            }
                        });

                        Ok(Json(error_response))
                    }
                }
            } else {
                debug!("No tool calls in response, returning direct content");

                let content = choice.message.content.clone();

                let a2a_response = serde_json::json!({
                    "jsonrpc": "2.0",
                    "id": payload.get("id"),
                    "result": {
                        "status": "completed",
                        "message": {
                            "role": "assistant",
                            "parts": [{
                                "kind": "text",
                                "content": content
                            }]
                        },
                        "timestamp": chrono::Utc::now().to_rfc3339()
                    }
                });

                debug!("A2A response generated via SDK");
                Ok(Json(a2a_response))
            }
        }
        Err(e) => {
            error!("Failed to generate content via SDK: {}", e);

            let error_response = serde_json::json!({
                "jsonrpc": "2.0",
                "id": payload.get("id"),
                "error": {
                    "code": -32603,
                    "message": "Internal error",
                    "data": format!("SDK error: {}", e)
                }
            });

            Ok(Json(error_response))
        }
    }
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
}

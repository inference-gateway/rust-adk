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
    InferenceGatewayAPI, InferenceGatewayClient, Message, MessageRole, Provider,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{debug, error, info};

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

#[derive(Debug, Clone)]
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
}

pub struct A2AServerBuilder {
    config: Option<Config>,
    agent_card: Option<AgentCard>,
    agent_card_path: Option<String>,
    agent: Option<Agent>,
    gateway_url: Option<String>,
}

pub struct AgentBuilder {
    config: Option<AgentConfig>,
    system_prompt: Option<String>,
    max_chat_completion: u32,
    max_conversation_history: u32,
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

    pub fn with_agent_card_from_file(mut self, path: impl Into<String>) -> Self {
        self.agent_card_path = Some(path.into());
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

        let agent_card = if let Some(path) = self.agent_card_path {
            match tokio::fs::read_to_string(&path).await {
                Ok(content) => match serde_json::from_str::<AgentCard>(&content) {
                    Ok(card) => {
                        info!("Loaded agent card from: {}", path);
                        Some(card)
                    }
                    Err(e) => {
                        error!("Failed to parse agent card from {}: {}", path, e);
                        None
                    }
                },
                Err(e) => {
                    debug!("Could not load agent card from {}: {}", path, e);
                    None
                }
            }
        } else {
            self.agent_card
        };

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
        })
    }
}

impl Default for AgentBuilder {
    fn default() -> Self {
        Self::new()
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

    let default_card = serde_json::from_str::<AgentCard>(
        r#"{
        "name": "A2A Server with Inference Gateway SDK",
        "description": "A2A compatible server built with Rust ADK and Inference Gateway SDK",
        "version": "0.1.0",
        "capabilities": {
            "streaming": true,
            "push_notifications": false,
            "state_transition_history": false
        },
        "interface": {
            "protocol": "a2a",
            "version": "1.0"
        }
    }"#,
    )
    .map_err(|e| {
        error!("Failed to create default agent card: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    debug!("Returning default agent card");
    Ok(Json(default_card))
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

    let (provider, model) = match &state.server.agent {
        Some(agent) => (agent.provider, agent.model.clone()),
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

    match gateway_client
        .generate_content(provider, &model, final_messages)
        .await
    {
        Ok(response) => {
            let content = response
                .choices
                .first()
                .map(|c| c.message.content.clone())
                .unwrap_or_else(|| "No response generated".to_string());

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
                    let server = A2AServerBuilder::new().build().await;
                    assert!(server.is_ok(), "Default builder should succeed");
                }
                "with_config" => {
                    let config = Config::default();
                    let server = A2AServerBuilder::new().with_config(config).build().await;
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
                "empty_model_error" => {
                    let mut config = AgentConfig::default();
                    config.model = "".to_string();
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
                    let mut config = AgentConfig::default();
                    config.provider = "invalid_provider".to_string();
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

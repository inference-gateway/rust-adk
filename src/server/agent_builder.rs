use super::agent::Agent;
use super::agent_llm_client::{LLMClient, OpenAICompatibleLLMClient};
use super::agent_toolbox::{AsyncFunctionToolHandler, FunctionToolHandler, ToolHandler};
use super::mcp::McpClient;
use crate::config::AgentConfig;
use anyhow::Result;
use inference_gateway_sdk::ChatCompletionTool;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

/// Builder for [`Agent`]. Accepts either an entire [`AgentConfig`] via
/// [`with_config`](AgentBuilder::with_config) or per-field setters
/// (`with_provider`, `with_model`, ...). Setters layered on top of a
/// previously-set config override that field only.
pub struct AgentBuilder {
    config: Option<AgentConfig>,
    provider: Option<String>,
    model: Option<String>,
    api_key: Option<String>,
    base_url: Option<String>,
    timeout: Option<Duration>,
    max_retries: Option<u32>,
    max_chat_completion_iterations: Option<u32>,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
    system_prompt: Option<String>,
    enable_usage_metadata: Option<bool>,
    max_chat_completion: u32,
    max_conversation_history: u32,
    toolbox: Option<Vec<ChatCompletionTool>>,
    tool_handlers: HashMap<String, Box<dyn ToolHandler>>,
    llm_client: Option<Arc<dyn LLMClient>>,
}

impl AgentBuilder {
    pub fn new() -> Self {
        Self {
            config: None,
            provider: None,
            model: None,
            api_key: None,
            base_url: None,
            timeout: None,
            max_retries: None,
            max_chat_completion_iterations: None,
            max_tokens: None,
            temperature: None,
            system_prompt: None,
            enable_usage_metadata: None,
            max_chat_completion: 10,
            max_conversation_history: 20,
            toolbox: None,
            tool_handlers: HashMap::new(),
            llm_client: None,
        }
    }

    /// Seed the builder with an entire [`AgentConfig`]. Later per-field
    /// setters take precedence over fields set via this method.
    pub fn with_config(mut self, config: &AgentConfig) -> Self {
        self.config = Some(config.clone());
        self
    }

    pub fn with_provider(mut self, provider: impl Into<String>) -> Self {
        self.provider = Some(provider.into());
        self
    }

    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }

    pub fn with_api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Override the LLM gateway base URL used by the default
    /// [`OpenAICompatibleLLMClient`]. Ignored when a custom LLM client has
    /// been provided via [`with_llm_client`].
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = Some(max_retries);
        self
    }

    pub fn with_max_chat_completion_iterations(mut self, n: u32) -> Self {
        self.max_chat_completion_iterations = Some(n);
        self
    }

    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    pub fn with_system_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.system_prompt = Some(prompt.into());
        self
    }

    /// Override whether the built [`Agent`] prefers usage metadata to be
    /// attached to terminal tasks. Overrides the value from any
    /// [`AgentConfig`] supplied via [`with_config`](Self::with_config).
    pub fn with_enable_usage_metadata(mut self, enable: bool) -> Self {
        self.enable_usage_metadata = Some(enable);
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

    /// Plug in a custom [`LLMClient`] implementation. Overrides the default
    /// [`OpenAICompatibleLLMClient`] that `build()` would otherwise construct
    /// from [`AgentConfig`].
    pub fn with_llm_client<C: LLMClient + 'static>(mut self, client: C) -> Self {
        self.llm_client = Some(Arc::new(client));
        self
    }

    /// Register an [`McpClient`]'s two selector tools (`mcp_list_tools` and
    /// `mcp_call_tool`) on the agent: their definitions are appended to the
    /// toolbox and handlers wired to delegate to `client`. Regardless of how
    /// many tools the MCP servers expose, only these two enter the LLM context.
    ///
    /// Call [`McpClient::start`] to kick off background discovery (order
    /// relative to this call does not matter). Build the client with
    /// [`McpClient::from_config`], which returns `None` when MCP is disabled.
    pub fn with_mcp_client(mut self, client: Arc<McpClient>) -> Self {
        let mut toolbox = self.toolbox.take().unwrap_or_default();
        toolbox.extend(McpClient::selector_tools());
        self.toolbox = Some(toolbox);

        let list_client = Arc::clone(&client);
        let call_client = client;
        self.with_async_function_tool("mcp_list_tools".to_string(), move |args| {
            let c = Arc::clone(&list_client);
            async move { c.handle_list(args).await }
        })
        .with_async_function_tool("mcp_call_tool".to_string(), move |args| {
            let c = Arc::clone(&call_client);
            async move { c.handle_call(args).await }
        })
    }

    pub async fn build(self) -> Result<Agent> {
        let mut effective = self.config.clone().unwrap_or_default();
        if let Some(v) = self.provider.clone() {
            effective.provider = v;
        }
        if let Some(v) = self.model.clone() {
            effective.model = v;
        }
        if let Some(v) = self.api_key.clone() {
            effective.api_key = Some(v);
        }
        if let Some(v) = self.base_url.clone() {
            effective.base_url = Some(v);
        }
        if let Some(v) = self.timeout {
            effective.timeout_secs = v.as_secs();
        }
        if let Some(v) = self.max_retries {
            effective.max_retries = v;
        }
        if let Some(v) = self.max_chat_completion_iterations {
            effective.max_chat_completion_iterations = v;
        }
        if let Some(v) = self.max_tokens {
            effective.max_tokens = v;
        }
        if let Some(v) = self.temperature {
            effective.temperature = v;
        }
        if let Some(v) = self.system_prompt.clone() {
            effective.system_prompt = Some(v);
        }
        if let Some(v) = self.enable_usage_metadata {
            effective.enable_usage_metadata = v;
        }

        let llm_client: Arc<dyn LLMClient> = match self.llm_client {
            Some(client) => client,
            None => Arc::new(OpenAICompatibleLLMClient::new(&effective)?),
        };

        Ok(Agent {
            system_prompt: effective.system_prompt.clone(),
            llm_client,
            max_chat_completion: self.max_chat_completion,
            max_conversation_history: self.max_conversation_history,
            toolbox: self.toolbox,
            tool_handlers: self.tool_handlers,
            enable_usage_metadata: effective.enable_usage_metadata,
        })
    }
}

impl Default for AgentBuilder {
    fn default() -> Self {
        Self::new()
    }
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

        let valid_config = || AgentConfig {
            provider: "openai".to_string(),
            model: "gpt-4".to_string(),
            ..Default::default()
        };

        for test_case in test_cases {
            match test_case.name {
                "default_agent" => {
                    let agent = AgentBuilder::new().build().await;
                    assert!(
                        agent.is_err(),
                        "Default agent builder must fail when provider/model are unset"
                    );
                    assert!(
                        agent
                            .unwrap_err()
                            .to_string()
                            .contains("provider is required"),
                    );
                }
                "with_system_prompt" => {
                    let agent = AgentBuilder::new()
                        .with_config(&valid_config())
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
                        .with_config(&valid_config())
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
                        .with_config(&valid_config())
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
                        provider: "openai".to_string(),
                        model: String::new(),
                        ..Default::default()
                    };
                    let agent = AgentBuilder::new().with_config(&config).build().await;
                    assert!(agent.is_err(), "Agent builder should fail with empty model");
                    assert!(agent.unwrap_err().to_string().contains("model is required"));
                }
                "invalid_provider_error" => {
                    let config = AgentConfig {
                        provider: "invalid_provider".to_string(),
                        model: "gpt-4".to_string(),
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
    async fn enable_usage_metadata_round_trips_from_config_and_override() {
        let base = || AgentConfig {
            provider: "openai".to_string(),
            model: "gpt-4".to_string(),
            ..Default::default()
        };

        let agent = AgentBuilder::new()
            .with_config(&base())
            .build()
            .await
            .expect("agent builds");
        assert!(
            agent.usage_metadata_enabled(),
            "AgentConfig defaults usage metadata on"
        );

        let disabled_cfg = AgentConfig {
            enable_usage_metadata: false,
            ..base()
        };
        let agent = AgentBuilder::new()
            .with_config(&disabled_cfg)
            .build()
            .await
            .expect("agent builds");
        assert!(
            !agent.usage_metadata_enabled(),
            "config flag should disable usage metadata"
        );

        let agent = AgentBuilder::new()
            .with_config(&disabled_cfg)
            .with_enable_usage_metadata(true)
            .build()
            .await
            .expect("agent builds");
        assert!(
            agent.usage_metadata_enabled(),
            "with_enable_usage_metadata should override the config flag"
        );
    }
}

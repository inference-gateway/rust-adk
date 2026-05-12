use super::agent::Agent;
use super::agent_llm_client::{LLMClient, OpenAICompatibleLLMClient};
use super::agent_toolbox::{AsyncFunctionToolHandler, FunctionToolHandler, ToolHandler};
use crate::config::AgentConfig;
use anyhow::Result;
use inference_gateway_sdk::ChatCompletionTool;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

pub struct AgentBuilder {
    config: Option<AgentConfig>,
    system_prompt: Option<String>,
    max_chat_completion: u32,
    max_conversation_history: u32,
    toolbox: Option<Vec<ChatCompletionTool>>,
    tool_handlers: HashMap<String, Box<dyn ToolHandler>>,
    llm_client: Option<Arc<dyn LLMClient>>,
    base_url: Option<String>,
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
            llm_client: None,
            base_url: None,
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

    /// Plug in a custom [`LLMClient`] implementation. Overrides the default
    /// [`OpenAICompatibleLLMClient`] that `build()` would otherwise construct
    /// from [`AgentConfig`].
    pub fn with_llm_client<C: LLMClient + 'static>(mut self, client: C) -> Self {
        self.llm_client = Some(Arc::new(client));
        self
    }

    /// Override the LLM gateway base URL used by the default
    /// [`OpenAICompatibleLLMClient`]. Equivalent to setting
    /// `AgentConfig::base_url`. Ignored when a custom LLM client has been
    /// provided via [`with_llm_client`].
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }

    pub async fn build(self) -> Result<Agent> {
        let config = self.config.unwrap_or_default();

        let llm_client: Arc<dyn LLMClient> = match self.llm_client {
            Some(client) => client,
            None => {
                let mut effective = config.clone();
                if let Some(url) = self.base_url.clone() {
                    effective.base_url = Some(url);
                }
                Arc::new(OpenAICompatibleLLMClient::new(&effective)?)
            }
        };

        Ok(Agent {
            system_prompt: self.system_prompt.or(config.system_prompt.clone()),
            llm_client,
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
                    assert!(agent.unwrap_err().to_string().contains("model is required"));
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

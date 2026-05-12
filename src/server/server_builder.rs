use super::agent::Agent;
use super::agent_card::AgentCardOverrides;
use super::server_core::A2AServer;
use super::task_handler::{
    DefaultBackgroundTaskHandler, DefaultStreamingTaskHandler, StreamableTaskHandler, TaskHandler,
};
use crate::a2a_types::AgentCard;
use crate::config::Config;
use crate::storage::InMemoryStorage;
use anyhow::{Result, anyhow};
use std::sync::Arc;
use tracing::info;

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
                DefaultBackgroundTaskHandler::new(self.agent.clone()),
            )
                as Arc<dyn TaskHandler>),
            None => None,
        };
        let streaming_task_handler = match self.streaming_task_handler {
            Some(h) => Some(h),
            None if self.use_default_streaming_task_handler => Some(Arc::new(
                DefaultStreamingTaskHandler::new(self.agent.clone()),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestCase {
        name: &'static str,
        #[allow(dead_code)]
        description: &'static str,
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
}

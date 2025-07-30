use crate::a2a_types::AgentCard;
use crate::config::ClientConfig;
use anyhow::{Result, anyhow};
use inference_gateway_sdk::{
    CreateChatCompletionResponse, InferenceGatewayAPI, InferenceGatewayClient, Message,
    MessageRole, Provider,
};
use serde::{Deserialize, Serialize};
use tracing::debug;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub status: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub details: Option<serde_json::Value>,
}

#[derive(Debug)]
pub struct A2AClient {
    gateway_client: InferenceGatewayClient,
    base_url: String,
    config: ClientConfig,
}

impl A2AClient {
    pub fn new(base_url: impl Into<String>) -> Result<Self> {
        let base_url = base_url.into();
        let config = ClientConfig::new(base_url.clone());

        // Create inference gateway client
        let gateway_client = InferenceGatewayClient::new(&base_url);

        Ok(Self {
            gateway_client,
            base_url,
            config,
        })
    }

    pub fn with_config(config: ClientConfig) -> Result<Self> {
        let gateway_client = InferenceGatewayClient::new(&config.base_url);

        Ok(Self {
            gateway_client,
            base_url: config.base_url.clone(),
            config,
        })
    }

    pub async fn get_health(&self) -> Result<HealthStatus> {
        debug!("Making health check request via SDK");

        let is_healthy = self
            .gateway_client
            .health_check()
            .await
            .map_err(|e| anyhow!("Health check request failed: {}", e))?;

        let status = if is_healthy { "healthy" } else { "unhealthy" };

        let health_status = HealthStatus {
            status: status.to_string(),
            timestamp: chrono::Utc::now(),
            details: Some(serde_json::json!({
                "gateway_available": is_healthy,
                "sdk_version": "0.11.0"
            })),
        };

        debug!("Health check response: {:?}", health_status);
        Ok(health_status)
    }

    pub async fn get_agent_card(&self) -> Result<AgentCard> {
        debug!("Agent card request - returning default A2A agent card");

        // Since the SDK doesn't have agent card functionality, return a default
        let agent_card = serde_json::from_str::<AgentCard>(
            r#"{
            "name": "A2A Agent",
            "description": "A2A compatible agent using Inference Gateway SDK",
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
        .map_err(|e| anyhow!("Failed to create default agent card: {}", e))?;

        debug!("Agent card response created");
        Ok(agent_card)
    }

    pub async fn send_task(&self, params: serde_json::Value) -> Result<serde_json::Value> {
        debug!("Making task request via SDK");

        // Extract messages from params or create default
        let messages = if let Some(messages_val) = params.get("messages") {
            serde_json::from_value(messages_val.clone()).unwrap_or_else(|_| {
                vec![Message {
                    role: MessageRole::User,
                    content: params.to_string(),
                    ..Default::default()
                }]
            })
        } else {
            vec![Message {
                role: MessageRole::User,
                content: params.to_string(),
                ..Default::default()
            }]
        };

        // Use default provider and model for now
        let provider = Provider::Groq;
        let model = "deepseek-r1-distill-llama-70b";

        let response: CreateChatCompletionResponse = self
            .gateway_client
            .generate_content(provider, model, messages)
            .await
            .map_err(|e| anyhow!("Task request failed: {}", e))?;

        // Convert response to A2A format
        let result = serde_json::json!({
            "jsonrpc": "2.0",
            "id": params.get("id"),
            "result": {
                "status": "completed",
                "message": {
                    "role": "assistant",
                    "parts": [{
                        "kind": "text",
                        "content": response.choices.get(0)
                            .map(|c| c.message.content.clone())
                            .unwrap_or_else(|| "No response generated".to_string())
                    }]
                },
                "timestamp": chrono::Utc::now().to_rfc3339()
            }
        });

        debug!("Task response received via SDK");
        Ok(result)
    }

    pub async fn send_task_streaming<F>(
        &self,
        params: serde_json::Value,
        mut event_handler: F,
    ) -> Result<()>
    where
        F: FnMut(serde_json::Value) -> Result<()>,
    {
        debug!("Making streaming task request via SDK");

        // For now, use non-streaming and call handler once
        // In a full implementation, we would use generate_content_stream
        let result = self.send_task(params).await?;
        event_handler(result)?;

        debug!("Streaming task completed");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = A2AClient::new("http://localhost:8080").unwrap();
        assert_eq!(client.base_url, "http://localhost:8080");
    }

    #[test]
    fn test_config_creation() {
        let config = ClientConfig::new("http://example.com");
        assert_eq!(config.base_url, "http://example.com");
        assert_eq!(config.timeout, std::time::Duration::from_secs(30));
        assert_eq!(config.max_retries, 3);
    }
}

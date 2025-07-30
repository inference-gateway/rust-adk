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
    http_client: reqwest::Client,
    #[allow(dead_code)]
    base_url: String,
    #[allow(dead_code)]
    config: ClientConfig,
}

impl A2AClient {
    pub fn new(base_url: impl Into<String>) -> Result<Self> {
        let base_url = base_url.into();
        let config = ClientConfig::new(base_url.clone());

        let gateway_client = InferenceGatewayClient::new(&base_url);
        let http_client = reqwest::Client::new();

        Ok(Self {
            gateway_client,
            http_client,
            base_url,
            config,
        })
    }

    pub fn with_config(config: ClientConfig) -> Result<Self> {
        let gateway_client = InferenceGatewayClient::new(&config.base_url);
        let http_client = reqwest::Client::new();

        Ok(Self {
            gateway_client,
            http_client,
            base_url: config.base_url.clone(),
            config,
        })
    }

    pub async fn get_health(&self) -> Result<HealthStatus> {
        debug!("Making health check request to A2A server");

        let url = format!("{}/health", self.base_url);
        let response = self
            .http_client
            .get(&url)
            .send()
            .await
            .map_err(|e| anyhow!("Health check request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(anyhow!("Health check failed: HTTP {}", response.status()));
        }

        let health_status = response
            .json::<HealthStatus>()
            .await
            .map_err(|e| anyhow!("Failed to parse health response: {}", e))?;

        debug!("Health check response: {:?}", health_status);
        Ok(health_status)
    }

    pub async fn get_agent_card(&self) -> Result<AgentCard> {
        debug!("Fetching agent card from server");

        let url = format!("{}/.well-known/agent.json", self.base_url);
        let response = self
            .http_client
            .get(&url)
            .send()
            .await
            .map_err(|e| anyhow!("Failed to fetch agent card: {}", e))?;

        if !response.status().is_success() {
            return Err(anyhow!(
                "Failed to fetch agent card: HTTP {}",
                response.status()
            ));
        }

        let agent_card = response
            .json::<AgentCard>()
            .await
            .map_err(|e| anyhow!("Failed to parse agent card response: {}", e))?;

        debug!("Agent card retrieved successfully");
        Ok(agent_card)
    }

    pub async fn send_task(&self, params: serde_json::Value) -> Result<serde_json::Value> {
        debug!("Making task request via SDK");

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

        let provider = Provider::Groq;
        let model = "deepseek-r1-distill-llama-70b";

        let response: CreateChatCompletionResponse = self
            .gateway_client
            .generate_content(provider, model, messages)
            .await
            .map_err(|e| anyhow!("Task request failed: {}", e))?;

        let result = serde_json::json!({
            "jsonrpc": "2.0",
            "id": params.get("id"),
            "result": {
                "status": "completed",
                "message": {
                    "role": "assistant",
                    "parts": [{
                        "kind": "text",
                        "content": response.choices.first()
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

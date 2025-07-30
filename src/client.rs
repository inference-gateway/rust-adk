use crate::a2a_types::{AgentCard, JsonrpcMessage, JsonrpcMessageId};
use crate::config::ClientConfig;
use anyhow::{anyhow, Result};
use reqwest;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::debug;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub status: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub details: Option<serde_json::Value>,
}

#[derive(Debug, Clone)]
pub struct A2AClient {
    client: reqwest::Client,
    base_url: String,
    config: ClientConfig,
}

impl A2AClient {
    pub fn new(base_url: impl Into<String>) -> Result<Self> {
        let base_url = base_url.into();
        let config = ClientConfig::new(base_url.clone());
        
        let client = reqwest::Client::builder()
            .timeout(config.timeout)
            .build()
            .map_err(|e| anyhow!("Failed to create HTTP client: {}", e))?;

        Ok(Self {
            client,
            base_url,
            config,
        })
    }

    pub fn with_config(config: ClientConfig) -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(config.timeout)
            .build()
            .map_err(|e| anyhow!("Failed to create HTTP client: {}", e))?;

        Ok(Self {
            client,
            base_url: config.base_url.clone(),
            config,
        })
    }

    pub async fn get_health(&self) -> Result<HealthStatus> {
        let url = format!("{}/health", self.base_url);
        debug!("Making health check request to: {}", url);

        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| anyhow!("Health check request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(anyhow!("Health check failed with status: {}", response.status()));
        }

        let health_status: HealthStatus = response
            .json()
            .await
            .map_err(|e| anyhow!("Failed to parse health response: {}", e))?;

        debug!("Health check response: {:?}", health_status);
        Ok(health_status)
    }

    pub async fn get_agent_card(&self) -> Result<AgentCard> {
        let url = format!("{}/.well-known/agent.json", self.base_url);
        debug!("Making agent card request to: {}", url);

        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| anyhow!("Agent card request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(anyhow!("Agent card request failed with status: {}", response.status()));
        }

        let agent_card: AgentCard = response
            .json()
            .await
            .map_err(|e| anyhow!("Failed to parse agent card response: {}", e))?;

        debug!("Agent card response received");
        Ok(agent_card)
    }

    pub async fn send_task(&self, _params: serde_json::Value) -> Result<serde_json::Value> {
        let url = format!("{}/a2a", self.base_url);
        debug!("Making task request to: {}", url);

        // Create JSON-RPC request
        let request = JsonrpcMessage {
            jsonrpc: "2.0".to_string(),
            id: Some(JsonrpcMessageId::String(uuid::Uuid::new_v4().to_string())),
        };

        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| anyhow!("Task request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(anyhow!("Task request failed with status: {}", response.status()));
        }

        let result: serde_json::Value = response
            .json()
            .await
            .map_err(|e| anyhow!("Failed to parse task response: {}", e))?;

        debug!("Task response received");
        Ok(result)
    }

    pub async fn send_task_streaming<F>(&self, _params: serde_json::Value, mut event_handler: F) -> Result<()>
    where
        F: FnMut(serde_json::Value) -> Result<()>,
    {
        let url = format!("{}/a2a", self.base_url);
        debug!("Making streaming task request to: {}", url);

        // Create JSON-RPC request
        let request = JsonrpcMessage {
            jsonrpc: "2.0".to_string(),
            id: Some(JsonrpcMessageId::String(uuid::Uuid::new_v4().to_string())),
        };

        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| anyhow!("Streaming task request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(anyhow!("Streaming task request failed with status: {}", response.status()));
        }

        // For now, treat as non-streaming and call the handler once
        let result: serde_json::Value = response
            .json()
            .await
            .map_err(|e| anyhow!("Failed to parse streaming task response: {}", e))?;

        event_handler(result)?;

        debug!("Streaming task completed");
        Ok(())
    }

    async fn retry_request<F, Fut, T>(&self, operation: F) -> Result<T>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<T>>,
    {
        let mut last_error = None;
        
        for attempt in 0..=self.config.max_retries {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    last_error = Some(e);
                    if attempt < self.config.max_retries {
                        let delay = Duration::from_millis(100 * (2_u64.pow(attempt)));
                        tokio::time::sleep(delay).await;
                        debug!("Retrying request, attempt {} of {}", attempt + 1, self.config.max_retries);
                    }
                }
            }
        }

        Err(last_error.unwrap_or_else(|| anyhow!("All retry attempts failed")))
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
        assert_eq!(config.timeout, Duration::from_secs(30));
        assert_eq!(config.max_retries, 3);
    }
}
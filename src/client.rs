use crate::a2a_types::{
    AgentCard, DeleteTaskPushNotificationConfigParams, GetTaskPushNotificationConfigParams,
    ListTaskPushNotificationConfigParams, MessageSendParams, TaskIdParams,
    TaskPushNotificationConfig, TaskQueryParams,
};
use crate::config::ClientConfig;
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tracing::debug;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub status: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub details: Option<serde_json::Value>,
}

/// A2A JSON-RPC client.
///
/// Builds JSON-RPC envelopes against the canonical A2A spec methods. Each
/// snake_case method on this struct corresponds to one method string in the
/// spec (`message/send`, `tasks/get`, etc.) and parses the matching response
/// into either a strongly-typed value (where useful) or an opaque
/// [`serde_json::Value`] containing the JSON-RPC envelope.
#[derive(Debug)]
pub struct A2AClient {
    http_client: reqwest::Client,
    base_url: String,
    #[allow(dead_code)]
    config: ClientConfig,
}

impl A2AClient {
    pub fn new(base_url: impl Into<String>) -> Result<Self> {
        let base_url = base_url.into();
        let config = ClientConfig::new(base_url.clone());
        let http_client = reqwest::Client::new();

        Ok(Self {
            http_client,
            base_url,
            config,
        })
    }

    pub fn with_config(config: ClientConfig) -> Result<Self> {
        let http_client = reqwest::Client::new();

        Ok(Self {
            http_client,
            base_url: config.base_url.clone(),
            config,
        })
    }

    /// The endpoint base URL this client targets.
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// `GET /health`
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

    /// `GET /.well-known/agent.json`
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

    /// `message/send`
    pub async fn send_message(&self, params: MessageSendParams) -> Result<Value> {
        self.jsonrpc_call("message/send", serde_json::to_value(&params)?)
            .await
    }

    /// `message/stream` — opens an SSE connection and yields each event
    /// to `event_handler` as a parsed JSON-RPC envelope.
    pub async fn send_streaming_message<F>(
        &self,
        params: MessageSendParams,
        event_handler: F,
    ) -> Result<()>
    where
        F: FnMut(Value) -> Result<()> + Send,
    {
        let body = json!({
            "jsonrpc": "2.0",
            "id": Uuid::new_v4().to_string(),
            "method": "message/stream",
            "params": serde_json::to_value(&params)?,
        });
        self.consume_sse(body, event_handler).await
    }

    /// `tasks/get`
    pub async fn get_task(&self, params: TaskQueryParams) -> Result<Value> {
        self.jsonrpc_call("tasks/get", serde_json::to_value(&params)?)
            .await
    }

    /// `tasks/cancel`
    pub async fn cancel_task(&self, params: TaskIdParams) -> Result<Value> {
        self.jsonrpc_call("tasks/cancel", serde_json::to_value(&params)?)
            .await
    }

    /// `tasks/pushNotificationConfig/set`
    pub async fn set_task_push_notification_config(
        &self,
        params: TaskPushNotificationConfig,
    ) -> Result<Value> {
        self.jsonrpc_call(
            "tasks/pushNotificationConfig/set",
            serde_json::to_value(&params)?,
        )
        .await
    }

    /// `tasks/pushNotificationConfig/get`
    pub async fn get_task_push_notification_config(
        &self,
        params: GetTaskPushNotificationConfigParams,
    ) -> Result<Value> {
        self.jsonrpc_call(
            "tasks/pushNotificationConfig/get",
            serde_json::to_value(&params)?,
        )
        .await
    }

    /// `tasks/pushNotificationConfig/list`
    pub async fn list_task_push_notification_configs(
        &self,
        params: ListTaskPushNotificationConfigParams,
    ) -> Result<Value> {
        self.jsonrpc_call(
            "tasks/pushNotificationConfig/list",
            serde_json::to_value(&params)?,
        )
        .await
    }

    /// `tasks/pushNotificationConfig/delete`
    pub async fn delete_task_push_notification_config(
        &self,
        params: DeleteTaskPushNotificationConfigParams,
    ) -> Result<Value> {
        self.jsonrpc_call(
            "tasks/pushNotificationConfig/delete",
            serde_json::to_value(&params)?,
        )
        .await
    }

    /// `tasks/resubscribe` — opens an SSE connection on an existing task
    /// and yields each event to `event_handler`.
    pub async fn resubscribe_task<F>(
        &self,
        params: TaskIdParams,
        event_handler: F,
    ) -> Result<()>
    where
        F: FnMut(Value) -> Result<()> + Send,
    {
        let body = json!({
            "jsonrpc": "2.0",
            "id": Uuid::new_v4().to_string(),
            "method": "tasks/resubscribe",
            "params": serde_json::to_value(&params)?,
        });
        self.consume_sse(body, event_handler).await
    }

    /// Backwards-compatible alias for [`A2AClient::send_message`] that accepts a raw
    /// `serde_json::Value` payload. Newer code should prefer the typed entry points.
    pub async fn send_task(&self, params: Value) -> Result<Value> {
        self.jsonrpc_call("message/send", params).await
    }

    /// Backwards-compatible alias for [`A2AClient::send_streaming_message`] that
    /// accepts a raw `serde_json::Value` payload. Newer code should prefer the
    /// typed entry points.
    pub async fn send_task_streaming<F>(
        &self,
        params: Value,
        event_handler: F,
    ) -> Result<()>
    where
        F: FnMut(Value) -> Result<()> + Send,
    {
        let body = json!({
            "jsonrpc": "2.0",
            "id": Uuid::new_v4().to_string(),
            "method": "message/stream",
            "params": params,
        });
        self.consume_sse(body, event_handler).await
    }

    async fn jsonrpc_call(&self, method: &str, params: Value) -> Result<Value> {
        let body = json!({
            "jsonrpc": "2.0",
            "id": Uuid::new_v4().to_string(),
            "method": method,
            "params": params,
        });

        debug!("→ {} {}", method, body);

        let url = format!("{}/a2a", self.base_url);
        let response = self
            .http_client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| anyhow!("JSON-RPC call ({method}) failed: {}", e))?;

        if !response.status().is_success() {
            return Err(anyhow!(
                "JSON-RPC call ({method}) failed: HTTP {}",
                response.status()
            ));
        }

        let value: Value = response
            .json()
            .await
            .map_err(|e| anyhow!("Failed to parse JSON-RPC response for {method}: {}", e))?;

        debug!("← {} {}", method, value);
        Ok(value)
    }

    async fn consume_sse<F>(&self, body: Value, mut event_handler: F) -> Result<()>
    where
        F: FnMut(Value) -> Result<()> + Send,
    {
        let url = format!("{}/a2a", self.base_url);
        let response = self
            .http_client
            .post(&url)
            .header("Accept", "text/event-stream")
            .json(&body)
            .send()
            .await
            .map_err(|e| anyhow!("Streaming JSON-RPC call failed: {}", e))?;

        let status = response.status();
        let content_type = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string();
        let body_text = response
            .text()
            .await
            .map_err(|e| anyhow!("Failed to read streaming body: {}", e))?;

        if !status.is_success() {
            return Err(anyhow!(
                "Streaming JSON-RPC call failed: HTTP {} body={}",
                status,
                body_text
            ));
        }

        // The server may legitimately respond with a JSON-RPC error envelope
        // (Content-Type: application/json) when the request can be rejected
        // before any SSE frame is emitted — e.g. invalid params or task not
        // found on tasks/resubscribe. Pass that envelope to the handler as a
        // single "event" so callers can react uniformly.
        if !content_type.contains("text/event-stream") {
            let value: Value = serde_json::from_str(&body_text).map_err(|e| {
                anyhow!("Streaming response was neither SSE nor JSON: {e} ({body_text})")
            })?;
            return event_handler(value);
        }

        // SSE frames are separated by blank lines. Each frame can have multiple
        // `data:` lines; we concatenate them and parse the joined payload as JSON.
        for frame in body_text.split("\n\n") {
            let mut payload = String::new();
            for line in frame.lines() {
                if let Some(rest) = line.strip_prefix("data:") {
                    if !payload.is_empty() {
                        payload.push('\n');
                    }
                    payload.push_str(rest.trim_start());
                }
            }
            if payload.is_empty() {
                continue;
            }
            let value: Value = serde_json::from_str(&payload)
                .map_err(|e| anyhow!("Failed to parse SSE event payload: {} ({payload})", e))?;
            event_handler(value)?;
        }

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

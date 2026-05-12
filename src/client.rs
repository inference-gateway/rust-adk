use crate::a2a_types::{
    AgentCard, CancelTaskRequest, DeleteTaskPushNotificationConfigRequest,
    GetTaskPushNotificationConfigRequest, GetTaskRequest, ListTaskPushNotificationConfigRequest,
    ListTaskPushNotificationConfigResponse, ListTasksRequest, ListTasksResponse,
    SendMessageRequest, SendMessageResponse, SetTaskPushNotificationConfigRequest, StreamResponse,
    Task, TaskPushNotificationConfig, TaskState,
};
use crate::config::ClientConfig;
use anyhow::{Result, anyhow};
use eventsource_stream::Eventsource;
use futures::stream::{Stream, StreamExt};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::Value;
use tracing::debug;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub status: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub details: Option<serde_json::Value>,
}

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

    /// POST a raw JSON-RPC envelope to `/a2a` and return the response body.
    ///
    /// Most callers should prefer the typed helpers below (e.g.
    /// `send_message`, `get_task`); this entry point exists primarily for
    /// custom payloads and for examples.
    pub async fn send_task(&self, params: serde_json::Value) -> Result<serde_json::Value> {
        debug!("Posting JSON-RPC envelope to A2A server");
        self.post_raw(params).await
    }

    /// Send a request and stream the response back through `event_handler`.
    ///
    /// Real SSE streaming is tracked in a follow-up ticket; for now this
    /// helper performs a non-streaming `send_task` and invokes the handler
    /// once with the full response payload.
    pub async fn send_task_streaming<F>(
        &self,
        params: serde_json::Value,
        mut event_handler: F,
    ) -> Result<()>
    where
        F: FnMut(serde_json::Value) -> Result<()>,
    {
        debug!("Making streaming task request via SDK");
        let result = self.send_task(params).await?;
        event_handler(result)?;
        debug!("Streaming task completed");
        Ok(())
    }

    // -------------------------------------------------------------------
    // Typed JSON-RPC methods (one per A2A specification method)
    // -------------------------------------------------------------------

    /// `message/send` — dispatch a message and return the resulting task /
    /// agent response.
    pub async fn send_message(&self, params: SendMessageRequest) -> Result<SendMessageResponse> {
        self.call_typed("message/send", params).await
    }

    /// `message/stream` — open an SSE stream and yield each
    /// [`StreamResponse`] event as it arrives.
    ///
    /// The first event typically carries the freshly-created `Task` in
    /// `Submitted`; subsequent events are `TaskStatusUpdateEvent` /
    /// `TaskArtifactUpdateEvent` deltas. The stream terminates after the
    /// server emits an event with `final: true` (or closes the connection).
    pub async fn stream_message(
        &self,
        params: SendMessageRequest,
    ) -> Result<impl Stream<Item = Result<StreamResponse>> + Send + 'static> {
        let envelope = serde_json::json!({
            "jsonrpc": "2.0",
            "id": Uuid::new_v4().to_string(),
            "method": "message/stream",
            "params": serde_json::to_value(params)
                .map_err(|e| anyhow!("failed to serialize stream params: {e}"))?,
        });

        let url = format!("{}/a2a", self.base_url);
        let response = self
            .http_client
            .post(&url)
            .header("Accept", "text/event-stream")
            .json(&envelope)
            .send()
            .await
            .map_err(|e| anyhow!("message/stream request failed: {e}"))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(anyhow!("message/stream failed: HTTP {status}: {body}"));
        }

        let event_stream = response.bytes_stream().eventsource();

        let stream = event_stream.filter_map(|event| async move {
            match event {
                Ok(event) => {
                    if event.data.is_empty() {
                        return None;
                    }
                    let parsed: Value = match serde_json::from_str(&event.data) {
                        Ok(v) => v,
                        Err(e) => {
                            return Some(Err(anyhow!(
                                "failed to parse SSE event as JSON: {e}: {data}",
                                data = event.data
                            )));
                        }
                    };
                    if let Some(err) = parsed.get("error") {
                        return Some(Err(anyhow!("JSON-RPC error in stream: {err}")));
                    }
                    let result = match parsed.get("result").cloned() {
                        Some(v) => v,
                        None => {
                            return Some(Err(anyhow!(
                                "SSE event missing `result`: {data}",
                                data = event.data
                            )));
                        }
                    };
                    match serde_json::from_value::<StreamResponse>(result) {
                        Ok(r) => Some(Ok(r)),
                        Err(e) => Some(Err(anyhow!("failed to decode StreamResponse: {e}"))),
                    }
                }
                Err(e) => Some(Err(anyhow!("SSE transport error: {e}"))),
            }
        });

        Ok(stream)
    }

    /// `message/stream` — drain the SSE stream and return a single
    /// [`SendMessageResponse`] assembled from the last task seen and the
    /// final agent message (if any). Kept for callers that prefer a
    /// `message/send`-shaped response; use [`A2AClient::stream_message`]
    /// when you want to observe state transitions as they happen.
    pub async fn send_streaming_message(
        &self,
        params: SendMessageRequest,
    ) -> Result<SendMessageResponse> {
        let mut stream = Box::pin(self.stream_message(params).await?);

        let mut latest_task: Option<Task> = None;
        let mut final_message: Option<crate::a2a_types::Message> = None;

        while let Some(event) = stream.next().await {
            let response = event?;
            if let Some(task) = response.task {
                latest_task = Some(task);
            }
            if let Some(update) = response.status_update.as_ref() {
                if let Some(ref mut task) = latest_task {
                    task.status = update.status.clone();
                }
                if matches!(
                    update.status.state,
                    TaskState::TaskStateCompleted
                        | TaskState::TaskStateFailed
                        | TaskState::TaskStateCancelled
                        | TaskState::TaskStateRejected
                ) && let Some(msg) = update.status.message.clone()
                {
                    final_message = Some(msg);
                }
            }
            if let Some(msg) = response.message {
                final_message = Some(msg);
            }
        }

        Ok(SendMessageResponse {
            message: final_message,
            task: latest_task,
        })
    }

    /// `tasks/get` — fetch a stored task by resource name (`tasks/{task_id}`).
    pub async fn get_task(&self, params: GetTaskRequest) -> Result<Task> {
        self.call_typed("tasks/get", params).await
    }

    /// `tasks/list` — page through stored tasks.
    pub async fn list_tasks(&self, params: ListTasksRequest) -> Result<ListTasksResponse> {
        self.call_typed("tasks/list", params).await
    }

    /// `tasks/cancel` — request cancellation of a stored task.
    pub async fn cancel_task(&self, params: CancelTaskRequest) -> Result<Task> {
        self.call_typed("tasks/cancel", params).await
    }

    /// `tasks/pushNotificationConfig/set` — create/replace a push
    /// notification configuration for a task.
    pub async fn set_task_push_notification_config(
        &self,
        params: SetTaskPushNotificationConfigRequest,
    ) -> Result<TaskPushNotificationConfig> {
        self.call_typed("tasks/pushNotificationConfig/set", params)
            .await
    }

    /// `tasks/pushNotificationConfig/get` — fetch a push notification
    /// configuration by resource name.
    pub async fn get_task_push_notification_config(
        &self,
        params: GetTaskPushNotificationConfigRequest,
    ) -> Result<TaskPushNotificationConfig> {
        self.call_typed("tasks/pushNotificationConfig/get", params)
            .await
    }

    /// `tasks/pushNotificationConfig/list` — list push notification configs
    /// belonging to a parent task.
    pub async fn list_task_push_notification_configs(
        &self,
        params: ListTaskPushNotificationConfigRequest,
    ) -> Result<ListTaskPushNotificationConfigResponse> {
        self.call_typed("tasks/pushNotificationConfig/list", params)
            .await
    }

    /// `tasks/pushNotificationConfig/delete` — remove a push notification
    /// configuration.
    pub async fn delete_task_push_notification_config(
        &self,
        params: DeleteTaskPushNotificationConfigRequest,
    ) -> Result<Value> {
        self.call_typed("tasks/pushNotificationConfig/delete", params)
            .await
    }

    // -------------------------------------------------------------------
    // Internals
    // -------------------------------------------------------------------

    async fn post_raw(&self, body: Value) -> Result<Value> {
        let url = format!("{}/a2a", self.base_url);
        let response = self
            .http_client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| anyhow!("Task request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(anyhow!("Task request failed: HTTP {}", response.status()));
        }

        let body = response
            .json::<Value>()
            .await
            .map_err(|e| anyhow!("Failed to parse task response: {}", e))?;

        Ok(body)
    }

    async fn call_typed<P, R>(&self, method: &str, params: P) -> Result<R>
    where
        P: Serialize,
        R: DeserializeOwned,
    {
        let params_value = serde_json::to_value(params)
            .map_err(|e| anyhow!("failed to serialize params for {method}: {e}"))?;

        let envelope = serde_json::json!({
            "jsonrpc": "2.0",
            "id": Uuid::new_v4().to_string(),
            "method": method,
            "params": params_value,
        });

        let response = self.post_raw(envelope).await?;

        if let Some(err) = response.get("error").cloned() {
            return Err(anyhow!("JSON-RPC error for {method}: {err}"));
        }

        let result = response
            .get("result")
            .cloned()
            .ok_or_else(|| anyhow!("missing `result` in JSON-RPC response for {method}"))?;

        serde_json::from_value(result)
            .map_err(|e| anyhow!("failed to deserialize result for {method}: {e}"))
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

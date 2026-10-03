use super::storage::Storage;
use crate::a2a_types::{StreamResponse, Task};
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, warn};

const DELIVERY_TIMEOUT: Duration = Duration::from_secs(10);

/// POST the task as a `StreamResponse` to every webhook registered for it
/// (A2A spec 7.2). Delivery is best-effort: failures are logged, not retried.
// ponytail: at-most-once, inline with the caller; add a retry queue once
// webhook flakiness actually matters.
pub(super) async fn notify(storage: &Arc<dyn Storage>, task: &Task) {
    let configs = storage.list_push_notification_configs(&task.id).await;
    if configs.is_empty() {
        return;
    }

    let payload = StreamResponse {
        task: Some(task.clone()),
        ..Default::default()
    };
    let client = reqwest::Client::new();

    for config in configs {
        let mut request = client
            .post(&config.url)
            .timeout(DELIVERY_TIMEOUT)
            .json(&payload);
        if let Some(auth) = config.authentication.as_ref() {
            request = request.header(
                reqwest::header::AUTHORIZATION,
                format!(
                    "{} {}",
                    auth.scheme,
                    auth.credentials.as_deref().unwrap_or_default()
                ),
            );
        }
        if let Some(token) = config.token.as_deref() {
            request = request.header("X-A2A-Notification-Token", token);
        }

        match request.send().await {
            Ok(response) => debug!(
                task_id = %task.id,
                url = %config.url,
                status = response.status().as_u16(),
                "push notification delivered",
            ),
            Err(e) => warn!(
                task_id = %task.id,
                url = %config.url,
                error = %e,
                "push notification delivery failed",
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::a2a_types::{AuthenticationInfo, TaskPushNotificationConfig, TaskState, TaskStatus};
    use crate::server::storage::InMemoryStorage;
    use axum::{Router, extract::State, http::HeaderMap, routing::post};
    use serde_json::Value;
    use tokio::sync::mpsc;

    fn task(id: &str) -> Task {
        Task {
            artifacts: vec![],
            context_id: Some("ctx".to_string()),
            history: vec![],
            id: id.to_string(),
            metadata: None,
            status: TaskStatus {
                message: None,
                state: TaskState::TaskStateCompleted,
                timestamp: None,
            },
        }
    }

    #[tokio::test]
    async fn delivers_stream_response_with_configured_auth() {
        let (tx, mut rx) = mpsc::channel::<(HeaderMap, Value)>(4);
        let app = Router::new()
            .route(
                "/hook",
                post(
                    |State(tx): State<mpsc::Sender<(HeaderMap, Value)>>,
                     headers: HeaderMap,
                     body: String| async move {
                        let parsed = serde_json::from_str(&body).expect("webhook body is JSON");
                        tx.send((headers, parsed)).await.expect("send");
                        "ok"
                    },
                ),
            )
            .with_state(tx);
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind");
        let url = format!("http://{}/hook", listener.local_addr().expect("addr"));
        tokio::spawn(async move {
            let _ = axum::serve(listener, app).await;
        });

        let storage: Arc<dyn Storage> = Arc::new(InMemoryStorage::new());
        storage
            .put_push_notification_config(TaskPushNotificationConfig {
                authentication: Some(AuthenticationInfo {
                    credentials: Some("secret".to_string()),
                    scheme: "Bearer".to_string(),
                }),
                id: Some("cfg-1".to_string()),
                task_id: Some("t1".to_string()),
                tenant: None,
                token: Some("notification-token".to_string()),
                url,
            })
            .await;

        notify(&storage, &task("t1")).await;

        let (headers, body) = tokio::time::timeout(Duration::from_secs(5), rx.recv())
            .await
            .expect("webhook called")
            .expect("payload");
        assert_eq!(
            headers.get("authorization").and_then(|v| v.to_str().ok()),
            Some("Bearer secret"),
        );
        assert_eq!(
            headers
                .get("x-a2a-notification-token")
                .and_then(|v| v.to_str().ok()),
            Some("notification-token"),
        );
        assert_eq!(body["task"]["id"], "t1");
    }

    #[tokio::test]
    async fn skips_tasks_without_configs() {
        let storage: Arc<dyn Storage> = Arc::new(InMemoryStorage::new());
        notify(&storage, &task("t2")).await;
    }
}

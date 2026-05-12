//! Integration tests covering the JSON-RPC dispatch surface required by the
//! A2A specification. Each test asserts the success path of one (or more)
//! methods end-to-end against a running `A2AServer`.

use inference_gateway_adk::{A2AClient, A2AServerBuilder, TaskHandler, a2a_types};
use serde_json::{Value, json};
use std::net::{SocketAddr, TcpListener as StdTcpListener};
use std::sync::OnceLock;
use std::time::Duration;
use tokio::time::timeout;

/// Background handler used by the integration test server. Leaves the task
/// in `Submitted` so the cancel/get tests have a non-terminal task to act on.
#[derive(Debug)]
struct SubmittedTaskHandler;

#[async_trait::async_trait]
impl TaskHandler for SubmittedTaskHandler {
    async fn handle_task(
        &self,
        task: a2a_types::Task,
        _message: Option<a2a_types::Message>,
    ) -> anyhow::Result<a2a_types::Task> {
        Ok(task)
    }
}

static SUITE: OnceLock<Suite> = OnceLock::new();

#[derive(Debug)]
struct Suite {
    base_url: String,
    server_addr: SocketAddr,
    timeout_duration: Duration,
}

fn allocate_port() -> u16 {
    let listener = StdTcpListener::bind("127.0.0.1:0").expect("bind ephemeral port");
    listener.local_addr().expect("local_addr available").port()
}

fn ensure_suite() -> &'static Suite {
    SUITE.get_or_init(|| {
        let port = allocate_port();
        let server_addr: SocketAddr = format!("127.0.0.1:{port}").parse().expect("addr parses");

        // Spawn the server on a dedicated OS thread with its own tokio
        // runtime so it outlives the per-test runtimes created by
        // `#[tokio::test]`.
        let (ready_tx, ready_rx) = std::sync::mpsc::channel();
        let server_addr_clone = server_addr;
        std::thread::Builder::new()
            .name("a2a-test-server".into())
            .spawn(move || {
                let rt = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .expect("server runtime builds");
                rt.block_on(async move {
                    let agent_card_json = json!({
                        "name": "Test A2A Agent",
                        "description": "A test agent for validating A2A server functionality",
                        "version": "1.0.0",
                        "protocolVersion": "0.2.6",
                        "url": format!("http://{server_addr_clone}/a2a"),
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
                                "id": "general-conversation",
                                "name": "General Conversation",
                                "description": "Can engage in general conversation and answer questions",
                                "tags": ["conversation", "qa"]
                            }
                        ],
                        "provider": {
                            "organization": "Test Organization",
                            "url": "https://example.com"
                        }
                    });

                    let agent_card: a2a_types::AgentCard =
                        serde_json::from_value(agent_card_json).expect("agent card parses");

                    let server = A2AServerBuilder::new()
                        .with_gateway_url("http://127.0.0.1:1/v1") // intentionally unreachable
                        .with_agent_card(agent_card)
                        .with_background_task_handler(SubmittedTaskHandler)
                        .with_default_streaming_task_handler()
                        .build()
                        .await
                        .expect("server builds");

                    let _ = ready_tx.send(());
                    if let Err(e) = server.serve(server_addr_clone).await {
                        eprintln!("test server stopped: {e}");
                    }
                });
            })
            .expect("spawn server thread");

        ready_rx
            .recv_timeout(Duration::from_secs(5))
            .expect("server thread became ready");

        // Probe TCP until the axum listener starts accepting connections.
        for _ in 0..100 {
            if std::net::TcpStream::connect_timeout(&server_addr, Duration::from_millis(50)).is_ok()
            {
                break;
            }
            std::thread::sleep(Duration::from_millis(50));
        }

        Suite {
            base_url: format!("http://{server_addr}"),
            server_addr,
            timeout_duration: Duration::from_secs(30),
        }
    })
}

async fn post_jsonrpc(suite: &Suite, request: Value) -> Value {
    let client = reqwest::Client::new();
    let url = format!("http://{}/a2a", suite.server_addr);
    let response = timeout(
        suite.timeout_duration,
        client.post(&url).json(&request).send(),
    )
    .await
    .expect("HTTP request did not time out")
    .expect("HTTP request succeeded");
    response.json::<Value>().await.expect("JSON response body")
}

fn message_payload(message_id: &str, text: &str) -> Value {
    json!({
        "messageId": message_id,
        "role": "ROLE_USER",
        "parts": [{ "text": text }],
    })
}

fn send_message_params(message_id: &str, text: &str) -> Value {
    json!({
        "tenant": "test",
        "message": message_payload(message_id, text),
    })
}

async fn create_task(suite: &Suite, text: &str) -> a2a_types::Task {
    let request = json!({
        "jsonrpc": "2.0",
        "id": format!("create-task-{}", uuid::Uuid::new_v4()),
        "method": "message/send",
        "params": send_message_params(&uuid::Uuid::new_v4().to_string(), text),
    });
    let response = post_jsonrpc(suite, request).await;
    let result = response
        .get("result")
        .cloned()
        .unwrap_or_else(|| panic!("expected result in message/send response: {response}"));
    let send_response: a2a_types::SendMessageResponse =
        serde_json::from_value(result).expect("SendMessageResponse parses");
    send_response.task.expect("server returned a task")
}

#[tokio::test]
async fn health_endpoint_reports_status() {
    let suite = ensure_suite();
    let client = A2AClient::new(&suite.base_url).expect("client builds");
    let health = client.get_health().await.expect("health request succeeds");
    assert!(!health.status.is_empty(), "status should not be empty");
}

#[tokio::test]
async fn agent_card_endpoint_returns_card() {
    let suite = ensure_suite();
    let client = A2AClient::new(&suite.base_url).expect("client builds");
    let card = client.get_agent_card().await.expect("agent card retrieved");
    assert_eq!(card.name, "Test A2A Agent");
    assert!(!card.description.is_empty());
}

#[tokio::test]
async fn message_send_returns_task() {
    let suite = ensure_suite();
    let request = json!({
        "jsonrpc": "2.0",
        "id": "test-message-send-001",
        "method": "message/send",
        "params": send_message_params("msg-001", "Hello via message/send"),
    });
    let response = post_jsonrpc(suite, request).await;
    assert!(
        response.get("error").is_none(),
        "expected success but got error: {response}"
    );
    let result = response
        .get("result")
        .unwrap_or_else(|| panic!("expected `result` in response: {response}"));
    let typed: a2a_types::SendMessageResponse =
        serde_json::from_value(result.clone()).expect("SendMessageResponse parses");
    assert!(typed.task.is_some(), "task should be present in result");
}

#[tokio::test]
async fn message_send_rejects_empty_message_id() {
    let suite = ensure_suite();
    let request = json!({
        "jsonrpc": "2.0",
        "id": "test-message-send-empty-id",
        "method": "message/send",
        "params": {
            "tenant": "test",
            "message": {
                "messageId": "",
                "role": "ROLE_USER",
                "parts": [{ "text": "missing id" }],
            },
        },
    });
    let response = post_jsonrpc(suite, request).await;
    let err = response
        .get("error")
        .unwrap_or_else(|| panic!("expected error, got: {response}"));
    assert_eq!(err.get("code").and_then(|v| v.as_i64()), Some(-32602));
    let data = err.get("data").and_then(|v| v.as_str()).unwrap_or_default();
    assert!(
        data.contains("messageId"),
        "error data should mention messageId, got: {data}"
    );
}

#[tokio::test]
async fn message_send_rejects_empty_parts() {
    let suite = ensure_suite();
    let request = json!({
        "jsonrpc": "2.0",
        "id": "test-message-send-empty-parts",
        "method": "message/send",
        "params": {
            "tenant": "test",
            "message": {
                "messageId": "msg-no-parts",
                "role": "ROLE_USER",
                "parts": [],
            },
        },
    });
    let response = post_jsonrpc(suite, request).await;
    let err = response
        .get("error")
        .unwrap_or_else(|| panic!("expected error, got: {response}"));
    assert_eq!(err.get("code").and_then(|v| v.as_i64()), Some(-32602));
    let data = err.get("data").and_then(|v| v.as_str()).unwrap_or_default();
    assert!(
        data.contains("parts"),
        "error data should mention parts, got: {data}"
    );
}

#[tokio::test]
async fn message_stream_emits_sse_state_transitions() {
    use futures::StreamExt;

    let suite = ensure_suite();
    let client = A2AClient::new(&suite.base_url).expect("client builds");

    let request = a2a_types::SendMessageRequest {
        configuration: None,
        message: Some(a2a_types::Message {
            context_id: None,
            extensions: vec![],
            message_id: "msg-stream-001".to_string(),
            metadata: None,
            parts: vec![a2a_types::Part {
                data: None,
                file: None,
                metadata: None,
                text: Some("Hello via message/stream".to_string()),
            }],
            reference_task_ids: vec![],
            role: a2a_types::Role::RoleUser,
            task_id: None,
        }),
        metadata: None,
        tenant: "test".to_string(),
    };

    let mut stream = Box::pin(client.stream_message(request).await.expect("stream opens"));

    let mut saw_task = false;
    let mut saw_working = false;
    let mut saw_completed_final = false;
    while let Some(event) = stream.next().await {
        let event = event.expect("event decodes");
        if event.task.is_some() {
            saw_task = true;
        }
        if let Some(update) = event.status_update {
            match update.status.state {
                a2a_types::TaskState::TaskStateWorking => saw_working = true,
                a2a_types::TaskState::TaskStateCompleted if update.final_ => {
                    saw_completed_final = true;
                }
                _ => {}
            }
        }
    }

    assert!(saw_task, "stream should carry the initial Task");
    assert!(saw_working, "stream should emit TaskStateWorking");
    assert!(
        saw_completed_final,
        "stream should terminate with TaskStateCompleted (final=true)"
    );
}

#[tokio::test]
async fn tasks_get_returns_stored_task() {
    let suite = ensure_suite();
    let task = create_task(suite, "tasks/get scenario").await;
    let request = json!({
        "jsonrpc": "2.0",
        "id": "test-tasks-get-001",
        "method": "tasks/get",
        "params": {
            "name": format!("tasks/{}", task.id),
        },
    });
    let response = post_jsonrpc(suite, request).await;
    assert!(
        response.get("error").is_none(),
        "expected success but got error: {response}"
    );
    let result = response.get("result").expect("result present");
    let fetched: a2a_types::Task = serde_json::from_value(result.clone()).expect("Task parses");
    assert_eq!(fetched.id, task.id);
}

#[tokio::test]
async fn tasks_list_returns_paged_response() {
    let suite = ensure_suite();
    let _ = create_task(suite, "tasks/list seed").await;
    let request = json!({
        "jsonrpc": "2.0",
        "id": "test-tasks-list-001",
        "method": "tasks/list",
        "params": {
            "contextId": "",
            "lastUpdatedAfter": 0,
            "pageToken": "",
            "status": "TASK_STATE_UNSPECIFIED",
            "tenant": "test",
            "pageSize": 10
        },
    });
    let response = post_jsonrpc(suite, request).await;
    assert!(
        response.get("error").is_none(),
        "expected success but got error: {response}"
    );
    let result = response.get("result").expect("result present");
    let listed: a2a_types::ListTasksResponse =
        serde_json::from_value(result.clone()).expect("ListTasksResponse parses");
    assert!(listed.total_size >= 1);
}

#[tokio::test]
async fn tasks_cancel_marks_task_cancelled() {
    let suite = ensure_suite();
    // `message/send` (non-blocking by default) leaves the new task in the
    // SUBMITTED state, so cancelling it should succeed.
    let task = create_task(suite, "tasks/cancel scenario").await;
    let request = json!({
        "jsonrpc": "2.0",
        "id": "test-tasks-cancel-001",
        "method": "tasks/cancel",
        "params": {
            "name": format!("tasks/{}", task.id),
            "tenant": "test",
        },
    });
    let response = post_jsonrpc(suite, request).await;
    assert!(
        response.get("error").is_none(),
        "tasks/cancel returned error: {response}"
    );
    let result = response.get("result").expect("result present");
    let cancelled: a2a_types::Task = serde_json::from_value(result.clone()).expect("Task parses");
    assert_eq!(
        cancelled.status.state,
        a2a_types::TaskState::TaskStateCancelled
    );
}

#[tokio::test]
async fn push_notification_config_round_trip() {
    let suite = ensure_suite();
    let task = create_task(suite, "push config scenario").await;
    let parent = format!("tasks/{}", task.id);
    let config_id = "cfg-001";
    let config_name = format!("{parent}/pushNotificationConfigs/{config_id}");

    // set
    let set_request = json!({
        "jsonrpc": "2.0",
        "id": "test-push-config-set-001",
        "method": "tasks/pushNotificationConfig/set",
        "params": {
            "parent": parent,
            "configId": config_id,
            "config": {
                "name": config_name,
                "pushNotificationConfig": {
                    "url": "http://localhost:9999/webhook",
                    "token": "test-token-123"
                }
            }
        },
    });
    let set_response = post_jsonrpc(suite, set_request).await;
    assert!(
        set_response.get("error").is_none(),
        "set push config failed: {set_response}"
    );
    let set_result = set_response.get("result").expect("result present");
    let set_typed: a2a_types::TaskPushNotificationConfig =
        serde_json::from_value(set_result.clone()).expect("config parses");
    assert_eq!(set_typed.name, config_name);

    // get
    let get_request = json!({
        "jsonrpc": "2.0",
        "id": "test-push-config-get-001",
        "method": "tasks/pushNotificationConfig/get",
        "params": {
            "name": config_name,
            "tenant": "test",
        },
    });
    let get_response = post_jsonrpc(suite, get_request).await;
    assert!(
        get_response.get("error").is_none(),
        "get push config failed: {get_response}"
    );
    let get_result = get_response.get("result").expect("result present");
    let get_typed: a2a_types::TaskPushNotificationConfig =
        serde_json::from_value(get_result.clone()).expect("config parses");
    assert_eq!(
        get_typed.push_notification_config.url,
        "http://localhost:9999/webhook"
    );

    // list
    let list_request = json!({
        "jsonrpc": "2.0",
        "id": "test-push-config-list-001",
        "method": "tasks/pushNotificationConfig/list",
        "params": {
            "parent": parent,
            "pageSize": 10,
            "pageToken": "",
            "tenant": "test",
        },
    });
    let list_response = post_jsonrpc(suite, list_request).await;
    assert!(
        list_response.get("error").is_none(),
        "list push configs failed: {list_response}"
    );
    let list_result = list_response.get("result").expect("result present");
    let list_typed: a2a_types::ListTaskPushNotificationConfigResponse =
        serde_json::from_value(list_result.clone()).expect("list parses");
    assert!(list_typed.configs.iter().any(|c| c.name == config_name));

    // delete
    let delete_request = json!({
        "jsonrpc": "2.0",
        "id": "test-push-config-delete-001",
        "method": "tasks/pushNotificationConfig/delete",
        "params": {
            "name": config_name,
            "tenant": "test",
        },
    });
    let delete_response = post_jsonrpc(suite, delete_request).await;
    assert!(
        delete_response.get("error").is_none(),
        "delete push config failed: {delete_response}"
    );
    assert!(delete_response.get("result").is_some());

    // confirm the delete actually removed the config.
    let post_delete_get = post_jsonrpc(
        suite,
        json!({
            "jsonrpc": "2.0",
            "id": "test-push-config-get-after-delete",
            "method": "tasks/pushNotificationConfig/get",
            "params": { "name": config_name, "tenant": "test" },
        }),
    )
    .await;
    assert!(post_delete_get.get("error").is_some());
}

#[tokio::test]
async fn unknown_method_returns_method_not_found() {
    let suite = ensure_suite();
    let request = json!({
        "jsonrpc": "2.0",
        "id": "test-method-not-found-001",
        "method": "nonexistent/method",
        "params": {}
    });
    let response = post_jsonrpc(suite, request).await;
    let code = response
        .get("error")
        .and_then(|e| e.get("code"))
        .and_then(|c| c.as_i64());
    assert_eq!(code, Some(-32601), "expected -32601, got {response}");
}

#[tokio::test]
async fn invalid_params_returns_invalid_params_error() {
    let suite = ensure_suite();
    let request = json!({
        "jsonrpc": "2.0",
        "id": "test-invalid-params-001",
        "method": "message/send",
        "params": {
            "invalid": "parameters"
        }
    });
    let response = post_jsonrpc(suite, request).await;
    let code = response
        .get("error")
        .and_then(|e| e.get("code"))
        .and_then(|c| c.as_i64());
    assert_eq!(code, Some(-32602), "expected -32602, got {response}");
}

#[tokio::test]
async fn invalid_jsonrpc_envelope_returns_invalid_request() {
    let suite = ensure_suite();
    let request = json!({ "invalid": "not a valid json-rpc request" });
    let response = post_jsonrpc(suite, request).await;
    let code = response
        .get("error")
        .and_then(|e| e.get("code"))
        .and_then(|c| c.as_i64());
    assert_eq!(code, Some(-32600), "expected -32600, got {response}");
}

#[tokio::test]
async fn client_typed_helpers_round_trip_send_and_list() {
    let suite = ensure_suite();
    let client = A2AClient::new(&suite.base_url).expect("client builds");

    let send_params = a2a_types::SendMessageRequest {
        configuration: None,
        message: Some(a2a_types::Message {
            context_id: None,
            extensions: vec![],
            message_id: uuid::Uuid::new_v4().to_string(),
            metadata: None,
            parts: vec![a2a_types::Part {
                data: None,
                file: None,
                metadata: None,
                text: Some("typed-client roundtrip".to_string()),
            }],
            reference_task_ids: vec![],
            role: a2a_types::Role::RoleUser,
            task_id: None,
        }),
        metadata: None,
        tenant: "test".to_string(),
    };

    let send_response = client
        .send_message(send_params)
        .await
        .expect("send_message ok");
    let task = send_response.task.expect("task present");

    let fetched = client
        .get_task(a2a_types::GetTaskRequest {
            history_length: None,
            name: format!("tasks/{}", task.id),
            tenant: Some("test".to_string()),
        })
        .await
        .expect("get_task ok");
    assert_eq!(fetched.id, task.id);

    let listed = client
        .list_tasks(a2a_types::ListTasksRequest {
            context_id: String::new(),
            history_length: None,
            include_artifacts: None,
            last_updated_after: 0,
            page_size: Some(50),
            page_token: String::new(),
            status: a2a_types::TaskState::TaskStateUnspecified,
            tenant: "test".to_string(),
        })
        .await
        .expect("list_tasks ok");
    assert!(listed.total_size >= 1);
}

#[tokio::test]
async fn tasks_resubscribe_returns_snapshot_and_final_event() {
    use futures::StreamExt;

    let suite = ensure_suite();
    let task = create_task(suite, "resubscribe round-trip").await;

    let client = A2AClient::new(&suite.base_url).expect("client builds");
    let mut stream = Box::pin(
        client
            .resubscribe_task(a2a_types::SubscribeToTaskRequest {
                name: format!("tasks/{}", task.id),
                tenant: "test".to_string(),
            })
            .await
            .expect("resubscribe_task ok"),
    );

    let first = timeout(suite.timeout_duration, stream.next())
        .await
        .expect("first event arrives")
        .expect("stream not empty")
        .expect("first event decodes");
    let snapshot = first.task.expect("first event carries task snapshot");
    assert_eq!(snapshot.id, task.id);

    drop(stream);
}

#[tokio::test]
async fn tasks_resubscribe_unknown_task_returns_task_not_found() {
    let suite = ensure_suite();
    let request = json!({
        "jsonrpc": "2.0",
        "id": "test-resubscribe-not-found",
        "method": "tasks/resubscribe",
        "params": {
            "name": "tasks/does-not-exist",
            "tenant": "test"
        }
    });
    let response = post_jsonrpc(suite, request).await;
    let code = response
        .get("error")
        .and_then(|e| e.get("code"))
        .and_then(|c| c.as_i64());
    assert_eq!(
        code,
        Some(-32001),
        "expected TASK_NOT_FOUND, got {response}"
    );
}

#[tokio::test]
async fn get_authenticated_extended_card_rejects_when_not_supported() {
    let suite = ensure_suite();
    let request = json!({
        "jsonrpc": "2.0",
        "id": "test-get-extended-card-disabled",
        "method": "agent/getAuthenticatedExtendedCard",
        "params": { "tenant": "test" }
    });
    let response = post_jsonrpc(suite, request).await;
    let code = response
        .get("error")
        .and_then(|e| e.get("code"))
        .and_then(|c| c.as_i64());
    assert_eq!(
        code,
        Some(-32601),
        "expected METHOD_NOT_FOUND, got {response}"
    );
}

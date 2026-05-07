//! Integration tests for the A2A JSON-RPC server endpoints exposed by
//! `inference_gateway_adk::A2AServerBuilder` and consumed by
//! `inference_gateway_adk::A2AClient`.
//!
//! Each test case spins up an isolated A2A server bound to an ephemeral port
//! (port 0) and a matching client that talks to it over loopback. The server
//! is built without a configured agent so handlers fall back to a
//! deterministic stub that does not require a running inference gateway.

use inference_gateway_adk::a2a_types::{
    AgentCard, DeleteTaskPushNotificationConfigParams, GetTaskPushNotificationConfigParams,
    ListTaskPushNotificationConfigParams, Message as A2aMessage, MessageRole as A2aMessageRole,
    MessageSendParams, Part, PushNotificationConfig, TaskIdParams, TaskPushNotificationConfig,
    TaskQueryParams, TextPart,
};
use inference_gateway_adk::{A2AClient, A2AServerBuilder};
use serde_json::{Value, json};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::time::sleep;

/// Bring up a fresh server on an ephemeral port and return a client targeting it.
async fn spawn_server() -> (A2AClient, SocketAddr) {
    // Bind once to discover a free port, then immediately drop the listener
    // so `A2AServer::serve` can re-bind on it. The brief window between drop
    // and re-bind is acceptable for isolated test runs.
    let probe = TcpListener::bind("127.0.0.1:0").await.expect("bind");
    let addr = probe.local_addr().expect("local_addr");
    drop(probe);

    let agent_card_json = json!({
        "name": "Test A2A Agent",
        "description": "Agent used for A2A integration tests",
        "version": "1.0.0",
        "protocolVersion": "0.2.6",
        "url": format!("http://{}/a2a", addr),
        "preferredTransport": "JSONRPC",
        "capabilities": {
            "streaming": true,
            "pushNotifications": true,
            "stateTransitionHistory": false
        },
        "defaultInputModes": ["text/plain"],
        "defaultOutputModes": ["text/plain"],
        "skills": [{
            "id": "echo",
            "name": "Echo",
            "description": "Echoes the user's message",
            "tags": ["test"]
        }]
    });
    let agent_card: AgentCard = serde_json::from_value(agent_card_json).expect("agent card");

    let server = A2AServerBuilder::new()
        .with_agent_card(agent_card)
        .build()
        .await
        .expect("server build");

    tokio::spawn(async move {
        let _ = server.serve(addr).await;
    });

    // Give axum a moment to start listening on the bound port. The test
    // would also retry on its first call, but a tiny delay keeps things
    // deterministic across slower CI hosts.
    for _ in 0..40 {
        if tokio::net::TcpStream::connect(addr).await.is_ok() {
            break;
        }
        sleep(Duration::from_millis(25)).await;
    }

    let client = A2AClient::new(format!("http://{}", addr)).expect("client");
    (client, addr)
}

fn user_message(text: &str) -> A2aMessage {
    A2aMessage {
        context_id: None,
        extensions: vec![],
        kind: "message".to_string(),
        message_id: format!("msg-{}", uuid::Uuid::new_v4()),
        metadata: Default::default(),
        parts: vec![Part::TextPart(TextPart {
            kind: "text".to_string(),
            metadata: Default::default(),
            text: text.to_string(),
        })],
        reference_task_ids: vec![],
        role: A2aMessageRole::User,
        task_id: None,
    }
}

fn extract_task_id(envelope: &Value) -> String {
    envelope
        .get("result")
        .and_then(|r| r.get("id"))
        .and_then(Value::as_str)
        .expect("expected result.id in envelope")
        .to_string()
}

#[tokio::test]
async fn health_endpoint_responds() {
    let (client, _addr) = spawn_server().await;
    let health = client.get_health().await.expect("health");
    assert!(!health.status.is_empty(), "expected non-empty health status");
}

#[tokio::test]
async fn agent_card_endpoint_returns_configured_card() {
    let (client, _addr) = spawn_server().await;
    let card = client.get_agent_card().await.expect("agent card");
    assert_eq!(card.name, "Test A2A Agent");
}

#[tokio::test]
async fn message_send_creates_and_returns_task() {
    let (client, _addr) = spawn_server().await;

    let params = MessageSendParams {
        configuration: None,
        message: user_message("hello there"),
        metadata: Default::default(),
    };
    let envelope = client.send_message(params).await.expect("send_message");
    assert_eq!(envelope["jsonrpc"], "2.0");
    assert!(envelope.get("error").is_none(), "unexpected error: {envelope}");
    let result = envelope.get("result").expect("result");
    assert_eq!(result["kind"], "task");
    assert!(result["id"].is_string());
}

#[tokio::test]
async fn tasks_get_returns_stored_task() {
    let (client, _addr) = spawn_server().await;

    let send = client
        .send_message(MessageSendParams {
            configuration: None,
            message: user_message("first"),
            metadata: Default::default(),
        })
        .await
        .expect("send_message");
    let task_id = extract_task_id(&send);

    let got = client
        .get_task(TaskQueryParams {
            history_length: None,
            id: task_id.clone(),
            metadata: Default::default(),
        })
        .await
        .expect("get_task");
    assert_eq!(got["result"]["id"], task_id);
    assert_eq!(got["result"]["kind"], "task");
}

#[tokio::test]
async fn tasks_get_unknown_id_returns_task_not_found() {
    let (client, _addr) = spawn_server().await;
    let envelope = client
        .get_task(TaskQueryParams {
            history_length: None,
            id: "does-not-exist".to_string(),
            metadata: Default::default(),
        })
        .await
        .expect("get_task");
    assert_eq!(envelope["error"]["code"], -32001);
}

#[tokio::test]
async fn tasks_cancel_unknown_returns_not_found_then_cancels_completed_as_not_cancelable() {
    let (client, _addr) = spawn_server().await;

    // 1. Unknown task id -> task not found.
    let env = client
        .cancel_task(TaskIdParams {
            id: "missing".to_string(),
            metadata: Default::default(),
        })
        .await
        .expect("cancel_task");
    assert_eq!(env["error"]["code"], -32001);

    // 2. A task that ran to completion synchronously is not cancelable.
    let send = client
        .send_message(MessageSendParams {
            configuration: None,
            message: user_message("second"),
            metadata: Default::default(),
        })
        .await
        .expect("send_message");
    let task_id = extract_task_id(&send);
    let env = client
        .cancel_task(TaskIdParams {
            id: task_id,
            metadata: Default::default(),
        })
        .await
        .expect("cancel_task");
    assert_eq!(env["error"]["code"], -32002);
}

#[tokio::test]
async fn push_notification_config_round_trip() {
    let (client, _addr) = spawn_server().await;

    let task_id = "push-task-1".to_string();

    // SET
    let set_env = client
        .set_task_push_notification_config(TaskPushNotificationConfig {
            push_notification_config: PushNotificationConfig {
                authentication: None,
                id: None,
                token: Some("secret".to_string()),
                url: "https://example.com/webhook".to_string(),
            },
            task_id: task_id.clone(),
        })
        .await
        .expect("set");
    assert!(set_env.get("error").is_none(), "set error: {set_env}");
    let stored_id = set_env["result"]["pushNotificationConfig"]["id"]
        .as_str()
        .expect("server should assign an id")
        .to_string();

    // GET (by task id only — should match the only stored config for the task)
    let get_env = client
        .get_task_push_notification_config(GetTaskPushNotificationConfigParams {
            id: task_id.clone(),
            metadata: Default::default(),
            push_notification_config_id: None,
        })
        .await
        .expect("get");
    assert_eq!(get_env["result"]["pushNotificationConfig"]["id"], stored_id);

    // LIST
    let list_env = client
        .list_task_push_notification_configs(ListTaskPushNotificationConfigParams {
            id: task_id.clone(),
            metadata: Default::default(),
        })
        .await
        .expect("list");
    let configs = list_env["result"].as_array().expect("array");
    assert_eq!(configs.len(), 1);

    // DELETE
    let del_env = client
        .delete_task_push_notification_config(DeleteTaskPushNotificationConfigParams {
            id: task_id.clone(),
            metadata: Default::default(),
            push_notification_config_id: stored_id.clone(),
        })
        .await
        .expect("delete");
    assert!(
        del_env.get("error").is_none(),
        "delete error: {del_env}"
    );

    // Verify it's gone via LIST.
    let list_env = client
        .list_task_push_notification_configs(ListTaskPushNotificationConfigParams {
            id: task_id,
            metadata: Default::default(),
        })
        .await
        .expect("list2");
    assert!(list_env["result"].as_array().expect("array").is_empty());
}

#[tokio::test]
async fn message_stream_emits_sse_events() {
    let (client, _addr) = spawn_server().await;

    let received: Arc<Mutex<Vec<Value>>> = Arc::new(Mutex::new(Vec::new()));
    let received_clone = received.clone();

    client
        .send_streaming_message(
            MessageSendParams {
                configuration: None,
                message: user_message("stream please"),
                metadata: Default::default(),
            },
            move |event| {
                received_clone.lock().expect("lock").push(event);
                Ok(())
            },
        )
        .await
        .expect("stream");

    let events = received.lock().expect("lock");
    assert!(events.len() >= 2, "expected ≥2 SSE events, got {events:?}");
    // First frame contains the initial Task envelope.
    assert_eq!(events[0]["result"]["kind"], "task");
    // Final frame contains a status-update with final=true.
    let last = events.last().unwrap();
    assert_eq!(last["result"]["kind"], "status-update");
    assert_eq!(last["result"]["final"], true);
}

#[tokio::test]
async fn tasks_resubscribe_emits_current_state() {
    let (client, _addr) = spawn_server().await;

    let send = client
        .send_message(MessageSendParams {
            configuration: None,
            message: user_message("resubscribe me"),
            metadata: Default::default(),
        })
        .await
        .expect("send_message");
    let task_id = extract_task_id(&send);

    let received: Arc<Mutex<Vec<Value>>> = Arc::new(Mutex::new(Vec::new()));
    let received_clone = received.clone();
    client
        .resubscribe_task(
            TaskIdParams {
                id: task_id.clone(),
                metadata: Default::default(),
            },
            move |event| {
                received_clone.lock().expect("lock").push(event);
                Ok(())
            },
        )
        .await
        .expect("resubscribe");

    let events = received.lock().expect("lock");
    assert_eq!(events.len(), 1, "expected single status-update event");
    assert_eq!(events[0]["result"]["kind"], "status-update");
    assert_eq!(events[0]["result"]["taskId"], task_id);
}

#[tokio::test]
async fn invalid_json_rpc_envelope_returns_invalid_request() {
    let (_client, addr) = spawn_server().await;
    let url = format!("http://{}/a2a", addr);
    let resp = reqwest::Client::new()
        .post(&url)
        .json(&json!({ "garbage": true }))
        .send()
        .await
        .expect("post");
    let body: Value = resp.json().await.expect("json");
    assert_eq!(body["error"]["code"], -32600);
}

#[tokio::test]
async fn unknown_method_returns_method_not_found() {
    let (_client, addr) = spawn_server().await;
    let url = format!("http://{}/a2a", addr);
    let resp = reqwest::Client::new()
        .post(&url)
        .json(&json!({
            "jsonrpc": "2.0",
            "id": "x",
            "method": "no/such/method",
            "params": {}
        }))
        .send()
        .await
        .expect("post");
    let body: Value = resp.json().await.expect("json");
    assert_eq!(body["error"]["code"], -32601);
}

#[tokio::test]
async fn invalid_params_for_known_method_returns_invalid_params() {
    let (_client, addr) = spawn_server().await;
    let url = format!("http://{}/a2a", addr);
    let resp = reqwest::Client::new()
        .post(&url)
        .json(&json!({
            "jsonrpc": "2.0",
            "id": "y",
            "method": "message/send",
            "params": { "not_a_message": 1 }
        }))
        .send()
        .await
        .expect("post");
    let body: Value = resp.json().await.expect("json");
    assert_eq!(body["error"]["code"], -32602);
}

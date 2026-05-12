use super::errors::{
    invalid_params, invalid_params_message, json_rpc_error, json_rpc_success, jsonrpc_errors,
};
use super::server_core::A2AServer;
use super::storage::{TaskFilter, parse_task_name};
use super::task_handler::StreamEmitter;
use crate::a2a_types::{
    CancelTaskRequest, DeleteTaskPushNotificationConfigRequest,
    GetTaskPushNotificationConfigRequest, GetTaskRequest, ListTaskPushNotificationConfigRequest,
    ListTaskPushNotificationConfigResponse, ListTasksRequest, ListTasksResponse,
    SendMessageRequest, SendMessageResponse, SetTaskPushNotificationConfigRequest, StreamResponse,
    Task, TaskState, TaskStatus, Timestamp,
};
use axum::{
    extract::State,
    response::{
        IntoResponse, Json, Response,
        sse::{Event, KeepAlive, Sse},
    },
};
use futures_util::stream::{Stream, StreamExt};
use serde_json::Value;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tracing::{debug, error, warn};

#[derive(Debug)]
pub(crate) struct AppState {
    pub(crate) server: A2AServer,
}

pub(crate) async fn a2a_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Value>,
) -> Response {
    debug!("A2A request received: {payload:?}");

    let id = payload.get("id").cloned().unwrap_or(Value::Null);

    let jsonrpc = payload.get("jsonrpc").and_then(|v| v.as_str());
    if jsonrpc != Some("2.0") {
        return json_rpc_error(
            id,
            jsonrpc_errors::INVALID_REQUEST,
            "Invalid Request",
            Some(Value::String(
                "Missing or invalid \"jsonrpc\" field; must be \"2.0\"".to_string(),
            )),
        )
        .into_response();
    }

    let method = match payload.get("method").and_then(|v| v.as_str()) {
        Some(m) => m.to_string(),
        None => {
            return json_rpc_error(
                id,
                jsonrpc_errors::INVALID_REQUEST,
                "Invalid Request",
                Some(Value::String("Missing \"method\" field".to_string())),
            )
            .into_response();
        }
    };

    let params = payload.get("params").cloned().unwrap_or(Value::Null);

    match method.as_str() {
        "message/send" => handle_message_send(&state, id, params)
            .await
            .into_response(),
        "message/stream" => handle_message_stream(state.clone(), id, params).await,
        "tasks/get" => handle_tasks_get(&state, id, params).await.into_response(),
        "tasks/list" => handle_tasks_list(&state, id, params).await.into_response(),
        "tasks/cancel" => handle_tasks_cancel(&state, id, params)
            .await
            .into_response(),
        "tasks/pushNotificationConfig/set" => handle_set_push_config(&state, id, params)
            .await
            .into_response(),
        "tasks/pushNotificationConfig/get" => handle_get_push_config(&state, id, params)
            .await
            .into_response(),
        "tasks/pushNotificationConfig/list" => handle_list_push_configs(&state, id, params)
            .await
            .into_response(),
        "tasks/pushNotificationConfig/delete" => handle_delete_push_config(&state, id, params)
            .await
            .into_response(),
        other => {
            warn!("Unknown JSON-RPC method requested: {other}");
            json_rpc_error(
                id,
                jsonrpc_errors::METHOD_NOT_FOUND,
                "Method not found",
                Some(Value::String(other.to_string())),
            )
            .into_response()
        }
    }
}

/// Validate the A2A-spec-required content of a `message/send` /
/// `message/stream` request. Returns an error suitable for surfacing as the
/// `data` field of a JSON-RPC `-32602` response.
fn validate_send_message_request(req: &SendMessageRequest) -> Result<(), String> {
    let Some(msg) = req.message.as_ref() else {
        return Err("`message` is required".to_string());
    };
    if msg.message_id.is_empty() {
        return Err(
            "`message.messageId` must be a non-empty string - per the A2A spec the message \
             creator owns this identifier (used by the server for duplicate detection)"
                .to_string(),
        );
    }
    if msg.parts.is_empty() {
        return Err("`message.parts` must contain at least one part".to_string());
    }
    Ok(())
}

fn build_task_from_request(req: &SendMessageRequest) -> Task {
    let task_id = uuid::Uuid::new_v4().to_string();
    let context_id = req
        .message
        .as_ref()
        .and_then(|m| m.context_id.clone())
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

    let mut history = Vec::new();
    if let Some(mut msg) = req.message.clone() {
        if msg.context_id.is_none() {
            msg.context_id = Some(context_id.clone());
        }
        if msg.task_id.is_none() {
            msg.task_id = Some(task_id.clone());
        }
        history.push(msg);
    }

    Task {
        artifacts: vec![],
        context_id,
        history,
        id: task_id,
        metadata: None,
        status: TaskStatus {
            message: None,
            state: TaskState::TaskStateSubmitted,
            timestamp: Some(Timestamp(chrono::Utc::now())),
        },
    }
}

async fn handle_message_send(state: &Arc<AppState>, id: Value, params: Value) -> Json<Value> {
    let request: SendMessageRequest = match serde_json::from_value(params) {
        Ok(r) => r,
        Err(e) => return invalid_params(id, e),
    };

    if let Err(detail) = validate_send_message_request(&request) {
        return invalid_params_message(id, detail);
    }

    if state.server.background_task_handler.is_none() {
        return json_rpc_error(
            id,
            jsonrpc_errors::METHOD_NOT_FOUND,
            "Method not found",
            Some(Value::String(
                "message/send is not supported: no background task handler is configured"
                    .to_string(),
            )),
        );
    }

    let initial_task = build_task_from_request(&request);

    if let Err(e) = state.server.storage.create_active_task(&initial_task).await {
        error!("create_active_task failed: {e}");
        return json_rpc_error(
            id,
            jsonrpc_errors::INTERNAL_ERROR,
            "Internal error",
            Some(Value::String(e.to_string())),
        );
    }

    if let Err(e) = state
        .server
        .storage
        .enqueue_task(initial_task.clone(), id.clone())
        .await
    {
        error!("enqueue_task failed: {e}");
        return json_rpc_error(
            id,
            jsonrpc_errors::INTERNAL_ERROR,
            "Internal error",
            Some(Value::String(e.to_string())),
        );
    }

    let response = SendMessageResponse {
        message: None,
        task: Some(initial_task),
    };

    match serde_json::to_value(response) {
        Ok(v) => json_rpc_success(id, v),
        Err(e) => json_rpc_error(
            id,
            jsonrpc_errors::INTERNAL_ERROR,
            "Internal error",
            Some(Value::String(e.to_string())),
        ),
    }
}

async fn handle_message_stream(state: Arc<AppState>, id: Value, params: Value) -> Response {
    let request: SendMessageRequest = match serde_json::from_value(params) {
        Ok(r) => r,
        Err(e) => return invalid_params(id, e).into_response(),
    };

    if let Err(detail) = validate_send_message_request(&request) {
        return invalid_params_message(id, detail).into_response();
    }

    let Some(handler) = state.server.streaming_task_handler.as_ref().cloned() else {
        return json_rpc_error(
            id,
            jsonrpc_errors::METHOD_NOT_FOUND,
            "Method not found",
            Some(Value::String(
                "message/stream is not supported: no streaming task handler is configured"
                    .to_string(),
            )),
        )
        .into_response();
    };

    let task = build_task_from_request(&request);
    state.server.storage.put_task(task.clone()).await;

    let (tx, rx) = mpsc::channel::<StreamResponse>(32);

    let initial = StreamResponse {
        artifact_update: None,
        message: None,
        status_update: None,
        task: Some(task.clone()),
    };
    if tx.send(initial).await.is_err() {
        return json_rpc_error(
            id,
            jsonrpc_errors::INTERNAL_ERROR,
            "Internal error",
            Some(Value::String(
                "stream receiver closed before initial event".to_string(),
            )),
        )
        .into_response();
    }

    let emitter = StreamEmitter::new(tx, Arc::clone(&state.server.storage));
    let task_id = task.id.clone();
    let message = request.message;
    tokio::spawn(async move {
        if let Err(e) = handler.handle_streaming_task(task, message, emitter).await {
            error!("streaming task handler for task {task_id} failed: {e}");
        }
    });

    let envelope_id = id.clone();
    let stream = ReceiverStream::new(rx).map(move |response| {
        let envelope = serde_json::json!({
            "jsonrpc": "2.0",
            "id": envelope_id.clone(),
            "result": response,
        });
        Ok::<_, Infallible>(
            Event::default()
                .json_data(envelope)
                .unwrap_or_else(|e| Event::default().data(format!("serialization error: {e}"))),
        )
    });

    let stream: Box<dyn Stream<Item = Result<Event, Infallible>> + Send + Unpin> =
        Box::new(Box::pin(stream));

    Sse::new(stream)
        .keep_alive(KeepAlive::default())
        .into_response()
}

async fn handle_tasks_get(state: &Arc<AppState>, id: Value, params: Value) -> Json<Value> {
    let request: GetTaskRequest = match serde_json::from_value(params) {
        Ok(r) => r,
        Err(e) => return invalid_params(id, e),
    };

    let task_id = match parse_task_name(&request.name) {
        Some(parsed) => parsed,
        None => {
            return json_rpc_error(
                id,
                jsonrpc_errors::INVALID_PARAMS,
                "Invalid params",
                Some(Value::String(format!(
                    "`name` must be of the form tasks/{{task_id}} (got {:?})",
                    request.name
                ))),
            );
        }
    };

    match state.server.storage.get_task(task_id).await {
        Some(mut task) => {
            if let Some(limit) = request.history_length {
                let limit = limit.max(0) as usize;
                if task.history.len() > limit {
                    let skip = task.history.len() - limit;
                    task.history = task.history.split_off(skip);
                }
            }
            match serde_json::to_value(task) {
                Ok(v) => json_rpc_success(id, v),
                Err(e) => json_rpc_error(
                    id,
                    jsonrpc_errors::INTERNAL_ERROR,
                    "Internal error",
                    Some(Value::String(e.to_string())),
                ),
            }
        }
        None => json_rpc_error(
            id,
            jsonrpc_errors::TASK_NOT_FOUND,
            "Task not found",
            Some(Value::String(request.name)),
        ),
    }
}

async fn handle_tasks_list(state: &Arc<AppState>, id: Value, params: Value) -> Json<Value> {
    let request: ListTasksRequest = match serde_json::from_value(params) {
        Ok(r) => r,
        Err(e) => return invalid_params(id, e),
    };

    let mut tasks = state.server.storage.list_tasks(TaskFilter::default()).await;

    if !request.context_id.is_empty() {
        tasks.retain(|t| t.context_id == request.context_id);
    }
    if !matches!(request.status, TaskState::TaskStateUnspecified) {
        tasks.retain(|t| t.status.state == request.status);
    }

    let total_size = tasks.len() as i32;
    let page_size = request.page_size.unwrap_or(50).clamp(1, 100);
    if tasks.len() > page_size as usize {
        tasks.truncate(page_size as usize);
    }

    let response = ListTasksResponse {
        next_page_token: String::new(),
        page_size,
        tasks,
        total_size,
    };

    match serde_json::to_value(response) {
        Ok(v) => json_rpc_success(id, v),
        Err(e) => json_rpc_error(
            id,
            jsonrpc_errors::INTERNAL_ERROR,
            "Internal error",
            Some(Value::String(e.to_string())),
        ),
    }
}

async fn handle_tasks_cancel(state: &Arc<AppState>, id: Value, params: Value) -> Json<Value> {
    let request: CancelTaskRequest = match serde_json::from_value(params) {
        Ok(r) => r,
        Err(e) => return invalid_params(id, e),
    };

    let task_id = match parse_task_name(&request.name) {
        Some(t) => t.to_string(),
        None => {
            return json_rpc_error(
                id,
                jsonrpc_errors::INVALID_PARAMS,
                "Invalid params",
                Some(Value::String(format!(
                    "`name` must be of the form tasks/{{task_id}} (got {:?})",
                    request.name
                ))),
            );
        }
    };

    let existing = match state.server.storage.get_task(&task_id).await {
        Some(t) => t,
        None => {
            return json_rpc_error(
                id,
                jsonrpc_errors::TASK_NOT_FOUND,
                "Task not found",
                Some(Value::String(request.name)),
            );
        }
    };

    if matches!(
        existing.status.state,
        TaskState::TaskStateCompleted
            | TaskState::TaskStateFailed
            | TaskState::TaskStateCancelled
            | TaskState::TaskStateRejected
    ) {
        return json_rpc_error(
            id,
            jsonrpc_errors::TASK_NOT_CANCELABLE,
            "Task cannot be cancelled in its current state",
            Some(Value::String(format!(
                "task {:?} is in terminal state {:?}",
                task_id, existing.status.state
            ))),
        );
    }

    let mut updated = existing;
    updated.status = TaskStatus {
        message: None,
        state: TaskState::TaskStateCancelled,
        timestamp: Some(Timestamp(chrono::Utc::now())),
    };
    if let Err(e) = state.server.storage.store_dead_letter_task(&updated).await {
        return json_rpc_error(
            id,
            jsonrpc_errors::INTERNAL_ERROR,
            "Internal error",
            Some(Value::String(e.to_string())),
        );
    }

    match serde_json::to_value(updated) {
        Ok(v) => json_rpc_success(id, v),
        Err(e) => json_rpc_error(
            id,
            jsonrpc_errors::INTERNAL_ERROR,
            "Internal error",
            Some(Value::String(e.to_string())),
        ),
    }
}

async fn handle_set_push_config(state: &Arc<AppState>, id: Value, params: Value) -> Json<Value> {
    let request: SetTaskPushNotificationConfigRequest = match serde_json::from_value(params) {
        Ok(r) => r,
        Err(e) => return invalid_params(id, e),
    };

    let canonical_name = format!(
        "{}/pushNotificationConfigs/{}",
        request.parent, request.config_id
    );

    let mut config = request.config;
    if config.name.is_empty() {
        config.name = canonical_name.clone();
    }

    state
        .server
        .storage
        .put_push_notification_config(config.clone())
        .await;

    match serde_json::to_value(config) {
        Ok(v) => json_rpc_success(id, v),
        Err(e) => json_rpc_error(
            id,
            jsonrpc_errors::INTERNAL_ERROR,
            "Internal error",
            Some(Value::String(e.to_string())),
        ),
    }
}

async fn handle_get_push_config(state: &Arc<AppState>, id: Value, params: Value) -> Json<Value> {
    let request: GetTaskPushNotificationConfigRequest = match serde_json::from_value(params) {
        Ok(r) => r,
        Err(e) => return invalid_params(id, e),
    };

    match state
        .server
        .storage
        .get_push_notification_config(&request.name)
        .await
    {
        Some(config) => match serde_json::to_value(config) {
            Ok(v) => json_rpc_success(id, v),
            Err(e) => json_rpc_error(
                id,
                jsonrpc_errors::INTERNAL_ERROR,
                "Internal error",
                Some(Value::String(e.to_string())),
            ),
        },
        None => json_rpc_error(
            id,
            jsonrpc_errors::TASK_NOT_FOUND,
            "Push notification config not found",
            Some(Value::String(request.name)),
        ),
    }
}

async fn handle_list_push_configs(state: &Arc<AppState>, id: Value, params: Value) -> Json<Value> {
    let request: ListTaskPushNotificationConfigRequest = match serde_json::from_value(params) {
        Ok(r) => r,
        Err(e) => return invalid_params(id, e),
    };

    let configs = state
        .server
        .storage
        .list_push_notification_configs(&request.parent)
        .await;

    let response = ListTaskPushNotificationConfigResponse {
        configs,
        next_page_token: String::new(),
    };

    match serde_json::to_value(response) {
        Ok(v) => json_rpc_success(id, v),
        Err(e) => json_rpc_error(
            id,
            jsonrpc_errors::INTERNAL_ERROR,
            "Internal error",
            Some(Value::String(e.to_string())),
        ),
    }
}

async fn handle_delete_push_config(state: &Arc<AppState>, id: Value, params: Value) -> Json<Value> {
    let request: DeleteTaskPushNotificationConfigRequest = match serde_json::from_value(params) {
        Ok(r) => r,
        Err(e) => return invalid_params(id, e),
    };

    let removed = state
        .server
        .storage
        .delete_push_notification_config(&request.name)
        .await;

    if !removed {
        return json_rpc_error(
            id,
            jsonrpc_errors::TASK_NOT_FOUND,
            "Push notification config not found",
            Some(Value::String(request.name)),
        );
    }

    json_rpc_success(id, serde_json::json!({}))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::a2a_types::{AgentCard, Message as A2AMessage, Part, Role};
    use crate::server::server_builder::A2AServerBuilder;
    use crate::server::task_handler::{
        StreamEmitter, StreamableTaskHandler, build_agent_text_message,
    };
    use anyhow::Result;
    use axum::Router;
    use axum::routing::post;
    use tokio::net::TcpListener;

    #[tokio::test]
    async fn message_stream_emits_state_transitions_end_to_end() {
        use crate::A2AClient;
        use futures_util::StreamExt;

        #[derive(Debug)]
        struct EchoStream;

        #[async_trait::async_trait]
        impl StreamableTaskHandler for EchoStream {
            async fn handle_streaming_task(
                &self,
                task: Task,
                message: Option<A2AMessage>,
                emitter: StreamEmitter,
            ) -> Result<()> {
                emitter
                    .emit_status(
                        &task.id,
                        &task.context_id,
                        TaskState::TaskStateWorking,
                        None,
                        false,
                    )
                    .await?;
                let user_text = message
                    .as_ref()
                    .map(|m| {
                        m.parts
                            .iter()
                            .filter_map(|p| p.text.clone())
                            .collect::<Vec<_>>()
                            .join("")
                    })
                    .unwrap_or_default();
                let reply_text = format!("Echo: {user_text}");
                emitter
                    .emit_text_artifact(&task.id, &task.context_id, reply_text.clone(), true)
                    .await?;
                let reply_message = build_agent_text_message(&task, &reply_text);
                emitter
                    .emit_status(
                        &task.id,
                        &task.context_id,
                        TaskState::TaskStateCompleted,
                        Some(reply_message),
                        true,
                    )
                    .await
            }
        }

        let agent_card: AgentCard = serde_json::from_value(serde_json::json!({
            "name": "Test Stream Agent",
            "description": "Streaming SSE end-to-end test",
            "version": "0.0.0",
            "protocolVersion": "0.2.6",
            "url": "http://localhost/a2a",
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
                    "id": "echo",
                    "name": "echo",
                    "description": "echo",
                    "tags": ["echo"]
                }
            ]
        }))
        .unwrap();

        let server = A2AServerBuilder::new()
            .with_agent_card(agent_card)
            .with_streaming_task_handler(EchoStream)
            .build()
            .await
            .expect("server builds");

        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("local addr");
        let app = Router::new()
            .route("/a2a", post(a2a_handler))
            .with_state(Arc::new(AppState { server }));
        tokio::spawn(async move {
            axum::serve(listener, app).await.ok();
        });

        let client = A2AClient::new(format!("http://{addr}")).expect("client");

        let request = SendMessageRequest {
            configuration: None,
            message: Some(A2AMessage {
                context_id: None,
                extensions: vec![],
                message_id: "msg-1".to_string(),
                metadata: None,
                parts: vec![Part {
                    data: None,
                    file: None,
                    metadata: None,
                    text: Some("ping".to_string()),
                }],
                reference_task_ids: vec![],
                role: Role::RoleUser,
                task_id: None,
            }),
            metadata: None,
            tenant: "tests".to_string(),
        };

        let mut stream = Box::pin(client.stream_message(request).await.expect("stream"));
        let mut events: Vec<StreamResponse> = Vec::new();
        while let Some(item) = stream.next().await {
            events.push(item.expect("event"));
        }

        assert_eq!(
            events.len(),
            4,
            "expected 4 events, got {}: {:?}",
            events.len(),
            events
        );

        let initial_task = events[0]
            .task
            .as_ref()
            .expect("first event carries the task");
        assert_eq!(initial_task.status.state, TaskState::TaskStateSubmitted);

        let working = events[1]
            .status_update
            .as_ref()
            .expect("second event is a status update");
        assert_eq!(working.status.state, TaskState::TaskStateWorking);
        assert!(!working.final_);

        let artifact = events[2]
            .artifact_update
            .as_ref()
            .expect("third event is an artifact update");
        let text = artifact
            .artifact
            .parts
            .iter()
            .filter_map(|p| p.text.clone())
            .collect::<String>();
        assert_eq!(text, "Echo: ping");

        let completed = events[3]
            .status_update
            .as_ref()
            .expect("fourth event is a status update");
        assert_eq!(completed.status.state, TaskState::TaskStateCompleted);
        assert!(completed.final_);
        let final_message_text = completed
            .status
            .message
            .as_ref()
            .expect("completed status carries the final agent message")
            .parts
            .iter()
            .filter_map(|p| p.text.clone())
            .collect::<String>();
        assert_eq!(final_message_text, "Echo: ping");
    }

    #[tokio::test]
    async fn message_stream_uses_custom_handler() {
        use crate::A2AClient;
        use futures_util::StreamExt;

        #[derive(Debug)]
        struct TwoStateHandler;

        #[async_trait::async_trait]
        impl StreamableTaskHandler for TwoStateHandler {
            async fn handle_streaming_task(
                &self,
                task: Task,
                _message: Option<A2AMessage>,
                emitter: StreamEmitter,
            ) -> anyhow::Result<()> {
                emitter
                    .emit_status(
                        &task.id,
                        &task.context_id,
                        TaskState::TaskStateFailed,
                        None,
                        true,
                    )
                    .await
            }
        }

        let agent_card: AgentCard = serde_json::from_value(serde_json::json!({
            "name": "Custom Handler Test",
            "description": "Verifies with_streaming_task_handler is used",
            "version": "0.0.0",
            "protocolVersion": "0.2.6",
            "url": "http://localhost/a2a",
            "preferredTransport": "JSONRPC",
            "capabilities": {"streaming": true, "pushNotifications": false, "stateTransitionHistory": false},
            "defaultInputModes": ["text/plain"],
            "defaultOutputModes": ["text/plain"],
            "skills": [{"id": "x", "name": "x", "description": "x", "tags": ["x"]}]
        }))
        .unwrap();

        let server = A2AServerBuilder::new()
            .with_agent_card(agent_card)
            .with_streaming_task_handler(TwoStateHandler)
            .build()
            .await
            .expect("server");

        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");
        let app = Router::new()
            .route("/a2a", post(a2a_handler))
            .with_state(Arc::new(AppState { server }));
        tokio::spawn(async move {
            axum::serve(listener, app).await.ok();
        });

        let client = A2AClient::new(format!("http://{addr}")).expect("client");

        let request = SendMessageRequest {
            configuration: None,
            message: Some(A2AMessage {
                context_id: None,
                extensions: vec![],
                message_id: "msg-2".to_string(),
                metadata: None,
                parts: vec![Part {
                    data: None,
                    file: None,
                    metadata: None,
                    text: Some("hi".to_string()),
                }],
                reference_task_ids: vec![],
                role: Role::RoleUser,
                task_id: None,
            }),
            metadata: None,
            tenant: "tests".to_string(),
        };

        let mut stream = Box::pin(client.stream_message(request).await.expect("stream"));
        let mut events: Vec<StreamResponse> = Vec::new();
        while let Some(item) = stream.next().await {
            events.push(item.expect("event"));
        }

        assert_eq!(events.len(), 2);
        assert!(events[0].task.is_some());
        let final_update = events[1].status_update.as_ref().expect("status update");
        assert_eq!(final_update.status.state, TaskState::TaskStateFailed);
        assert!(final_update.final_);
    }
}

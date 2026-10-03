use super::auth::{AuthVerifier, AuthenticatedPrincipal};
use super::errors::{
    internal_error, invalid_params, invalid_params_message, json_rpc_error, json_rpc_result,
    json_rpc_success, jsonrpc_errors, task_not_found,
};
use super::server_core::A2AServer;
use super::storage::TaskFilter;
use super::task_handler::StreamEmitter;
use super::tls::PeerCert;
use crate::a2a_types::{
    A2aMethod, CancelTaskRequest, DeleteTaskPushNotificationConfigRequest,
    GetExtendedAgentCardRequest, GetTaskPushNotificationConfigRequest, GetTaskRequest,
    ListTaskPushNotificationConfigsRequest, ListTaskPushNotificationConfigsResponse,
    ListTasksRequest, ListTasksResponse, SendMessageRequest, SendMessageResponse, StreamResponse,
    SubscribeToTaskRequest, Task, TaskPushNotificationConfig, TaskState, TaskStatus,
    TaskStatusUpdateEvent,
};
use axum::{
    extract::State,
    response::{
        IntoResponse, Json, Response,
        sse::{Event, KeepAlive, Sse},
    },
};
use futures_util::stream::StreamExt;
use serde_json::Value;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tracing::{debug, error, warn};

#[derive(Debug)]
pub(crate) struct AppState {
    pub(crate) server: A2AServer,
    /// Bearer-token verifier consulted by the auth middleware. `None`
    /// when `AuthConfig.enable == false` (or no `AuthConfig` is set) -
    /// in that mode the middleware is not attached and routes behave
    /// exactly as they did before authentication landed.
    pub(crate) auth_verifier: Option<Arc<dyn AuthVerifier>>,
}

impl AppState {
    /// Construct an `AppState` with no auth verifier. The middleware is
    /// a no-op in this mode and `POST /a2a` is reachable without a
    /// bearer token.
    pub(crate) fn new(server: A2AServer) -> Self {
        Self {
            server,
            auth_verifier: None,
        }
    }

    /// Construct an `AppState` with an auth verifier wired up. Callers
    /// must also attach [`super::auth::auth_middleware`] to the routes
    /// they want protected - the verifier alone has no effect.
    pub(crate) fn with_auth(server: A2AServer, verifier: Arc<dyn AuthVerifier>) -> Self {
        Self {
            server,
            auth_verifier: Some(verifier),
        }
    }
}

pub(crate) async fn a2a_handler(
    State(state): State<Arc<AppState>>,
    principal: Option<axum::Extension<AuthenticatedPrincipal>>,
    peer_cert: Option<axum::Extension<PeerCert>>,
    headers: axum::http::HeaderMap,
    body: axum::body::Bytes,
) -> Response {
    // Principal is plumbed in by the auth middleware. We log it for
    // observability and keep it available to handlers via a future
    // extension - per-tenant filtering of the extended card is the
    // first planned consumer.
    if let Some(axum::Extension(p)) = principal.as_ref() {
        debug!(
            subject = %p.subject,
            tenant = %p.tenant,
            issuer = %p.issuer,
            "authenticated A2A request",
        );
    }

    if let Some(axum::Extension(cert)) = peer_cert.as_ref()
        && let Some(p) = cert.0.as_ref()
    {
        debug!(
            cert_subject = %p.subject,
            cert_common_name = ?p.common_name,
            cert_issuer = %p.issuer,
            "mTLS A2A request",
        );
    }

    if let Some(content_type) = unsupported_content_type(&headers) {
        return json_rpc_error(
            Value::Null,
            jsonrpc_errors::CONTENT_TYPE_NOT_SUPPORTED,
            "Content type not supported",
            Some(Value::String(format!(
                "Content-Type {content_type} is not supported; use application/json"
            ))),
        )
        .into_response();
    }

    let payload: Value = match serde_json::from_slice(&body) {
        Ok(v) => v,
        Err(e) => {
            return json_rpc_error(
                Value::Null,
                jsonrpc_errors::PARSE_ERROR,
                "Parse error",
                Some(Value::String(e.to_string())),
            )
            .into_response();
        }
    };
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

    if let Some(version) = unsupported_a2a_version(&headers) {
        return json_rpc_error(
            id,
            jsonrpc_errors::VERSION_NOT_SUPPORTED,
            "Version not supported",
            Some(Value::String(format!(
                "A2A-Version {version} is not supported"
            ))),
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

    let params = camelize_keys(payload.get("params").cloned().unwrap_or(Value::Null));

    let Ok(a2a_method) = method.parse::<A2aMethod>() else {
        warn!("Unknown JSON-RPC method requested: {method}");
        return json_rpc_error(
            id,
            jsonrpc_errors::METHOD_NOT_FOUND,
            "Method not found",
            Some(Value::String(method)),
        )
        .into_response();
    };

    if is_push_notification_config_method(a2a_method) && !push_notifications_enabled(&state) {
        return json_rpc_error(
            id,
            jsonrpc_errors::PUSH_NOTIFICATION_NOT_SUPPORTED,
            "Push Notification is not supported",
            Some(Value::String(format!(
                "{method} is not supported: the agent card sets capabilities.pushNotifications to false"
            ))),
        )
        .into_response();
    }

    match a2a_method {
        A2aMethod::SendMessage => handle_message_send(&state, id, params)
            .await
            .into_response(),
        A2aMethod::SendStreamingMessage => handle_message_stream(state.clone(), id, params).await,
        A2aMethod::GetTask => handle_tasks_get(&state, id, params).await.into_response(),
        A2aMethod::ListTasks => handle_tasks_list(&state, id, params).await.into_response(),
        A2aMethod::CancelTask => handle_tasks_cancel(&state, id, params)
            .await
            .into_response(),
        A2aMethod::CreateTaskPushNotificationConfig => handle_set_push_config(&state, id, params)
            .await
            .into_response(),
        A2aMethod::GetTaskPushNotificationConfig => handle_get_push_config(&state, id, params)
            .await
            .into_response(),
        A2aMethod::ListTaskPushNotificationConfigs => handle_list_push_configs(&state, id, params)
            .await
            .into_response(),
        A2aMethod::DeleteTaskPushNotificationConfig => {
            handle_delete_push_config(&state, id, params)
                .await
                .into_response()
        }
        A2aMethod::SubscribeToTask => handle_tasks_resubscribe(state.clone(), id, params).await,
        A2aMethod::GetExtendedAgentCard => {
            handle_get_authenticated_extended_card(&state, id, params)
                .await
                .into_response()
        }
    }
}

/// Fields whose object values are caller-owned maps, so their keys must survive verbatim.
const OPAQUE_PARAM_FIELDS: [&str; 4] = ["metadata", "data", "header", "params"];

/// Accept the proto field names the A2A JSON binding allows alongside their
/// lowerCamelCase form (spec 1.4) by rewriting request param keys to camelCase.
fn camelize_keys(params: Value) -> Value {
    match params {
        Value::Object(map) => Value::Object(
            map.into_iter()
                .map(|(key, value)| {
                    let value = if OPAQUE_PARAM_FIELDS.contains(&key.as_str()) {
                        value
                    } else {
                        camelize_keys(value)
                    };
                    (to_lower_camel_case(&key), value)
                })
                .collect(),
        ),
        Value::Array(items) => Value::Array(items.into_iter().map(camelize_keys).collect()),
        other => other,
    }
}

fn to_lower_camel_case(key: &str) -> String {
    let mut segments = key.split('_');
    let Some(first) = segments.next() else {
        return key.to_string();
    };
    segments.fold(first.to_string(), |mut out, segment| {
        let mut chars = segment.chars();
        match chars.next() {
            Some(c) => {
                out.extend(c.to_uppercase());
                out.push_str(chars.as_str());
            }
            None => out.push('_'),
        }
        out
    })
}

fn is_push_notification_config_method(method: A2aMethod) -> bool {
    matches!(
        method,
        A2aMethod::CreateTaskPushNotificationConfig
            | A2aMethod::GetTaskPushNotificationConfig
            | A2aMethod::ListTaskPushNotificationConfigs
            | A2aMethod::DeleteTaskPushNotificationConfig
    )
}

fn push_notifications_enabled(state: &Arc<AppState>) -> bool {
    state
        .server
        .agent_card
        .as_ref()
        .and_then(|card| card.capabilities.push_notifications)
        .unwrap_or(false)
}

/// Returns the request's `Content-Type` when it isn't JSON, so the caller can answer with a
/// JSON-RPC `-32005` rather than letting the extractor reply with a bodiless HTTP 415.
/// A missing header is accepted - clients that omit it keep working.
fn unsupported_content_type(headers: &axum::http::HeaderMap) -> Option<String> {
    let content_type = headers.get(axum::http::header::CONTENT_TYPE)?;
    let content_type = content_type.to_str().unwrap_or("invalid");
    let media_type = content_type
        .split(';')
        .next()
        .unwrap_or_default()
        .trim()
        .to_ascii_lowercase();
    let supported =
        media_type.is_empty() || media_type == "application/json" || media_type.ends_with("+json");
    (!supported).then(|| content_type.to_string())
}

/// Returns the requested `A2A-Version` when this server doesn't speak it (A2A spec 3.6).
// ponytail: an empty header is accepted so clients that omit it keep working, although spec
// section 3.6 reads it as 0.3; reject it once 0.3 clients are gone.
fn unsupported_a2a_version(headers: &axum::http::HeaderMap) -> Option<String> {
    let version = headers.get("A2A-Version")?.to_str().unwrap_or("invalid");
    (!version.is_empty() && version != crate::A2A_PROTOCOL_VERSION).then(|| version.to_string())
}

/// Validate the A2A-spec-required content of a `SendMessage` /
/// `SendStreamingMessage` request. Returns an error suitable for surfacing as the
/// `data` field of a JSON-RPC `-32602` response.
fn validate_send_message_request(req: &SendMessageRequest) -> Result<(), String> {
    let msg = &req.message;
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
        .context_id
        .clone()
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

    let mut msg = req.message.clone();
    if msg.context_id.is_none() {
        msg.context_id = Some(context_id.clone());
    }
    if msg.task_id.is_none() {
        msg.task_id = Some(task_id.clone());
    }
    let history = vec![msg];

    Task {
        artifacts: vec![],
        context_id: Some(context_id),
        history,
        id: task_id,
        metadata: None,
        status: TaskStatus::now(TaskState::TaskStateSubmitted, None),
    }
}

/// Why a `SendMessage` naming an existing `taskId` cannot continue it: a terminal task
/// accepts no more input (A2A spec 3.1) and a `contextId` must match the task's own
/// (spec 3.4.3).
fn continuation_rejection(
    task: &Task,
    message: &crate::a2a_types::Message,
) -> Option<(i64, &'static str, String)> {
    if task.status.state.is_terminal() {
        return Some((
            jsonrpc_errors::UNSUPPORTED_OPERATION,
            "Unsupported operation",
            format!(
                "task {} is in terminal state {:?} and cannot accept more messages",
                task.id, task.status.state
            ),
        ));
    }
    let context_id = message.context_id.as_deref()?;
    (context_id != task.context_id_str()).then(|| {
        (
            jsonrpc_errors::INVALID_PARAMS,
            "Invalid params",
            format!(
                "`message.contextId` {context_id} does not match the contextId of task {}",
                task.id
            ),
        )
    })
}

/// Appends `message` to an existing task and re-submits it, so a `SendMessage` naming a
/// `taskId` continues that task (for example answering an `input-required` one).
fn continue_task(mut task: Task, message: &crate::a2a_types::Message) -> Task {
    let mut message = message.clone();
    if message.context_id.is_none() {
        message.context_id = task.context_id.clone();
    }
    message.task_id = Some(task.id.clone());
    task.history.push(message);
    task.status = TaskStatus::now(TaskState::TaskStateSubmitted, None);
    task
}

/// A2A spec 3.2.2: `SendMessage` returns once the task reaches a terminal or an
/// interrupted state.
fn is_settled(state: TaskState) -> bool {
    state.is_terminal()
        || matches!(
            state,
            TaskState::TaskStateInputRequired | TaskState::TaskStateAuthRequired
        )
}

/// Keep only the last `history_length` messages; `None` leaves the history untouched
/// (A2A spec 7.1).
fn trim_history(task: &mut Task, history_length: Option<i32>) {
    let Some(limit) = history_length else {
        return;
    };
    let limit = limit.max(0) as usize;
    if task.history.len() > limit {
        let skip = task.history.len() - limit;
        task.history = task.history.split_off(skip);
    }
}

/// How long `SendMessage` waits for a task to settle before answering with its
/// latest known state.
const SETTLE_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(30);
const SETTLE_POLL_INTERVAL: std::time::Duration = std::time::Duration::from_millis(25);
/// How often `SubscribeToTask` re-reads the task to spot a state change.
const RESUBSCRIBE_POLL_INTERVAL: std::time::Duration = std::time::Duration::from_millis(100);

/// Poll storage until the task settles, the timeout elapses, or the task disappears.
// ponytail: polling keeps the Storage trait unchanged; swap in a per-task notifier if
// the poll interval ever shows up in latency numbers.
async fn wait_until_settled(state: &Arc<AppState>, submitted: Task) -> Task {
    let deadline = tokio::time::Instant::now() + SETTLE_TIMEOUT;
    let mut latest = submitted;
    while tokio::time::Instant::now() < deadline {
        if let Some(task) = state.server.storage.get_task(&latest.id).await {
            if is_settled(task.status.state) {
                return task;
            }
            latest = task;
        }
        tokio::time::sleep(SETTLE_POLL_INTERVAL).await;
    }
    warn!(task_id = %latest.id, "task did not settle within the SendMessage timeout");
    latest
}

/// Registers a `taskPushNotificationConfig` sent inline with the message against the
/// task it was sent with (A2A spec 7.2); ignored when the card disables push notifications.
async fn register_inline_push_config(
    state: &Arc<AppState>,
    request: &SendMessageRequest,
    task_id: &str,
) {
    let Some(mut config) = request
        .configuration
        .as_ref()
        .and_then(|c| c.task_push_notification_config.clone())
    else {
        return;
    };
    if !push_notifications_enabled(state) {
        warn!(
            "ignoring inline taskPushNotificationConfig: the agent card disables push notifications"
        );
        return;
    }
    config.task_id = Some(task_id.to_string());
    if config.id.as_deref().unwrap_or_default().is_empty() {
        config.id = Some(uuid::Uuid::new_v4().to_string());
    }
    state
        .server
        .storage
        .put_push_notification_config(config)
        .await;
}

async fn handle_message_send(state: &Arc<AppState>, id: Value, params: Value) -> Json<Value> {
    let request: SendMessageRequest = match serde_json::from_value(params) {
        Ok(r) => r,
        Err(e) => return invalid_params(id, e),
    };

    if let Err(detail) = validate_send_message_request(&request) {
        return invalid_params_message(id, detail);
    }

    let existing = match request.message.task_id.as_deref() {
        Some(task_id) => match state.server.storage.get_task(task_id).await {
            Some(task) => {
                if let Some((code, message, detail)) =
                    continuation_rejection(&task, &request.message)
                {
                    return json_rpc_error(id, code, message, Some(Value::String(detail)));
                }
                Some(task)
            }
            None => return task_not_found(id, task_id),
        },
        None => None,
    };

    let Some(handler) = state.server.background_task_handler.as_ref() else {
        return json_rpc_error(
            id,
            jsonrpc_errors::METHOD_NOT_FOUND,
            "Method not found",
            Some(Value::String(
                "SendMessage is not supported: no background task handler is configured"
                    .to_string(),
            )),
        );
    };

    match handler.handle_message(&request.message).await {
        Ok(Some(reply)) => {
            return json_rpc_result(
                id,
                SendMessageResponse {
                    message: Some(reply),
                    task: None,
                },
            );
        }
        Ok(None) => {}
        Err(e) => {
            error!("handle_message failed: {e}");
            return internal_error(id, e);
        }
    }

    let submitted = match existing {
        Some(task) => {
            let task = continue_task(task, &request.message);
            state.server.storage.put_task(task.clone()).await;
            task
        }
        None => {
            let task = build_task_from_request(&request);
            if let Err(e) = state.server.storage.create_active_task(&task).await {
                error!("create_active_task failed: {e}");
                return internal_error(id, e);
            }
            task
        }
    };

    register_inline_push_config(state, &request, &submitted.id).await;

    if let Err(e) = state
        .server
        .storage
        .enqueue_task(submitted.clone(), id.clone())
        .await
    {
        error!("enqueue_task failed: {e}");
        return internal_error(id, e);
    }

    let configuration = request.configuration.as_ref();
    let mut task = if configuration.and_then(|c| c.return_immediately) == Some(true) {
        submitted
    } else {
        wait_until_settled(state, submitted).await
    };
    trim_history(&mut task, configuration.and_then(|c| c.history_length));

    json_rpc_result(
        id,
        SendMessageResponse {
            message: None,
            task: Some(task),
        },
    )
}

/// SSE response for an A2A streaming method: the task snapshot first, then
/// every `StreamResponse` from `rx`, each in a JSON-RPC envelope.
fn task_event_stream(id: Value, initial: Task, rx: mpsc::Receiver<StreamResponse>) -> Response {
    let snapshot = StreamResponse {
        artifact_update: None,
        message: None,
        status_update: None,
        task: Some(initial),
    };
    let stream = futures_util::stream::once(async move { snapshot })
        .chain(ReceiverStream::new(rx))
        .map(move |response| {
            let envelope = serde_json::json!({
                "jsonrpc": "2.0",
                "id": id.clone(),
                "result": response,
            });
            Ok::<_, Infallible>(
                Event::default()
                    .json_data(envelope)
                    .unwrap_or_else(|e| Event::default().data(format!("serialization error: {e}"))),
            )
        });

    Sse::new(stream)
        .keep_alive(KeepAlive::default())
        .into_response()
}

async fn handle_message_stream(state: Arc<AppState>, id: Value, params: Value) -> Response {
    let request: SendMessageRequest = match serde_json::from_value(params) {
        Ok(r) => r,
        Err(e) => return invalid_params(id, e).into_response(),
    };

    if let Err(detail) = validate_send_message_request(&request) {
        return invalid_params_message(id, detail).into_response();
    }

    if let Some(task_id) = request.message.task_id.as_deref()
        && state.server.storage.get_task(task_id).await.is_none()
    {
        return task_not_found(id, task_id).into_response();
    }

    let Some(handler) = state.server.streaming_task_handler.as_ref().cloned() else {
        return json_rpc_error(
            id,
            jsonrpc_errors::METHOD_NOT_FOUND,
            "Method not found",
            Some(Value::String(
                "SendStreamingMessage is not supported: no streaming task handler is configured"
                    .to_string(),
            )),
        )
        .into_response();
    };

    let task = build_task_from_request(&request);
    state.server.storage.put_task(task.clone()).await;

    let (tx, rx) = mpsc::channel::<StreamResponse>(32);

    let emitter = StreamEmitter::new(tx, Arc::clone(&state.server.storage))
        .with_artifact_service(state.server.artifact_service.clone());
    let task_id = task.id.clone();
    let message = Some(request.message);
    let snapshot = task.clone();
    tokio::spawn(async move {
        if let Err(e) = handler.handle_streaming_task(task, message, emitter).await {
            error!("streaming task handler for task {task_id} failed: {e}");
        }
    });

    task_event_stream(id, snapshot, rx)
}

async fn handle_tasks_get(state: &Arc<AppState>, id: Value, params: Value) -> Json<Value> {
    let request: GetTaskRequest = match serde_json::from_value(params) {
        Ok(r) => r,
        Err(e) => return invalid_params(id, e),
    };

    let task_id = request.id.as_str();

    match state.server.storage.get_task(task_id).await {
        Some(mut task) => {
            trim_history(&mut task, request.history_length);
            json_rpc_result(id, task)
        }
        None => task_not_found(id, task_id),
    }
}

async fn handle_tasks_list(state: &Arc<AppState>, id: Value, params: Value) -> Json<Value> {
    let request: ListTasksRequest = match serde_json::from_value(params) {
        Ok(r) => r,
        Err(e) => return invalid_params(id, e),
    };

    let mut tasks = state.server.storage.list_tasks(TaskFilter::default()).await;

    if let Some(context_id) = request.context_id.filter(|c| !c.is_empty()) {
        tasks.retain(|t| t.context_id.as_deref() == Some(context_id.as_str()));
    }
    if let Some(status) = request
        .status
        .filter(|s| *s != TaskState::TaskStateUnspecified)
    {
        tasks.retain(|t| t.status.state == status);
    }

    let total_size = tasks.len() as i32;
    let page_size = request.page_size.unwrap_or(50).clamp(1, 100);
    let offset = match parse_page_token(request.page_token.as_deref()) {
        Some(offset) => offset,
        None => return invalid_params_message(id, "invalid `pageToken`"),
    };
    let page: Vec<Task> = tasks
        .into_iter()
        .skip(offset)
        .take(page_size as usize)
        .collect();
    let next = offset + page.len();
    let next_page_token = if next < total_size as usize {
        next.to_string()
    } else {
        String::new()
    };

    let response = ListTasksResponse {
        next_page_token,
        page_size,
        tasks: page,
        total_size,
    };

    json_rpc_result(id, response)
}

/// The `ListTasks` page token is the offset of the page; an empty token is the first page.
fn parse_page_token(token: Option<&str>) -> Option<usize> {
    match token.unwrap_or_default() {
        "" => Some(0),
        raw => raw.parse().ok(),
    }
}

async fn handle_tasks_cancel(state: &Arc<AppState>, id: Value, params: Value) -> Json<Value> {
    let request: CancelTaskRequest = match serde_json::from_value(params) {
        Ok(r) => r,
        Err(e) => return invalid_params(id, e),
    };
    let task_id = request.id.clone();

    let existing = match state.server.storage.get_task(&task_id).await {
        Some(t) => t,
        None => return task_not_found(id, &task_id),
    };

    if existing.status.state.is_terminal() {
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
    updated.status = TaskStatus::now(TaskState::TaskStateCanceled, None);
    if let Err(e) = state.server.storage.store_dead_letter_task(&updated).await {
        return internal_error(id, e);
    }

    json_rpc_result(id, updated)
}

async fn handle_set_push_config(state: &Arc<AppState>, id: Value, params: Value) -> Json<Value> {
    let mut config: TaskPushNotificationConfig = match serde_json::from_value(params) {
        Ok(r) => r,
        Err(e) => return invalid_params(id, e),
    };
    if config.task_id.as_deref().unwrap_or_default().is_empty() {
        return invalid_params_message(id, "`taskId` is required");
    }
    if config.id.as_deref().unwrap_or_default().is_empty() {
        config.id = Some(uuid::Uuid::new_v4().to_string());
    }

    state
        .server
        .storage
        .put_push_notification_config(config.clone())
        .await;

    json_rpc_result(id, config)
}

async fn handle_get_push_config(state: &Arc<AppState>, id: Value, params: Value) -> Json<Value> {
    let request: GetTaskPushNotificationConfigRequest = match serde_json::from_value(params) {
        Ok(r) => r,
        Err(e) => return invalid_params(id, e),
    };

    match state
        .server
        .storage
        .get_push_notification_config(&request.task_id, &request.id)
        .await
    {
        Some(config) => json_rpc_result(id, config),
        None => json_rpc_error(
            id,
            jsonrpc_errors::TASK_NOT_FOUND,
            "Push notification config not found",
            Some(Value::String(format!("{}/{}", request.task_id, request.id))),
        ),
    }
}

async fn handle_list_push_configs(state: &Arc<AppState>, id: Value, params: Value) -> Json<Value> {
    let request: ListTaskPushNotificationConfigsRequest = match serde_json::from_value(params) {
        Ok(r) => r,
        Err(e) => return invalid_params(id, e),
    };

    let configs = state
        .server
        .storage
        .list_push_notification_configs(&request.task_id)
        .await;

    let response = ListTaskPushNotificationConfigsResponse {
        configs,
        next_page_token: None,
    };

    json_rpc_result(id, response)
}

/// Deleting a push notification config is idempotent (A2A spec 3.11): an unknown
/// `(taskId, id)` pair succeeds too.
async fn handle_delete_push_config(state: &Arc<AppState>, id: Value, params: Value) -> Json<Value> {
    let request: DeleteTaskPushNotificationConfigRequest = match serde_json::from_value(params) {
        Ok(r) => r,
        Err(e) => return invalid_params(id, e),
    };

    state
        .server
        .storage
        .delete_push_notification_config(&request.task_id, &request.id)
        .await;

    json_rpc_success(id, serde_json::json!({}))
}

/// `SubscribeToTask` - re-attach to an existing task by id, emit the
/// current task state, and replay subsequent state transitions as SSE
/// events. The stream terminates with a `TaskStatusUpdateEvent` whose
/// state is terminal (or when the task is removed from storage).
///
/// The in-memory storage does not expose a pub/sub primitive, so the
/// implementation polls the storage at a short interval and emits a
/// status update whenever the observed `state` changes. Custom
/// `Storage` backends can rely on the same behaviour because the
/// `Storage` trait does not require change-stream support.
async fn handle_tasks_resubscribe(state: Arc<AppState>, id: Value, params: Value) -> Response {
    let request: SubscribeToTaskRequest = match serde_json::from_value(params) {
        Ok(r) => r,
        Err(e) => return invalid_params(id, e).into_response(),
    };
    let task_id = request.id.clone();

    let task = match state.server.storage.get_task(&task_id).await {
        Some(t) => t,
        None => return task_not_found(id, &task_id).into_response(),
    };

    if task.status.state.is_terminal() {
        return json_rpc_error(
            id,
            jsonrpc_errors::UNSUPPORTED_OPERATION,
            "Unsupported operation",
            Some(Value::String(format!(
                "task {task_id} is in terminal state {:?}; there is nothing left to stream",
                task.status.state
            ))),
        )
        .into_response();
    }

    let (tx, rx) = mpsc::channel::<StreamResponse>(32);

    let storage = Arc::clone(&state.server.storage);
    let initial_state = task.status.state;
    let task_id_for_poll = task_id.clone();

    tokio::spawn(async move {
        let mut last_state = initial_state;
        loop {
            tokio::time::sleep(RESUBSCRIBE_POLL_INTERVAL).await;
            if tx.is_closed() {
                break;
            }
            let Some(updated) = storage.get_task(&task_id_for_poll).await else {
                debug!(
                    "resubscribe: task {task_id_for_poll} disappeared from storage; closing stream"
                );
                break;
            };
            let current_state = updated.status.state;
            if current_state == last_state {
                continue;
            }
            let is_final = current_state.is_terminal();
            let event = TaskStatusUpdateEvent {
                context_id: updated.context_id_str().to_string(),
                metadata: None,
                status: updated.status.clone(),
                task_id: task_id_for_poll.clone(),
            };
            if tx
                .send(StreamResponse {
                    artifact_update: None,
                    message: None,
                    status_update: Some(event),
                    task: None,
                })
                .await
                .is_err()
            {
                break;
            }
            last_state = current_state;
            if is_final {
                break;
            }
        }
    });

    task_event_stream(id, task, rx)
}

/// `GetExtendedAgentCard` - return the authenticated extended
/// [`AgentCard`] for the calling tenant, following the A2A spec (8.2) contract:
///
/// - `supportsExtendedAgentCard` absent or false -> `-32004`
///   (UnsupportedOperationError)
/// - flag true but no extended card configured -> `-32007`
///   (AuthenticatedExtendedCardNotConfiguredError)
/// - otherwise -> the configured extended card
async fn handle_get_authenticated_extended_card(
    state: &Arc<AppState>,
    id: Value,
    params: Value,
) -> Json<Value> {
    let _request: GetExtendedAgentCardRequest = match serde_json::from_value(params) {
        Ok(r) => r,
        Err(e) => return invalid_params(id, e),
    };

    let Some(agent_card) = state.server.agent_card.as_ref() else {
        return internal_error(id, "no agent card configured on this server");
    };

    if !agent_card.capabilities.extended_agent_card.unwrap_or(false) {
        return json_rpc_error(
            id,
            jsonrpc_errors::UNSUPPORTED_OPERATION,
            "This operation is not supported",
            Some(Value::String(
                "GetExtendedAgentCard is not supported by this agent \
                 (set supportsExtendedAgentCard=true and configure an extended card \
                 to enable it)"
                    .to_string(),
            )),
        );
    }

    let Some(extended_card) = state.server.extended_agent_card.as_ref() else {
        return json_rpc_error(
            id,
            jsonrpc_errors::AUTHENTICATED_EXTENDED_CARD_NOT_CONFIGURED,
            "Authenticated extended card not configured",
            Some(Value::String(
                "the agent advertises supportsExtendedAgentCard=true but no extended \
                 card is configured (use A2AServerBuilder::with_extended_agent_card())"
                    .to_string(),
            )),
        );
    };

    json_rpc_result(id, extended_card)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::a2a_types::{AgentCard, Message as A2AMessage, Part, Role};
    use crate::server::server_builder::A2AServerBuilder;
    use crate::server::task_handler::{
        StreamEmitter, StreamableTaskHandler, TaskHandler, build_agent_text_message,
    };
    use anyhow::Result;
    use axum::Router;
    use axum::routing::post;
    use serde_json::json;
    use tokio::net::TcpListener;

    #[test]
    fn camelize_keys_accepts_proto_names_and_leaves_opaque_maps_alone() {
        let cases = [
            (json!({"page_size": 10}), json!({"pageSize": 10})),
            (json!({"pageSize": 10}), json!({"pageSize": 10})),
            (
                json!({"status_timestamp_after": "now"}),
                json!({"statusTimestampAfter": "now"}),
            ),
            (
                json!({"message": {"message_id": "m1", "metadata": {"my_key": 1}}}),
                json!({"message": {"messageId": "m1", "metadata": {"my_key": 1}}}),
            ),
            (
                json!({"parts": [{"data": {"raw_key": true}}]}),
                json!({"parts": [{"data": {"raw_key": true}}]}),
            ),
        ];

        for (input, expected) in cases {
            assert_eq!(camelize_keys(input.clone()), expected, "input: {input}");
        }
    }

    #[test]
    fn list_tasks_params_accept_proto_names_and_unknown_fields() {
        let params = camelize_keys(json!({
            "context_id": "ctx-1",
            "page_size": 10,
            "surprise": "ignored",
        }));
        let request: ListTasksRequest =
            serde_json::from_value(params).expect("snake_case params parse");

        assert_eq!(request.context_id.as_deref(), Some("ctx-1"));
        assert_eq!(request.page_size, Some(10));
    }

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
                        task.context_id_str(),
                        TaskState::TaskStateWorking,
                        None,
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
                    .emit_text_artifact(&task.id, task.context_id_str(), reply_text.clone(), true)
                    .await?;
                let reply_message = build_agent_text_message(&task, &reply_text);
                emitter
                    .emit_status(
                        &task.id,
                        task.context_id_str(),
                        TaskState::TaskStateCompleted,
                        Some(reply_message),
                    )
                    .await
            }
        }

        let agent_card: AgentCard = serde_json::from_value(serde_json::json!({
            "name": "Test Stream Agent",
            "description": "Streaming SSE end-to-end test",
            "version": "0.0.0",
            "supportedInterfaces": [{"url": "http://localhost/a2a", "protocolBinding": "JSONRPC", "protocolVersion": "1.0"}],
            "capabilities": {
                "streaming": true,
                "pushNotifications": false
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
            .with_state(Arc::new(AppState::new(server)));
        tokio::spawn(async move {
            axum::serve(listener, app).await.ok();
        });

        let client = A2AClient::new(format!("http://{addr}")).expect("client");

        let request = SendMessageRequest {
            configuration: None,
            message: A2AMessage {
                context_id: None,
                extensions: vec![],
                message_id: "msg-1".to_string(),
                metadata: None,
                parts: vec![Part {
                    text: Some("ping".to_string()),
                    ..Default::default()
                }],
                reference_task_ids: vec![],
                role: Role::RoleUser,
                task_id: None,
            },
            metadata: None,
            tenant: Some("tests".to_string()),
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
        assert!(!working.status.state.is_terminal());

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
        assert!(completed.status.state.is_terminal());
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
                        task.context_id_str(),
                        TaskState::TaskStateFailed,
                        None,
                    )
                    .await
            }
        }

        let agent_card: AgentCard = serde_json::from_value(serde_json::json!({
            "name": "Custom Handler Test",
            "description": "Verifies with_streaming_task_handler is used",
            "version": "0.0.0",
            "supportedInterfaces": [{"url": "http://localhost/a2a", "protocolBinding": "JSONRPC", "protocolVersion": "1.0"}],
            "capabilities": {"streaming": true, "pushNotifications": false},
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
            .with_state(Arc::new(AppState::new(server)));
        tokio::spawn(async move {
            axum::serve(listener, app).await.ok();
        });

        let client = A2AClient::new(format!("http://{addr}")).expect("client");

        let request = SendMessageRequest {
            configuration: None,
            message: A2AMessage {
                context_id: None,
                extensions: vec![],
                message_id: "msg-2".to_string(),
                metadata: None,
                parts: vec![Part {
                    text: Some("hi".to_string()),
                    ..Default::default()
                }],
                reference_task_ids: vec![],
                role: Role::RoleUser,
                task_id: None,
            },
            metadata: None,
            tenant: Some("tests".to_string()),
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
        assert!(final_update.status.state.is_terminal());
    }

    // ----- SubscribeToTask -------------------------------------------

    fn minimal_agent_card_for_resubscribe() -> AgentCard {
        serde_json::from_value(serde_json::json!({
            "name": "Resubscribe Test Agent",
            "description": "SubscribeToTask E2E test",
            "version": "0.0.0",
            "supportedInterfaces": [{"url": "http://localhost/a2a", "protocolBinding": "JSONRPC", "protocolVersion": "1.0"}],
            "capabilities": {
                "streaming": true,
                "pushNotifications": false
            },
            "defaultInputModes": ["text/plain"],
            "defaultOutputModes": ["text/plain"],
            "skills": [
                {"id": "x", "name": "x", "description": "x", "tags": ["x"]}
            ]
        }))
        .expect("agent card builds")
    }

    async fn spawn_test_server(server: super::A2AServer) -> std::net::SocketAddr {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");
        let app = Router::new()
            .route("/a2a", post(a2a_handler))
            .with_state(Arc::new(AppState::new(server)));
        tokio::spawn(async move {
            axum::serve(listener, app).await.ok();
        });
        addr
    }

    /// Resubscribing to a task that already reached a terminal state is rejected
    /// (A2A spec 3.1.6 / TCK STREAM-SUB-003).
    #[tokio::test]
    async fn resubscribe_rejects_terminal_task() {
        use crate::A2AClient;
        use crate::a2a_types::SubscribeToTaskRequest;

        let server = A2AServerBuilder::new()
            .with_agent_card(minimal_agent_card_for_resubscribe())
            .with_default_streaming_task_handler()
            .build()
            .await
            .expect("server builds");

        let storage = server.storage();
        let task_id = uuid::Uuid::new_v4().to_string();
        let context_id = uuid::Uuid::new_v4().to_string();
        let terminal_task = Task {
            artifacts: vec![],
            context_id: Some(context_id.clone()),
            history: vec![],
            id: task_id.clone(),
            metadata: None,
            status: TaskStatus::now(TaskState::TaskStateCompleted, None),
        };
        storage
            .create_active_task(&terminal_task)
            .await
            .expect("create active");
        storage
            .store_dead_letter_task(&terminal_task)
            .await
            .expect("dead-letter");

        let addr = spawn_test_server(server).await;
        let client = A2AClient::new(format!("http://{addr}")).expect("client");

        let err = client
            .resubscribe_task(SubscribeToTaskRequest {
                id: task_id.to_string(),
                tenant: Some("tests".to_string()),
            })
            .await
            .err()
            .expect("resubscribe against a terminal task must error");
        let message = err.to_string();
        assert!(
            message.contains("Unsupported operation") || message.contains("-32004"),
            "expected UNSUPPORTED_OPERATION error, got: {message}"
        );
    }

    /// Resubscribing to a live task should emit the snapshot, then a
    /// status update per observed state change, terminating with
    /// `final: true` once the task reaches a terminal state.
    #[tokio::test]
    async fn resubscribe_replays_live_state_transitions() {
        use crate::A2AClient;
        use crate::a2a_types::SubscribeToTaskRequest;
        use futures_util::StreamExt;

        let server = A2AServerBuilder::new()
            .with_agent_card(minimal_agent_card_for_resubscribe())
            .with_default_streaming_task_handler()
            .build()
            .await
            .expect("server builds");

        let storage = server.storage();
        let task_id = uuid::Uuid::new_v4().to_string();
        let context_id = uuid::Uuid::new_v4().to_string();
        let initial_task = Task {
            artifacts: vec![],
            context_id: Some(context_id.clone()),
            history: vec![],
            id: task_id.clone(),
            metadata: None,
            status: TaskStatus::now(TaskState::TaskStateWorking, None),
        };
        storage
            .create_active_task(&initial_task)
            .await
            .expect("create active");

        let storage_for_driver = Arc::clone(&storage);
        let task_id_for_driver = task_id.clone();
        let context_id_for_driver = context_id.clone();
        tokio::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_millis(250)).await;
            let completed = Task {
                artifacts: vec![],
                context_id: Some(context_id_for_driver),
                history: vec![],
                id: task_id_for_driver,
                metadata: None,
                status: TaskStatus::now(TaskState::TaskStateCompleted, None),
            };
            storage_for_driver.put_task(completed).await;
        });

        let addr = spawn_test_server(server).await;
        let client = A2AClient::new(format!("http://{addr}")).expect("client");

        let mut stream = Box::pin(
            client
                .resubscribe_task(SubscribeToTaskRequest {
                    id: task_id.to_string(),
                    tenant: Some("tests".to_string()),
                })
                .await
                .expect("resubscribe"),
        );

        let mut events: Vec<StreamResponse> = Vec::new();
        while let Some(item) = stream.next().await {
            events.push(item.expect("event"));
        }

        assert!(
            events.len() >= 2,
            "expected snapshot + at least one status update, got {events:?}"
        );
        let snapshot = events[0].task.as_ref().expect("first event is the task");
        assert_eq!(snapshot.id, task_id);
        assert_eq!(snapshot.status.state, TaskState::TaskStateWorking);

        let last = events.last().expect("at least one event");
        let final_update = last
            .status_update
            .as_ref()
            .expect("last event is a status update");
        assert!(
            final_update.status.state.is_terminal(),
            "stream must terminate with final=true"
        );
        assert_eq!(final_update.status.state, TaskState::TaskStateCompleted);
    }

    /// `SubscribeToTask` against a missing task should surface a
    /// JSON-RPC `TASK_NOT_FOUND` error rather than opening an empty
    /// stream that never closes.
    #[tokio::test]
    async fn resubscribe_returns_task_not_found_for_unknown_task() {
        use crate::A2AClient;
        use crate::a2a_types::SubscribeToTaskRequest;

        let server = A2AServerBuilder::new()
            .with_agent_card(minimal_agent_card_for_resubscribe())
            .with_default_streaming_task_handler()
            .build()
            .await
            .expect("server builds");

        let addr = spawn_test_server(server).await;
        let client = A2AClient::new(format!("http://{addr}")).expect("client");

        let result = client
            .resubscribe_task(SubscribeToTaskRequest {
                id: "does-not-exist".to_string(),
                tenant: Some("tests".to_string()),
            })
            .await;
        let err = result
            .err()
            .expect("resubscribe against missing task must error");
        let message = err.to_string();
        assert!(
            message.contains("Task not found") || message.contains("-32001"),
            "expected TASK_NOT_FOUND error, got: {message}"
        );
    }

    // ----- GetExtendedAgentCard --------------------------

    fn agent_card_with_extended(supports: bool) -> AgentCard {
        serde_json::from_value(serde_json::json!({
            "name": "Extended Card Agent",
            "description": "GetExtendedAgentCard test",
            "version": "1.2.3",
            "supportedInterfaces": [{"url": "http://localhost/a2a", "protocolBinding": "JSONRPC", "protocolVersion": "1.0"}],
            "capabilities": {
                "streaming": true,
                "pushNotifications": false,
                "extendedAgentCard": supports
            },
            "defaultInputModes": ["text/plain"],
            "defaultOutputModes": ["text/plain"],
            "skills": [
                {"id": "x", "name": "x", "description": "x", "tags": ["x"]}
            ]
        }))
        .expect("agent card builds")
    }

    #[tokio::test]
    async fn get_authenticated_extended_card_returns_card_when_supported() {
        use crate::A2AClient;
        use crate::a2a_types::GetExtendedAgentCardRequest;

        let server = A2AServerBuilder::new()
            .with_agent_card(agent_card_with_extended(true))
            .with_extended_agent_card(agent_card_with_extended(true))
            .with_default_streaming_task_handler()
            .build()
            .await
            .expect("server builds");

        let addr = spawn_test_server(server).await;
        let client = A2AClient::new(format!("http://{addr}")).expect("client");

        let card = client
            .get_authenticated_extended_card(GetExtendedAgentCardRequest {
                tenant: Some("tests".to_string()),
            })
            .await
            .expect("extended card");

        assert_eq!(card.name, "Extended Card Agent");
        assert_eq!(card.version, "1.2.3");
        assert_eq!(card.capabilities.extended_agent_card, Some(true));
    }

    #[tokio::test]
    async fn get_authenticated_extended_card_rejects_when_not_supported() {
        use crate::A2AClient;
        use crate::a2a_types::GetExtendedAgentCardRequest;

        let server = A2AServerBuilder::new()
            .with_agent_card(agent_card_with_extended(false))
            .with_default_streaming_task_handler()
            .build()
            .await
            .expect("server builds");

        let addr = spawn_test_server(server).await;
        let client = A2AClient::new(format!("http://{addr}")).expect("client");

        let err = client
            .get_authenticated_extended_card(GetExtendedAgentCardRequest {
                tenant: Some("tests".to_string()),
            })
            .await
            .expect_err("expected UNSUPPORTED_OPERATION when extended card disabled");
        let message = err.to_string();
        assert!(
            message.contains("not supported") || message.contains("-32004"),
            "expected UNSUPPORTED_OPERATION, got: {message}"
        );
    }

    #[tokio::test]
    async fn get_authenticated_extended_card_error_contract() {
        use crate::A2AClient;
        use crate::a2a_types::GetExtendedAgentCardRequest;

        let cases = [
            ("flag_absent_or_false", false, false, Some(-32004_i64)),
            ("flag_true_no_extended_card", true, false, Some(-32007)),
            ("flag_true_with_extended_card", true, true, None),
        ];

        for (name, supports, with_extended, expected) in cases {
            let mut builder = A2AServerBuilder::new()
                .with_agent_card(agent_card_with_extended(supports))
                .with_default_streaming_task_handler();
            if with_extended {
                let mut ext = agent_card_with_extended(true);
                ext.name = "Extended Only".to_string();
                builder = builder.with_extended_agent_card(ext);
            }
            let server = builder.build().await.expect("server builds");
            let addr = spawn_test_server(server).await;
            let client = A2AClient::new(format!("http://{addr}")).expect("client");

            let result = client
                .get_authenticated_extended_card(GetExtendedAgentCardRequest {
                    tenant: Some("tests".to_string()),
                })
                .await;

            match expected {
                Some(code) => {
                    let message = result
                        .err()
                        .unwrap_or_else(|| panic!("{name}: expected error {code}"))
                        .to_string();
                    assert!(
                        message.contains(&code.to_string()),
                        "{name}: expected error {code}, got: {message}"
                    );
                }
                None => {
                    let card = result.unwrap_or_else(|e| panic!("{name}: expected card, got {e}"));
                    assert_eq!(card.name, "Extended Only", "{name}: wrong card returned");
                }
            }
        }
    }

    // ----- Push notification config ----------------------

    fn agent_card_with_push_notifications(supports: bool) -> AgentCard {
        serde_json::from_value(serde_json::json!({
            "name": "Push Config Agent",
            "description": "Push notification capability test",
            "version": "1.0.0",
            "supportedInterfaces": [{"url": "http://localhost/a2a", "protocolBinding": "JSONRPC", "protocolVersion": "1.0"}],
            "capabilities": {"streaming": true, "pushNotifications": supports},
            "defaultInputModes": ["text/plain"],
            "defaultOutputModes": ["text/plain"],
            "skills": [{"id": "x", "name": "x", "description": "x", "tags": ["x"]}]
        }))
        .expect("agent card builds")
    }

    #[tokio::test]
    async fn push_config_methods_rejected_when_card_disables_push_notifications() {
        let methods = [
            "CreateTaskPushNotificationConfig",
            "GetTaskPushNotificationConfig",
            "ListTaskPushNotificationConfigs",
            "DeleteTaskPushNotificationConfig",
        ];

        for supports in [false, true] {
            let server = A2AServerBuilder::new()
                .with_agent_card(agent_card_with_push_notifications(supports))
                .with_default_streaming_task_handler()
                .build()
                .await
                .expect("server builds");
            let addr = spawn_test_server(server).await;
            let client = reqwest::Client::new();

            for method in methods {
                let response: Value = client
                    .post(format!("http://{addr}/a2a"))
                    .json(&serde_json::json!({
                        "jsonrpc": "2.0",
                        "id": method,
                        "method": method,
                        "params": {"taskId": "task-1", "id": "cfg-1", "url": "http://localhost/hook"},
                    }))
                    .send()
                    .await
                    .expect("request sent")
                    .json()
                    .await
                    .expect("json body");

                if supports {
                    assert_ne!(
                        response["error"]["code"], -32003,
                        "{method}: push notifications enabled, got {response}"
                    );
                    continue;
                }
                assert_eq!(response["error"]["code"], -32003, "{method}: {response}");
                assert_eq!(
                    response["error"]["data"][0]["@type"],
                    "type.googleapis.com/google.rpc.ErrorInfo"
                );
                assert_eq!(
                    response["error"]["data"][0]["reason"],
                    "PUSH_NOTIFICATION_NOT_SUPPORTED"
                );
                assert_eq!(response["error"]["data"][0]["domain"], "a2a-protocol.org");
            }
        }
    }

    // ----- SendMessage semantics (A2A spec 3.2.2) --------------------

    /// Settles a task by messageId prefix so the blocking-`SendMessage` tests can
    /// drive each state, and answers `direct-*` messages with a `Message`.
    #[derive(Debug)]
    struct SettlingHandler;

    #[async_trait::async_trait]
    impl TaskHandler for SettlingHandler {
        async fn handle_task(&self, mut task: Task, message: Option<A2AMessage>) -> Result<Task> {
            let message_id = message.map(|m| m.message_id).unwrap_or_default();
            let state = if message_id.starts_with("input-") {
                TaskState::TaskStateInputRequired
            } else {
                TaskState::TaskStateCompleted
            };
            task.status = TaskStatus::now(state, Some(build_agent_text_message(&task, "handled")));
            Ok(task)
        }

        async fn handle_message(&self, message: &A2AMessage) -> Result<Option<A2AMessage>> {
            if !message.message_id.starts_with("direct-") {
                return Ok(None);
            }
            let mut reply = message.clone();
            reply.role = Role::RoleAgent;
            reply.message_id = uuid::Uuid::new_v4().to_string();
            reply.parts = vec![Part {
                text: Some("Direct message response".to_string()),
                ..Default::default()
            }];
            Ok(Some(reply))
        }
    }

    /// Background handlers require a card with streaming disabled (see
    /// `A2AServerBuilder::build`); push notifications stay on for the inline-config case.
    fn agent_card_for_background_handler() -> AgentCard {
        serde_json::from_value(serde_json::json!({
            "name": "SendMessage Agent",
            "description": "Blocking SendMessage test",
            "version": "1.0.0",
            "supportedInterfaces": [{"url": "http://localhost/a2a", "protocolBinding": "JSONRPC", "protocolVersion": "1.0"}],
            "capabilities": {"streaming": false, "pushNotifications": true},
            "defaultInputModes": ["text/plain"],
            "defaultOutputModes": ["text/plain"],
            "skills": [{"id": "x", "name": "x", "description": "x", "tags": ["x"]}]
        }))
        .expect("agent card builds")
    }

    async fn spawn_settling_server() -> (
        std::net::SocketAddr,
        Arc<dyn crate::server::storage::Storage>,
        crate::server::task_manager::TaskManagerRunner,
    ) {
        let mut server = A2AServerBuilder::new()
            .with_agent_card(agent_card_for_background_handler())
            .with_background_task_handler(SettlingHandler)
            .build()
            .await
            .expect("server builds");
        let runner = server
            .task_manager
            .take()
            .expect("task manager configured")
            .start();
        let storage = server.storage();
        (spawn_test_server(server).await, storage, runner)
    }

    async fn send_message(addr: std::net::SocketAddr, params: Value) -> Value {
        rpc(addr, "SendMessage", params).await
    }

    async fn rpc(addr: std::net::SocketAddr, method: &str, params: Value) -> Value {
        reqwest::Client::new()
            .post(format!("http://{addr}/a2a"))
            .json(&json!({
                "jsonrpc": "2.0",
                "id": "rpc-1",
                "method": method,
                "params": params,
            }))
            .send()
            .await
            .expect("request sent")
            .json()
            .await
            .expect("json body")
    }

    #[tokio::test]
    async fn send_message_waits_for_the_task_to_settle_and_continues_it() {
        let (addr, storage, runner) = spawn_settling_server().await;

        let interrupted = send_message(
            addr,
            json!({
                "message": {
                    "messageId": "input-1",
                    "role": "ROLE_USER",
                    "parts": [{"text": "need input"}],
                },
                "configuration": {
                    "taskPushNotificationConfig": {"url": "http://localhost/hook"},
                },
            }),
        )
        .await;
        let task = &interrupted["result"]["task"];
        assert_eq!(task["status"]["state"], "TASK_STATE_INPUT_REQUIRED");
        let task_id = task["id"].as_str().expect("task id").to_string();
        assert_eq!(
            storage.list_push_notification_configs(&task_id).await.len(),
            1,
            "inline taskPushNotificationConfig should be registered against the task",
        );

        let completed = send_message(
            addr,
            json!({
                "message": {
                    "messageId": "answer-1",
                    "role": "ROLE_USER",
                    "parts": [{"text": "here it is"}],
                    "taskId": task_id,
                },
                "configuration": {"historyLength": 1},
            }),
        )
        .await;
        let task = &completed["result"]["task"];
        assert_eq!(
            task["id"], task_id,
            "an existing taskId continues that task"
        );
        assert_eq!(task["status"]["state"], "TASK_STATE_COMPLETED");
        assert_eq!(
            task["history"].as_array().expect("history").len(),
            1,
            "historyLength caps the returned history",
        );

        runner.shutdown().await;
    }

    #[tokio::test]
    async fn send_message_returns_immediately_when_asked() {
        let (addr, _storage, runner) = spawn_settling_server().await;

        let response = send_message(
            addr,
            json!({
                "message": {
                    "messageId": "input-2",
                    "role": "ROLE_USER",
                    "parts": [{"text": "need input"}],
                },
                "configuration": {"returnImmediately": true},
            }),
        )
        .await;

        assert_eq!(
            response["result"]["task"]["status"]["state"],
            "TASK_STATE_SUBMITTED"
        );
        runner.shutdown().await;
    }

    #[tokio::test]
    async fn delete_push_config_is_idempotent() {
        let (addr, _storage, runner) = spawn_settling_server().await;

        let created = send_message(
            addr,
            json!({
                "message": {
                    "messageId": "input-4",
                    "role": "ROLE_USER",
                    "parts": [{"text": "need input"}],
                },
                "configuration": {
                    "taskPushNotificationConfig": {"id": "hook-1", "url": "http://localhost/hook"},
                },
            }),
        )
        .await;
        let task_id = created["result"]["task"]["id"]
            .as_str()
            .expect("task id")
            .to_string();

        let params = json!({"taskId": task_id, "id": "hook-1"});
        for attempt in 1..=2 {
            let response = rpc(addr, "DeleteTaskPushNotificationConfig", params.clone()).await;
            assert!(
                response["error"].is_null(),
                "delete attempt {attempt} must succeed: {response}"
            );
        }

        runner.shutdown().await;
    }

    #[tokio::test]
    async fn send_message_rejects_terminal_tasks_and_mismatching_contexts() {
        let (addr, _storage, runner) = spawn_settling_server().await;

        let interrupted = send_message(
            addr,
            json!({
                "message": {
                    "messageId": "input-3",
                    "role": "ROLE_USER",
                    "parts": [{"text": "need input"}],
                },
            }),
        )
        .await;
        let task_id = interrupted["result"]["task"]["id"]
            .as_str()
            .expect("task id")
            .to_string();

        let mismatched = send_message(
            addr,
            json!({
                "message": {
                    "messageId": "answer-3",
                    "role": "ROLE_USER",
                    "parts": [{"text": "wrong context"}],
                    "taskId": task_id,
                    "contextId": "not-the-tasks-context",
                },
            }),
        )
        .await;
        assert_eq!(
            mismatched["error"]["code"],
            jsonrpc_errors::INVALID_PARAMS,
            "a mismatching contextId is rejected: {mismatched}"
        );

        let completed = send_message(
            addr,
            json!({
                "message": {
                    "messageId": "answer-4",
                    "role": "ROLE_USER",
                    "parts": [{"text": "here it is"}],
                    "taskId": task_id,
                },
            }),
        )
        .await;
        assert_eq!(
            completed["result"]["task"]["status"]["state"],
            "TASK_STATE_COMPLETED"
        );

        let after_terminal = send_message(
            addr,
            json!({
                "message": {
                    "messageId": "answer-5",
                    "role": "ROLE_USER",
                    "parts": [{"text": "too late"}],
                    "taskId": task_id,
                },
            }),
        )
        .await;
        assert_eq!(
            after_terminal["error"]["code"],
            jsonrpc_errors::UNSUPPORTED_OPERATION,
            "a terminal task accepts no more messages: {after_terminal}"
        );

        runner.shutdown().await;
    }

    #[tokio::test]
    async fn send_message_can_answer_with_a_direct_message() {
        let (addr, _storage, runner) = spawn_settling_server().await;

        let response = send_message(
            addr,
            json!({
                "message": {
                    "messageId": "direct-1",
                    "role": "ROLE_USER",
                    "parts": [{"text": "hi"}],
                },
            }),
        )
        .await;

        assert!(
            response["result"]["task"].is_null(),
            "a direct message reply carries no task: {response}"
        );
        assert_eq!(
            response["result"]["message"]["parts"][0]["text"],
            "Direct message response"
        );
        runner.shutdown().await;
    }

    #[test]
    fn unsupported_content_type_accepts_json_and_rejects_the_rest() {
        let cases = [
            (None, None),
            (Some("application/json"), None),
            (Some("application/json; charset=utf-8"), None),
            (Some("application/vnd.a2a+json"), None),
            (Some("text/plain"), Some("text/plain")),
            (Some("application/xml"), Some("application/xml")),
        ];

        for (header, expected) in cases {
            let mut headers = axum::http::HeaderMap::new();
            if let Some(value) = header {
                headers.insert(
                    axum::http::header::CONTENT_TYPE,
                    value.parse().expect("header value"),
                );
            }
            assert_eq!(
                unsupported_content_type(&headers).as_deref(),
                expected,
                "Content-Type: {header:?}"
            );
        }
    }

    #[tokio::test]
    async fn wrong_content_type_answers_with_a_json_rpc_error() {
        let server = A2AServerBuilder::new()
            .with_agent_card(agent_card_with_push_notifications(true))
            .with_default_streaming_task_handler()
            .build()
            .await
            .expect("server builds");
        let addr = spawn_test_server(server).await;

        let response = reqwest::Client::new()
            .post(format!("http://{addr}/a2a"))
            .header("Content-Type", "text/plain")
            .body(r#"{"jsonrpc":"2.0","id":1,"method":"GetTask","params":{"id":"x"}}"#)
            .send()
            .await
            .expect("request sent");

        assert_eq!(response.status(), axum::http::StatusCode::OK);
        let body: Value = response.json().await.expect("json body");
        assert_eq!(body["error"]["code"], -32005, "{body}");
        assert_eq!(
            body["error"]["data"][0]["reason"],
            "CONTENT_TYPE_NOT_SUPPORTED"
        );
    }
}

use axum::response::Json;
use serde_json::Value;

/// JSON-RPC standard error codes plus A2A-specific extensions.
pub(super) mod jsonrpc_errors {
    /// Server received invalid JSON. The default axum extractor rejects
    /// malformed JSON before it reaches our handler, but the constant is
    /// kept for parity with the spec and for future custom parsers.
    #[allow(dead_code)]
    pub const PARSE_ERROR: i64 = -32700;
    pub const INVALID_REQUEST: i64 = -32600;
    pub const METHOD_NOT_FOUND: i64 = -32601;
    pub const INVALID_PARAMS: i64 = -32602;
    pub const INTERNAL_ERROR: i64 = -32603;

    /// Task not found.
    pub const TASK_NOT_FOUND: i64 = -32001;
    /// Task cannot be cancelled in its current state.
    pub const TASK_NOT_CANCELABLE: i64 = -32002;
    /// Push notifications are not supported by this agent.
    #[allow(dead_code)]
    pub const PUSH_NOTIFICATION_NOT_SUPPORTED: i64 = -32003;
}

pub(super) fn json_rpc_success(id: Value, result: Value) -> Json<Value> {
    Json(serde_json::json!({
        "jsonrpc": "2.0",
        "id": id,
        "result": result,
    }))
}

pub(super) fn json_rpc_error(
    id: Value,
    code: i64,
    message: &str,
    data: Option<Value>,
) -> Json<Value> {
    let mut err = serde_json::json!({
        "code": code,
        "message": message,
    });
    if let Some(d) = data {
        err["data"] = d;
    }
    Json(serde_json::json!({
        "jsonrpc": "2.0",
        "id": id,
        "error": err,
    }))
}

pub(super) fn invalid_params(id: Value, e: serde_json::Error) -> Json<Value> {
    json_rpc_error(
        id,
        jsonrpc_errors::INVALID_PARAMS,
        "Invalid params",
        Some(Value::String(e.to_string())),
    )
}

pub(super) fn invalid_params_message(id: Value, detail: impl Into<String>) -> Json<Value> {
    json_rpc_error(
        id,
        jsonrpc_errors::INVALID_PARAMS,
        "Invalid params",
        Some(Value::String(detail.into())),
    )
}

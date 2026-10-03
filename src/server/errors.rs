use axum::response::Json;
use serde_json::Value;

/// JSON-RPC standard error codes plus A2A-specific extensions.
pub(super) mod jsonrpc_errors {
    pub const INVALID_REQUEST: i64 = -32600;
    pub const METHOD_NOT_FOUND: i64 = -32601;
    pub const INVALID_PARAMS: i64 = -32602;
    pub const INTERNAL_ERROR: i64 = -32603;

    /// Task not found.
    pub const TASK_NOT_FOUND: i64 = -32001;
    /// Task cannot be cancelled in its current state.
    pub const TASK_NOT_CANCELABLE: i64 = -32002;
    /// The requested operation is not supported by this agent (A2A spec 8.2).
    pub const UNSUPPORTED_OPERATION: i64 = -32004;
    /// The agent supports an extended card but none is configured (A2A spec 8.2).
    pub const AUTHENTICATED_EXTENDED_CARD_NOT_CONFIGURED: i64 = -32007;
    /// The requested `A2A-Version` is not supported (A2A spec 3.6).
    pub const VERSION_NOT_SUPPORTED: i64 = -32009;
}

/// The `google.rpc.ErrorInfo` reason of an A2A error code (A2A spec 5.4), or `None`
/// for plain JSON-RPC codes.
fn a2a_error_reason(code: i64) -> Option<&'static str> {
    match code {
        jsonrpc_errors::TASK_NOT_FOUND => Some("TASK_NOT_FOUND"),
        jsonrpc_errors::TASK_NOT_CANCELABLE => Some("TASK_NOT_CANCELABLE"),
        jsonrpc_errors::UNSUPPORTED_OPERATION => Some("UNSUPPORTED_OPERATION"),
        jsonrpc_errors::AUTHENTICATED_EXTENDED_CARD_NOT_CONFIGURED => {
            Some("EXTENDED_AGENT_CARD_NOT_CONFIGURED")
        }
        jsonrpc_errors::VERSION_NOT_SUPPORTED => Some("VERSION_NOT_SUPPORTED"),
        _ => None,
    }
}

/// A2A errors carry their details as a `google.rpc.ErrorInfo` array (A2A spec 9.5);
/// a string detail moves into the ErrorInfo metadata.
fn a2a_error_details(code: i64, detail: Option<&Value>) -> Option<Value> {
    let reason = a2a_error_reason(code)?;
    let mut info = serde_json::json!({
        "@type": "type.googleapis.com/google.rpc.ErrorInfo",
        "reason": reason,
        "domain": "a2a-protocol.org",
    });
    if let Some(Value::String(detail)) = detail {
        info["metadata"] = serde_json::json!({ "detail": detail });
    }
    Some(Value::Array(vec![info]))
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
    if let Some(d) = a2a_error_details(code, data.as_ref()).or(data) {
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

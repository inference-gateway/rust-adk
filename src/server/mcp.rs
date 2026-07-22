//! Optional MCP (Model Context Protocol) client, mirroring the Go ADK.
//!
//! Rather than registering every discovered MCP tool into the LLM context,
//! this wires just two *selector* tools onto the agent regardless of how many
//! tools the configured servers expose:
//!
//! - `mcp_list_tools` - list the discovered catalog (server, name, description,
//!   input schema), optionally filtered by a `search` substring.
//! - `mcp_call_tool` - invoke a tool by `name` (optionally disambiguated by
//!   `server`) with an `arguments` object.
//!
//! Catalogs are fetched in the background and refreshed on an interval, so
//! `mcp_list_tools` answers from an in-memory snapshot. A server that fails to
//! connect or refresh never drops the catalog of the healthy ones. Only the
//! Streamable HTTP transport is supported (the cloud-native case).
//!
//! Build one from [`McpConfig`] and attach it to an agent:
//!
//! ```no_run
//! # use inference_gateway_adk::{AgentBuilder, McpClient, McpConfig};
//! # async fn wire(config: &McpConfig) -> anyhow::Result<()> {
//! let mut builder = AgentBuilder::new().with_provider("openai").with_model("gpt-4o");
//! if let Some(client) = McpClient::from_config(config) {
//!     client.start();
//!     builder = builder.with_mcp_client(client);
//! }
//! # let _ = builder; Ok(())
//! # }
//! ```

use crate::config::McpConfig;
use anyhow::{Result, anyhow};
use inference_gateway_sdk::{
    ChatCompletionTool, ChatCompletionToolType, FunctionObject, FunctionParameters,
};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{Mutex, RwLock};
use tokio::time::sleep;
use tracing::{debug, warn};

/// Latest MCP protocol revision we advertise in `initialize`. Servers reply
/// with the version they actually speak; we echo that back on later requests.
const PROTOCOL_VERSION: &str = "2025-06-18";

/// A single tool discovered from an MCP server.
#[derive(Clone, Debug, serde::Serialize)]
pub struct DiscoveredTool {
    /// Base URL of the MCP server that exposes this tool.
    pub server: String,
    pub name: String,
    pub description: String,
    /// The tool's JSON-Schema input definition, verbatim from `tools/list`.
    pub input_schema: Value,
}

/// Per-server connection state. `url` is the full Streamable-HTTP endpoint we
/// POST to; `base` is what the LLM sees / disambiguates on.
#[derive(Debug)]
struct Server {
    base: String,
    url: String,
    /// `Mcp-Session-Id` handed back by the server on `initialize`, if any.
    /// Stateless servers leave this `None` and get re-initialized each cycle.
    session_id: Mutex<Option<String>>,
    protocol_version: Mutex<Option<String>>,
}

/// Streamable-HTTP MCP client holding a background-refreshed tool catalog.
#[derive(Debug)]
pub struct McpClient {
    http: reqwest::Client,
    servers: Vec<Arc<Server>>,
    /// server base URL -> its last successfully fetched tools.
    catalog: RwLock<HashMap<String, Vec<DiscoveredTool>>>,
    config: McpConfig,
}

impl McpClient {
    /// Build a client from [`McpConfig`], or `None` when MCP is disabled or no
    /// servers are configured (so callers can gate registration on the result).
    pub fn from_config(config: &McpConfig) -> Option<Arc<Self>> {
        if !config.enable {
            return None;
        }
        let bases: Vec<&str> = config
            .servers
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .collect();
        if bases.is_empty() {
            warn!("MCP_ENABLE is set but MCP_SERVERS is empty; MCP client not started");
            return None;
        }

        let servers = bases
            .into_iter()
            .map(|base| {
                Arc::new(Server {
                    url: join_endpoint(base, &config.endpoint),
                    base: base.to_string(),
                    session_id: Mutex::new(None),
                    protocol_version: Mutex::new(None),
                })
            })
            .collect();

        Some(Arc::new(Self {
            http: reqwest::Client::new(),
            servers,
            catalog: RwLock::new(HashMap::new()),
            config: config.clone(),
        }))
    }

    /// Spawn one background task per server that connects (with retry backoff)
    /// and then refreshes the tool catalog on the configured interval. Returns
    /// immediately; discovery happens asynchronously.
    pub fn start(self: &Arc<Self>) {
        for server in &self.servers {
            let this = Arc::clone(self);
            let server = Arc::clone(server);
            tokio::spawn(async move { this.run_server(server).await });
        }
    }

    /// The two selector tool definitions to register into the agent toolbox.
    pub fn selector_tools() -> Vec<ChatCompletionTool> {
        vec![
            tool(
                "mcp_list_tools",
                "List tools discovered from the configured MCP servers. Optionally pass \
                 `search` to filter by a case-insensitive substring of the tool name or \
                 description. Returns each tool's server, name, description and input schema.",
                json!({
                    "type": "object",
                    "properties": {
                        "search": {
                            "type": "string",
                            "description": "Case-insensitive substring filter over name and description"
                        }
                    }
                }),
            ),
            tool(
                "mcp_call_tool",
                "Invoke a tool discovered via mcp_list_tools. Pass the tool `name` and an \
                 `arguments` object matching its input schema. When the same name exists on \
                 multiple servers, disambiguate with `server`.",
                json!({
                    "type": "object",
                    "properties": {
                        "name": {"type": "string", "description": "Tool name from mcp_list_tools"},
                        "server": {"type": "string", "description": "Optional MCP server base URL to disambiguate"},
                        "arguments": {"type": "object", "description": "Arguments matching the tool's input schema"}
                    },
                    "required": ["name"]
                }),
            ),
        ]
    }

    /// Handler for the `mcp_list_tools` selector tool.
    pub async fn handle_list(&self, args: Value) -> Result<String> {
        let search = args
            .get("search")
            .and_then(Value::as_str)
            .map(str::to_lowercase);
        let catalog = self.catalog.read().await;
        let mut tools: Vec<&DiscoveredTool> = catalog.values().flatten().collect();
        if let Some(q) = &search {
            tools.retain(|t| {
                t.name.to_lowercase().contains(q) || t.description.to_lowercase().contains(q)
            });
        }
        tools.sort_by(|a, b| a.name.cmp(&b.name));
        let list: Vec<Value> = tools
            .iter()
            .map(|t| {
                json!({
                    "server": t.server,
                    "name": t.name,
                    "description": t.description,
                    "input_schema": t.input_schema,
                })
            })
            .collect();
        Ok(json!({ "count": list.len(), "tools": list }).to_string())
    }

    /// Handler for the `mcp_call_tool` selector tool.
    pub async fn handle_call(&self, args: Value) -> Result<String> {
        let name = args
            .get("name")
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow!("mcp_call_tool requires a string `name`"))?;
        let server_hint = args.get("server").and_then(Value::as_str);
        let arguments = args.get("arguments").cloned().unwrap_or_else(|| json!({}));

        // Resolve which server hosts the tool from the discovered catalog.
        let base = {
            let catalog = self.catalog.read().await;
            catalog
                .values()
                .flatten()
                .find(|t| t.name == name && server_hint.is_none_or(|s| s == t.server))
                .map(|t| t.server.clone())
        }
        .ok_or_else(|| match server_hint {
            Some(s) => anyhow!("no MCP tool named `{name}` on server `{s}`"),
            None => anyhow!("no MCP tool named `{name}` was discovered"),
        })?;

        let server = self
            .servers
            .iter()
            .find(|s| s.base == base)
            .ok_or_else(|| anyhow!("MCP server `{base}` is not configured"))?
            .clone();

        self.ensure_initialized(&server).await?;
        let req = json!({
            "jsonrpc": "2.0",
            "id": 3,
            "method": "tools/call",
            "params": {"name": name, "arguments": arguments},
        });
        let result = self.rpc(&server, req, self.config.call_timeout).await?;
        Ok(content_to_string(&result))
    }

    /// Connect-then-refresh loop for one server. The initial connect honours
    /// `max_retries` (0 = forever); once connected, refresh failures back off
    /// and retry indefinitely while keeping the last good catalog.
    async fn run_server(self: Arc<Self>, server: Arc<Server>) {
        let mut backoff = self.config.retry_interval;
        let mut connected = false;
        let mut attempts = 0u32;
        let mut delay = Duration::ZERO;

        loop {
            if !delay.is_zero() {
                sleep(delay).await;
            }
            match self.refresh_server(&server).await {
                Ok(n) => {
                    if !connected {
                        debug!("mcp: connected to {}, {n} tools", server.base);
                        connected = true;
                    } else {
                        debug!("mcp: refreshed {}, {n} tools", server.base);
                    }
                    backoff = self.config.retry_interval;
                    delay = self.config.refresh_interval;
                }
                Err(e) => {
                    attempts += 1;
                    if !connected
                        && self.config.max_retries != 0
                        && attempts >= self.config.max_retries
                    {
                        warn!(
                            "mcp: giving up on {} after {attempts} attempts: {e}",
                            server.base
                        );
                        return;
                    }
                    warn!("mcp: {} unavailable, backing off: {e}", server.base);
                    // On a failed refresh the previous catalog is untouched.
                    delay = backoff;
                    backoff = (backoff * 2).min(self.config.retry_max_interval);
                }
            }
        }
    }

    /// Initialize (if needed) and pull the full tool list into the catalog.
    async fn refresh_server(&self, server: &Server) -> Result<usize> {
        self.ensure_initialized(server).await?;
        let tools = self.list_tools_remote(server).await?;
        let n = tools.len();
        self.catalog
            .write()
            .await
            .insert(server.base.clone(), tools);
        Ok(n)
    }

    /// Perform the MCP `initialize` handshake unless a session already exists.
    async fn ensure_initialized(&self, server: &Server) -> Result<()> {
        if server.session_id.lock().await.is_some() {
            return Ok(());
        }
        let init = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "initialize",
            "params": {
                "protocolVersion": PROTOCOL_VERSION,
                "capabilities": {},
                "clientInfo": {"name": "inference-gateway-adk", "version": env!("CARGO_PKG_VERSION")},
            },
        });
        let result = self.rpc(server, init, self.config.dial_timeout).await?;
        if let Some(pv) = result.get("protocolVersion").and_then(Value::as_str) {
            *server.protocol_version.lock().await = Some(pv.to_string());
        }
        // Best-effort `initialized` notification (servers accept 202/empty).
        let note = json!({"jsonrpc": "2.0", "method": "notifications/initialized"});
        let _ = self.rpc(server, note, self.config.dial_timeout).await;
        Ok(())
    }

    /// Page through `tools/list` and map results into [`DiscoveredTool`]s.
    async fn list_tools_remote(&self, server: &Server) -> Result<Vec<DiscoveredTool>> {
        let mut out = Vec::new();
        let mut cursor: Option<String> = None;
        loop {
            let mut params = json!({});
            if let Some(c) = &cursor {
                params["cursor"] = json!(c);
            }
            let req = json!({"jsonrpc": "2.0", "id": 2, "method": "tools/list", "params": params});
            let result = self.rpc(server, req, self.config.dial_timeout).await?;
            for t in result
                .get("tools")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
            {
                out.push(DiscoveredTool {
                    server: server.base.clone(),
                    name: t
                        .get("name")
                        .and_then(Value::as_str)
                        .unwrap_or_default()
                        .to_string(),
                    description: t
                        .get("description")
                        .and_then(Value::as_str)
                        .unwrap_or_default()
                        .to_string(),
                    input_schema: t.get("inputSchema").cloned().unwrap_or_else(|| json!({})),
                });
            }
            match result.get("nextCursor").and_then(Value::as_str) {
                Some(c) if !c.is_empty() => cursor = Some(c.to_string()),
                _ => break,
            }
        }
        Ok(out)
    }

    /// Send one JSON-RPC message over Streamable HTTP and return its `result`
    /// (or `Null` for accepted notifications). Handles both `application/json`
    /// and `text/event-stream` responses and tracks the session id header.
    async fn rpc(&self, server: &Server, body: Value, timeout: Duration) -> Result<Value> {
        let mut req = self
            .http
            .post(&server.url)
            .header("content-type", "application/json")
            .header("accept", "application/json, text/event-stream")
            .timeout(timeout)
            .json(&body);
        if let Some(s) = server.session_id.lock().await.as_ref() {
            req = req.header("mcp-session-id", s);
        }
        if let Some(p) = server.protocol_version.lock().await.as_ref() {
            req = req.header("mcp-protocol-version", p);
        }

        let resp = req.send().await?;
        if let Some(sid) = resp
            .headers()
            .get("mcp-session-id")
            .and_then(|v| v.to_str().ok())
        {
            *server.session_id.lock().await = Some(sid.to_string());
        }
        let status = resp.status();
        let ctype = resp
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or_default()
            .to_string();
        let text = resp.text().await?;

        if !status.is_success() {
            // A dropped/expired session: forget it so the next cycle re-inits.
            if status.as_u16() == 404 {
                *server.session_id.lock().await = None;
            }
            return Err(anyhow!("mcp http {status}: {}", text.trim()));
        }

        let msg = if ctype.contains("text/event-stream") {
            parse_sse(&text)?
        } else if text.trim().is_empty() {
            return Ok(Value::Null); // notification ack
        } else {
            serde_json::from_str(&text)?
        };

        if let Some(err) = msg.get("error") {
            return Err(anyhow!("mcp rpc error: {err}"));
        }
        Ok(msg.get("result").cloned().unwrap_or(Value::Null))
    }
}

/// Join a server base URL with the endpoint path, tolerating trailing/leading
/// slashes (`http://h:3000` + `/mcp` -> `http://h:3000/mcp`).
fn join_endpoint(base: &str, endpoint: &str) -> String {
    let base = base.trim_end_matches('/');
    if endpoint.is_empty() {
        return base.to_string();
    }
    format!("{base}/{}", endpoint.trim_start_matches('/'))
}

/// Extract the JSON-RPC message from a Streamable-HTTP SSE body: accumulate
/// `data:` lines per event and return the first that parses as a JSON-RPC
/// object.
fn parse_sse(text: &str) -> Result<Value> {
    let mut data = String::new();
    let flush = |data: &str| -> Option<Value> {
        serde_json::from_str::<Value>(data.trim())
            .ok()
            .filter(|v| v.get("jsonrpc").is_some())
    };
    for line in text.lines() {
        if let Some(rest) = line.strip_prefix("data:") {
            if !data.is_empty() {
                data.push('\n');
            }
            data.push_str(rest.strip_prefix(' ').unwrap_or(rest));
        } else if line.is_empty() {
            if let Some(v) = flush(&data) {
                return Ok(v);
            }
            data.clear();
        }
    }
    flush(&data).ok_or_else(|| anyhow!("no JSON-RPC message in SSE response"))
}

/// Flatten a `tools/call` result's `content` array into text. Non-text content
/// (or an absent array) falls back to the raw JSON.
fn content_to_string(result: &Value) -> String {
    let text = result
        .get("content")
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(|it| it.get("text").and_then(Value::as_str))
                .collect::<Vec<_>>()
                .join("\n")
        })
        .unwrap_or_default();

    if text.is_empty() {
        return result.to_string();
    }
    if result
        .get("isError")
        .and_then(Value::as_bool)
        .unwrap_or(false)
    {
        return format!("tool error: {text}");
    }
    text
}

fn tool(name: &str, description: &str, params: Value) -> ChatCompletionTool {
    ChatCompletionTool {
        type_: ChatCompletionToolType::Function,
        function: FunctionObject {
            name: name.to_string(),
            description: Some(description.to_string()),
            parameters: Some(FunctionParameters(
                params
                    .as_object()
                    .expect("tool schema is an object")
                    .clone(),
            )),
            strict: false,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{Json, Router, extract::State, http::HeaderMap, routing::post};
    use std::net::SocketAddr;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn join_endpoint_normalizes_slashes() {
        assert_eq!(join_endpoint("http://h:3000", "/mcp"), "http://h:3000/mcp");
        assert_eq!(join_endpoint("http://h:3000/", "mcp"), "http://h:3000/mcp");
        assert_eq!(
            join_endpoint("http://h:3000/", "/mcp/"),
            "http://h:3000/mcp/"
        );
    }

    #[test]
    fn parse_sse_extracts_jsonrpc_message() {
        let body =
            "event: message\ndata: {\"jsonrpc\":\"2.0\",\"id\":1,\"result\":{\"ok\":true}}\n\n";
        let v = parse_sse(body).expect("parses");
        assert_eq!(v["result"]["ok"], json!(true));
    }

    #[test]
    fn content_to_string_joins_text_and_flags_errors() {
        let ok = json!({"content": [{"type": "text", "text": "hello"}, {"type": "text", "text": "world"}]});
        assert_eq!(content_to_string(&ok), "hello\nworld");
        let err = json!({"content": [{"type": "text", "text": "boom"}], "isError": true});
        assert_eq!(content_to_string(&err), "tool error: boom");
        let raw = json!({"structuredContent": {"n": 1}});
        assert_eq!(content_to_string(&raw), raw.to_string());
    }

    /// Minimal Streamable-HTTP MCP server: answers initialize / tools/list /
    /// tools/call with plain `application/json` and no session id (stateless).
    async fn mock_server() -> SocketAddr {
        async fn handler(
            State(calls): State<Arc<AtomicUsize>>,
            _headers: HeaderMap,
            body: String,
        ) -> Json<Value> {
            let req: Value = serde_json::from_str(&body).unwrap_or_else(|_| json!({}));
            let method = req.get("method").and_then(Value::as_str).unwrap_or("");
            let id = req.get("id").cloned().unwrap_or(Value::Null);
            let result = match method {
                "initialize" => json!({"protocolVersion": "2025-06-18", "capabilities": {}}),
                "tools/list" => json!({"tools": [{
                    "name": "echo",
                    "description": "Echo back the input",
                    "inputSchema": {"type": "object", "properties": {"text": {"type": "string"}}}
                }]}),
                "tools/call" => {
                    calls.fetch_add(1, Ordering::SeqCst);
                    let text = req["params"]["arguments"]["text"].as_str().unwrap_or("");
                    json!({"content": [{"type": "text", "text": format!("echo: {text}")}]})
                }
                _ => return Json(json!({"jsonrpc": "2.0", "id": id})), // notification ack
            };
            Json(json!({"jsonrpc": "2.0", "id": id, "result": result}))
        }

        let calls = Arc::new(AtomicUsize::new(0));
        let app = Router::new().route("/mcp", post(handler)).with_state(calls);
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
        addr
    }

    #[tokio::test]
    async fn discovers_and_calls_tools_over_http() {
        let addr = mock_server().await;
        let cfg = McpConfig {
            enable: true,
            servers: format!("http://{addr}"),
            ..Default::default()
        };
        let client = McpClient::from_config(&cfg).expect("client builds");

        // Drive discovery directly (no background loop) for a deterministic test.
        let n = client.refresh_server(&client.servers[0]).await.unwrap();
        assert_eq!(n, 1, "one tool discovered");

        let listed: Value =
            serde_json::from_str(&client.handle_list(json!({})).await.unwrap()).unwrap();
        assert_eq!(listed["count"], json!(1));
        assert_eq!(listed["tools"][0]["name"], json!("echo"));
        assert_eq!(
            listed["tools"][0]["server"],
            json!(format!("http://{addr}"))
        );

        // Search filter that matches nothing.
        let empty: Value =
            serde_json::from_str(&client.handle_list(json!({"search": "nope"})).await.unwrap())
                .unwrap();
        assert_eq!(empty["count"], json!(0));

        let out = client
            .handle_call(json!({"name": "echo", "arguments": {"text": "hi"}}))
            .await
            .unwrap();
        assert_eq!(out, "echo: hi");

        let missing = client.handle_call(json!({"name": "ghost"})).await;
        assert!(missing.is_err(), "unknown tool should error");
    }

    #[test]
    fn from_config_gates_on_enable_and_servers() {
        let disabled = McpConfig {
            enable: false,
            servers: "http://x".to_string(),
            ..Default::default()
        };
        assert!(McpClient::from_config(&disabled).is_none());

        let no_servers = McpConfig {
            enable: true,
            servers: "  ".to_string(),
            ..Default::default()
        };
        assert!(McpClient::from_config(&no_servers).is_none());
    }
}

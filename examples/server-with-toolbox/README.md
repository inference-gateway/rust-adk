# Server With Toolbox Example

A2A server that exposes custom tools to the LLM via `AgentBuilder::with_toolbox()` plus per-tool sync and async handlers.

## Directory Structure

```
server-with-toolbox/
├── agent-card.json   # Agent metadata loaded at startup
├── server/main.rs    # Server registering three tools (weather, math, web search)
├── client/main.rs    # Client exercising non-streaming, streaming, and health checks
└── README.md
```

## Prerequisites

- An Inference Gateway reachable at `http://localhost:8080/v1` (or set `INFERENCE_GATEWAY_URL`).
- LLM provider/model configuration via `Config::from_env()` (see the top-level [README](../../README.md)).

## Running

```bash
# Server (uses the colocated agent-card.json)
cargo run --example server-with-toolbox-server
# or: task examples:server-with-toolbox-server

# Client (in another terminal)
cargo run --example server-with-toolbox-client
# or: task examples:server-with-toolbox-client
```

The server listens on `0.0.0.0:8082`. The client connects to `http://localhost:8082`.

## What This Shows

- **`with_toolbox(tools)`**: register a list of `ChatCompletionTool` schemas with the agent
- **`with_function_tool(name, fn)`**: synchronous tool handler (weather, math)
- **`with_async_function_tool(name, async fn)`**: async tool handler (web search)
- **Streaming + non-streaming task flows** from the client
- **Periodic health monitoring** showing `gateway_healthy` propagation

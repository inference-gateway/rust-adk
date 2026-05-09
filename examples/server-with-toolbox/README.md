# Server With Toolbox Example

A2A server that exposes custom tools to the LLM via `AgentBuilder::with_toolbox()` plus per-tool sync and async handlers.

## Directory Structure

```
server-with-toolbox/
├── docker-compose.yaml  # Server + client + inference-gateway:latest
├── .env.example         # DEEPSEEK_API_KEY + provider/model overrides
├── agent-card.json      # Agent metadata loaded at startup
├── server/main.rs       # Server registering three tools (weather, math, web search)
├── client/main.rs       # Client exercising non-streaming, streaming, and health checks
└── README.md
```

## What This Shows

- **`with_toolbox(tools)`**: register a list of `ChatCompletionTool` schemas with the agent
- **`with_function_tool(name, fn)`**: synchronous tool handler (weather, math)
- **`with_async_function_tool(name, async fn)`**: async tool handler (web search)
- **Periodic health monitoring** showing `gateway_healthy` propagation
- **Streaming flow** from the client — note: the server does not currently
  expose a server-sent-events streaming endpoint, so the client's streaming
  call falls back / errors gracefully. The non-streaming path is the
  recommended one to follow.

## Running with Docker Compose

```bash
cd examples/server-with-toolbox
cp .env.example .env
# Set DEEPSEEK_API_KEY (or another provider's key + matching AGENT_CLIENT_PROVIDER)
docker compose up --build
```

The stack starts three services on a private Docker network:

- `inference-gateway` (image `ghcr.io/inference-gateway/inference-gateway:latest`)
- `server` — built from `examples/Dockerfile.server`, listens on port 8082 inside the network
- `client` — built from `examples/Dockerfile.client`, runs after the server is healthy

Defaults: `AGENT_CLIENT_PROVIDER=deepseek`, `AGENT_CLIENT_MODEL=deepseek-v4-flash`.
Override via `.env` to switch to any other provider supported by the gateway
(`groq`, `google`, `openai`, `anthropic`, `cohere`, `cloudflare`, `ollama`).

## Running locally

```bash
# Start an Inference Gateway separately, then:
cargo run --example server-with-toolbox-server
# or: task examples:server-with-toolbox-server

cargo run --example server-with-toolbox-client
# or: task examples:server-with-toolbox-client
```

The server listens on `0.0.0.0:8082`. The client honours `SERVER_URL`
(default `http://localhost:8082`).

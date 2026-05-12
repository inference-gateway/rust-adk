# ai-powered example

A2A server backed by an LLM agent with three custom tools (`get_current_weather`,
`calculate_math`, `search_web`) registered through `AgentBuilder::with_toolbox()`.
The client sends two prompts via `message/send` and polls each task to terminal.

For the streaming variant of the same server shape, see
[`../ai-powered-streaming`](../ai-powered-streaming).

## What's in the box

```
ai-powered/
├── server/main.rs                 LLM agent + sync/async function tools
├── server/.well-known/agent.json  Agent metadata loaded at startup
├── client/main.rs                 Two-prompt demo via message/send + poll
├── docker-compose.yaml            Server + client + inference-gateway:latest
├── .env.example                   DEEPSEEK_API_KEY + provider/model overrides
└── README.md
```

## What this shows

- **`AgentBuilder::with_toolbox(tools)`** — register a list of
  `ChatCompletionTool` schemas with the agent.
- **`with_function_tool(name, fn)`** — synchronous tool handler
  (weather, math).
- **`with_async_function_tool(name, async fn)`** — async tool handler
  (web search).
- **`with_default_task_handlers()`** — uses the built-in background
  handler, which drives the LLM tool loop and writes the final reply
  onto the task.

## Running with Docker Compose

```bash
cd examples/ai-powered
cp .env.example .env
# Set DEEPSEEK_API_KEY (or another provider's key + matching AGENT_CLIENT_PROVIDER)
docker compose up --build
```

The stack starts three services on a private Docker network:

- `inference-gateway` (image `ghcr.io/inference-gateway/inference-gateway:latest`)
- `server` — built from `examples/Dockerfile.server`, listens on port 8082
- `client` — built from `examples/Dockerfile.client`, runs after the server is healthy

Defaults: `AGENT_CLIENT_PROVIDER=deepseek`, `AGENT_CLIENT_MODEL=deepseek-v4-flash`.
Override via `.env` to switch to any other provider supported by the gateway
(`groq`, `google`, `openai`, `anthropic`, `cohere`, `cloudflare`, `ollama`).

## Running locally

```bash
# Start an Inference Gateway separately, then run the server from inside its
# subdir so .well-known/agent.json resolves correctly:
cd examples/ai-powered/server
cargo run --example ai-powered-server
# or: task examples:ai-powered-server

cargo run --example ai-powered-client
# or: task examples:ai-powered-client
```

The server listens on `0.0.0.0:8082`. The client honours `SERVER_URL`
(default `http://localhost:8082`).

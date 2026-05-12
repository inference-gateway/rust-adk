# ai-powered-streaming example

LLM-backed A2A server that streams delta chunks over `message/stream`.
Uses `A2AServerBuilder::with_default_task_handlers()` — the built-in
`DefaultStreamingTaskHandler` converts the LLM's streaming response
into a sequence of `TaskArtifactUpdateEvent`s ending with
`TaskStateCompleted`.

Compare to:

- [`../streaming`](../streaming) — same streaming wire-up, but the
  chunks come from a hardcoded sentence rather than an LLM.
- [`../ai-powered`](../ai-powered) — same LLM agent shape but uses
  `message/send` + polling instead of streaming.

## What's in the box

```
ai-powered-streaming/
├── server/main.rs                 LLM agent + default streaming handler
├── server/.well-known/agent.json  Agent metadata loaded at startup
├── client/main.rs                 Consumes message/stream + prints per-event timestamps
├── docker-compose.yaml            Server + client + inference-gateway:latest
├── .env.example                   DEEPSEEK_API_KEY + provider/model overrides
└── README.md
```

## What this shows

- **`with_default_task_handlers()`** wires the built-in streaming
  handler, which calls
  `LLMClient::create_streaming_chat_completion(...)` and forwards
  every delta chunk as a `TaskArtifactUpdateEvent`.
- The client opens an SSE stream via `A2AClient::stream_message(...)`
  and prints elapsed wall time for each event, making token cadence
  visible.

## Running with Docker Compose

```bash
cd examples/ai-powered-streaming
cp .env.example .env
# Set DEEPSEEK_API_KEY (or another provider's key + matching AGENT_CLIENT_PROVIDER)
docker compose up --build
```

The stack starts three services:

- `inference-gateway` (image `ghcr.io/inference-gateway/inference-gateway:latest`)
- `server` — built from `examples/Dockerfile.server`, listens on port 8084
- `client` — built from `examples/Dockerfile.client`, runs after the server is healthy

## Running locally

```bash
# Start an Inference Gateway separately, then run the server from inside its
# subdir so .well-known/agent.json resolves correctly:
cd examples/ai-powered-streaming/server
cargo run --example ai-powered-streaming-server
# or: task examples:ai-powered-streaming-server

cargo run --example ai-powered-streaming-client
# or: task examples:ai-powered-streaming-client
```

The server listens on `0.0.0.0:8084`. The client honours `SERVER_URL`
(default `http://localhost:8084`).

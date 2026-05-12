# default-handlers example

Smallest possible "with AI" server: an LLM agent attached, **no custom
`TaskHandler` code**, just `A2AServerBuilder::with_default_task_handlers()`.
The library's built-in handler drives the LLM tool loop and writes the
final reply onto the task.

Compare to:

- [`../minimal`](../minimal) — same builder shape but **no agent**, so
  the default handler returns the built-in `NO_AGENT_REPLY` echo
  instead of an LLM response.
- [`../ai-powered`](../ai-powered) — same builder shape plus a custom
  toolbox (weather / math / search). Use this example when you want to
  ship an LLM-backed agent without writing handler code; use
  `ai-powered/` when you want tools.

## What's in the box

```
default-handlers/
├── server/main.rs                 LLM agent + with_default_task_handlers(), no custom code
├── server/.well-known/agent.json  Agent metadata loaded at startup
├── client/main.rs                 Single message/send + poll demo
├── docker-compose.yaml            Server + client + inference-gateway:latest
├── .env.example                   DEEPSEEK_API_KEY + provider/model overrides
└── README.md
```

## What this shows

- **`A2AServerBuilder::with_default_task_handlers()`** is the
  zero-handler-code path. Combined with `with_agent(...)`, it produces a
  working LLM-driven server in fewer than 40 lines.
- The same default handler returns the built-in echo when no agent is
  attached (see `minimal/`), so this is also the bridge between
  "no AI" and "with AI" — flip the agent in or out without changing
  handler code.

## Running with Docker Compose

```bash
cd examples/default-handlers
cp .env.example .env
# Set DEEPSEEK_API_KEY (or another provider's key + matching AGENT_CLIENT_PROVIDER)
docker compose up --build
```

The stack starts three services on port 8085 (server), with
`inference-gateway` on its own internal port:

- `inference-gateway` (image `ghcr.io/inference-gateway/inference-gateway:latest`)
- `server` — built from `examples/Dockerfile.server`, listens on port 8085
- `client` — built from `examples/Dockerfile.client`, runs after the server is healthy

## Running locally

```bash
# Start an Inference Gateway separately, then run the server from inside its
# subdir so .well-known/agent.json resolves correctly:
cd examples/default-handlers/server
cargo run --example default-handlers-server
# or: task examples:default-handlers-server

cargo run --example default-handlers-client
# or: task examples:default-handlers-client
```

The server listens on `0.0.0.0:8085`. The client honours `SERVER_URL`
(default `http://localhost:8085`).

# usage-metadata example

A2A server backed by an LLM agent that attaches **token usage** and
**execution statistics** to every task it drives to a terminal state
(`completed` / `failed` / `cancelled`). The client sends a prompt that triggers
a tool call, polls the task to terminal, and prints the `usage` +
`execution_stats` blocks it finds on `task.metadata`.

## What's in the box

```
usage-metadata/
├── server/main.rs                 LLM agent + one calculate_sum tool
├── server/.well-known/agent.json  Agent metadata loaded at startup
├── client/main.rs                 message/send + poll + metadata renderer
├── docker-compose.yaml            Server + client + inference-gateway:latest
├── .env.example                   Provider key + ENABLE_USAGE_METADATA toggle
└── README.md
```

## What this shows

- **`AgentConfig::enable_usage_metadata`** (env
  `A2A_AGENT_CLIENT_ENABLE_USAGE_METADATA`, default `true`) - the single knob
  that turns the feature on or off. `A2AServerBuilder` forwards it to the
  bundled task handlers.
- **Terminal-only attachment** - the default handlers merge metadata into
  `task.metadata` *only* on the terminal transition, never mid-flight.
- **The emitted shape** - a `usage` block (summed across the gateway's
  `CompletionUsage` responses) plus an `execution_stats` block counting the
  agent loop:

  ```jsonc
  {
    "usage": { "prompt_tokens": 123, "completion_tokens": 45, "total_tokens": 168 },
    "execution_stats": { "iterations": 2, "messages": 1, "tool_calls": 1, "failed_tools": 0 }
  }
  ```

Both the background (`message/send`) and streaming (`message/stream`) default
handlers attach the same blocks; this example demonstrates the background path
because the client can read it straight off the polled task.

## Running with Docker Compose

```bash
cd examples/usage-metadata
cp .env.example .env
# Set DEEPSEEK_API_KEY (or another provider's key + matching A2A_AGENT_CLIENT_PROVIDER)
docker compose up --build
```

The stack starts three services on a private Docker network:

- `inference-gateway` (image `ghcr.io/inference-gateway/inference-gateway:latest`)
- `server` - built from `examples/Dockerfile.server`, listens on port 8080
- `client` - built from `examples/Dockerfile.client`, runs after the server is healthy

To watch the metadata disappear, set `A2A_AGENT_CLIENT_ENABLE_USAGE_METADATA=false`
in `.env` and re-run - the client then reports that no usage metadata was attached.

## Running locally

```bash
# Start an Inference Gateway separately, then run the server from inside its
# subdir so .well-known/agent.json resolves correctly:
cd examples/usage-metadata/server
cargo run -p usage-metadata-server
# or: task examples:usage-metadata-server

cargo run -p usage-metadata-client
# or: task examples:usage-metadata-client
```

The server listens on `0.0.0.0:8080`. The client honours `SERVER_URL`
(default `http://localhost:8080`).

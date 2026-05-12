# Rust ADK Examples

Self-contained scenarios demonstrating different capabilities of the Rust Agent
Development Kit (ADK).

## Structure

Each example is its own directory with a server, a client, an optional colocated
agent card at `server/.well-known/agent.json` (mirroring the A2A-spec URL the
server exposes), and a README:

```text
examples/
├── minimal/                # Bare-bones server/client without an agent or agent card
├── static-agent-card/      # Loading agent metadata from a JSON file with runtime overrides
├── server-with-toolbox/    # LLM agent with custom sync + async function tools
├── a2a-methods/            # One runnable client per A2A JSON-RPC method
└── queue-storage/          # Queue-driven message/send with in-memory vs Redis Storage
```

## Running

Each scenario exposes two Cargo examples (`<name>-server` and `<name>-client`).
You can run them via Cargo, via the corresponding Taskfile entries, or via the
colocated `docker-compose.yaml`.

### Cargo / Taskfile

```bash
# Cargo
cargo run --example minimal-server
cargo run --example minimal-client

# Taskfile
task examples:minimal-server
task examples:minimal-client
```

> Examples that load `.well-known/agent.json` resolve it relative to the
> current working directory. Run those servers from inside their `server/`
> directory, or pass an absolute path via `with_agent_card_from_file(...)`.

### Docker Compose

Each scenario ships a `docker-compose.yaml` that builds the server and client
containers from the shared `examples/Dockerfile.server` /
`examples/Dockerfile.client` and wires them together on a private network.

```bash
cd examples/<scenario>
cp .env.example .env
# edit .env — set DEEPSEEK_API_KEY (or another provider's key) where applicable
docker compose up --build
```

- `minimal/` runs server + client only — no Inference Gateway, since it has no
  agent and `POST /a2a` is expected to return a JSON-RPC "no agent configured"
  error. The colocated `.env.example` exists for layout consistency; the
  minimal stack doesn't actually consume any provider keys.
- `static-agent-card/` and `server-with-toolbox/` start an
  `inference-gateway:latest` container alongside the server and client.
  Defaults target DeepSeek (`AGENT_CLIENT_PROVIDER=deepseek`,
  `AGENT_CLIENT_MODEL=deepseek-v4-flash`); override via `.env` to use any
  other provider supported by the gateway.
- `a2a-methods/` ships one server plus nine clients (one per JSON-RPC method)
  wired into per-method Compose profiles. Use
  `docker compose --profile <method> up --build` to exercise a single method,
  or `--profile all-clients` to run every client back-to-back against the
  shared server. No provider keys required — the server is offline.

## Available Examples

### `minimal/`

Simplest possible setup: a server with `A2AServerBuilder::new()` and a client
performing health, agent card, and a single task. No agent, no agent card file.

### `static-agent-card/`

Demonstrates `with_agent_card_from_file()` plus `AgentCardOverrides` for
runtime field overrides. Useful for environment-specific configurations.

### `server-with-toolbox/`

Server with an LLM agent registering three tools (`get_current_weather`,
`calculate_math`, `search_web`) — two synchronous and one async — plus a client
that exercises streaming and non-streaming flows.

### `a2a-methods/`

One client binary per JSON-RPC method exposed by the A2A specification
(`message/send`, `message/stream`, `tasks/get`, `tasks/list`, `tasks/cancel`,
and the four `tasks/pushNotificationConfig/*` operations). All clients share
the offline server in `a2a-methods/server/`. See
[`a2a-methods/README.md`](./a2a-methods/README.md) for the full method matrix.

## Configuration

Examples that integrate with the Inference Gateway use `Config::from_env()` and
the `INFERENCE_GATEWAY_URL` environment variable. See the top-level
[README](../README.md) for the full env var reference.

## Learning Path

1. **`minimal/`** — understand the bare A2A server + client wire-up
2. **`static-agent-card/`** — externalise agent metadata to JSON
3. **`server-with-toolbox/`** — add LLM-driven tools and streaming

# Rust ADK Examples

Self-contained scenarios demonstrating different capabilities of the Rust Agent
Development Kit (ADK).

## Structure

Each example is its own directory with a server, a client, an optional colocated
`agent-card.json`, and a README:

```text
examples/
├── minimal/                # Bare-bones server/client without an agent or agent card
├── static-agent-card/      # Loading agent metadata from a JSON file with runtime overrides
└── server-with-toolbox/    # LLM agent with custom sync + async function tools
```

## Running

Each scenario exposes two Cargo examples (`<name>-server` and `<name>-client`).
You can run them via Cargo or via the corresponding Taskfile entries:

```bash
# Cargo
cargo run --example minimal-server
cargo run --example minimal-client

# Taskfile
task examples:minimal-server
task examples:minimal-client
```

> Examples that load `agent-card.json` resolve it relative to the current
> working directory. Run those servers from inside their example directory, or
> pass an absolute path via `with_agent_card_from_file(...)`.

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

## Configuration

Examples that integrate with the Inference Gateway use `Config::from_env()` and
the `INFERENCE_GATEWAY_URL` environment variable. See the top-level
[README](../README.md) for the full env var reference.

## Learning Path

1. **`minimal/`** — understand the bare A2A server + client wire-up
2. **`static-agent-card/`** — externalise agent metadata to JSON
3. **`server-with-toolbox/`** — add LLM-driven tools and streaming

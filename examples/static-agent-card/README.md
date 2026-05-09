# Static Agent Card Example

Demonstrates `with_agent_card_from_file()` — loading the agent card from a colocated JSON file with optional runtime overrides via `AgentCardOverrides`.

## Directory Structure

```
static-agent-card/
├── agent-card.json   # Agent metadata loaded at startup
├── server/main.rs    # Server using with_agent_card_from_file + AgentCardOverrides
├── client/main.rs    # Client demonstrating health, agent card, and a task
└── README.md
```

## Prerequisites

- An Inference Gateway reachable at `http://localhost:8080/v1` (or set `INFERENCE_GATEWAY_URL`).
- LLM provider/model configuration via `Config::from_env()` (see the top-level [README](../../README.md) for the full env var list).

## Running

```bash
# Server (uses the colocated agent-card.json)
cargo run --example static-agent-card-server
# or: task examples:static-agent-card-server

# Client (in another terminal)
cargo run --example static-agent-card-client
# or: task examples:static-agent-card-client
```

> Note: `with_agent_card_from_file("agent-card.json", ...)` resolves relative to the current working directory. Run the server from this example's directory, or pass an absolute path.

## What This Shows

- **JSON-based agent metadata**: name, description, capabilities, skills, provider — all in `agent-card.json`
- **Runtime overrides**: `AgentCardOverrides::new().with_name(...).with_version(...).with_description(...)` change selected fields without editing the file
- **Env-driven LLM config**: `Config::from_env()` for provider, model, and API key

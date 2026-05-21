# Static Agent Card Example

Demonstrates `with_agent_card_from_file()` - loading the agent card from a colocated JSON file with optional runtime overrides via `AgentCardOverrides`.

## Directory Structure

```
static-agent-card/
├── docker-compose.yaml          # Server + client + inference-gateway:latest
├── .env.example                 # DEEPSEEK_API_KEY + provider/model overrides
├── server/
│   ├── .well-known/agent.json   # Agent metadata loaded at startup
│   └── main.rs                  # Server using with_agent_card_from_file + AgentCardOverrides
├── client/main.rs               # Client demonstrating health, agent card, and a task
└── README.md
```

## What This Shows

- **JSON-based agent metadata**: name, description, capabilities, skills, provider - all in `server/.well-known/agent.json` (path mirrors the A2A-spec `/.well-known/agent.json` URL the server exposes)
- **Runtime overrides**: `AgentCardOverrides::new().with_name(...).with_version(...).with_description(...)` change selected fields without editing the file
- **Env-driven LLM config**: `Config::from_env()` reads provider, model, and API key

## Running with Docker Compose

```bash
cd examples/static-agent-card
cp .env.example .env
# Set DEEPSEEK_API_KEY (or another provider's key + matching AGENT_CLIENT_PROVIDER)
docker compose up --build
```

The stack starts three services on a private Docker network:

- `inference-gateway` (image `ghcr.io/inference-gateway/inference-gateway:latest`)
- `server` - built from `examples/Dockerfile.server`, runs the example server
- `client` - built from `examples/Dockerfile.client`, runs after the server is healthy

Defaults: `AGENT_CLIENT_PROVIDER=deepseek`, `AGENT_CLIENT_MODEL=deepseek-v4-flash`.
Override via `.env` to switch to any other provider supported by the gateway
(`groq`, `google`, `openai`, `anthropic`, `cohere`, `cloudflare`, `ollama`).

## Running locally

```bash
# Start an Inference Gateway separately, then run the server from inside its
# subdir so `.well-known/agent.json` resolves correctly:
cd examples/static-agent-card/server
cargo run -p static-agent-card-server
# or: task examples:static-agent-card-server

cargo run -p static-agent-card-client
# or: task examples:static-agent-card-client
```

> `with_agent_card_from_file(".well-known/agent.json", ...)` resolves relative to the current working directory. Run the server from its `server/` directory (or pass an absolute path). The Docker image sets `WORKDIR /app` and stages the agent card to `/app/.well-known/agent.json`, so this is automatic in the compose flow.

The client honours `SERVER_URL` (default `http://localhost:8081`).

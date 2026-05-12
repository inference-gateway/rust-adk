# Minimal A2A Example

The simplest A2A server and client setup using the Rust ADK.

## Directory Structure

```
minimal/
├── docker-compose.yaml  # Server + client (no Inference Gateway needed)
├── .env.example         # Layout consistency only - minimal doesn't consume any provider keys
├── server/main.rs       # Basic A2A server with no agent configured
├── client/main.rs       # Client that performs health check, agent card lookup, and one task
└── README.md
```

## What This Shows

- `A2AServerBuilder::new()` without an agent
- Default A2A endpoints (`/.well-known/agent.json`, `/health`, `POST /a2a`) served out of the box
- `A2AClient` performing the three core interactions: health, agent card, single task
- Because no agent is registered, `POST /a2a` returns a JSON-RPC error
  (`"No agent configured..."`) - that is the expected, documented behavior of
  the minimal scenario; it is the contract the other two examples build on.

## Running with Docker Compose

```bash
cd examples/minimal
docker compose up --build
```

The compose stack runs only the server and client - no Inference Gateway is
needed, since the minimal server never calls one. The client connects to the
server via the internal Docker network (`SERVER_URL=http://server:8081`).

## Running locally

In one terminal:

```bash
cargo run --example minimal-server
# or: task examples:minimal-server
```

In another terminal:

```bash
cargo run --example minimal-client
# or: task examples:minimal-client
```

The server listens on `0.0.0.0:8081`. The client honours `SERVER_URL`
(default `http://localhost:8081`).

# Minimal A2A Example

The simplest A2A server and client setup using the Rust ADK.

## Directory Structure

```
minimal/
├── server/main.rs   # Basic A2A server pointed at an Inference Gateway
├── client/main.rs   # Client that performs health check, agent card lookup, and one task
└── README.md
```

## Prerequisites

An Inference Gateway reachable at `http://localhost:8080/v1`.

## Running

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

The server listens on `0.0.0.0:8081`. The client expects the server at `http://localhost:8081`.

## What This Shows

- `A2AServerBuilder::new()` with only a gateway URL — no agent, no agent card file
- Default A2A endpoints (`/.well-known/agent.json`, health, JSON-RPC) served out of the box
- `A2AClient` performing the three core interactions: health, agent card, single task

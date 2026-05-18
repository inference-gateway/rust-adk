# streaming example

A2A server with a custom `StreamableTaskHandler` that emits a fixed
sentence one word at a time. No LLM, no Inference Gateway — the cadence
and content are hardcoded so the SSE machinery is the only thing on
display.

## What's in the box

```
streaming/
├── server/main.rs                 WordByWordStreamHandler
├── server/.well-known/agent.json  Agent metadata loaded at startup
├── client/main.rs                 Consumes message/stream + prints per-event timestamps
├── docker-compose.yaml            Server + client only (no inference-gateway)
└── README.md
```

## What this shows

- **`StreamableTaskHandler::handle_streaming_task`** is the entry
  point. The handler decides when (and what) to emit.
- **`StreamEmitter::emit_status(...)`** updates the task state and
  publishes a `TaskStatusUpdateEvent`.
- **`StreamEmitter::emit_text_artifact(...)`** appends a text artifact
  to the task and publishes a `TaskArtifactUpdateEvent`. Set
  `last_chunk=true` on the final emit so the client knows the stream
  has ended.
- The server also registers `with_default_background_task_handler()`
  so `message/send` still works alongside (returns the built-in echo
  reply because no agent is configured).

Expected client log (≈ 9 words × 150ms ≈ 1.5s wall time):

```
[0.00s] #1  → task <id> created (state TaskStateSubmitted)
[0.00s] #2  → status: TaskStateWorking
[0.15s] #3  → chunk: "The "
[0.30s] #4  → chunk: "quick "
[0.45s] #5  → chunk: "brown "
…
[1.35s] #11 → chunk: "dog." (last chunk)
[1.35s] #12 → status: TaskStateCompleted (final)
```

## Running with Docker Compose

```bash
cd examples/streaming
docker compose up --build
```

No `.env` needed — there are no provider keys.

## Running locally

```bash
cd examples/streaming/server
cargo run -p streaming-server
# or: task examples:streaming-server

cargo run -p streaming-client
# or: task examples:streaming-client
```

The server listens on `0.0.0.0:8080`. The client honours `SERVER_URL`
(default `http://localhost:8080`).

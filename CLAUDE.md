# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this crate is

`inference-gateway-adk` is a Rust Agent Development Kit for building servers and clients that speak the **Agent-to-Agent (A2A) JSON-RPC protocol**. It is one of several sibling ADKs (Go, TypeScript) maintained under `inference-gateway/`. The wire types in `src/a2a_types.rs` are **generated** from `schema.json` via `cargo typify` - do not hand-edit; regenerate via `task a2a:generate-types` and run `cargo fmt` (the task already does this).

Rust edition is **2024** and toolchain pinned to **1.94.1** in CI (`README` advertises 1.88+). The `redis` feature is optional and gates `RedisStorage`.

## Common commands

```bash
task lint        # cargo fmt --all -- --check
task lint:fix    # cargo fmt --all
task analyse     # cargo clippy --all-targets --all-features -- -D warnings
task test        # cargo test --all-targets --all-features
```

Run a single test: `cargo test --all-features <test_name>` (e.g. `cargo test --all-features test_a2a_types_serialization`). Use `--all-features` so the Redis-gated paths compile.

Run a single example (Cargo example names are flat - see the `[[example]]` table in `Cargo.toml`):

```bash
cargo run --example minimal-server
cargo run --example a2a-methods-tasks-list
```

Examples that load `.well-known/agent.json` resolve the path **relative to CWD**. Run those from inside the example's `server/` directory, or pass an absolute path to `with_agent_card_from_file(...)`.

Pre-commit/CI gate: `task lint && task analyse && task test` (this is what `.github/workflows/ci.yml` runs).

## Architecture

The crate's public surface is re-exported from `lib.rs`; everything server-side lives under `src/server/` and is fan-out via `src/server.rs`.

**Request flow for `message/send`** (and similarly for `message/stream`):

1. `axum` router in `server_core.rs` exposes three routes: `GET /health`, `GET /.well-known/agent.json`, `POST /a2a`.
2. `protocol.rs::a2a_handler` parses the JSON-RPC envelope, validates `jsonrpc == "2.0"`, dispatches by `method` to typed request structs from `a2a_types`.
3. For `message/send`, the server builds a `Task` in `TaskStateSubmitted`, persists it via `Storage`, then either invokes the registered `TaskHandler` inline OR enqueues onto the storage queue for the background workers depending on configuration.
4. `DefaultTaskManager` (`task_manager.rs`) spawns N workers (`QueueConfig::workers`) that block on `Storage::dequeue_task` (Redis: `BRPOP`; in-memory: `Notify`), run the handler to a terminal state, then route to active store or dead-letter store based on `status.state`.
5. `message/stream` uses `StreamableTaskHandler` + `StreamEmitter` (an `mpsc::Sender<StreamResponse>`-backed object). The emitter both writes to the SSE channel AND keeps the stored task in sync. The handler MUST emit a final `TaskStatusUpdateEvent` with `final: true` - callers will hang otherwise.

**Key trait boundaries** (these are the extension points):

- `Storage` (`src/server/storage.rs`) - queue + active-task store + dead-letter store + context bookkeeping + push-config store. `InMemoryStorage` is the default. `RedisStorage` is feature-gated. The factory is `create_storage(&QueueConfig)`.
- `TaskHandler` - synchronous-style handler for `message/send`. Built-in: `DefaultBackgroundTaskHandler` (delegates to `Agent`).
- `StreamableTaskHandler` - streaming handler for `message/stream`. Built-in: `DefaultStreamingTaskHandler`.
- `LLMClient` - pluggable LLM transport. `OpenAICompatibleLLMClient` wraps `inference-gateway-sdk` and is what `AgentBuilder` constructs by default.
- `ToolHandler` (`agent_toolbox.rs`) - sync (`FunctionToolHandler`) and async (`AsyncFunctionToolHandler`) wrappers exist for closures.

**Construction pattern**: builders, not raw struct literals. `A2AServerBuilder::new().with_agent(...).with_storage(...).with_task_handler(...).build().await` returns an `A2AServer`. Likewise `AgentBuilder` for the `Agent`. Calling `.serve(addr)` on `A2AServer` consumes it, spawns the task manager, and blocks on Axum until SIGINT, then drains workers via `TaskManagerRunner::shutdown`.

**Config** (`src/config.rs`): `Config::from_env()` is the entry point. Nested sub-configs: `AgentConfig` (`AGENT_CLIENT_*`), `CapabilitiesConfig`, `TlsConfig`, `AuthConfig`, `QueueConfig` (`QUEUE_*` - `provider` picks `Memory` vs `Redis`), `ServerConfig`, `TelemetryConfig`.

## Schema regeneration

`src/a2a_types.rs` is generated. Workflow:

```bash
task a2a:download-schema   # pulls latest schema.json / schema.yaml
task a2a:generate-types    # cargo typify + prepends allow-lints + cargo fmt
```

The generated file has several `#![allow(...)]` attributes prepended by the task - keep them when regenerating.

## Examples layout

Examples are split into "without AI" (no provider key needed: `minimal`, `static-agent-card`, `streaming`, `input-required`) and "with AI" (Inference Gateway container + provider key: `default-handlers`, `ai-powered`, `ai-powered-streaming`). Each scenario has `server/`, `client/`, and a `docker-compose.yaml`. `a2a-methods/` exposes one client binary per JSON-RPC method. `queue-storage/` demonstrates `InMemoryStorage` vs `RedisStorage` via Compose profiles.

## Testing conventions (from CONTRIBUTING.md)

- **Table-driven tests** with isolated mocks per case
- Use `tokio::test` for async tests
- Mock external dependencies via the trait abstractions above (`Storage`, `LLMClient`, `TaskHandler`) - don't hit the network

## Notes that bite

- Generated `a2a_types.rs` is large (~50k lines). Avoid loading it in full; grep for the specific struct/enum you need.
- The `redis` dependency is `optional` and pinned at `0.27.6` with only `tokio-comp` + `connection-manager` features. Don't enable extra features without checking compile times.
- The crate's `Cargo.toml` lists every example explicitly with both a flat `name` (e.g. `a2a-methods-tasks-get`) and a `path` - add new examples there or Cargo won't pick them up.
- Tracing is set up via `tracing-subscriber` with env-filter; examples typically call `tracing_subscriber::init()` at the top of `main`.

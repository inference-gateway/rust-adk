# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this repo is

`inference-gateway-adk` is the **Rust** Agent Development Kit (ADK) for building servers and
clients that speak the A2A (Agent-to-Agent) JSON-RPC protocol. It is the Rust counterpart to
the Go and TypeScript ADKs in the same org and shares their `A2A_*` env-var conventions, agent
card shape, and JSON-RPC surface.

It is a Cargo **workspace**: the library lives at the repo root; the 22 packages under
`examples/<scenario>/{server,client}/` are workspace members, each with its own `Cargo.toml`.

## Common commands

The project uses [`task`](https://taskfile.dev) (Taskfile.yml) as the canonical entry point;
each task wraps a `cargo` command:

| Task               | Underlying command                                              |
| ------------------ | --------------------------------------------------------------- |
| `task lint`        | `cargo fmt --all -- --check`                                    |
| `task lint:fix`    | `cargo fmt --all`                                               |
| `task analyse`     | `cargo clippy --all-targets --all-features -- -D warnings`      |
| `task test`        | `cargo test --all-targets --all-features`                       |

CI runs `lint` → `analyse` → `build` → `test` on **Rust 1.95.0** (`actions-rust-lang/setup-rust-toolchain`);
`clippy -- -D warnings` means any new warning fails CI.

**Run a single test:** `cargo test --all-features <test_name>` (e.g.
`cargo test --all-features build_fails_when_no_handler_configured`). For an integration test
file: `cargo test --all-features --test a2a_server_test`.

**Run an example pair:** examples expose `<name>-server` and `<name>-client` Cargo packages, e.g.
`task examples:minimal-server` then in another shell `task examples:minimal-client`. Servers that
load `.well-known/agent.json` resolve it **relative to CWD** — the task definitions `cd` into
`examples/<name>/`, so prefer the task targets over raw `cargo run -p ...` from the repo root.
See `examples/README.md` for the matrix of with-AI vs. without-AI scenarios and Compose profiles.

### Regenerating A2A types

`src/a2a_types.rs` is **generated** by [`cargo-typify`](https://crates.io/crates/cargo-typify) from
`schema.json` (mirrored from `inference-gateway/schemas`). Do not hand-edit it.

```bash
task a2a:download-schema      # refresh schema.{json,yaml}
task a2a:generate-types       # cargo typify + prepend allow-attrs + cargo fmt
```

The `generate-types` task also prepends a block of `#![allow(...)]` attributes the generator
otherwise trips on — they live in the task itself, not in source, so they must be re-applied
every regeneration. Don't paste them in by hand.

## Architecture

### Crate layout

- `src/lib.rs` re-exports a small, deliberately flat public API from `client`, `config`,
  and `server`. The generated `a2a_types` module is exported as-is.
- `src/server.rs` is a façade that only declares submodules and re-exports their public
  items; real logic lives under `src/server/*.rs`.
- `examples/` are workspace members (one binary per directory); their `Cargo.toml` lists
  each binary's exact dependencies so a reader can see what a given scenario actually needs.

### Server pipeline (`A2AServerBuilder` → `A2AServer::serve`)

`A2AServerBuilder` is a fluent builder that wires together five pluggable subsystems before
producing an `A2AServer`:

1. **Agent card** (required) — `with_agent_card(...)` / `with_agent_card_from_file(...)`.
   `build()` returns `Err` if no card is configured. `AgentCardOverrides` is layered on top of
   the file-loaded baseline.
2. **Agent** (optional) — built via `AgentBuilder`, holds the LLM client + toolbox + tool
   handlers + system prompt. `AgentBuilder::build()` fails fast when `provider` or `model` is
   unset.
3. **Task handlers**:
   - `TaskHandler` drives `message/send` (background queue path).
   - `StreamableTaskHandler` drives `message/stream` (SSE path).
   - `with_default_task_handlers()` wires bundled defaults that delegate to the registered
     `Agent` (or echo when none is present).
   - **Validation matrix in `build()`**: streaming-enabled card requires a streaming handler;
     streaming-disabled card requires a background handler; both-absent is rejected. Mismatches
     fail at startup, not at the first request.
4. **Storage** — `Arc<dyn Storage>`. `InMemoryStorage` is the default; `RedisStorage` is gated
   behind the `redis` Cargo feature (`Cargo.toml`). The trait covers queue (enqueue/blocking
   dequeue), active store, dead-letter, contexts, push-notification configs, and stats.
5. **Auth** — `AuthVerifier` trait. `OidcJwtVerifier` is auto-constructed when `auth_config.enable`
   is true; `with_auth_verifier(...)` overrides regardless of config. The middleware gates `POST /a2a`
   only — `GET /health` and `GET /.well-known/agent.json` are always public.

`A2AServer::serve` spawns the `DefaultTaskManager` worker pool (one per `with_workers(n)` slot),
mounts a public router (`/health`, `/.well-known/agent.json`) and an optionally-auth-gated
protected router (`/a2a`), and binds either a plain Axum listener or `axum-server` + `rustls`
(when `tls_config.enable`). SIGINT triggers graceful drain of both the HTTP server and the
queue workers.

### JSON-RPC dispatch

`src/server/protocol.rs::a2a_handler` is the single entry point for `POST /a2a`. It validates
the envelope (`jsonrpc == "2.0"`), then dispatches `params` to per-method handlers for:

```
message/send, message/stream,
tasks/get, tasks/list, tasks/cancel, tasks/resubscribe,
tasks/pushNotificationConfig/{set,get,list,delete},
agent/getAuthenticatedExtendedCard
```

`message/stream` and `tasks/resubscribe` return SSE; the rest return JSON-RPC envelopes.
`StreamEmitter` (`src/server/task_handler.rs`) is how streaming handlers push events back to
clients while also keeping `Storage` in sync — terminal events **must** carry `final: true`.

### Background task manager

`DefaultTaskManager` is spawned only when a background `TaskHandler` is configured. Each worker
loops on `Storage::dequeue_task` (blocking), moves the task into the active store, drives the
handler to a terminal state, and routes the result to the dead-letter store on terminal states
or back to the active store otherwise. Workers cooperate via a `CancellationToken`; an
in-flight handler call is allowed to finish before shutdown.

### Config

`Config` (`src/config.rs`) is a plain `serde` struct composed of nested sub-configs
(`agent_config`, `tls_config`, `auth_config`, `queue_config`, `server_config`, …). The library
**does not** read env vars itself — every example uses `envy::prefixed("A2A_").from_env::<Config>()`,
but any serde loader works.

The `de` module at the top of `src/config.rs` is **load-bearing**: it defines string-or-native
deserializers for `u16`/`u32`/`u64`/`usize`/`bool`/`f32`/`Duration`. `serde(flatten)` buffers
fields via `deserialize_any`, and `envy` exposes every env var as a string — without these
helpers, `A2A_SERVER_PORT=8080` cannot coerce into a `u16` inside a flattened sub-struct. Don't
remove or simplify them without verifying env-driven examples still load.

### Client (`src/client.rs`)

`A2AClient` provides one typed helper per JSON-RPC method (`send_message`, `get_task`,
`list_tasks`, `cancel_task`, `resubscribe_task`, the four `*_push_notification_config` helpers,
and `get_authenticated_extended_card`) backed by `reqwest`. SSE methods return
`Stream<StreamResponse>`. Each helper has a runnable counterpart in `examples/a2a-methods/`.

## Conventions to know

- **Conventional Commits** with semantic-release (`.releaserc.yaml`). In addition to the standard
  types, the release pipeline recognizes `impr` (improvements → patch release).
  `chore(release): 🔖 X.Y.Z [skip ci]` commits are produced by `@semantic-release/git` — don't
  author them manually.
- **Table-driven tests** with isolated per-case mocks/servers (see `tests/a2a_server_test.rs` and
  `src/server/server_builder.rs::tests`). When adding tests, follow that shape rather than
  spawning a shared global fixture.
- `task analyse` before pushing — clippy warnings break CI.
- Workspace example deps are pinned centrally under `[workspace.dependencies]` in the root
  `Cargo.toml`; per-example `Cargo.toml`s should refer to those rather than re-pinning.
- The `redis` feature is **off by default**; enable it explicitly (`--features redis`) in
  packages that import `RedisStorage`.

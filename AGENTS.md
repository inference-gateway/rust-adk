# AGENTS.md

`inference-gateway-adk` is the Rust Agent Development Kit: a Cargo workspace
whose root crate builds A2A (Agent-to-Agent, JSON-RPC) servers and clients. It
is the Rust counterpart of the Go and TypeScript ADKs in the same org and
shares their `A2A_*` env-var conventions and agent-card shape. This file is
for coding agents; human docs live in README.md and CONTRIBUTING.md.

## Repository layout

- `src/lib.rs` - flat public API re-exported from `client`, `config`, `server`.
- `src/client.rs` - `A2AClient`, one typed helper per A2A JSON-RPC method.
- `src/server.rs` - facade only; real logic lives in `src/server/*.rs`
  (builder, protocol dispatch, task handlers/manager, storage, auth, TLS,
  artifacts, MCP, usage tracking).
- `src/a2a_types.rs` - **generated** by `cargo-typify` from `schema.json`.
  Do not hand-edit. Regenerate with `task a2a:generate-types` (installs
  `cargo-typify` if missing; run `task a2a:download-schema` first to refresh
  the schema). The task also prepends the `#![allow(...)]` attributes the
  generated code needs; they live in the task, so don't paste them by hand.
- `tests/` - integration tests (`a2a_server_test.rs`, `auth_test.rs`,
  `tls_test.rs`, `artifacts_integration_test.rs`).
- `examples/<scenario>/{server,client}/` - workspace members, one binary per
  directory, each with its own `Cargo.toml`; `examples/README.md` catalogs
  them. `examples/tls/make-certs.sh` mints dev certificates.

## Commands

| Task | What it runs |
| --- | --- |
| `task lint` | `cargo fmt --all -- --check` |
| `task lint:fix` | `cargo fmt --all` |
| `task analyse` | `cargo clippy --all-targets --all-features -- -D warnings` |
| `task test` | `cargo test --all-targets --all-features` |
| `task --list` | example runners, e.g. `task examples:minimal-server` |

CI runs lint -> analyse -> build -> test on Rust 1.95.0 (the crate's MSRV);
clippy `-D warnings` means any new warning fails CI. CI's test step is plain
`cargo test` - `task test` (all features/targets) is the stricter local gate.
Run one test with `cargo test --all-features <test_name>`, or one integration
file with `cargo test --all-features --test a2a_server_test`. Example servers
that load `.well-known/agent.json` resolve it relative to CWD - use the task
targets (they `cd` into the example dir) rather than raw `cargo run -p ...`
from the repo root.

## Architecture

- `A2AServerBuilder::build()` wires: agent card (required - `build()` errors
  without one; `AgentCardOverrides` layer on a file-loaded card), optional
  `Agent` (`AgentBuilder::build()` fails fast without provider or model), task
  handlers, `Arc<dyn Storage>` (`InMemoryStorage` default, `RedisStorage`
  behind `redis`), and auth.
- Handler validation in `build()`: a streaming-enabled card needs a
  `StreamableTaskHandler` (`message/stream`), a streaming-disabled card needs a
  background `TaskHandler` (`message/send`), and neither is rejected - so
  mismatches fail at startup. `with_default_task_handlers()` delegates to the
  registered `Agent`, or echoes when none is present.
- Auth: `OidcJwtVerifier` is auto-built when `auth_config.enable` is true;
  `with_auth_verifier(...)` overrides it regardless. The middleware gates only
  `POST /a2a`; `GET /health` and `GET /.well-known/agent.json` stay public.
- `src/server/protocol.rs::a2a_handler` is the single `POST /a2a` entry point:
  it validates `jsonrpc == "2.0"` and dispatches `message/*`, `tasks/*`,
  `tasks/pushNotificationConfig/*` and `agent/getAuthenticatedExtendedCard`.
  `message/stream` and `tasks/resubscribe` return SSE.
- Streaming handlers push events through `StreamEmitter`
  (`src/server/task_handler.rs`), which also keeps `Storage` in sync; terminal
  events **must** carry `final: true`.
- `DefaultTaskManager` runs only when a background `TaskHandler` is set: one
  worker per `with_workers(n)` slot, each blocking on `Storage::dequeue_task`,
  driving the handler, then routing terminal tasks to the dead-letter store
  and others back to the active store. SIGINT drains HTTP and workers via a
  `CancellationToken`; in-flight handler calls finish first.

## Conventions

- Rust 2024 edition, standard rustfmt; `.editorconfig` = 4 spaces (Rust),
  2 spaces (YAML/TOML/JSON).
- Strong typed APIs, early returns, explicit `Result<T, E>`; `thiserror` for
  domain errors, `anyhow` for application context.
- Runtime config is plain serde; the library never reads env itself - examples
  load it via `envy::prefixed("A2A_")`. The string-or-native deserializers in
  `src/config.rs` (the `de` module) are load-bearing; don't simplify them
  without re-checking env-driven examples.
- Cargo features `redis`, `minio`, `telemetry` are off by default; enable them
  explicitly in packages that need them.
- Table-driven tests with isolated per-case mocks/servers (see
  `tests/a2a_server_test.rs`, `src/server/server_builder.rs` tests), not a
  shared global fixture; async tests use `#[tokio::test]`.
- Run `task analyse` before pushing - clippy warnings break CI.
- Conventional Commits with semantic-release (`.releaserc.yaml`), which also
  recognizes `impr` (improvements -> patch). Never author
  `chore(release): ... [skip ci]` commits manually.
- Shared example deps are pinned under `[workspace.dependencies]` in the root
  `Cargo.toml`; per-example manifests refer to those.

## Code Readability

- Write self-explanatory code: clear names and small, single-purpose functions carry the intent.
  If a block needs a comment to be understood, extract it into a well-named function or variable.
- No inline comments inside function bodies.
- Doc comments on functions and types are at most 5 lines: what it does and why, not how.
- No comments above modules, packages, or files.
- Tool directives are not comments and stay where the tool needs them (lint suppressions, build
  tags, compiler pragmas, code generation markers).

## Security

- Never commit real credentials; `.env` is gitignored (`**/.env`) and
  examples use `.env.example` templates.
- Certificates under `examples/tls/` are development artifacts only -
  generate fresh material for real deployments.

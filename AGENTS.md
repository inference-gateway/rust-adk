# AGENTS.md

`inference-gateway-adk` is the Rust Agent Development Kit: a Cargo workspace
whose root crate builds A2A (Agent-to-Agent, JSON-RPC) servers and clients. It
is the Rust counterpart of the Go and TypeScript ADKs in the same org and
shares their `A2A_*` env-var conventions and agent-card shape. This file is
for coding agents; human docs live in README.md and CONTRIBUTING.md.

## Repository layout

- `src/lib.rs` — flat public API re-exported from `client`, `config`, `server`.
- `src/client.rs` — `A2AClient`, one typed helper per A2A JSON-RPC method.
- `src/server.rs` — façade only; real logic lives in `src/server/*.rs`
  (builder, protocol dispatch, task handlers/manager, storage, auth, TLS,
  artifacts, MCP, usage tracking).
- `src/a2a_types.rs` — **generated** by `cargo-typify` from `schema.json`.
  Do not hand-edit. Regenerate with `task a2a:generate-types` (run
  `task a2a:download-schema` first to refresh the schema).
- `tests/` — integration tests (`a2a_server_test.rs`, `auth_test.rs`,
  `tls_test.rs`, `artifacts_integration_test.rs`).
- `examples/<scenario>/{server,client}/` — workspace members, one binary per
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

CI runs lint → analyse → build → test on Rust 1.95.0; clippy `-D warnings`
means any new warning fails CI. Run one test with
`cargo test --all-features <test_name>`. Example servers that load
`.well-known/agent.json` resolve it relative to CWD — use the task targets
(they `cd` into the example dir) rather than raw `cargo run -p ...` from the
repo root.

## Conventions

- Rust 2024 edition, standard rustfmt; `.editorconfig` = 4 spaces (Rust),
  2 spaces (YAML/TOML/JSON).
- Strong typed APIs, early returns, explicit `Result<T, E>`; `thiserror` for
  domain errors, `anyhow` for application context.
- Runtime config is plain serde; the library never reads env itself — examples
  load it via `envy::prefixed("A2A_")`. The string-or-native deserializers in
  `src/config.rs` (the `de` module) are load-bearing; don't simplify them
  without re-checking env-driven examples.
- Cargo features `redis`, `minio`, `telemetry` are off by default; enable them
  explicitly in packages that need them.
- Table-driven tests with isolated per-case mocks/servers (see
  `tests/a2a_server_test.rs`); async tests use `#[tokio::test]`.
- Conventional Commits with semantic-release (`.releaserc.yaml`), which also
  recognizes `impr` (improvements → patch). Never author
  `chore(release): … [skip ci]` commits manually.
- Shared example deps are pinned under `[workspace.dependencies]` in the root
  `Cargo.toml`; per-example manifests refer to those.

## Security

- Never commit real credentials; examples use `.env.example` templates.
- Certificates under `examples/tls/` are development artifacts only —
  generate fresh material for real deployments.

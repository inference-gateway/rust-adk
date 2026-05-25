# Repository Guidelines

## Project Structure & Module Organization

This repository is the Rust Agent Development Kit crate `inference-gateway-adk`.
Core library code lives in `src/`: `client.rs` contains the A2A client,
`server.rs` and `src/server/` contain server, auth, storage, TLS, and task
handling modules, and `a2a_types.rs` is generated from `schema.json`. Integration
tests live in `tests/` (`*_test.rs`). Runnable examples are workspace members
under `examples/<scenario>/{server,client}` with per-example READMEs and
Docker Compose files where needed.

## Build, Test, and Development Commands

- `cargo build --all-targets --all-features`: compile the crate, examples, and tests.
- `task lint`: run `cargo fmt --all -- --check`.
- `task lint:fix`: format all Rust code with `cargo fmt --all`.
- `task analyse`: run Clippy with all targets/features and `-D warnings`.
- `task test`: run `cargo test --all-targets --all-features`.
- `task --list`: show example runners such as `task examples:minimal-server`.
- `task a2a:generate-types`: regenerate `src/a2a_types.rs` from `schema.json`.

## Coding Style & Naming Conventions

Use Rust 2024 style and standard `rustfmt` formatting. `.editorconfig` sets
4-space indentation for Rust and 2 spaces for YAML, TOML, and JSON. Prefer
strong typed APIs, early returns, and explicit `Result<T, E>` error handling.
Use `thiserror` for domain errors and `anyhow` for application-level context.
Name modules and files in `snake_case`, public types in `UpperCamelCase`, and
functions, variables, and test functions in `snake_case`. Keep generated A2A
types isolated in `src/a2a_types.rs`; do not hand-edit generated sections unless
the schema generator cannot express the required fix.

## Testing Guidelines

Use Rust integration tests in `tests/` plus focused unit tests beside the code
when useful. Async tests should use `#[tokio::test]`. Follow the existing
`*_test.rs` file pattern and descriptive `test_*` function names. Prefer
isolated fixtures or mock servers per case so tests remain deterministic. Before
opening a PR, run `task lint`, `task analyse`, and `task test`.

## Commit & Pull Request Guidelines

Recent history uses Conventional Commit-style messages such as `docs: ...`,
`chore(deps): ...`, and `chore(docs): ...`; follow that pattern and keep the
subject imperative and concise. PRs should include a clear summary, linked issues
when relevant, notes for breaking API changes, and tests or examples for new
behavior. Update README, example docs, and schema-generated files when behavior
or configuration changes.

## Security & Configuration Tips

Do not commit real credentials. Use `.env.example` files in examples as templates
for local configuration. TLS certificates under `examples/tls/` are development
artifacts only; generate fresh material for real deployments.

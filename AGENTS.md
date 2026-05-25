# Repository Guidelines

## Project Structure & Module Organization

This is the Rust ADK crate `inference-gateway-adk` using Rust 2024. Core library code lives in `src/`: `client.rs` implements the A2A client, `config.rs` contains configuration types, `server.rs` re-exports the server module tree, and `src/server/` contains auth, storage, task handling, TLS, agent, and protocol logic. `src/a2a_types.rs` is generated from `schema.json`; regenerate it instead of hand-editing. Integration tests are in `tests/`. Runnable examples are workspace members under `examples/<scenario>/{server,client}`.

## Build, Test, and Development Commands

Use Task when available; it wraps the expected Cargo commands.

- `cargo build --workspace` builds the crate and example workspace members.
- `task lint` runs `cargo fmt --all -- --check`.
- `task lint:fix` formats all Rust code.
- `task analyse` runs `cargo clippy --all-targets --all-features -- -D warnings`.
- `task test` runs `cargo test --all-targets --all-features`.
- `task --list` shows example runners, such as `task examples:minimal-server`.
- `task a2a:download-schema` and `task a2a:generate-types` refresh protocol schemas and generated Rust types.

## Coding Style & Naming Conventions

Follow standard Rust formatting with 4-space indentation via `cargo fmt`. Keep public APIs documented and prefer explicit, typed configuration structs. Use `Result<T, E>` for recoverable failures, `thiserror` for domain errors, and `anyhow` for contextual errors. Rust items follow normal conventions: modules and functions in `snake_case`, types and traits in `PascalCase`, constants in `SCREAMING_SNAKE_CASE`.

## Testing Guidelines

Use unit tests near the code under `#[cfg(test)]` for focused behavior and files in `tests/` for end-to-end A2A server/client coverage. Async tests use Tokio. Prefer table-driven cases and isolated mock servers or ephemeral localhost ports. Before submitting, run `task lint`, `task analyse`, and `task test`.

## Commit & Pull Request Guidelines

Recent history uses conventional-style commits, for example `chore: Regenerate infer CLI configurations`, `chore(deps): ...`, and `ci(deps): ...`. Keep messages imperative and scoped when helpful: `fix(auth): validate bearer token issuer`. Pull requests should describe behavior changes, link issues, call out breaking API or configuration changes, and update README/example docs when public usage changes. Include tests or explain unchanged coverage.

## Security & Configuration Tips

Do not commit secrets, generated certificates, or local `.env` files. Use `.env.example` files under examples as templates. TLS, auth, Redis queue storage, and AI-powered examples often require local services; document ports and environment variables in the relevant README when changing them.

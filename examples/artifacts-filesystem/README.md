# Artifacts (filesystem) example

End-to-end demonstration of the ADK's artifacts subsystem with the
filesystem-backed storage provider:

* `server/` runs an `A2AServer` plus the standalone artifacts HTTP server.
  A `StreamableTaskHandler` produces a small text report and emits it as
  a `FilePart` whose `fileWithUri` points at the artifacts server.
* `client/` opens `message/stream`, collects the file artifact URI, then
  downloads the artifact directly from the artifacts HTTP server and
  prints its contents.

## Running

In two terminals from the repo root:

```bash
# Terminal 1 - server (A2A on :8087, artifacts on :8088)
cd examples/artifacts-filesystem/server
cargo run --example artifacts-filesystem-server

# Terminal 2 - client
cargo run --example artifacts-filesystem-client
```

The on-disk store ends up at `./artifacts-data/<artifact_id>/<filename>`
relative to the working directory the server was launched from.

## Environment variables

When `ARTIFACTS_ENABLE=true`, `Config::from_env()` reads:

| Variable | Default | Description |
| --- | --- | --- |
| `ARTIFACTS_ENABLE` | `false` | Master switch for the subsystem. |
| `ARTIFACTS_SERVER_HOST` | `0.0.0.0` | Bind address of the artifacts HTTP server. |
| `ARTIFACTS_SERVER_PORT` | `8081` | Port of the artifacts HTTP server. |
| `ARTIFACTS_STORAGE_PROVIDER` | `filesystem` | `filesystem` or `s3`. |
| `ARTIFACTS_STORAGE_BASE_PATH` | `./artifacts` | Filesystem root for the filesystem provider. |
| `ARTIFACTS_STORAGE_BASE_URL` | `http://localhost:8081` | Public URL prefix baked into file artifact URIs. |
| `ARTIFACTS_RETENTION_MAX_ARTIFACTS` | `5` | Cap on retained artifacts. |
| `ARTIFACTS_RETENTION_MAX_AGE` | `168h` | Maximum age before a blob is pruned. |
| `ARTIFACTS_RETENTION_CLEANUP_INTERVAL` | `24h` | Frequency of the retention loop. |

Duration values accept Go-style suffixes (`30s`, `15m`, `2h`, `7d`) or
bare integers (seconds).

# Artifacts (filesystem) example

End-to-end demonstration of the ADK's artifacts subsystem with the
filesystem-backed storage provider:

- `server/` runs an `A2AServer` plus the standalone artifacts HTTP
  server. A `StreamableTaskHandler` produces a small text report and
  emits it as a `FilePart` whose `fileWithUri` points at the artifacts
  server.
- `client/` opens `message/stream`, collects the file artifact URI,
  then downloads the artifact directly from the artifacts HTTP server
  and prints its contents.

## What's in the box

```
artifacts-filesystem/
├── server/main.rs                 ReportHandler emits a file artifact via emit_file_artifact
├── server/.well-known/agent.json  Agent metadata loaded at startup
├── client/main.rs                 Streams the task, downloads the artifact URI via reqwest
├── docker-compose.yaml            Server + client only (no inference-gateway)
├── .gitignore                     Ignores generated server/artifacts-data/
└── README.md
```

## Topology

```
                  message/stream                   GET /artifacts/<id>/<file>
   client ───────────────────────► A2A :8087       ┌──────────────────────────► artifacts :8088
                                   (FilePart.fileWithUri points here)          └─► filesystem
                                                                                  ./server/artifacts-data/
```

Two HTTP servers run inside the same process: the A2A JSON-RPC server on
`8087` and the standalone artifacts server on `8088`. The artifacts
server exposes `GET /artifacts/{artifact_id}/{filename}` (plus a
`GET /health`).

## What this shows

- **`StreamableTaskHandler::handle_streaming_task`** drives the stream.
  The handler emits a working status, then a single file artifact, then
  a final completed status with `final: true`.
- **`StreamEmitter::emit_file_artifact(...)`** writes the file bytes
  through `ArtifactService` into the configured storage provider, then
  publishes a `TaskArtifactUpdateEvent` whose `FilePart.fileWithUri`
  points at the artifacts HTTP server (URL prefix taken from
  `ARTIFACTS_STORAGE_BASE_URL`).
- The filesystem provider lays files out under
  `<base_path>/<artifact_id>/<filename>`. With the local-run defaults
  in `server/main.rs` that resolves to
  `./artifacts-data/<artifact_id>/report.txt` relative to the server's
  CWD.
- The client treats the URI as opaque and fetches it with a plain
  `reqwest::get(uri)` - it never needs to know about artifact IDs or
  filesystem layout.

## Running with Docker Compose

```bash
cd examples/artifacts-filesystem
docker compose up --build
```

No `.env` needed - there are no provider keys.

Notes:

- `ARTIFACTS_STORAGE_BASE_URL=http://server:8088` in the compose env is
  what makes the baked-in artifact URIs resolvable from the client
  container. The artifacts server is also published on host `:8088` so
  you can hit it directly for debugging.
- The container-side store at `/data/artifacts` is bind-mounted to
  `./server/artifacts-data/` on the host, so produced files are
  inspectable after the run (and gitignored).
- Debug snippet (host-side curl against the artifacts HTTP server):

  ```bash
  curl -sS http://localhost:8088/<artifact-id>/<filename>
  ```

  The full URI is printed by the client log line that begins
  `received file artifact`.
- Cleanup: `docker compose down` to stop, and
  `rm -rf server/artifacts-data` to drop the produced files.

## Running locally

```bash
cd examples/artifacts-filesystem/server
cargo run --example artifacts-filesystem-server
# or: task examples:artifacts-filesystem-server

cargo run --example artifacts-filesystem-client
# or: task examples:artifacts-filesystem-client
```

The server listens on `0.0.0.0:8087` for A2A JSON-RPC and
`0.0.0.0:8088` for the artifacts HTTP server. The client honours
`SERVER_URL` (default `http://localhost:8087`). The on-disk store ends
up at `./artifacts-data/<artifact_id>/<filename>` relative to the
working directory the server was launched from.

## Environment variables

The server loads these via `envy::prefixed("ARTIFACTS_").from_env::<ArtifactsConfig>()`
and assigns the result onto `Config::artifacts_config`:

| Variable | Default | Description |
| --- | --- | --- |
| `ARTIFACTS_ENABLE` | `false` | Master switch for the subsystem. |
| `ARTIFACTS_SERVER_HOST` | `0.0.0.0` | Bind address of the artifacts HTTP server. |
| `ARTIFACTS_SERVER_PORT` | `8081` | Port of the artifacts HTTP server. |
| `ARTIFACTS_STORAGE_PROVIDER` | `filesystem` | `filesystem` or `minio`. |
| `ARTIFACTS_STORAGE_BASE_PATH` | `./artifacts` | Filesystem root for the filesystem provider. |
| `ARTIFACTS_STORAGE_BASE_URL` | `http://localhost:8081` | Public URL prefix baked into file artifact URIs. |
| `ARTIFACTS_RETENTION_MAX_ARTIFACTS` | `5` | Cap on retained artifacts. |
| `ARTIFACTS_RETENTION_MAX_AGE` | `168h` | Maximum age before a blob is pruned. |
| `ARTIFACTS_RETENTION_CLEANUP_INTERVAL` | `24h` | Frequency of the retention loop. |

Duration values accept Go-style suffixes (`30s`, `15m`, `2h`, `7d`) or
bare integers (seconds).

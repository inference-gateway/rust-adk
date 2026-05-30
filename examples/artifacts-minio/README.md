# Artifacts (MinIO) example

End-to-end demonstration of the ADK's artifacts subsystem with the
MinIO storage provider, talking to a local on-premises MinIO server.

- `server/` runs an `A2AServer` plus the standalone artifacts HTTP
  server. A `StreamableTaskHandler` produces a small text report and
  emits it as a `FilePart` whose `fileWithUri` points **directly at
  MinIO** (the example bucket has anonymous-read enabled).
- `client/` opens `message/stream`, collects the file artifact URI,
  then downloads the artifact straight from MinIO via HTTP and prints
  its contents.

The server is gated by the `minio` Cargo feature: it pulls in the
[`minio`](https://crates.io/crates/minio) crate and wires
`MinioArtifactStorage` into the artifact subsystem.

## What's in the box

```
artifacts-minio/
├── server/main.rs                 ReportHandler emits a file artifact via emit_file_artifact (MinIO backend)
├── server/.well-known/agent.json  Agent metadata loaded at startup
├── client/main.rs                 Streams the task, downloads the artifact URI via reqwest
├── docker-compose.yaml            minio + createbucket (mc) + server + client
├── .gitignore                     Ignores docker-compose .env overrides
└── README.md
```

## Topology

```
                   message/stream            FilePart.fileWithUri → http://minio:9000/artifacts/<id>/<file>
   client ────────────────────────► A2A :8089                ┌─────────► MinIO :9000  (anonymous read)
                                    (server uploads to minio:9000)
```

Three HTTP services run in the compose stack:

- **A2A JSON-RPC** on `:8089` (server)
- **Artifacts HTTP server** on `:8090` (server, started but unused — downloads bypass it)
- **MinIO API** on `:9000` (host-exposed) and **MinIO console** on `:9001`

## What this shows

- **`MinioArtifactStorage`** (gated by `feature = "minio"`) implements
  the same `ArtifactStorage` trait as `FilesystemArtifactStorage`. The
  server builder picks it up when `ArtifactsStorageProvider::Minio` is
  configured.
- **Bucket bootstrap**: `MinioArtifactStorage::from_config` calls
  `bucket_exists` and `create_bucket` so the agent starts cleanly
  against a fresh MinIO. The docker-compose `createbucket` service
  *also* runs `mc anonymous set download local/artifacts` so the
  bucket is publicly readable — a deployment-level concern handled
  out-of-band, mirroring the Go ADK example.
- **Direct-to-MinIO download**: setting
  `ARTIFACTS_STORAGE_BASE_URL=http://minio:9000` makes
  `MinioArtifactStorage::url()` emit
  `http://minio:9000/<bucket>/<artifact_id>/<filename>` — a plain
  path-style GET against MinIO. Clients don't proxy through the ADK's
  artifacts HTTP server at all; this is how you'd offload bulk
  transfer in production.

## Running with Docker Compose

```bash
cd examples/artifacts-minio
docker compose up --build
```

No `.env` needed — credentials default to `minioadmin / minioadmin` and
the bucket name is `artifacts`. The compose stack:

1. Starts MinIO with a `curl /minio/health/live` healthcheck.
2. Runs `minio/mc` to create the `artifacts` bucket (idempotent) and
   attach a public-read policy.
3. Builds the server with `CARGO_FEATURES=minio` so the `minio` crate
   is compiled in, then waits for both MinIO and the bucket setup
   before starting.
4. Waits for the server's `/health` to pass, then runs the client.

Inspect the artifact in MinIO directly:

```bash
# From the host (port-published):
curl -sS http://localhost:9000/artifacts/<artifact-id>/<filename>

# Or browse the MinIO console:
open http://localhost:9001   # login minioadmin / minioadmin
```

The full URI is printed by the client log line that begins `received
file artifact`. Note the URI inside the FilePart is
`http://minio:9000/...` — that's the Docker service name, only
resolvable from inside the compose network. Replace `minio` with
`localhost` to fetch from your host shell.

Cleanup: `docker compose down -v` to drop the `minio-data` named volume.

## Running locally

You need a MinIO instance reachable on `localhost:9000`. Quickest is:

```bash
docker run --rm -d --name minio-local \
  -p 9000:9000 -p 9001:9001 \
  -e MINIO_ROOT_USER=minioadmin -e MINIO_ROOT_PASSWORD=minioadmin \
  minio/minio:latest server /data --console-address ":9001"

# Create the bucket with anonymous-read policy (one-off):
docker run --rm --network host minio/mc:latest sh -c \
  "mc alias set local http://localhost:9000 minioadmin minioadmin && \
   mc mb --ignore-existing local/artifacts && \
   mc anonymous set download local/artifacts"
```

Then run the server and client (server needs the `minio` feature):

```bash
cd examples/artifacts-minio/server
cargo run --features minio --example artifacts-minio-server
# or: task examples:artifacts-minio-server

cargo run --example artifacts-minio-client
# or: task examples:artifacts-minio-client
```

The server listens on `0.0.0.0:8089` for A2A JSON-RPC and
`0.0.0.0:8090` for the artifacts HTTP server (unused for downloads in
this example). The client honours `SERVER_URL` (default
`http://localhost:8089`).

## Environment variables

| Variable | Default | Description |
| --- | --- | --- |
| `ARTIFACTS_STORAGE_ENDPOINT` | `http://localhost:9000` | MinIO endpoint URL. Scheme prefix is honoured (`https://` for SSL). |
| `ARTIFACTS_STORAGE_ACCESS_KEY` | `minioadmin` | Static access key. |
| `ARTIFACTS_STORAGE_SECRET_KEY` | `minioadmin` | Static secret key. |
| `ARTIFACTS_STORAGE_BUCKET_NAME` | `artifacts` | Target bucket. Created on startup if missing. |
| `ARTIFACTS_STORAGE_BASE_URL` | `http://localhost:9000` | Public URL prefix baked into `FilePart.fileWithUri`. |

The full `ARTIFACTS_*` env-var surface from
`examples/artifacts-filesystem/README.md` also applies (retention,
artifacts-server bind address, etc.).

## Production notes

- **Anonymous-read buckets are a deployment choice, not a default.** If
  your MinIO bucket isn't public, set
  `ARTIFACTS_STORAGE_BASE_URL=http://<artifacts-server-host>` so URIs
  point back at the ADK's artifacts HTTP server, which proxies the
  fetch via `ArtifactStorage::retrieve()`. The trade-off is bulk
  traffic flowing through your agent process instead of straight from
  the object store.
- **Pre-signed URLs are not yet wired in.** A future enhancement could
  have `MinioArtifactStorage::url()` mint a time-limited pre-signed
  GET so the bucket can stay private without proxying.
- **Retention** is driven by `cleanup_expired` / `cleanup_oldest` over
  `list_objects` — fine for thousands of artifacts, but you should
  prefer MinIO bucket lifecycle policies once the working set grows
  past that.

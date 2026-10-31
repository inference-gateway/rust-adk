# Artifacts (S3 / RustFS) example

End-to-end demonstration of the ADK's artifacts subsystem with the `s3`
storage provider, talking to [RustFS](https://github.com/rustfs/rustfs)
(an actively maintained, S3-compatible object store written in Rust).

- `server/` runs an `A2AServer` plus the standalone artifacts HTTP
  server. A `StreamableTaskHandler` produces a small text report and
  emits it as a file part whose `url` points at the artifacts HTTP
  server (proxy mode).
- `client/` opens `SendStreamingMessage`, collects the file artifact
  URI, then downloads the artifact over HTTP and prints its contents.

The server depends on the `s3` Cargo feature: it pulls in the
[`minio`](https://crates.io/crates/minio) crate (minio-rs) and wires
`S3ArtifactStorage` into the artifact subsystem. The client library
speaks plain S3, so the backend works with RustFS, MinIO, AWS S3, or
any other S3-compatible endpoint.

## What's in the box

```
artifacts-s3/
├── server/Cargo.toml + server/main.rs  ReportHandler emits a file artifact via emit_file_artifact (S3 backend)
├── server/.well-known/agent-card.json  Agent metadata loaded at startup
├── client/Cargo.toml + client/main.rs  Streams the task, downloads the artifact URI via reqwest
├── docker-compose.yaml                 rustfs + server + client (no init container)
├── .gitignore                          Ignores docker-compose .env overrides
└── README.md
```

## Topology

```
   client ──SendStreamingMessage──► A2A :8089 ──(PUT object)────► RustFS :9000
   client ──GET /artifacts/...─────► Artifacts HTTP :8090 ──(S3 GET)──► RustFS :9000
```

Three HTTP services run in the compose stack:

- **A2A JSON-RPC** on `:8089` (server)
- **Artifacts HTTP server** on `:8090` (server) - proxies downloads from RustFS
- **RustFS S3 API** on `:9000` (host-exposed) and **RustFS console** on `:9001`

## What this shows

- **`S3ArtifactStorage`** (gated by the `s3` feature) implements the
  same `ArtifactStorage` trait as `FilesystemArtifactStorage`. The
  server builder wires it when `ArtifactsStorageProvider::S3` is
  configured.
- **Bucket bootstrap**: `S3ArtifactStorage::from_config` calls
  `bucket_exists` and `create_bucket` so the agent starts cleanly
  against a fresh store - the compose stack needs no `mc` init
  container or any other S3 bootstrap tooling.
- **Proxy-mode downloads**: `ARTIFACTS_STORAGE_BASE_URL=http://server:8090`
  makes `S3ArtifactStorage::url()` emit a URI at the artifacts HTTP
  server, which streams the object via `ArtifactStorage::retrieve`.
  The bucket stays private (no anonymous-read policy needed) and
  access control sits with the artifacts server deployment.
- **Direct-mode downloads** (opt-in): set
  `ARTIFACTS_STORAGE_BASE_URL=http://rustfs:9000` instead so URIs are
  path-style addresses straight to the store, and attach an
  anonymous-read policy to the bucket yourself (via the RustFS console
  on `:9001` or any S3 client). Access control is then a storage-level
  concern, mirroring the Go ADK example.

## Running with Docker Compose

```bash
cd examples/artifacts-s3
docker compose up --build
```

The compose stack:

1. Fixes permissions on the `rustfs-data` named volume (RustFS runs as
   UID/GID 10001, copied from the official RustFS reference compose)
   and starts RustFS with a `curl /health` healthcheck.
2. Builds and starts the server (A2A on `:8089`, artifacts HTTP on
   `:8090`). On startup the ADK connects to RustFS and creates the
   `artifacts` bucket when missing.
3. Waits for the server's `/health` to pass, then runs the client.

Watch the client log: the `received file artifact` line shows the URI
(the `http://server:8090/...` service name is only resolvable inside
the compose network - replace `server` with `localhost` to fetch from
the host), and the final log line prints the downloaded report.

Cleanup: `docker compose down -v` to drop the `rustfs-data` volume.

## Running locally

Start a RustFS instance matching the compose defaults:

```bash
docker run --rm -d --name rustfs-local \
  -p 9000:9000 -p 9001:9001 \
  -e RUSTFS_VOLUMES=/data \
  -e RUSTFS_ADDRESS=0.0.0.0:9000 \
  -e RUSTFS_CONSOLE_ADDRESS=0.0.0.0:9001 \
  -e RUSTFS_CONSOLE_ENABLE=true \
  -e RUSTFS_ACCESS_KEY=rustfsadmin -e RUSTFS_SECRET_KEY=rustfsadmin \
  rustfs/rustfs:1.0.4
```

Then run the server and client:

```bash
cd examples/artifacts-s3/server
cargo run -p artifacts-s3-server
# or: task examples:artifacts-s3-server

cd ../client
cargo run -p artifacts-s3-client
# or: task examples:artifacts-s3-client
```

The server listens on `0.0.0.0:8089` for A2A JSON-RPC and
`0.0.0.0:8090` for the artifacts HTTP server (downloads go through it
in this example). The client honours `SERVER_URL` (default
`http://localhost:8089`).

## Environment variables

| Variable | Default | Description |
| --- | --- | --- |
| `ARTIFACTS_STORAGE_ENDPOINT` | `http://localhost:9000` | S3-compatible endpoint URL. Scheme prefix is honoured (`https://` for SSL). |
| `ARTIFACTS_STORAGE_ACCESS_KEY` | `rustfsadmin` | Static access key. |
| `ARTIFACTS_STORAGE_SECRET_KEY` | `rustfsadmin` | Static secret key. |
| `ARTIFACTS_STORAGE_BUCKET_NAME` | `artifacts` | Target bucket. Created on startup if missing. Keep the default `artifacts` name for proxy mode (the bucket name composes the artifacts server route prefix - see Production notes). |
| `ARTIFACTS_STORAGE_BASE_URL` | `http://localhost:8090` | Public URL prefix baked into the file part `url`. Point at the artifacts HTTP server for proxy mode, or at the store endpoint for direct mode. |

The full `ARTIFACTS_*` env-var surface from
`examples/artifacts-filesystem/README.md` also applies (retention,
artifacts-server bind address, etc.).

## S3 client choice (measured)

The `s3` provider keeps the [`minio`](https://crates.io/crates/minio)
crate (minio-rs) as its S3 client. It is the client library only - the
`minio/minio` server image is gone - and it remains maintained while
working against any S3-compatible endpoint (RustFS included). The
obvious alternative, `aws-sdk-s3`, was measured before keeping it:

| Binary | Size | Delta vs baseline |
| --- | --- | --- |
| baseline (bare binary, no tokio, no S3 client) | 444,784 bytes | - |
| `minio` crate (minio-rs) 0.4.0 (kept) | 10,321,176 bytes | +9.42 MB |
| `aws-sdk-s3` 1.157.0 | 10,120,696 bytes | +9.23 MB |

Measured with rustc 1.99.1, linux/amd64, `cargo build --release`
(default release profile, no strip, no LTO). Each binary is a tiny
program linking one client and exercising the same operations the
storage uses (bucket exists, create, put, get, stat, delete, list).
The two clients land within 200 KB of each other, so the Go ADK reason
for keeping its client (a 3.7 MB gap between its Go candidates) does
not apply here. The decision is to keep minio-rs because the sizes are
a wash, the client is already wired in and maintained, and it offers
the path-style addressing plus bucket auto-creation the ADK relies on.
If the balance ever changes, swap `MinioClient` in
`src/server/artifact_storage_s3.rs` behind the same `ArtifactStorage`
trait.

## Production notes

- **Proxy vs direct is a deployment choice.** Keep proxy mode when the
  bucket is private - go direct when the object store should serve the
  bulk transfer and you can grant anonymous read on the bucket.
- **Route prefix**: generated URIs are
  `<base_url>/<bucket>/<context_id>/<artifact_id>/<filename>`, so in
  proxy mode the bucket must be named `artifacts` to compose the
  artifacts server's `/artifacts/...` route (the default here). Rename
  the bucket only in direct mode.
- **Pre-signed URLs are not yet wired in.** A future enhancement could
  have `S3ArtifactStorage::url()` mint a time-limited pre-signed GET so
  the bucket can stay private without proxying.
- **Retention** is driven by `cleanup_expired` / `cleanup_oldest` over
  S3 `list_objects` - fine for thousands of artifacts, but prefer
  store-side lifecycle policies (RustFS and MinIO both support them)
  once the working set grows past that.
//! Standalone HTTP server that exposes stored artifacts to A2A clients.
//!
//! Lives on its own [`SocketAddr`] (typically `:8081`) so the main A2A
//! JSON-RPC surface isn't entangled with bulk file transfer. The router
//! exposes two routes:
//!
//! - `GET  /health`                            - simple liveness probe.
//! - `GET  /artifacts/:artifact_id/:filename`  - stream a stored artifact.
//!
//! Range requests are honoured for bulk downloads via the standard
//! `Range: bytes=...` syntax, and content-disposition headers carry the
//! original filename so curl/browser downloads land sensibly.

use super::artifact_service::{ArtifactService, infer_mime_type};
use super::tls::{MtlsAcceptor, build_server_config};
use crate::config::{ArtifactsServerConfig, TlsConfig};
use anyhow::{Result, anyhow};
use axum::Router;
use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, HeaderValue, StatusCode, header};
use axum::response::{IntoResponse, Json, Response};
use axum::routing::get;
use axum_server::Handle;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{debug, error, info, warn};

/// Standalone server that surfaces stored artifacts over HTTP.
#[derive(Clone)]
pub struct ArtifactsServer {
    config: ArtifactsServerConfig,
    tls: Option<TlsConfig>,
    service: Arc<dyn ArtifactService>,
}

impl std::fmt::Debug for ArtifactsServer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ArtifactsServer")
            .field("config", &self.config)
            .finish_non_exhaustive()
    }
}

#[derive(Clone)]
struct ArtifactsState {
    service: Arc<dyn ArtifactService>,
}

impl ArtifactsServer {
    /// Construct an artifacts server.
    pub fn new(config: ArtifactsServerConfig, service: Arc<dyn ArtifactService>) -> Self {
        Self {
            config,
            tls: None,
            service,
        }
    }

    /// Attach a TLS configuration. When set, the server terminates TLS
    /// (and optionally mTLS) just like the main A2A server.
    pub fn with_tls(mut self, tls: Option<TlsConfig>) -> Self {
        self.tls = tls;
        self
    }

    /// Build the Axum router for this server. Exposed so tests can plug
    /// the router into an ephemeral [`TcpListener`] without going
    /// through [`serve`](Self::serve).
    pub fn router(&self) -> Router {
        let state = Arc::new(ArtifactsState {
            service: Arc::clone(&self.service),
        });
        Router::new()
            .route("/health", get(health_handler))
            .route("/artifacts/{artifact_id}/{filename}", get(download_handler))
            .with_state(state)
    }

    /// Run the artifacts server until SIGINT (or until the server task
    /// errors). Mirrors the dual TLS / plaintext path used by the main
    /// A2A server.
    pub async fn serve(self, addr: SocketAddr) -> Result<()> {
        let app = self.router();
        match self.tls.as_ref() {
            Some(tls) if tls.enable => serve_tls(app, addr, tls).await,
            _ => serve_plain(app, addr).await,
        }
    }
}

async fn serve_plain(app: Router, addr: SocketAddr) -> Result<()> {
    info!("Artifacts server starting on http://{}", addr);
    let listener = TcpListener::bind(addr)
        .await
        .map_err(|e| anyhow!("failed to bind artifacts server to {addr}: {e}"))?;
    let serve = axum::serve(listener, app);
    tokio::select! {
        res = serve => res.map_err(|e| anyhow!("artifacts server error: {e}")),
        _ = tokio::signal::ctrl_c() => {
            info!("artifacts server: SIGINT received, shutting down");
            Ok(())
        }
    }
}

async fn serve_tls(app: Router, addr: SocketAddr, tls: &TlsConfig) -> Result<()> {
    let server_config = build_server_config(tls)
        .map_err(|e| anyhow!("artifacts TLS configuration invalid: {e}"))?;
    info!(
        cert = %tls.cert_path,
        "Artifacts server starting on https://{}",
        addr,
    );
    let acceptor = MtlsAcceptor::new(server_config);
    let handle = Handle::new();
    let server = axum_server::bind(addr)
        .acceptor(acceptor)
        .handle(handle.clone())
        .serve(app.into_make_service());
    tokio::select! {
        res = server => res.map_err(|e| anyhow!("artifacts TLS server error: {e}")),
        _ = tokio::signal::ctrl_c() => {
            info!("artifacts server: SIGINT received, shutting down");
            handle.graceful_shutdown(Some(std::time::Duration::from_secs(5)));
            Ok(())
        }
    }
}

async fn health_handler() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "artifacts",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

async fn download_handler(
    State(state): State<Arc<ArtifactsState>>,
    Path((artifact_id, filename)): Path<(String, String)>,
    headers: HeaderMap,
) -> Response {
    debug!(
        artifact_id = %artifact_id,
        filename = %filename,
        "artifacts download requested",
    );

    if let Err(e) = super::artifact_storage::sanitize_segment(&artifact_id, "artifact_id") {
        warn!("rejecting artifact request: {e}");
        return (StatusCode::BAD_REQUEST, e.to_string()).into_response();
    }
    if let Err(e) = super::artifact_storage::sanitize_segment(&filename, "filename") {
        warn!("rejecting artifact request: {e}");
        return (StatusCode::BAD_REQUEST, e.to_string()).into_response();
    }

    let bytes = match state.service.retrieve(&artifact_id, &filename).await {
        Ok(b) => b,
        Err(e) => {
            debug!("artifact not found `{artifact_id}/{filename}`: {e}");
            return (StatusCode::NOT_FOUND, "artifact not found".to_string()).into_response();
        }
    };
    let total_len = bytes.len() as u64;
    let media_type = infer_mime_type(&filename);

    let mut response_headers = HeaderMap::new();
    response_headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_str(media_type)
            .unwrap_or(HeaderValue::from_static("application/octet-stream")),
    );
    let disposition = format!(
        "attachment; filename=\"{}\"",
        sanitize_disposition(&filename)
    );
    if let Ok(value) = HeaderValue::from_str(&disposition) {
        response_headers.insert(header::CONTENT_DISPOSITION, value);
    }
    response_headers.insert(header::ACCEPT_RANGES, HeaderValue::from_static("bytes"));

    if let Some(range_header) = headers.get(header::RANGE) {
        match range_header
            .to_str()
            .ok()
            .and_then(|s| parse_range(s, total_len))
        {
            Some(parsed) => {
                let (start, end) = parsed;
                let slice = bytes[start as usize..=end as usize].to_vec();
                response_headers.insert(
                    header::CONTENT_LENGTH,
                    HeaderValue::from_str(&slice.len().to_string())
                        .unwrap_or(HeaderValue::from_static("0")),
                );
                let cr = format!("bytes {start}-{end}/{total_len}");
                if let Ok(value) = HeaderValue::from_str(&cr) {
                    response_headers.insert(header::CONTENT_RANGE, value);
                }
                let mut response = (StatusCode::PARTIAL_CONTENT, Body::from(slice)).into_response();
                let response_headers_mut = response.headers_mut();
                for (k, v) in response_headers.iter() {
                    response_headers_mut.insert(k, v.clone());
                }
                return response;
            }
            None => {
                let cr = format!("bytes */{total_len}");
                let mut resp = (
                    StatusCode::RANGE_NOT_SATISFIABLE,
                    "invalid byte range".to_string(),
                )
                    .into_response();
                if let Ok(value) = HeaderValue::from_str(&cr) {
                    resp.headers_mut().insert(header::CONTENT_RANGE, value);
                }
                return resp;
            }
        }
    }

    response_headers.insert(
        header::CONTENT_LENGTH,
        HeaderValue::from_str(&total_len.to_string()).unwrap_or(HeaderValue::from_static("0")),
    );
    let mut response = (StatusCode::OK, Body::from(bytes)).into_response();
    let response_headers_mut = response.headers_mut();
    for (k, v) in response_headers.iter() {
        response_headers_mut.insert(k, v.clone());
    }
    response
}

/// Strip characters that would mangle the `Content-Disposition` header
/// (quotes, control bytes) without trying to be a full RFC 5987 encoder
/// — the filename is already sanitized for path semantics upstream.
fn sanitize_disposition(filename: &str) -> String {
    filename
        .chars()
        .filter(|c| !c.is_control() && *c != '"' && *c != '\\')
        .collect()
}

/// Parse a single-range `Range: bytes=start-end` header against the
/// known total length. Returns `(start, end)` inclusive on success or
/// `None` if the header is unsupported / out-of-bounds.
fn parse_range(header: &str, total_len: u64) -> Option<(u64, u64)> {
    if total_len == 0 {
        return None;
    }
    let rest = header.strip_prefix("bytes=")?;
    // Only single-range requests are supported. Multipart/byterange
    // would need a different response body which is overkill for this
    // use case.
    if rest.contains(',') {
        return None;
    }
    let (start_str, end_str) = rest.split_once('-')?;
    let last = total_len - 1;
    match (start_str.trim(), end_str.trim()) {
        ("", "") => None,
        ("", suffix) => {
            // `-N` form: last N bytes.
            let n: u64 = suffix.parse().ok()?;
            if n == 0 {
                return None;
            }
            let start = total_len.saturating_sub(n);
            Some((start, last))
        }
        (start, "") => {
            let s: u64 = start.parse().ok()?;
            if s > last {
                return None;
            }
            Some((s, last))
        }
        (start, end) => {
            let s: u64 = start.parse().ok()?;
            let e: u64 = end.parse().ok()?;
            if s > e || s > last {
                return None;
            }
            Some((s, e.min(last)))
        }
    }
}

/// Spawn a background loop that periodically applies the configured
/// retention policy to the artifact service. The returned handle can
/// be awaited / aborted during graceful shutdown.
pub fn spawn_retention_task(
    service: Arc<dyn ArtifactService>,
    cleanup_interval: std::time::Duration,
    max_age: std::time::Duration,
    max_count: Option<usize>,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let mut ticker = tokio::time::interval(cleanup_interval);
        // Skip the immediate first tick - we want the first cleanup to
        // happen after `cleanup_interval`, not on startup.
        ticker.tick().await;
        loop {
            ticker.tick().await;
            match service.cleanup_expired(max_age).await {
                Ok(n) if n > 0 => info!("artifacts retention: expired {n} blob(s)"),
                Ok(_) => debug!("artifacts retention: no expired blobs"),
                Err(e) => error!("artifacts retention: cleanup_expired failed: {e}"),
            }
            if let Some(cap) = max_count {
                match service.cleanup_oldest(cap).await {
                    Ok(n) if n > 0 => info!("artifacts retention: trimmed to {cap} ({n} dropped)"),
                    Ok(_) => debug!("artifacts retention: within cap"),
                    Err(e) => error!("artifacts retention: cleanup_oldest failed: {e}"),
                }
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::ArtifactsServerConfig;
    use crate::server::artifact_service::DefaultArtifactService;
    use crate::server::artifact_storage::FilesystemArtifactStorage;

    fn tempdir(name: &str) -> std::path::PathBuf {
        let mut p = std::env::temp_dir();
        p.push(format!(
            "rust-adk-artifacts-srv-{}-{}-{}",
            std::process::id(),
            name,
            uuid::Uuid::new_v4()
        ));
        p
    }

    async fn spawn_test_server(
        root: &std::path::Path,
    ) -> (std::net::SocketAddr, Arc<DefaultArtifactService>) {
        let storage = Arc::new(FilesystemArtifactStorage::new(
            root,
            "http://localhost:8081",
        ));
        let service = Arc::new(DefaultArtifactService::new(storage));
        let server = ArtifactsServer::new(
            ArtifactsServerConfig::default(),
            service.clone() as Arc<dyn ArtifactService>,
        );
        let router = server.router();
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");
        tokio::spawn(async move {
            axum::serve(listener, router).await.ok();
        });
        (addr, service)
    }

    #[derive(Debug)]
    struct RangeCase {
        name: &'static str,
        header: &'static str,
        total: u64,
        expect: Option<(u64, u64)>,
    }

    #[test]
    fn parse_range_table_driven() {
        let cases = vec![
            RangeCase {
                name: "full_range",
                header: "bytes=0-4",
                total: 10,
                expect: Some((0, 4)),
            },
            RangeCase {
                name: "open_end",
                header: "bytes=5-",
                total: 10,
                expect: Some((5, 9)),
            },
            RangeCase {
                name: "suffix",
                header: "bytes=-3",
                total: 10,
                expect: Some((7, 9)),
            },
            RangeCase {
                name: "end_past_total_clamps",
                header: "bytes=2-100",
                total: 10,
                expect: Some((2, 9)),
            },
            RangeCase {
                name: "start_past_total_rejected",
                header: "bytes=100-200",
                total: 10,
                expect: None,
            },
            RangeCase {
                name: "missing_prefix_rejected",
                header: "0-4",
                total: 10,
                expect: None,
            },
            RangeCase {
                name: "multi_range_rejected",
                header: "bytes=0-1,2-3",
                total: 10,
                expect: None,
            },
            RangeCase {
                name: "empty_total_rejected",
                header: "bytes=0-1",
                total: 0,
                expect: None,
            },
        ];
        for case in cases {
            assert_eq!(
                parse_range(case.header, case.total),
                case.expect,
                "case `{}`",
                case.name,
            );
        }
    }

    #[tokio::test]
    async fn health_endpoint_returns_healthy() {
        let root = tempdir("health");
        let (addr, _svc) = spawn_test_server(&root).await;
        let body = reqwest::get(format!("http://{addr}/health"))
            .await
            .expect("health")
            .text()
            .await
            .expect("text");
        assert!(body.contains("\"status\":\"healthy\""));
        let _ = std::fs::remove_dir_all(&root);
    }

    #[tokio::test]
    async fn download_returns_stored_artifact() {
        let root = tempdir("download");
        let (addr, svc) = spawn_test_server(&root).await;
        let art = svc
            .create_file_artifact(
                "report",
                "demo",
                "demo.txt",
                b"hello world".to_vec(),
                Some("text/plain"),
            )
            .await
            .expect("create_file_artifact");
        let url = format!("http://{addr}/artifacts/{}/demo.txt", art.artifact_id);
        let response = reqwest::get(&url).await.expect("download");
        assert_eq!(response.status(), reqwest::StatusCode::OK);
        assert_eq!(
            response
                .headers()
                .get(reqwest::header::CONTENT_TYPE)
                .and_then(|v| v.to_str().ok()),
            Some("text/plain"),
        );
        let disposition = response
            .headers()
            .get(reqwest::header::CONTENT_DISPOSITION)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");
        assert!(disposition.contains("demo.txt"));
        let body = response.text().await.expect("body");
        assert_eq!(body, "hello world");
        let _ = std::fs::remove_dir_all(&root);
    }

    #[tokio::test]
    async fn download_supports_range_requests() {
        let root = tempdir("range");
        let (addr, svc) = spawn_test_server(&root).await;
        let art = svc
            .create_file_artifact(
                "data",
                "demo",
                "data.bin",
                b"0123456789".to_vec(),
                Some("application/octet-stream"),
            )
            .await
            .expect("create_file_artifact");
        let url = format!("http://{addr}/artifacts/{}/data.bin", art.artifact_id);
        let client = reqwest::Client::new();
        let response = client
            .get(&url)
            .header("Range", "bytes=2-5")
            .send()
            .await
            .expect("range");
        assert_eq!(response.status(), reqwest::StatusCode::PARTIAL_CONTENT);
        let cr = response
            .headers()
            .get(reqwest::header::CONTENT_RANGE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");
        assert_eq!(cr, "bytes 2-5/10");
        let body = response.text().await.expect("body");
        assert_eq!(body, "2345");
        let _ = std::fs::remove_dir_all(&root);
    }

    #[tokio::test]
    async fn download_missing_returns_404() {
        let root = tempdir("missing");
        let (addr, _svc) = spawn_test_server(&root).await;
        let response = reqwest::get(format!("http://{addr}/artifacts/missing-id/nope.txt"))
            .await
            .expect("response");
        assert_eq!(response.status(), reqwest::StatusCode::NOT_FOUND);
        let _ = std::fs::remove_dir_all(&root);
    }

    #[tokio::test]
    async fn download_rejects_path_traversal() {
        let root = tempdir("traversal-http");
        let (addr, _svc) = spawn_test_server(&root).await;
        // Build the path manually so the client doesn't normalize away
        // the traversal segment before it hits the server.
        let url = format!("http://{addr}/artifacts/..%2F..%2Fetc/passwd");
        let response = reqwest::get(&url).await.expect("response");
        // The router will either reject due to bad path or sanitize_segment
        // will. Either way we expect 400 or 404; 200 would be a bug.
        assert!(
            response.status() == reqwest::StatusCode::BAD_REQUEST
                || response.status() == reqwest::StatusCode::NOT_FOUND,
            "unexpected status: {}",
            response.status()
        );
        let _ = std::fs::remove_dir_all(&root);
    }
}

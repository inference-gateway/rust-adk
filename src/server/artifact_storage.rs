//! Pluggable storage backend for the artifacts subsystem.
//!
//! [`ArtifactStorage`] is the trait an [`ArtifactService`] holds as
//! `Arc<dyn ArtifactStorage>` to persist file/data artifacts produced by
//! task handlers and surface them via HTTP URLs rather than inline base64
//! bytes embedded in JSON-RPC responses.
//!
//! The bundled default is [`FilesystemArtifactStorage`], which lays
//! artifacts out under `<base_path>/<artifact_id>/<filename>` with
//! path-traversal sanitization. Production deployments can implement
//! [`ArtifactStorage`] themselves to wire in MinIO, GCS, or any other
//! object store.
//!
//! [`ArtifactService`]: super::artifact_service::ArtifactService
//! [`FilesystemArtifactStorage`]: FilesystemArtifactStorage

use anyhow::{Context, Result, anyhow};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::path::{Component, Path, PathBuf};
use std::time::{Duration, SystemTime};
use tokio::fs;
use tokio::io::AsyncWriteExt;
use tracing::{debug, warn};

/// Metadata describing a stored artifact entry.
#[derive(Debug, Clone)]
pub struct StoredArtifactInfo {
    pub artifact_id: String,
    pub filename: String,
    pub size: u64,
    pub modified: Option<DateTime<Utc>>,
}

/// Pluggable backend that persists artifact bytes and resolves them back
/// to a URL the artifacts HTTP server can hand to clients.
///
/// All methods are `async` so backends that need network/disk I/O fit
/// naturally. The trait is object-safe so callers can hold an
/// `Arc<dyn ArtifactStorage>`.
#[async_trait]
pub trait ArtifactStorage: Send + Sync + std::fmt::Debug {
    /// Persist `data` under `artifact_id`/`filename` and return the URL
    /// at which it can be retrieved.
    async fn store(&self, artifact_id: &str, filename: &str, data: Vec<u8>) -> Result<String>;

    /// Retrieve the raw bytes stored at `artifact_id`/`filename`.
    async fn retrieve(&self, artifact_id: &str, filename: &str) -> Result<Vec<u8>>;

    /// Whether a blob is stored at `artifact_id`/`filename`.
    async fn exists(&self, artifact_id: &str, filename: &str) -> Result<bool>;

    /// Delete the blob at `artifact_id`/`filename`. Returns `Ok(())` if
    /// it didn't exist - idempotent.
    async fn delete(&self, artifact_id: &str, filename: &str) -> Result<()>;

    /// Stable URL the [`ArtifactsServer`] would serve `artifact_id`/`filename` at.
    ///
    /// [`ArtifactsServer`]: super::artifacts_server::ArtifactsServer
    fn url(&self, artifact_id: &str, filename: &str) -> String;

    /// Delete every blob whose modified time is older than `max_age`.
    /// Returns the number of blobs removed.
    async fn cleanup_expired(&self, max_age: Duration) -> Result<usize>;

    /// Trim the store down so at most `max_count` blobs remain, deleting
    /// the oldest first. Returns the number removed.
    async fn cleanup_oldest(&self, max_count: usize) -> Result<usize>;

    /// Enumerate every stored artifact. Used by retention and tests.
    async fn list(&self) -> Result<Vec<StoredArtifactInfo>>;
}

/// Filesystem-backed [`ArtifactStorage`].
///
/// Lays each artifact out under `<base_path>/<artifact_id>/<filename>`.
/// The generated `base_url` follows the same pattern - configure
/// `base_url` to match wherever the [`ArtifactsServer`] is reachable so
/// clients can resolve the URL.
///
/// [`ArtifactsServer`]: super::artifacts_server::ArtifactsServer
#[derive(Debug, Clone)]
pub struct FilesystemArtifactStorage {
    base_path: PathBuf,
    base_url: String,
}

impl FilesystemArtifactStorage {
    /// Create a new filesystem-backed store. `base_url` is the URL prefix
    /// (without trailing slash) under which the [`ArtifactsServer`] is
    /// reachable. The on-disk root at `base_path` is created lazily on
    /// the first [`store`](ArtifactStorage::store) call.
    ///
    /// [`ArtifactsServer`]: super::artifacts_server::ArtifactsServer
    pub fn new(base_path: impl Into<PathBuf>, base_url: impl Into<String>) -> Self {
        Self {
            base_path: base_path.into(),
            base_url: base_url.into().trim_end_matches('/').to_string(),
        }
    }

    /// Resolve a sanitized path under `base_path` for `artifact_id`/`filename`.
    /// Rejects empty / traversal / absolute components so callers can't
    /// escape the configured root via `..` or leading slashes.
    fn resolve_path(&self, artifact_id: &str, filename: &str) -> Result<PathBuf> {
        let id = sanitize_segment(artifact_id, "artifact_id")?;
        let name = sanitize_segment(filename, "filename")?;
        Ok(self.base_path.join(id).join(name))
    }
}

/// Reject anything that would let a caller escape the configured base
/// path. This covers `..`, absolute paths, embedded path separators,
/// and the empty / whitespace cases.
pub(crate) fn sanitize_segment(value: &str, label: &str) -> Result<String> {
    if value.trim().is_empty() {
        return Err(anyhow!("{label} must not be empty"));
    }
    if value.contains('\0') {
        return Err(anyhow!("{label} contains a NUL byte"));
    }
    if value.contains('/') || value.contains('\\') {
        return Err(anyhow!(
            "{label} `{value}` must not contain path separators"
        ));
    }
    let path = Path::new(value);
    for component in path.components() {
        match component {
            Component::Normal(_) => {}
            _ => {
                return Err(anyhow!(
                    "{label} `{value}` must be a simple filename without traversal"
                ));
            }
        }
    }
    Ok(value.to_string())
}

#[async_trait]
impl ArtifactStorage for FilesystemArtifactStorage {
    async fn store(&self, artifact_id: &str, filename: &str, data: Vec<u8>) -> Result<String> {
        let target = self.resolve_path(artifact_id, filename)?;
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent).await.with_context(|| {
                format!("failed to create artifact directory `{}`", parent.display())
            })?;
        }
        let mut file = fs::File::create(&target)
            .await
            .with_context(|| format!("failed to create artifact file `{}`", target.display()))?;
        file.write_all(&data)
            .await
            .with_context(|| format!("failed to write artifact bytes to `{}`", target.display()))?;
        file.flush().await.ok();
        debug!(
            artifact_id,
            filename,
            bytes = data.len(),
            path = %target.display(),
            "stored artifact on filesystem",
        );
        Ok(self.url(artifact_id, filename))
    }

    async fn retrieve(&self, artifact_id: &str, filename: &str) -> Result<Vec<u8>> {
        let target = self.resolve_path(artifact_id, filename)?;
        fs::read(&target)
            .await
            .with_context(|| format!("failed to read artifact file `{}`", target.display()))
    }

    async fn exists(&self, artifact_id: &str, filename: &str) -> Result<bool> {
        let target = self.resolve_path(artifact_id, filename)?;
        match fs::metadata(&target).await {
            Ok(_) => Ok(true),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
            Err(e) => Err(anyhow!(
                "failed to stat artifact `{}`: {e}",
                target.display()
            )),
        }
    }

    async fn delete(&self, artifact_id: &str, filename: &str) -> Result<()> {
        let target = self.resolve_path(artifact_id, filename)?;
        match fs::remove_file(&target).await {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
            Err(e) => {
                return Err(anyhow!(
                    "failed to delete artifact `{}`: {e}",
                    target.display()
                ));
            }
        }
        // Best-effort: tidy up the artifact_id directory if it's now empty.
        if let Some(parent) = target.parent()
            && let Ok(mut read_dir) = fs::read_dir(parent).await
            && read_dir.next_entry().await.ok().flatten().is_none()
        {
            let _ = fs::remove_dir(parent).await;
        }
        Ok(())
    }

    fn url(&self, artifact_id: &str, filename: &str) -> String {
        format!(
            "{}/artifacts/{}/{}",
            self.base_url,
            urlencode_segment(artifact_id),
            urlencode_segment(filename),
        )
    }

    async fn cleanup_expired(&self, max_age: Duration) -> Result<usize> {
        let cutoff = SystemTime::now().checked_sub(max_age);
        let Some(cutoff) = cutoff else {
            return Ok(0);
        };
        let entries = self.list().await?;
        let mut removed = 0usize;
        for entry in entries {
            let modified_system = match entry.modified {
                Some(dt) => SystemTime::from(dt),
                None => continue,
            };
            if modified_system < cutoff {
                if let Err(e) = self.delete(&entry.artifact_id, &entry.filename).await {
                    warn!(
                        artifact_id = %entry.artifact_id,
                        filename = %entry.filename,
                        "cleanup_expired: delete failed: {e}",
                    );
                    continue;
                }
                removed += 1;
            }
        }
        Ok(removed)
    }

    async fn cleanup_oldest(&self, max_count: usize) -> Result<usize> {
        let mut entries = self.list().await?;
        if entries.len() <= max_count {
            return Ok(0);
        }
        entries.sort_by_key(|e| e.modified.unwrap_or_else(Utc::now));
        let drop_count = entries.len() - max_count;
        let mut removed = 0usize;
        for entry in entries.into_iter().take(drop_count) {
            if let Err(e) = self.delete(&entry.artifact_id, &entry.filename).await {
                warn!(
                    artifact_id = %entry.artifact_id,
                    filename = %entry.filename,
                    "cleanup_oldest: delete failed: {e}",
                );
                continue;
            }
            removed += 1;
        }
        Ok(removed)
    }

    async fn list(&self) -> Result<Vec<StoredArtifactInfo>> {
        let mut out = Vec::new();
        let mut top = match fs::read_dir(&self.base_path).await {
            Ok(r) => r,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(out),
            Err(e) => {
                return Err(anyhow!(
                    "failed to read artifacts root `{}`: {e}",
                    self.base_path.display()
                ));
            }
        };
        while let Some(entry) = top.next_entry().await? {
            let metadata = match entry.metadata().await {
                Ok(m) => m,
                Err(_) => continue,
            };
            if !metadata.is_dir() {
                continue;
            }
            let artifact_id = entry.file_name().to_string_lossy().to_string();
            let mut inner = match fs::read_dir(entry.path()).await {
                Ok(r) => r,
                Err(_) => continue,
            };
            while let Some(file) = inner.next_entry().await? {
                let file_meta = match file.metadata().await {
                    Ok(m) => m,
                    Err(_) => continue,
                };
                if !file_meta.is_file() {
                    continue;
                }
                let filename = file.file_name().to_string_lossy().to_string();
                let modified = file_meta.modified().ok().map(DateTime::<Utc>::from);
                out.push(StoredArtifactInfo {
                    artifact_id: artifact_id.clone(),
                    filename,
                    size: file_meta.len(),
                    modified,
                });
            }
        }
        Ok(out)
    }
}

/// Minimal percent-encoder for path segments. We only need to escape
/// characters that would break URL parsing or path semantics (space,
/// `#`, `?`, control chars). Everything else is preserved so generated
/// URLs remain readable.
pub(crate) fn urlencode_segment(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for byte in input.as_bytes() {
        let b = *byte;
        let is_safe = matches!(
            b,
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~'
        );
        if is_safe {
            out.push(b as char);
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tempdir(name: &str) -> PathBuf {
        let mut p = std::env::temp_dir();
        p.push(format!(
            "rust-adk-artifacts-{}-{}-{}",
            std::process::id(),
            name,
            uuid::Uuid::new_v4()
        ));
        p
    }

    #[derive(Debug)]
    struct SanitizeCase {
        name: &'static str,
        input: &'static str,
        expect_err: bool,
    }

    #[test]
    fn sanitize_segment_rejects_traversal_and_separators() {
        let cases = vec![
            SanitizeCase {
                name: "plain",
                input: "report.pdf",
                expect_err: false,
            },
            SanitizeCase {
                name: "dotdot",
                input: "..",
                expect_err: true,
            },
            SanitizeCase {
                name: "leading_slash",
                input: "/etc/passwd",
                expect_err: true,
            },
            SanitizeCase {
                name: "embedded_slash",
                input: "a/b",
                expect_err: true,
            },
            SanitizeCase {
                name: "backslash",
                input: "a\\b",
                expect_err: true,
            },
            SanitizeCase {
                name: "empty",
                input: "",
                expect_err: true,
            },
            SanitizeCase {
                name: "whitespace_only",
                input: "   ",
                expect_err: true,
            },
            SanitizeCase {
                name: "nul_byte",
                input: "a\0b",
                expect_err: true,
            },
        ];
        for case in cases {
            let result = sanitize_segment(case.input, "filename");
            if case.expect_err {
                assert!(
                    result.is_err(),
                    "case `{}` should have errored, got Ok",
                    case.name,
                );
            } else {
                assert!(
                    result.is_ok(),
                    "case `{}` should have succeeded, got {:?}",
                    case.name,
                    result.err(),
                );
            }
        }
    }

    #[tokio::test]
    async fn filesystem_store_retrieve_exists_delete_roundtrip() {
        let root = tempdir("roundtrip");
        let store = FilesystemArtifactStorage::new(&root, "http://localhost:8081");
        let id = "artifact-1";
        let name = "hello.txt";
        let url = store
            .store(id, name, b"hello world".to_vec())
            .await
            .expect("store");
        assert_eq!(url, "http://localhost:8081/artifacts/artifact-1/hello.txt");
        assert!(store.exists(id, name).await.expect("exists"));

        let bytes = store.retrieve(id, name).await.expect("retrieve");
        assert_eq!(bytes, b"hello world");

        store.delete(id, name).await.expect("delete");
        assert!(!store.exists(id, name).await.expect("exists after delete"));
        // deleting again is idempotent
        store.delete(id, name).await.expect("idempotent delete");

        let _ = std::fs::remove_dir_all(&root);
    }

    #[tokio::test]
    async fn filesystem_rejects_traversal_in_store() {
        let root = tempdir("traversal");
        let store = FilesystemArtifactStorage::new(&root, "http://localhost:8081");
        let err = store
            .store("..", "passwd", b"oops".to_vec())
            .await
            .expect_err("traversal must be rejected");
        assert!(err.to_string().contains("artifact_id"));
        let err = store
            .store("ok", "../etc/passwd", b"oops".to_vec())
            .await
            .expect_err("traversal must be rejected");
        assert!(err.to_string().contains("filename"));
        let _ = std::fs::remove_dir_all(&root);
    }

    #[tokio::test]
    async fn filesystem_cleanup_oldest_trims_to_cap() {
        let root = tempdir("cleanup-oldest");
        let store = FilesystemArtifactStorage::new(&root, "http://localhost:8081");
        for i in 0..5 {
            store
                .store(&format!("a{i}"), "f.bin", vec![i as u8])
                .await
                .expect("store");
            tokio::time::sleep(Duration::from_millis(15)).await;
        }
        let removed = store.cleanup_oldest(2).await.expect("cleanup_oldest");
        assert_eq!(removed, 3, "should remove 3 of 5 to leave 2");
        let remaining = store.list().await.expect("list");
        assert_eq!(remaining.len(), 2);
        let _ = std::fs::remove_dir_all(&root);
    }

    #[tokio::test]
    async fn filesystem_cleanup_expired_drops_old_entries() {
        let root = tempdir("cleanup-expired");
        let store = FilesystemArtifactStorage::new(&root, "http://localhost:8081");
        store
            .store("old", "f.bin", b"old".to_vec())
            .await
            .expect("store");
        tokio::time::sleep(Duration::from_millis(60)).await;
        let removed = store
            .cleanup_expired(Duration::from_millis(20))
            .await
            .expect("cleanup_expired");
        assert_eq!(removed, 1);
        assert!(!store.exists("old", "f.bin").await.expect("exists"));
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn url_encodes_special_characters() {
        let store = FilesystemArtifactStorage::new(std::env::temp_dir(), "http://localhost:8081/");
        let url = store.url("id 1", "report v1.pdf");
        assert_eq!(
            url,
            "http://localhost:8081/artifacts/id%201/report%20v1.pdf"
        );
    }
}

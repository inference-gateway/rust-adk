//! Pluggable storage backend for the artifacts subsystem.
//!
//! [`ArtifactStorage`] is the trait an [`ArtifactService`] holds as
//! `Arc<dyn ArtifactStorage>` to persist file/data artifacts produced by
//! task handlers and surface them via HTTP URLs rather than inline base64
//! bytes embedded in JSON-RPC responses.
//!
//! The bundled default is [`FilesystemArtifactStorage`], which lays
//! artifacts out under `<base_path>/<context_id>/<artifact_id>/<filename>`
//! with path-traversal sanitization. Production deployments can implement
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
    pub context_id: String,
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
    /// Persist `data` under `context_id`/`artifact_id`/`filename` and
    /// return the URL at which it can be retrieved.
    async fn store(
        &self,
        context_id: &str,
        artifact_id: &str,
        filename: &str,
        data: Vec<u8>,
    ) -> Result<String>;

    /// Retrieve the raw bytes stored at `context_id`/`artifact_id`/`filename`.
    async fn retrieve(
        &self,
        context_id: &str,
        artifact_id: &str,
        filename: &str,
    ) -> Result<Vec<u8>>;

    /// Whether a blob is stored at `context_id`/`artifact_id`/`filename`.
    async fn exists(&self, context_id: &str, artifact_id: &str, filename: &str) -> Result<bool>;

    /// Delete the blob at `context_id`/`artifact_id`/`filename`. Returns
    /// `Ok(())` if it didn't exist - idempotent.
    async fn delete(&self, context_id: &str, artifact_id: &str, filename: &str) -> Result<()>;

    /// Stable URL the [`ArtifactsServer`] would serve
    /// `context_id`/`artifact_id`/`filename` at.
    ///
    /// [`ArtifactsServer`]: super::artifacts_server::ArtifactsServer
    fn url(&self, context_id: &str, artifact_id: &str, filename: &str) -> String;

    /// Delete every blob whose modified time is older than `max_age`.
    /// Returns the number of blobs removed.
    async fn cleanup_expired(&self, max_age: Duration) -> Result<usize>;

    /// Trim each `contextId` down so at most `max_count` blobs remain
    /// under it, deleting the oldest first. `max_count == 0` means
    /// unlimited - nothing is removed. Returns the number removed.
    async fn cleanup_oldest(&self, max_count: usize) -> Result<usize>;

    /// Enumerate every stored artifact. Used by retention and tests.
    async fn list(&self) -> Result<Vec<StoredArtifactInfo>>;
}

/// Filesystem-backed [`ArtifactStorage`].
///
/// Lays each artifact out under
/// `<base_path>/<context_id>/<artifact_id>/<filename>`.
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

    /// Resolve a sanitized path under `base_path` for
    /// `context_id`/`artifact_id`/`filename`. Rejects empty / traversal /
    /// absolute components so callers can't escape the configured root
    /// via `..` or leading slashes.
    fn resolve_path(&self, context_id: &str, artifact_id: &str, filename: &str) -> Result<PathBuf> {
        let context = sanitize_segment(context_id, "context_id")?;
        let id = sanitize_segment(artifact_id, "artifact_id")?;
        let name = sanitize_segment(filename, "filename")?;
        Ok(self.base_path.join(context).join(id).join(name))
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
    async fn store(
        &self,
        context_id: &str,
        artifact_id: &str,
        filename: &str,
        data: Vec<u8>,
    ) -> Result<String> {
        let target = self.resolve_path(context_id, artifact_id, filename)?;
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
            context_id,
            artifact_id,
            filename,
            bytes = data.len(),
            path = %target.display(),
            "stored artifact on filesystem",
        );
        Ok(self.url(context_id, artifact_id, filename))
    }

    async fn retrieve(
        &self,
        context_id: &str,
        artifact_id: &str,
        filename: &str,
    ) -> Result<Vec<u8>> {
        let target = self.resolve_path(context_id, artifact_id, filename)?;
        fs::read(&target)
            .await
            .with_context(|| format!("failed to read artifact file `{}`", target.display()))
    }

    async fn exists(&self, context_id: &str, artifact_id: &str, filename: &str) -> Result<bool> {
        let target = self.resolve_path(context_id, artifact_id, filename)?;
        match fs::metadata(&target).await {
            Ok(_) => Ok(true),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
            Err(e) => Err(anyhow!(
                "failed to stat artifact `{}`: {e}",
                target.display()
            )),
        }
    }

    async fn delete(&self, context_id: &str, artifact_id: &str, filename: &str) -> Result<()> {
        let target = self.resolve_path(context_id, artifact_id, filename)?;
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
        remove_empty_ancestors(target.parent(), &self.base_path).await;
        Ok(())
    }

    fn url(&self, context_id: &str, artifact_id: &str, filename: &str) -> String {
        format!(
            "{}/artifacts/{}/{}/{}",
            self.base_url,
            urlencode_segment(context_id),
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
                if let Err(e) = self
                    .delete(&entry.context_id, &entry.artifact_id, &entry.filename)
                    .await
                {
                    warn!(
                        context_id = %entry.context_id,
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
        let mut removed = 0usize;
        for entry in surplus_per_context(self.list().await?, max_count) {
            if let Err(e) = self
                .delete(&entry.context_id, &entry.artifact_id, &entry.filename)
                .await
            {
                warn!(
                    context_id = %entry.context_id,
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
        while let Some(context_dir) = top.next_entry().await? {
            if !is_dir(&context_dir).await {
                continue;
            }
            let context_id = context_dir.file_name().to_string_lossy().to_string();
            let Ok(mut artifact_dirs) = fs::read_dir(context_dir.path()).await else {
                continue;
            };
            while let Some(artifact_dir) = artifact_dirs.next_entry().await? {
                if !is_dir(&artifact_dir).await {
                    continue;
                }
                let artifact_id = artifact_dir.file_name().to_string_lossy().to_string();
                let Ok(mut files) = fs::read_dir(artifact_dir.path()).await else {
                    continue;
                };
                while let Some(file) = files.next_entry().await? {
                    let Ok(file_meta) = file.metadata().await else {
                        continue;
                    };
                    if !file_meta.is_file() {
                        continue;
                    }
                    out.push(StoredArtifactInfo {
                        context_id: context_id.clone(),
                        artifact_id: artifact_id.clone(),
                        filename: file.file_name().to_string_lossy().to_string(),
                        size: file_meta.len(),
                        modified: file_meta.modified().ok().map(DateTime::<Utc>::from),
                    });
                }
            }
        }
        Ok(out)
    }
}

async fn is_dir(entry: &tokio::fs::DirEntry) -> bool {
    entry.metadata().await.map(|m| m.is_dir()).unwrap_or(false)
}

/// Walk up from `start`, removing directories that are now empty, and
/// stop at (without removing) `root`. Best-effort - any error ends the walk.
async fn remove_empty_ancestors(start: Option<&Path>, root: &Path) {
    let mut current = start;
    while let Some(dir) = current {
        if dir == root {
            return;
        }
        let Ok(mut read_dir) = fs::read_dir(dir).await else {
            return;
        };
        if read_dir.next_entry().await.ok().flatten().is_some() {
            return;
        }
        if fs::remove_dir(dir).await.is_err() {
            return;
        }
        current = dir.parent();
    }
}

/// Entries to evict so that every `contextId` keeps at most `max_count`
/// artifacts, oldest first. `max_count == 0` means unlimited.
pub(crate) fn surplus_per_context(
    entries: Vec<StoredArtifactInfo>,
    max_count: usize,
) -> Vec<StoredArtifactInfo> {
    if max_count == 0 {
        return Vec::new();
    }
    let mut by_context: std::collections::HashMap<String, Vec<StoredArtifactInfo>> =
        std::collections::HashMap::new();
    for entry in entries {
        by_context
            .entry(entry.context_id.clone())
            .or_default()
            .push(entry);
    }
    let mut surplus = Vec::new();
    for mut group in by_context.into_values() {
        if group.len() <= max_count {
            continue;
        }
        group.sort_by_key(|e| e.modified.unwrap_or_else(Utc::now));
        let drop_count = group.len() - max_count;
        surplus.extend(group.into_iter().take(drop_count));
    }
    surplus
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
        let ctx = "ctx-1";
        let id = "artifact-1";
        let name = "hello.txt";
        let url = store
            .store(ctx, id, name, b"hello world".to_vec())
            .await
            .expect("store");
        assert_eq!(
            url,
            "http://localhost:8081/artifacts/ctx-1/artifact-1/hello.txt"
        );
        assert!(store.exists(ctx, id, name).await.expect("exists"));

        let bytes = store.retrieve(ctx, id, name).await.expect("retrieve");
        assert_eq!(bytes, b"hello world");

        store.delete(ctx, id, name).await.expect("delete");
        assert!(
            !store
                .exists(ctx, id, name)
                .await
                .expect("exists after delete")
        );
        // deleting again is idempotent
        store
            .delete(ctx, id, name)
            .await
            .expect("idempotent delete");

        let _ = std::fs::remove_dir_all(&root);
    }

    #[tokio::test]
    async fn filesystem_rejects_traversal_in_store() {
        let root = tempdir("traversal");
        let store = FilesystemArtifactStorage::new(&root, "http://localhost:8081");
        let err = store
            .store("..", "id", "passwd", b"oops".to_vec())
            .await
            .expect_err("traversal must be rejected");
        assert!(err.to_string().contains("context_id"));
        let err = store
            .store("ctx", "..", "passwd", b"oops".to_vec())
            .await
            .expect_err("traversal must be rejected");
        assert!(err.to_string().contains("artifact_id"));
        let err = store
            .store("ctx", "ok", "../etc/passwd", b"oops".to_vec())
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
                .store("ctx", &format!("a{i}"), "f.bin", vec![i as u8])
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
    async fn filesystem_cleanup_oldest_caps_each_context_independently() {
        let root = tempdir("cleanup-per-context");
        let store = FilesystemArtifactStorage::new(&root, "http://localhost:8081");
        for i in 0..4 {
            store
                .store("busy", &format!("a{i}"), "f.bin", vec![i as u8])
                .await
                .expect("store");
            tokio::time::sleep(Duration::from_millis(15)).await;
        }
        store
            .store("quiet", "only", "f.bin", b"keep".to_vec())
            .await
            .expect("store");

        let removed = store.cleanup_oldest(2).await.expect("cleanup_oldest");
        assert_eq!(removed, 2, "only the busy context is over the cap");

        let remaining = store.list().await.expect("list");
        assert_eq!(remaining.len(), 3);
        assert!(
            remaining.iter().any(|e| e.context_id == "quiet"),
            "a quiet context must not be evicted by a busy one",
        );
        assert_eq!(
            remaining.iter().filter(|e| e.context_id == "busy").count(),
            2,
        );
        let _ = std::fs::remove_dir_all(&root);
    }

    #[tokio::test]
    async fn filesystem_cleanup_oldest_zero_means_unlimited() {
        let root = tempdir("cleanup-oldest-zero");
        let store = FilesystemArtifactStorage::new(&root, "http://localhost:8081");
        for i in 0..3 {
            store
                .store("ctx", &format!("a{i}"), "f.bin", vec![i as u8])
                .await
                .expect("store");
        }
        let removed = store.cleanup_oldest(0).await.expect("cleanup_oldest");
        assert_eq!(removed, 0, "max_count 0 must keep everything");
        assert_eq!(store.list().await.expect("list").len(), 3);
        let _ = std::fs::remove_dir_all(&root);
    }

    #[tokio::test]
    async fn filesystem_cleanup_expired_drops_old_entries() {
        let root = tempdir("cleanup-expired");
        let store = FilesystemArtifactStorage::new(&root, "http://localhost:8081");
        store
            .store("ctx", "old", "f.bin", b"old".to_vec())
            .await
            .expect("store");
        tokio::time::sleep(Duration::from_millis(60)).await;
        let removed = store
            .cleanup_expired(Duration::from_millis(20))
            .await
            .expect("cleanup_expired");
        assert_eq!(removed, 1);
        assert!(!store.exists("ctx", "old", "f.bin").await.expect("exists"));
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn url_encodes_special_characters() {
        let store = FilesystemArtifactStorage::new(std::env::temp_dir(), "http://localhost:8081/");
        let url = store.url("ctx 1", "id 1", "report v1.pdf");
        assert_eq!(
            url,
            "http://localhost:8081/artifacts/ctx%201/id%201/report%20v1.pdf"
        );
    }
}

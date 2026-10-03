//! High-level helpers for producing A2A [`Artifact`]s backed by an
//! [`ArtifactStorage`].
//!
//! Task handlers shouldn't have to reach into the storage backend
//! directly when they want to attach a file or structured-data artifact
//! to a task. [`ArtifactService`] is the thin facade that:
//!
//! - builds an [`Artifact`] with the right [`Part`] shape (text, file, or data),
//! - persists file bytes via [`ArtifactStorage`] when applicable,
//! - returns the URI-bearing [`Artifact`] ready to append to a [`Task`].
//!
//! The default implementation - [`DefaultArtifactService`] - is the one
//! the [`A2AServerBuilder`] wires up automatically when
//! `ARTIFACTS_ENABLED=true`.
//!
//! [`A2AServerBuilder`]: super::server_builder::A2AServerBuilder

use super::artifact_storage::ArtifactStorage;
use crate::a2a_types::{Artifact, Part, PartRaw, Task, Value};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use std::time::Duration;
use tracing::debug;

/// Builds A2A [`Artifact`]s and (for file artifacts) persists their bytes
/// via an [`ArtifactStorage`] so clients can fetch them over HTTP rather
/// than inline base64 in JSON-RPC responses.
#[async_trait]
pub trait ArtifactService: Send + Sync + std::fmt::Debug {
    /// Build a text artifact (single text [`Part`]). No storage I/O.
    fn create_text_artifact(&self, name: &str, description: &str, text: &str) -> Artifact;

    /// Persist `data` via the configured [`ArtifactStorage`] and build
    /// an [`Artifact`] whose single file part carries the resulting
    /// URL. If no storage is configured, the bytes are inlined as
    /// base64 via `file_with_bytes`.
    async fn create_file_artifact(
        &self,
        name: &str,
        description: &str,
        filename: &str,
        data: Vec<u8>,
        mime: Option<&str>,
    ) -> Result<Artifact>;

    /// Build a file-part artifact whose `url` field is
    /// pre-resolved (e.g. produced by a remote service). No storage I/O.
    fn create_file_artifact_from_uri(
        &self,
        name: &str,
        description: &str,
        filename: &str,
        uri: &str,
        mime: Option<&str>,
    ) -> Artifact;

    /// Build a data-part artifact carrying a structured JSON
    /// payload.
    ///
    fn create_data_artifact(
        &self,
        name: &str,
        description: &str,
        data: serde_json::Value,
    ) -> Artifact;

    /// Append `artifact` to `task.artifacts` in place.
    fn add_artifact_to_task(&self, task: &mut Task, artifact: Artifact);

    /// Retrieve raw bytes for `artifact_id`/`filename` via the
    /// configured storage. Errors if no storage is wired up.
    async fn retrieve(&self, artifact_id: &str, filename: &str) -> Result<Vec<u8>>;

    /// Whether a blob is stored at `artifact_id`/`filename`.
    async fn exists(&self, artifact_id: &str, filename: &str) -> Result<bool>;

    /// Drop blobs older than `max_age`. Returns the number removed.
    /// Returns `Ok(0)` when no storage is configured.
    async fn cleanup_expired(&self, max_age: Duration) -> Result<usize>;

    /// Trim to at most `max_count` blobs, `0` meaning unlimited. Returns
    /// the number removed, `Ok(0)` when no storage is configured.
    async fn cleanup_oldest(&self, max_count: usize) -> Result<usize>;

    /// Optional access to the underlying storage. Used by the
    /// artifacts HTTP server to stream blobs to clients.
    fn storage(&self) -> Option<Arc<dyn ArtifactStorage>>;
}

/// The default [`ArtifactService`] implementation - thin wrapper around
/// an [`ArtifactStorage`].
#[derive(Debug, Clone)]
pub struct DefaultArtifactService {
    storage: Option<Arc<dyn ArtifactStorage>>,
}

impl DefaultArtifactService {
    /// Wire the service to an [`ArtifactStorage`]. File artifacts will
    /// be persisted and resolved against this backend.
    pub fn new(storage: Arc<dyn ArtifactStorage>) -> Self {
        Self {
            storage: Some(storage),
        }
    }

    /// Construct a service without any backing storage. File artifacts
    /// will fall back to inline base64 bytes; cleanup is a no-op. This
    /// is mostly useful for tests.
    pub fn without_storage() -> Self {
        Self { storage: None }
    }
}

#[async_trait]
impl ArtifactService for DefaultArtifactService {
    fn create_text_artifact(&self, name: &str, description: &str, text: &str) -> Artifact {
        Artifact {
            artifact_id: uuid::Uuid::new_v4().to_string(),
            description: Some(description.to_string()),
            extensions: vec![],
            metadata: None,
            name: Some(name.to_string()),
            parts: vec![Part {
                text: Some(text.to_string()),
                ..Default::default()
            }],
        }
    }

    async fn create_file_artifact(
        &self,
        name: &str,
        description: &str,
        filename: &str,
        data: Vec<u8>,
        mime: Option<&str>,
    ) -> Result<Artifact> {
        let artifact_id = uuid::Uuid::new_v4().to_string();
        let media_type = Some(
            mime.map(|m| m.to_string())
                .unwrap_or_else(|| infer_mime_type(filename).to_string()),
        );

        let part = match self.storage.as_ref() {
            Some(storage) => {
                let uri = storage.store(&artifact_id, filename, data).await?;
                debug!(
                    artifact_id,
                    filename,
                    uri = %uri,
                    "persisted file artifact via storage backend",
                );
                Part {
                    filename: Some(filename.to_string()),
                    media_type,
                    url: Some(uri),
                    ..Default::default()
                }
            }
            None => {
                let raw = PartRaw::try_from(base64_encode_std(&data)).map_err(|e| {
                    anyhow::anyhow!("failed to encode artifact bytes as base64: {e}")
                })?;
                Part {
                    filename: Some(filename.to_string()),
                    media_type,
                    raw: Some(raw),
                    ..Default::default()
                }
            }
        };

        Ok(Artifact {
            artifact_id,
            description: Some(description.to_string()),
            extensions: vec![],
            metadata: None,
            name: Some(name.to_string()),
            parts: vec![part],
        })
    }

    fn create_file_artifact_from_uri(
        &self,
        name: &str,
        description: &str,
        filename: &str,
        uri: &str,
        mime: Option<&str>,
    ) -> Artifact {
        let media_type = Some(
            mime.map(|m| m.to_string())
                .unwrap_or_else(|| infer_mime_type(filename).to_string()),
        );
        Artifact {
            artifact_id: uuid::Uuid::new_v4().to_string(),
            description: Some(description.to_string()),
            extensions: vec![],
            metadata: None,
            name: Some(name.to_string()),
            parts: vec![Part {
                filename: Some(filename.to_string()),
                media_type,
                url: Some(uri.to_string()),
                ..Default::default()
            }],
        }
    }

    fn create_data_artifact(
        &self,
        name: &str,
        description: &str,
        data: serde_json::Value,
    ) -> Artifact {
        Artifact {
            artifact_id: uuid::Uuid::new_v4().to_string(),
            description: Some(description.to_string()),
            extensions: vec![],
            metadata: None,
            name: Some(name.to_string()),
            parts: vec![Part {
                data: Some(Value(data)),
                ..Default::default()
            }],
        }
    }

    fn add_artifact_to_task(&self, task: &mut Task, artifact: Artifact) {
        task.artifacts.push(artifact);
    }

    async fn retrieve(&self, artifact_id: &str, filename: &str) -> Result<Vec<u8>> {
        let Some(storage) = self.storage.as_ref() else {
            return Err(anyhow::anyhow!(
                "artifact service has no storage backend configured"
            ));
        };
        storage.retrieve(artifact_id, filename).await
    }

    async fn exists(&self, artifact_id: &str, filename: &str) -> Result<bool> {
        let Some(storage) = self.storage.as_ref() else {
            return Ok(false);
        };
        storage.exists(artifact_id, filename).await
    }

    async fn cleanup_expired(&self, max_age: Duration) -> Result<usize> {
        let Some(storage) = self.storage.as_ref() else {
            return Ok(0);
        };
        storage.cleanup_expired(max_age).await
    }

    async fn cleanup_oldest(&self, max_count: usize) -> Result<usize> {
        let Some(storage) = self.storage.as_ref() else {
            return Ok(0);
        };
        storage.cleanup_oldest(max_count).await
    }

    fn storage(&self) -> Option<Arc<dyn ArtifactStorage>> {
        self.storage.clone()
    }
}

/// Best-effort MIME-type inference from a filename extension. Returns
/// `application/octet-stream` as the fallback when the extension is
/// unknown.
pub fn infer_mime_type(filename: &str) -> &'static str {
    let ext = filename
        .rsplit('.')
        .next()
        .unwrap_or("")
        .to_ascii_lowercase();
    match ext.as_str() {
        "txt" | "log" | "md" => "text/plain",
        "html" | "htm" => "text/html",
        "css" => "text/css",
        "csv" => "text/csv",
        "json" => "application/json",
        "yaml" | "yml" => "application/yaml",
        "xml" => "application/xml",
        "pdf" => "application/pdf",
        "zip" => "application/zip",
        "gz" => "application/gzip",
        "tar" => "application/x-tar",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "svg" => "image/svg+xml",
        "mp3" => "audio/mpeg",
        "wav" => "audio/wav",
        "mp4" => "video/mp4",
        "webm" => "video/webm",
        _ => "application/octet-stream",
    }
}

/// Standard-alphabet base64 encoder. We only need this for the
/// fallback `file_with_bytes` path when no storage backend is
/// configured - keeping the implementation local avoids pulling in the
/// `base64` crate.
fn base64_encode_std(bytes: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0];
        let b1 = chunk.get(1).copied().unwrap_or(0);
        let b2 = chunk.get(2).copied().unwrap_or(0);
        let n = ((b0 as u32) << 16) | ((b1 as u32) << 8) | (b2 as u32);
        out.push(ALPHABET[((n >> 18) & 0x3f) as usize] as char);
        out.push(ALPHABET[((n >> 12) & 0x3f) as usize] as char);
        if chunk.len() > 1 {
            out.push(ALPHABET[((n >> 6) & 0x3f) as usize] as char);
        } else {
            out.push('=');
        }
        if chunk.len() > 2 {
            out.push(ALPHABET[(n & 0x3f) as usize] as char);
        } else {
            out.push('=');
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server::artifact_storage::FilesystemArtifactStorage;

    fn tempdir(name: &str) -> std::path::PathBuf {
        let mut p = std::env::temp_dir();
        p.push(format!(
            "rust-adk-art-svc-{}-{}-{}",
            std::process::id(),
            name,
            uuid::Uuid::new_v4()
        ));
        p
    }

    #[derive(Debug)]
    struct MimeCase {
        filename: &'static str,
        expect: &'static str,
    }

    #[test]
    fn infer_mime_type_matches_known_extensions() {
        let cases = vec![
            MimeCase {
                filename: "report.pdf",
                expect: "application/pdf",
            },
            MimeCase {
                filename: "data.json",
                expect: "application/json",
            },
            MimeCase {
                filename: "image.PNG",
                expect: "image/png",
            },
            MimeCase {
                filename: "notes.txt",
                expect: "text/plain",
            },
            MimeCase {
                filename: "no-extension",
                expect: "application/octet-stream",
            },
            MimeCase {
                filename: "config.weird",
                expect: "application/octet-stream",
            },
        ];
        for case in cases {
            assert_eq!(
                infer_mime_type(case.filename),
                case.expect,
                "filename `{}` should map to `{}`",
                case.filename,
                case.expect,
            );
        }
    }

    #[test]
    fn create_text_artifact_emits_single_text_part() {
        let svc = DefaultArtifactService::without_storage();
        let art = svc.create_text_artifact("name", "desc", "hello");
        assert_eq!(art.name.as_deref(), Some("name"));
        assert_eq!(art.description.as_deref(), Some("desc"));
        assert_eq!(art.parts.len(), 1);
        assert_eq!(art.parts[0].text.as_deref(), Some("hello"));
        assert!(art.parts[0].url.is_none() && art.parts[0].raw.is_none());
    }

    #[tokio::test]
    async fn create_file_artifact_uses_storage_uri() {
        let root = tempdir("file-artifact");
        let storage = Arc::new(FilesystemArtifactStorage::new(
            &root,
            "http://localhost:8081",
        ));
        let svc = DefaultArtifactService::new(storage.clone());
        let art = svc
            .create_file_artifact(
                "report",
                "Generated report",
                "report.pdf",
                b"%PDF-1.4 ...".to_vec(),
                None,
            )
            .await
            .expect("create_file_artifact");
        let file_part = &art.parts[0];
        assert_eq!(file_part.media_type.as_deref(), Some("application/pdf"));
        assert_eq!(file_part.filename.as_deref(), Some("report.pdf"));
        let uri = file_part.url.as_ref().expect("url");
        assert!(uri.contains("/artifacts/"));
        assert!(uri.ends_with("/report.pdf"));
        assert!(file_part.raw.is_none());
        let _ = std::fs::remove_dir_all(&root);
    }

    #[tokio::test]
    async fn create_file_artifact_without_storage_falls_back_to_bytes() {
        let svc = DefaultArtifactService::without_storage();
        let art = svc
            .create_file_artifact("name", "desc", "report.pdf", b"abc".to_vec(), None)
            .await
            .expect("create_file_artifact");
        let file_part = &art.parts[0];
        assert!(file_part.url.is_none());
        let bytes = file_part.raw.as_ref().expect("raw bytes");
        let encoded: &str = bytes;
        assert_eq!(encoded, "YWJj");
    }

    #[test]
    fn create_file_artifact_from_uri_emits_url() {
        let svc = DefaultArtifactService::without_storage();
        let art = svc.create_file_artifact_from_uri(
            "image",
            "remote image",
            "pic.png",
            "https://cdn.example.com/pic.png",
            Some("image/png"),
        );
        let file_part = &art.parts[0];
        assert_eq!(
            file_part.url.as_deref(),
            Some("https://cdn.example.com/pic.png")
        );
        assert_eq!(file_part.media_type.as_deref(), Some("image/png"));
    }

    #[test]
    fn create_data_artifact_emits_data_part() {
        let svc = DefaultArtifactService::without_storage();
        let art =
            svc.create_data_artifact("stats", "summary", serde_json::json!({"a": 1, "b": "two"}));
        assert!(
            art.metadata.is_none(),
            "structured payload belongs in the data part, not artifact metadata"
        );
        assert_eq!(art.parts.len(), 1);
        let part = &art.parts[0];
        assert!(part.text.is_none());
        assert!(part.url.is_none() && part.raw.is_none());
        let data = part.data.as_ref().expect("data part");
        assert_eq!(data.get("a").and_then(|v| v.as_i64()), Some(1));
        assert_eq!(data.get("b").and_then(|v| v.as_str()), Some("two"));
    }

    #[test]
    fn create_data_artifact_keeps_non_object_payload() {
        let svc = DefaultArtifactService::without_storage();
        let art = svc.create_data_artifact("nums", "list", serde_json::json!([1, 2, 3]));
        let data = art.parts[0].data.as_ref().expect("data part");
        assert!(data.is_array());
    }
}

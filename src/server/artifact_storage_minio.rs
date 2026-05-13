//! MinIO-backed implementation of [`ArtifactStorage`].
//!
//! Gated by the `minio` Cargo feature. Wraps the official [`minio`] crate
//! to talk to a MinIO server (the on-premises object store).
//!
//! ## URL shape
//!
//! Generated URIs use path-style addressing:
//! `<base_url>/<bucket>/<artifact_id>/<filename>`. Configure
//! [`ArtifactsStorageConfig::base_url`] to point at whichever host the
//! object can actually be read from — typically the MinIO endpoint when
//! the bucket has an anonymous-read policy attached, or the
//! [`ArtifactsServer`] host if you want the artifacts HTTP server to
//! proxy reads through [`ArtifactStorage::retrieve`].
//!
//! [`ArtifactsServer`]: super::artifacts_server::ArtifactsServer
//! [`ArtifactsStorageConfig::base_url`]: crate::config::ArtifactsStorageConfig::base_url

use anyhow::{Result, anyhow};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use futures_util::StreamExt;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tracing::{debug, info, warn};

use minio::s3::MinioClient;
use minio::s3::creds::StaticProvider;
use minio::s3::error::{Error as MinioError, S3ServerError};
use minio::s3::http::BaseUrl;
use minio::s3::segmented_bytes::SegmentedBytes;
use minio::s3::types::{S3Api, ToStream, minio_error_response::MinioErrorCode};

use crate::config::ArtifactsStorageConfig;

use super::artifact_storage::{ArtifactStorage, StoredArtifactInfo, sanitize_segment};

/// MinIO-backed [`ArtifactStorage`].
///
/// Objects are written at `<artifact_id>/<filename>` inside the
/// configured bucket. Generated URLs use path-style addressing
/// (`<base_url>/<bucket>/<artifact_id>/<filename>`).
pub struct MinioArtifactStorage {
    client: Arc<MinioClient>,
    bucket: String,
    base_url: String,
}

impl std::fmt::Debug for MinioArtifactStorage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MinioArtifactStorage")
            .field("bucket", &self.bucket)
            .field("base_url", &self.base_url)
            .finish()
    }
}

impl MinioArtifactStorage {
    /// Construct a new MinIO-backed store. Connects to the endpoint,
    /// creates the bucket if it doesn't already exist, and returns the
    /// wired-up storage handle.
    ///
    /// Required fields on `cfg`:
    /// - `endpoint` — MinIO endpoint host (e.g. `minio:9000`).
    /// - `access_key`, `secret_key` — static credentials.
    /// - `bucket_name` — target bucket.
    ///
    /// `use_ssl` selects http vs https for the endpoint scheme.
    pub async fn from_config(cfg: &ArtifactsStorageConfig) -> Result<Self> {
        let endpoint = cfg.endpoint.as_deref().ok_or_else(|| {
            anyhow!("ARTIFACTS_STORAGE_ENDPOINT must be set for the minio provider")
        })?;
        let access_key = cfg.access_key.as_deref().ok_or_else(|| {
            anyhow!("ARTIFACTS_STORAGE_ACCESS_KEY must be set for the minio provider")
        })?;
        let secret_key = cfg.secret_key.as_deref().ok_or_else(|| {
            anyhow!("ARTIFACTS_STORAGE_SECRET_KEY must be set for the minio provider")
        })?;
        let bucket = cfg.bucket_name.as_deref().ok_or_else(|| {
            anyhow!("ARTIFACTS_STORAGE_BUCKET_NAME must be set for the minio provider")
        })?;

        let scheme = if cfg.use_ssl { "https" } else { "http" };
        let parse_input = if endpoint.contains("://") {
            endpoint.to_string()
        } else {
            format!("{scheme}://{endpoint}")
        };
        let mut base_url: BaseUrl = parse_input
            .parse()
            .map_err(|e| anyhow!("invalid ARTIFACTS_STORAGE_ENDPOINT `{endpoint}`: {e}"))?;
        if let Some(region) = cfg.region.as_deref()
            && !region.is_empty()
        {
            base_url.region = region
                .parse()
                .map_err(|e| anyhow!("invalid ARTIFACTS_STORAGE_REGION `{region}`: {e}"))?;
        }

        let provider = StaticProvider::new(access_key, secret_key, None);
        let client = MinioClient::new(base_url, Some(provider), None, None)
            .map_err(|e| anyhow!("failed to build MinIO client: {e}"))?;

        let exists = client
            .bucket_exists(bucket)
            .map_err(|e| anyhow!("invalid bucket name `{bucket}`: {e}"))?
            .build()
            .send()
            .await
            .map_err(|e| anyhow!("bucket_exists({bucket}) failed: {e}"))?;
        if !exists.exists() {
            info!(bucket, "creating MinIO bucket");
            client
                .create_bucket(bucket)
                .map_err(|e| anyhow!("invalid bucket name `{bucket}`: {e}"))?
                .build()
                .send()
                .await
                .map_err(|e| anyhow!("create_bucket({bucket}) failed: {e}"))?;
        }

        Ok(Self {
            client: Arc::new(client),
            bucket: bucket.to_string(),
            base_url: cfg.base_url.trim_end_matches('/').to_string(),
        })
    }

    fn object_key(&self, artifact_id: &str, filename: &str) -> Result<String> {
        let id = sanitize_segment(artifact_id, "artifact_id")?;
        let name = sanitize_segment(filename, "filename")?;
        Ok(format!("{id}/{name}"))
    }
}

fn is_no_such_key(err: &MinioError) -> bool {
    matches!(
        err,
        MinioError::S3Server(S3ServerError::S3Error(resp))
            if matches!(resp.code(), MinioErrorCode::NoSuchKey | MinioErrorCode::NoSuchBucket)
    )
}

#[async_trait]
impl ArtifactStorage for MinioArtifactStorage {
    async fn store(&self, artifact_id: &str, filename: &str, data: Vec<u8>) -> Result<String> {
        let key = self.object_key(artifact_id, filename)?;
        let bytes_len = data.len();
        let body = SegmentedBytes::from(bytes::Bytes::from(data));
        self.client
            .put_object(self.bucket.clone(), key.clone(), body)
            .map_err(|e| anyhow!("put_object validation failed: {e}"))?
            .build()
            .send()
            .await
            .map_err(|e| anyhow!("put_object({key}) failed: {e}"))?;
        debug!(
            artifact_id,
            filename,
            bytes = bytes_len,
            bucket = %self.bucket,
            key = %key,
            "stored artifact in minio",
        );
        Ok(self.url(artifact_id, filename))
    }

    async fn retrieve(&self, artifact_id: &str, filename: &str) -> Result<Vec<u8>> {
        let key = self.object_key(artifact_id, filename)?;
        let resp = self
            .client
            .get_object(self.bucket.clone(), key.clone())
            .map_err(|e| anyhow!("get_object validation failed: {e}"))?
            .build()
            .send()
            .await
            .map_err(|e| anyhow!("get_object({key}) failed: {e}"))?;
        let bytes = resp
            .into_bytes()
            .await
            .map_err(|e| anyhow!("reading get_object body for {key} failed: {e}"))?;
        Ok(bytes.to_vec())
    }

    async fn exists(&self, artifact_id: &str, filename: &str) -> Result<bool> {
        let key = self.object_key(artifact_id, filename)?;
        let result = self
            .client
            .stat_object(self.bucket.clone(), key.clone())
            .map_err(|e| anyhow!("stat_object validation failed: {e}"))?
            .build()
            .send()
            .await;
        match result {
            Ok(_) => Ok(true),
            Err(e) if is_no_such_key(&e) => Ok(false),
            Err(e) => Err(anyhow!("stat_object({key}) failed: {e}")),
        }
    }

    async fn delete(&self, artifact_id: &str, filename: &str) -> Result<()> {
        let key = self.object_key(artifact_id, filename)?;
        let result = self
            .client
            .delete_object(self.bucket.clone(), key.clone())
            .map_err(|e| anyhow!("delete_object validation failed: {e}"))?
            .build()
            .send()
            .await;
        match result {
            Ok(_) => Ok(()),
            Err(e) if is_no_such_key(&e) => Ok(()),
            Err(e) => Err(anyhow!("delete_object({key}) failed: {e}")),
        }
    }

    fn url(&self, artifact_id: &str, filename: &str) -> String {
        format!(
            "{}/{}/{}/{}",
            self.base_url,
            urlencode_segment(&self.bucket),
            urlencode_segment(artifact_id),
            urlencode_segment(filename),
        )
    }

    async fn cleanup_expired(&self, max_age: Duration) -> Result<usize> {
        let Some(cutoff) = SystemTime::now().checked_sub(max_age) else {
            return Ok(0);
        };
        let cutoff = DateTime::<Utc>::from(cutoff);
        let entries = self.list().await?;
        let mut removed = 0usize;
        for entry in entries {
            let Some(modified) = entry.modified else {
                continue;
            };
            if modified < cutoff {
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
        let mut stream = self
            .client
            .list_objects(self.bucket.clone())
            .map_err(|e| anyhow!("list_objects validation failed: {e}"))?
            .recursive(true)
            .build()
            .to_stream()
            .await;

        let mut out = Vec::new();
        while let Some(page) = stream.next().await {
            let page = page.map_err(|e| anyhow!("list_objects page failed: {e}"))?;
            for item in page.contents {
                if item.is_prefix || item.is_delete_marker {
                    continue;
                }
                let (artifact_id, filename) = match item.name.split_once('/') {
                    Some((id, rest)) if !id.is_empty() && !rest.is_empty() => {
                        (id.to_string(), rest.to_string())
                    }
                    _ => continue,
                };
                out.push(StoredArtifactInfo {
                    artifact_id,
                    filename,
                    size: item.size.unwrap_or(0),
                    modified: item.last_modified,
                });
            }
        }
        Ok(out)
    }
}

fn urlencode_segment(input: &str) -> String {
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

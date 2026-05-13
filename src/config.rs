use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub agent_url: String,
    pub debug: bool,
    pub port: u16,
    pub streaming_status_update_interval: Duration,
    pub agent_config: AgentConfig,
    pub capabilities_config: CapabilitiesConfig,
    pub tls_config: Option<TlsConfig>,
    pub auth_config: Option<AuthConfig>,
    pub queue_config: QueueConfig,
    pub server_config: ServerConfig,
    pub telemetry_config: TelemetryConfig,
    pub artifacts_config: ArtifactsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub provider: String,
    pub model: String,
    pub base_url: Option<String>,
    pub api_key: Option<String>,
    pub timeout: Duration,
    pub max_retries: u32,
    pub max_chat_completion_iterations: u32,
    pub max_tokens: u32,
    pub temperature: f32,
    pub system_prompt: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilitiesConfig {
    pub streaming: bool,
    pub push_notifications: bool,
    pub state_transition_history: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsConfig {
    pub enable: bool,
    pub cert_path: String,
    pub key_path: String,
    /// Path to a PEM file containing trusted client CA certificates.
    /// When `Some(_)`, the server requires every TLS client to present a
    /// certificate signed by one of these CAs (mTLS). When `None`, client
    /// authentication is not requested and `TlsConfig` simply terminates
    /// TLS for the A2A endpoint.
    pub client_ca_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub enable: bool,
    pub issuer_url: String,
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum QueueProvider {
    #[default]
    Memory,
    Redis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueConfig {
    /// Selects which `Storage` backend the server factory wires up.
    pub provider: QueueProvider,
    /// Connection URL for the provider (e.g. `redis://host:6379`).
    /// Required when `provider == Redis`.
    pub url: Option<String>,
    /// Key prefix / namespace for backend keys.
    pub namespace: String,
    /// Number of `DefaultTaskManager` workers draining the queue.
    pub workers: usize,
    /// Max number of in-flight queue entries the in-memory backend will
    /// accept (advisory; current impl does not enforce).
    pub max_size: usize,
    /// Per-operation timeout for backend calls that support timeouts.
    pub timeout: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub tls_enable: bool,
    pub tls_cert_path: Option<String>,
    pub tls_key_path: Option<String>,
    /// Path to a PEM file containing trusted client CA certificates. When
    /// set, the server requires mutual TLS on `POST /a2a` (and any other
    /// route served on the TLS listener).
    pub tls_client_ca_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TelemetryConfig {
    pub enable: bool,
    pub endpoint: Option<String>,
}

/// Top-level configuration for the artifacts subsystem - the optional
/// HTTP server that serves persisted task artifacts, the pluggable
/// storage backend behind it, and the retention policy applied to
/// stored blobs.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ArtifactsConfig {
    /// Whether the artifacts server should be started when
    /// [`A2AServer::serve`](crate::A2AServer::serve) runs.
    pub enable: bool,
    pub server: ArtifactsServerConfig,
    pub storage: ArtifactsStorageConfig,
    pub retention: ArtifactRetentionConfig,
}

/// Listener-side knobs for the artifacts HTTP server. The server runs
/// on its own socket (defaulting to `:8081`) so the main A2A JSON-RPC
/// surface isn't entangled with bulk-download traffic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactsServerConfig {
    pub host: String,
    pub port: u16,
    /// Read timeout for incoming requests.
    pub read_timeout: Duration,
    /// Write timeout for outgoing responses.
    pub write_timeout: Duration,
    /// Optional TLS configuration for the artifacts endpoint. When set,
    /// re-uses the same machinery as the main A2A server.
    pub tls: Option<TlsConfig>,
}

/// Backend selector for the artifacts storage layer.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum ArtifactsStorageProvider {
    #[default]
    Filesystem,
    Minio,
}

/// Storage backend configuration. The full MinIO surface is included
/// here so callers can express the intent via env vars even when the
/// `minio` feature is not compiled in.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactsStorageConfig {
    pub provider: ArtifactsStorageProvider,
    /// Filesystem root for the `Filesystem` provider.
    pub base_path: String,
    /// Public URL prefix the [`ArtifactsServer`](crate::server::ArtifactsServer)
    /// can be reached at - used to build the URI baked into file artifacts.
    pub base_url: String,
    /// Endpoint URL of the MinIO server.
    pub endpoint: Option<String>,
    pub access_key: Option<String>,
    pub secret_key: Option<String>,
    pub bucket_name: Option<String>,
    pub region: Option<String>,
    pub use_ssl: bool,
}

/// Retention policy applied by the background cleanup task.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactRetentionConfig {
    /// Maximum number of artifacts retained per the backend's view.
    pub max_artifacts: usize,
    /// Maximum age of any artifact - older blobs are pruned.
    pub max_age: Duration,
    /// Frequency of the retention loop.
    pub cleanup_interval: Duration,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            agent_url: "http://helloworld-agent:8080".to_string(),
            debug: false,
            port: 8080,
            streaming_status_update_interval: Duration::from_secs(1),
            agent_config: AgentConfig::default(),
            capabilities_config: CapabilitiesConfig::default(),
            tls_config: None,
            auth_config: None,
            queue_config: QueueConfig::default(),
            server_config: ServerConfig::default(),
            telemetry_config: TelemetryConfig::default(),
            artifacts_config: ArtifactsConfig::default(),
        }
    }
}

impl Default for ArtifactsServerConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8081,
            read_timeout: Duration::from_secs(30),
            write_timeout: Duration::from_secs(30),
            tls: None,
        }
    }
}

impl Default for ArtifactsStorageConfig {
    fn default() -> Self {
        Self {
            provider: ArtifactsStorageProvider::Filesystem,
            base_path: "./artifacts".to_string(),
            base_url: "http://localhost:8081".to_string(),
            endpoint: None,
            access_key: None,
            secret_key: None,
            bucket_name: None,
            region: None,
            use_ssl: false,
        }
    }
}

impl Default for ArtifactRetentionConfig {
    fn default() -> Self {
        Self {
            max_artifacts: 5,
            max_age: Duration::from_secs(7 * 24 * 60 * 60),
            cleanup_interval: Duration::from_secs(24 * 60 * 60),
        }
    }
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            provider: "openai".to_string(),
            model: "gpt-4".to_string(),
            base_url: None,
            api_key: None,
            timeout: Duration::from_secs(30),
            max_retries: 3,
            max_chat_completion_iterations: 10,
            max_tokens: 4096,
            temperature: 0.7,
            system_prompt: None,
        }
    }
}

impl Default for CapabilitiesConfig {
    fn default() -> Self {
        Self {
            streaming: true,
            push_notifications: true,
            state_transition_history: false,
        }
    }
}

impl Default for QueueConfig {
    fn default() -> Self {
        Self {
            provider: QueueProvider::Memory,
            url: None,
            namespace: "a2a".to_string(),
            workers: 1,
            max_size: 1000,
            timeout: Duration::from_secs(30),
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8080,
            tls_enable: false,
            tls_cert_path: None,
            tls_key_path: None,
            tls_client_ca_path: None,
        }
    }
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let mut config = Config::default();

        if let Ok(port) = std::env::var("PORT") {
            config.port = port.parse().unwrap_or(8080);
            config.server_config.port = config.port;
        }

        if let Ok(debug) = std::env::var("DEBUG") {
            config.debug = debug.to_lowercase() == "true";
        }

        if let Ok(agent_url) = std::env::var("AGENT_URL") {
            config.agent_url = agent_url;
        }

        if let Ok(provider) = std::env::var("AGENT_CLIENT_PROVIDER") {
            config.agent_config.provider = provider;
        }

        if let Ok(model) = std::env::var("AGENT_CLIENT_MODEL") {
            config.agent_config.model = model;
        }

        if let Ok(base_url) = std::env::var("AGENT_CLIENT_BASE_URL") {
            config.agent_config.base_url = Some(base_url);
        }

        if let Ok(api_key) = std::env::var("AGENT_CLIENT_API_KEY") {
            config.agent_config.api_key = Some(api_key);
        }

        #[allow(clippy::collapsible_if)]
        if let Ok(timeout) = std::env::var("AGENT_CLIENT_TIMEOUT") {
            if let Ok(timeout_secs) = timeout.parse::<u64>() {
                config.agent_config.timeout = Duration::from_secs(timeout_secs);
            }
        }

        #[allow(clippy::collapsible_if)]
        if let Ok(max_retries) = std::env::var("AGENT_CLIENT_MAX_RETRIES") {
            if let Ok(retries) = max_retries.parse::<u32>() {
                config.agent_config.max_retries = retries;
            }
        }

        #[allow(clippy::collapsible_if)]
        if let Ok(max_tokens) = std::env::var("AGENT_CLIENT_MAX_TOKENS") {
            if let Ok(tokens) = max_tokens.parse::<u32>() {
                config.agent_config.max_tokens = tokens;
            }
        }

        #[allow(clippy::collapsible_if)]
        if let Ok(temperature) = std::env::var("AGENT_CLIENT_TEMPERATURE") {
            if let Ok(temp) = temperature.parse::<f32>() {
                config.agent_config.temperature = temp;
            }
        }

        if let Ok(system_prompt) = std::env::var("AGENT_CLIENT_SYSTEM_PROMPT") {
            config.agent_config.system_prompt = Some(system_prompt);
        }

        #[allow(clippy::collapsible_if)]
        if let Ok(max_iterations) = std::env::var("AGENT_CLIENT_MAX_CHAT_COMPLETION_ITERATIONS") {
            if let Ok(iterations) = max_iterations.parse::<u32>() {
                config.agent_config.max_chat_completion_iterations = iterations;
            }
        }

        if let Ok(streaming) = std::env::var("CAPABILITIES_STREAMING") {
            config.capabilities_config.streaming = streaming.to_lowercase() == "true";
        }

        if let Ok(push_notifications) = std::env::var("CAPABILITIES_PUSH_NOTIFICATIONS") {
            config.capabilities_config.push_notifications =
                push_notifications.to_lowercase() == "true";
        }

        if let Ok(state_history) = std::env::var("CAPABILITIES_STATE_TRANSITION_HISTORY") {
            config.capabilities_config.state_transition_history =
                state_history.to_lowercase() == "true";
        }

        #[allow(clippy::collapsible_if)]
        if let Ok(tls_enable) = std::env::var("SERVER_TLS_ENABLE") {
            if tls_enable.to_lowercase() == "true" {
                let client_ca_path = std::env::var("SERVER_TLS_CLIENT_CA_PATH")
                    .ok()
                    .filter(|s| !s.is_empty());
                config.tls_config = Some(TlsConfig {
                    enable: true,
                    cert_path: std::env::var("SERVER_TLS_CERT_PATH").unwrap_or_default(),
                    key_path: std::env::var("SERVER_TLS_KEY_PATH").unwrap_or_default(),
                    client_ca_path: client_ca_path.clone(),
                });
                config.server_config.tls_enable = true;
                config.server_config.tls_cert_path =
                    Some(config.tls_config.as_ref().unwrap().cert_path.clone());
                config.server_config.tls_key_path =
                    Some(config.tls_config.as_ref().unwrap().key_path.clone());
                config.server_config.tls_client_ca_path = client_ca_path;
            }
        }

        #[allow(clippy::collapsible_if)]
        if let Ok(auth_enable) = std::env::var("AUTH_ENABLE") {
            if auth_enable.to_lowercase() == "true" {
                config.auth_config = Some(AuthConfig {
                    enable: true,
                    issuer_url: std::env::var("AUTH_ISSUER_URL").unwrap_or_default(),
                    client_id: std::env::var("AUTH_CLIENT_ID").unwrap_or_default(),
                    client_secret: std::env::var("AUTH_CLIENT_SECRET").unwrap_or_default(),
                });
            }
        }

        if let Ok(provider) = std::env::var("A2A_QUEUE_PROVIDER") {
            config.queue_config.provider = match provider.to_lowercase().as_str() {
                "redis" => QueueProvider::Redis,
                "memory" | "" => QueueProvider::Memory,
                other => {
                    return Err(format!(
                        "A2A_QUEUE_PROVIDER must be one of `memory` or `redis` (got {other:?})"
                    )
                    .into());
                }
            };
        }
        if let Ok(url) = std::env::var("A2A_QUEUE_URL") {
            config.queue_config.url = Some(url);
        }
        if let Ok(ns) = std::env::var("A2A_QUEUE_NAMESPACE") {
            config.queue_config.namespace = ns;
        }
        if let Ok(workers) = std::env::var("A2A_QUEUE_WORKERS")
            && let Ok(n) = workers.parse::<usize>()
        {
            config.queue_config.workers = n.max(1);
        }
        if let Ok(max) = std::env::var("A2A_QUEUE_MAX_SIZE")
            && let Ok(n) = max.parse::<usize>()
        {
            config.queue_config.max_size = n;
        }

        if let Ok(enable) = std::env::var("ARTIFACTS_ENABLE") {
            config.artifacts_config.enable = enable.to_lowercase() == "true";
        }

        if let Ok(host) = std::env::var("ARTIFACTS_SERVER_HOST") {
            config.artifacts_config.server.host = host;
        }
        if let Ok(port) = std::env::var("ARTIFACTS_SERVER_PORT")
            && let Ok(n) = port.parse::<u16>()
        {
            config.artifacts_config.server.port = n;
        }
        if let Ok(rt) = std::env::var("ARTIFACTS_SERVER_READ_TIMEOUT")
            && let Some(d) = parse_duration(&rt)
        {
            config.artifacts_config.server.read_timeout = d;
        }
        if let Ok(wt) = std::env::var("ARTIFACTS_SERVER_WRITE_TIMEOUT")
            && let Some(d) = parse_duration(&wt)
        {
            config.artifacts_config.server.write_timeout = d;
        }

        if let Ok(provider) = std::env::var("ARTIFACTS_STORAGE_PROVIDER") {
            config.artifacts_config.storage.provider = match provider.to_lowercase().as_str() {
                "minio" => ArtifactsStorageProvider::Minio,
                "filesystem" | "fs" | "" => ArtifactsStorageProvider::Filesystem,
                other => {
                    return Err(format!(
                        "ARTIFACTS_STORAGE_PROVIDER must be one of `filesystem` or `minio` (got {other:?})"
                    )
                    .into());
                }
            };
        }
        if let Ok(base_path) = std::env::var("ARTIFACTS_STORAGE_BASE_PATH") {
            config.artifacts_config.storage.base_path = base_path;
        }
        if let Ok(base_url) = std::env::var("ARTIFACTS_STORAGE_BASE_URL") {
            config.artifacts_config.storage.base_url = base_url;
        }
        if let Ok(endpoint) = std::env::var("ARTIFACTS_STORAGE_ENDPOINT") {
            config.artifacts_config.storage.endpoint = Some(endpoint);
        }
        if let Ok(ak) = std::env::var("ARTIFACTS_STORAGE_ACCESS_KEY") {
            config.artifacts_config.storage.access_key = Some(ak);
        }
        if let Ok(sk) = std::env::var("ARTIFACTS_STORAGE_SECRET_KEY") {
            config.artifacts_config.storage.secret_key = Some(sk);
        }
        if let Ok(bucket) = std::env::var("ARTIFACTS_STORAGE_BUCKET_NAME") {
            config.artifacts_config.storage.bucket_name = Some(bucket);
        }
        if let Ok(region) = std::env::var("ARTIFACTS_STORAGE_REGION") {
            config.artifacts_config.storage.region = Some(region);
        }
        if let Ok(use_ssl) = std::env::var("ARTIFACTS_STORAGE_USE_SSL") {
            config.artifacts_config.storage.use_ssl = use_ssl.to_lowercase() == "true";
        }

        if let Ok(max) = std::env::var("ARTIFACTS_RETENTION_MAX_ARTIFACTS")
            && let Ok(n) = max.parse::<usize>()
        {
            config.artifacts_config.retention.max_artifacts = n;
        }
        if let Ok(age) = std::env::var("ARTIFACTS_RETENTION_MAX_AGE")
            && let Some(d) = parse_duration(&age)
        {
            config.artifacts_config.retention.max_age = d;
        }
        if let Ok(interval) = std::env::var("ARTIFACTS_RETENTION_CLEANUP_INTERVAL")
            && let Some(d) = parse_duration(&interval)
        {
            config.artifacts_config.retention.cleanup_interval = d;
        }

        Ok(config)
    }
}

/// Tiny Go-style duration parser, sufficient for the
/// `ARTIFACTS_RETENTION_*` env vars. Accepts plain seconds (`30`),
/// suffixed values (`30s`, `15m`, `2h`, `7d`), or a comma/space-separated
/// composite (`1h30m`). Returns `None` on any parse failure.
fn parse_duration(s: &str) -> Option<Duration> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }
    // Plain integer = seconds (matches the rest of the codebase).
    if let Ok(secs) = s.parse::<u64>() {
        return Some(Duration::from_secs(secs));
    }
    let mut total: u64 = 0;
    let mut num = String::new();
    for ch in s.chars() {
        if ch.is_ascii_digit() {
            num.push(ch);
            continue;
        }
        let n: u64 = num.parse().ok()?;
        num.clear();
        let mul = match ch {
            's' => 1,
            'm' => 60,
            'h' => 60 * 60,
            'd' => 60 * 60 * 24,
            _ => return None,
        };
        total = total.checked_add(n.checked_mul(mul)?)?;
    }
    if !num.is_empty() {
        // Trailing number with no unit -> seconds.
        let n: u64 = num.parse().ok()?;
        total = total.checked_add(n)?;
    }
    Some(Duration::from_secs(total))
}

#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub base_url: String,
    pub timeout: Duration,
    pub max_retries: u32,
}

impl ClientConfig {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            timeout: Duration::from_secs(30),
            max_retries: 3,
        }
    }
}

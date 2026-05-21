use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::time::Duration;

/// Deserialization helpers that accept both native types AND string
/// representations of those types. Needed because `serde(flatten)` buffers
/// fields via `deserialize_any`, and `envy` exposes every env var as a
/// string — without these helpers, `A2A_SERVER_PORT=8080` fails to coerce
/// into the `port: u16` field that lives inside a flattened sub-struct.
///
/// All helpers fall back to native deserialization too, so JSON / YAML /
/// any other format that drives this `Config` keeps working.
mod de {
    macro_rules! int_helper {
        ($mod_name:ident, $ty:ty) => {
            pub mod $mod_name {
                pub fn deserialize<'de, D>(d: D) -> Result<$ty, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    struct V;
                    impl<'de> serde::de::Visitor<'de> for V {
                        type Value = $ty;
                        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                            f.write_str(concat!(
                                stringify!($ty),
                                " (native or string representation)"
                            ))
                        }
                        fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<$ty, E> {
                            <$ty>::try_from(v).map_err(serde::de::Error::custom)
                        }
                        fn visit_i64<E: serde::de::Error>(self, v: i64) -> Result<$ty, E> {
                            <$ty>::try_from(v).map_err(serde::de::Error::custom)
                        }
                        fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<$ty, E> {
                            v.parse().map_err(serde::de::Error::custom)
                        }
                        fn visit_string<E: serde::de::Error>(self, v: String) -> Result<$ty, E> {
                            self.visit_str(&v)
                        }
                    }
                    d.deserialize_any(V)
                }
            }
        };
    }

    int_helper!(u16, u16);
    int_helper!(u32, u32);
    int_helper!(u64, u64);
    int_helper!(usize, usize);

    pub mod boolean {
        pub fn deserialize<'de, D>(d: D) -> Result<bool, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            struct V;
            impl<'de> serde::de::Visitor<'de> for V {
                type Value = bool;
                fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    f.write_str("bool (native or string representation)")
                }
                fn visit_bool<E: serde::de::Error>(self, v: bool) -> Result<bool, E> {
                    Ok(v)
                }
                fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<bool, E> {
                    match v.to_ascii_lowercase().as_str() {
                        "true" | "1" | "yes" | "on" => Ok(true),
                        "false" | "0" | "no" | "off" | "" => Ok(false),
                        other => Err(serde::de::Error::custom(format!("invalid bool: {other:?}"))),
                    }
                }
                fn visit_string<E: serde::de::Error>(self, v: String) -> Result<bool, E> {
                    self.visit_str(&v)
                }
            }
            d.deserialize_any(V)
        }
    }

    pub mod float32 {
        pub fn deserialize<'de, D>(d: D) -> Result<f32, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            struct V;
            impl<'de> serde::de::Visitor<'de> for V {
                type Value = f32;
                fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    f.write_str("f32 (native or string representation)")
                }
                fn visit_f64<E: serde::de::Error>(self, v: f64) -> Result<f32, E> {
                    Ok(v as f32)
                }
                fn visit_i64<E: serde::de::Error>(self, v: i64) -> Result<f32, E> {
                    Ok(v as f32)
                }
                fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<f32, E> {
                    Ok(v as f32)
                }
                fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<f32, E> {
                    v.parse().map_err(serde::de::Error::custom)
                }
                fn visit_string<E: serde::de::Error>(self, v: String) -> Result<f32, E> {
                    self.visit_str(&v)
                }
            }
            d.deserialize_any(V)
        }
    }

    pub mod queue_provider {
        use crate::config::QueueProvider;
        use std::str::FromStr;
        pub fn deserialize<'de, D>(d: D) -> Result<QueueProvider, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            struct V;
            impl<'de> serde::de::Visitor<'de> for V {
                type Value = QueueProvider;
                fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    f.write_str("`memory` or `redis`")
                }
                fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<QueueProvider, E> {
                    QueueProvider::from_str(v).map_err(serde::de::Error::custom)
                }
                fn visit_string<E: serde::de::Error>(self, v: String) -> Result<QueueProvider, E> {
                    self.visit_str(&v)
                }
            }
            d.deserialize_any(V)
        }
    }
}

/// Top-level ADK configuration.
///
/// The library does not load environment variables itself. Consumers pick a
/// loader (e.g. [`envy::prefixed("A2A_").from_env::<Config>()`][envy-prefixed])
/// and pass the resulting [`Config`] to [`A2AServerBuilder::with_config`].
///
/// Each leaf field carries a `#[serde(rename = "...")]` tag whose value is the
/// **flat, lowercase, unprefixed** env var name. With `envy::prefixed("A2A_")`
/// that turns into `A2A_<UPPERCASE_RENAME>` on the wire, matching the Go ADK's
/// `A2A_*` convention. Switch prefixes by changing the loader, not the tags.
///
/// All structs derive `Default`/`#[serde(default)]`, so any unset env var falls
/// back to the value in the `Default` impl below.
///
/// [`A2AServerBuilder::with_config`]: crate::server::A2AServerBuilder::with_config
/// [envy-prefixed]: https://docs.rs/envy/latest/envy/fn.prefixed.html
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub agent_url: String,

    pub debug: bool,

    pub streaming_status_update_interval_secs: u64,

    #[serde(flatten)]
    pub agent_config: AgentConfig,

    #[serde(flatten)]
    pub capabilities_config: CapabilitiesConfig,

    #[serde(flatten)]
    pub tls_config: TlsConfig,

    #[serde(flatten)]
    pub auth_config: AuthConfig,

    #[serde(flatten)]
    pub queue_config: QueueConfig,

    #[serde(flatten)]
    pub server_config: ServerConfig,

    #[serde(flatten)]
    pub telemetry_config: TelemetryConfig,
    pub artifacts_config: ArtifactsConfig,
}

impl Config {
    pub fn streaming_status_update_interval(&self) -> Duration {
        Duration::from_secs(self.streaming_status_update_interval_secs)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AgentConfig {
    #[serde(rename = "agent_client_provider")]
    pub provider: String,

    #[serde(rename = "agent_client_model")]
    pub model: String,

    #[serde(rename = "agent_client_base_url")]
    pub base_url: Option<String>,

    #[serde(rename = "agent_client_api_key")]
    pub api_key: Option<String>,

    #[serde(
        rename = "agent_client_timeout_secs",
        deserialize_with = "de::u64::deserialize"
    )]
    pub timeout_secs: u64,

    #[serde(
        rename = "agent_client_max_retries",
        deserialize_with = "de::u32::deserialize"
    )]
    pub max_retries: u32,

    #[serde(
        rename = "agent_client_max_chat_completion_iterations",
        deserialize_with = "de::u32::deserialize"
    )]
    pub max_chat_completion_iterations: u32,

    #[serde(
        rename = "agent_client_max_tokens",
        deserialize_with = "de::u32::deserialize"
    )]
    pub max_tokens: u32,

    #[serde(
        rename = "agent_client_temperature",
        deserialize_with = "de::float32::deserialize"
    )]
    pub temperature: f32,

    #[serde(rename = "agent_client_system_prompt")]
    pub system_prompt: Option<String>,
}

impl AgentConfig {
    pub fn timeout(&self) -> Duration {
        Duration::from_secs(self.timeout_secs)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CapabilitiesConfig {
    #[serde(
        rename = "capabilities_streaming",
        deserialize_with = "de::boolean::deserialize"
    )]
    pub streaming: bool,

    #[serde(
        rename = "capabilities_push_notifications",
        deserialize_with = "de::boolean::deserialize"
    )]
    pub push_notifications: bool,

    #[serde(
        rename = "capabilities_state_transition_history",
        deserialize_with = "de::boolean::deserialize"
    )]
    pub state_transition_history: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct TlsConfig {
    #[serde(
        rename = "server_tls_enable",
        deserialize_with = "de::boolean::deserialize"
    )]
    pub enable: bool,

    #[serde(rename = "server_tls_cert_path")]
    pub cert_path: String,

    #[serde(rename = "server_tls_key_path")]
    pub key_path: String,

    /// Path to a PEM file containing trusted client CA certificates.
    /// When `Some(_)`, the server requires every TLS client to present a
    /// certificate signed by one of these CAs (mTLS). When `None`, client
    /// authentication is not requested and `TlsConfig` simply terminates
    /// TLS for the A2A endpoint.
    #[serde(rename = "server_tls_client_ca_path")]
    pub client_ca_path: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct AuthConfig {
    #[serde(rename = "auth_enable", deserialize_with = "de::boolean::deserialize")]
    pub enable: bool,

    #[serde(rename = "auth_issuer_url")]
    pub issuer_url: String,

    #[serde(rename = "auth_client_id")]
    pub client_id: String,

    #[serde(rename = "auth_client_secret")]
    pub client_secret: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum QueueProvider {
    #[default]
    Memory,
    Redis,
}

impl FromStr for QueueProvider {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "memory" | "" => Ok(QueueProvider::Memory),
            "redis" => Ok(QueueProvider::Redis),
            other => Err(format!(
                "QUEUE_PROVIDER must be `memory` or `redis` (got {other:?})"
            )),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct QueueConfig {
    /// Selects which `Storage` backend the server factory wires up.
    #[serde(
        rename = "queue_provider",
        deserialize_with = "de::queue_provider::deserialize"
    )]
    pub provider: QueueProvider,

    /// Connection URL for the provider (e.g. `redis://host:6379`).
    /// Required when `provider == Redis`.
    #[serde(rename = "queue_url")]
    pub url: Option<String>,

    /// Key prefix / namespace for backend keys.
    #[serde(rename = "queue_namespace")]
    pub namespace: String,

    /// Number of `DefaultTaskManager` workers draining the queue.
    #[serde(rename = "queue_workers", deserialize_with = "de::usize::deserialize")]
    pub workers: usize,

    /// Max number of in-flight queue entries the in-memory backend will
    /// accept (advisory; current impl does not enforce).
    #[serde(rename = "queue_max_size", deserialize_with = "de::usize::deserialize")]
    pub max_size: usize,

    /// Per-operation timeout for backend calls, in seconds.
    #[serde(
        rename = "queue_timeout_secs",
        deserialize_with = "de::u64::deserialize"
    )]
    pub timeout_secs: u64,
}

impl QueueConfig {
    pub fn timeout(&self) -> Duration {
        Duration::from_secs(self.timeout_secs)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ServerConfig {
    #[serde(rename = "server_host")]
    pub host: String,

    #[serde(rename = "server_port", deserialize_with = "de::u16::deserialize")]
    pub port: u16,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct TelemetryConfig {
    #[serde(
        rename = "telemetry_enable",
        deserialize_with = "de::boolean::deserialize"
    )]
    pub enable: bool,

    #[serde(rename = "telemetry_endpoint")]
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
            streaming_status_update_interval_secs: 1,
            agent_config: AgentConfig::default(),
            capabilities_config: CapabilitiesConfig::default(),
            tls_config: TlsConfig::default(),
            auth_config: AuthConfig::default(),
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
            provider: String::new(),
            model: String::new(),
            base_url: None,
            api_key: None,
            timeout_secs: 30,
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
            timeout_secs: 30,
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8080,
        }
    }
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

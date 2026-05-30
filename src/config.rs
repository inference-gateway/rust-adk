use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::time::Duration;

/// Deserialization helpers that accept both native types AND string
/// representations of those types. Needed because `serde(flatten)` buffers
/// fields via `deserialize_any`, and `envy` exposes every env var as a
/// string - without these helpers, `A2A_SERVER_PORT=8080` fails to coerce
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

    pub mod duration {
        use std::time::Duration;
        pub fn deserialize<'de, D>(d: D) -> Result<Duration, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            struct V;
            impl<'de> serde::de::Visitor<'de> for V {
                type Value = Duration;
                fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    f.write_str(
                        "a duration (`30s`, `15m`, `2h`, `7d`, bare seconds, or {secs, nanos})",
                    )
                }
                fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<Duration, E> {
                    Ok(Duration::from_secs(v))
                }
                fn visit_i64<E: serde::de::Error>(self, v: i64) -> Result<Duration, E> {
                    u64::try_from(v)
                        .map(Duration::from_secs)
                        .map_err(serde::de::Error::custom)
                }
                fn visit_f64<E: serde::de::Error>(self, v: f64) -> Result<Duration, E> {
                    if v < 0.0 {
                        return Err(serde::de::Error::custom("duration cannot be negative"));
                    }
                    Ok(Duration::from_secs_f64(v))
                }
                fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Duration, E> {
                    parse(v).map_err(serde::de::Error::custom)
                }
                fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Duration, E> {
                    self.visit_str(&v)
                }
                fn visit_map<A>(self, map: A) -> Result<Duration, A::Error>
                where
                    A: serde::de::MapAccess<'de>,
                {
                    serde::Deserialize::deserialize(serde::de::value::MapAccessDeserializer::new(
                        map,
                    ))
                }
            }
            d.deserialize_any(V)
        }

        /// A bare integer is seconds; otherwise a single `s`/`m`/`h`/`d`
        /// suffix scales the leading integer.
        fn parse(s: &str) -> Result<Duration, String> {
            let s = s.trim();
            if s.is_empty() {
                return Ok(Duration::ZERO);
            }
            if let Ok(secs) = s.parse::<u64>() {
                return Ok(Duration::from_secs(secs));
            }
            let (value, unit) = s.split_at(s.len() - 1);
            let scale: u64 = match unit {
                "s" => 1,
                "m" => 60,
                "h" => 60 * 60,
                "d" => 24 * 60 * 60,
                _ => {
                    return Err(format!(
                        "invalid duration {s:?}: expected `s`/`m`/`h`/`d` suffix or bare seconds"
                    ));
                }
            };
            let value: u64 = value
                .trim()
                .parse()
                .map_err(|_| format!("invalid duration {s:?}"))?;
            value
                .checked_mul(scale)
                .map(Duration::from_secs)
                .ok_or_else(|| format!("duration {s:?} overflows"))
        }
    }

    pub mod artifacts_storage_provider {
        use crate::config::ArtifactsStorageProvider;
        use std::str::FromStr;
        pub fn deserialize<'de, D>(d: D) -> Result<ArtifactsStorageProvider, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            struct V;
            impl<'de> serde::de::Visitor<'de> for V {
                type Value = ArtifactsStorageProvider;
                fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    f.write_str("`filesystem` or `minio`")
                }
                fn visit_str<E: serde::de::Error>(
                    self,
                    v: &str,
                ) -> Result<ArtifactsStorageProvider, E> {
                    ArtifactsStorageProvider::from_str(v).map_err(serde::de::Error::custom)
                }
                fn visit_string<E: serde::de::Error>(
                    self,
                    v: String,
                ) -> Result<ArtifactsStorageProvider, E> {
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

    /// Artifacts subsystem. Loaded separately under the `ARTIFACTS_`
    /// prefix (see [`ArtifactsConfig`]) rather than flattened into the
    /// `A2A_` surface, so it is intentionally **not** populated by an
    /// `envy::prefixed("A2A_")` load of `Config`.
    #[serde(skip)]
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

    /// Whether the default task handlers attach usage metadata (token counts
    /// and execution statistics) to a task's `metadata` once it reaches a
    /// terminal state. Loaded from `A2A_AGENT_CLIENT_ENABLE_USAGE_METADATA`.
    #[serde(
        rename = "agent_client_enable_usage_metadata",
        deserialize_with = "de::boolean::deserialize"
    )]
    pub enable_usage_metadata: bool,
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
///
/// Unlike the rest of [`Config`] (which loads under the `A2A_` prefix),
/// this sub-config uses its own `ARTIFACTS_` prefix to match the Go ADK
/// and the bundled examples. Load it independently and assign the result
/// onto [`Config::artifacts_config`]:
///
/// ```no_run
/// # use inference_gateway_adk::{ArtifactsConfig, Config};
/// let mut config = Config::default();
/// config.artifacts_config = envy::prefixed("ARTIFACTS_")
///     .from_env::<ArtifactsConfig>()
///     .unwrap_or_default();
/// ```
///
/// The nested fields are `#[serde(flatten)]`-ed so flat env vars such as
/// `ARTIFACTS_SERVER_PORT`, `ARTIFACTS_STORAGE_PROVIDER` and
/// `ARTIFACTS_RETENTION_MAX_AGE` populate the right leaf.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ArtifactsConfig {
    /// Whether the artifacts server should be started when
    /// [`A2AServer::serve`](crate::A2AServer::serve) runs.
    #[serde(deserialize_with = "de::boolean::deserialize")]
    pub enable: bool,

    #[serde(flatten)]
    pub server: ArtifactsServerConfig,

    #[serde(flatten)]
    pub storage: ArtifactsStorageConfig,

    #[serde(flatten)]
    pub retention: ArtifactRetentionConfig,
}

/// Listener-side knobs for the artifacts HTTP server. The server runs
/// on its own socket (defaulting to `:8081`) so the main A2A JSON-RPC
/// surface isn't entangled with bulk-download traffic.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ArtifactsServerConfig {
    #[serde(rename = "server_host")]
    pub host: String,

    #[serde(rename = "server_port", deserialize_with = "de::u16::deserialize")]
    pub port: u16,

    /// Read timeout for incoming requests.
    #[serde(
        rename = "server_read_timeout",
        deserialize_with = "de::duration::deserialize"
    )]
    pub read_timeout: Duration,

    /// Write timeout for outgoing responses.
    #[serde(
        rename = "server_write_timeout",
        deserialize_with = "de::duration::deserialize"
    )]
    pub write_timeout: Duration,

    /// Optional TLS configuration for the artifacts endpoint. When set,
    /// re-uses the same machinery as the main A2A server. Configured
    /// programmatically only - not via the `ARTIFACTS_` env surface.
    #[serde(skip)]
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

impl FromStr for ArtifactsStorageProvider {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "filesystem" | "" => Ok(ArtifactsStorageProvider::Filesystem),
            "minio" => Ok(ArtifactsStorageProvider::Minio),
            other => Err(format!(
                "ARTIFACTS_STORAGE_PROVIDER must be `filesystem` or `minio` (got {other:?})"
            )),
        }
    }
}

/// Storage backend configuration. The full MinIO surface is included
/// here so callers can express the intent via env vars even when the
/// `minio` feature is not compiled in.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ArtifactsStorageConfig {
    #[serde(
        rename = "storage_provider",
        deserialize_with = "de::artifacts_storage_provider::deserialize"
    )]
    pub provider: ArtifactsStorageProvider,

    /// Filesystem root for the `Filesystem` provider.
    #[serde(rename = "storage_base_path")]
    pub base_path: String,

    /// Public URL prefix the [`ArtifactsServer`](crate::server::ArtifactsServer)
    /// can be reached at - used to build the URI baked into file artifacts.
    #[serde(rename = "storage_base_url")]
    pub base_url: String,

    /// Endpoint URL of the MinIO server.
    #[serde(rename = "storage_endpoint")]
    pub endpoint: Option<String>,

    #[serde(rename = "storage_access_key")]
    pub access_key: Option<String>,

    #[serde(rename = "storage_secret_key")]
    pub secret_key: Option<String>,

    #[serde(rename = "storage_bucket_name")]
    pub bucket_name: Option<String>,

    #[serde(rename = "storage_region")]
    pub region: Option<String>,

    #[serde(
        rename = "storage_use_ssl",
        deserialize_with = "de::boolean::deserialize"
    )]
    pub use_ssl: bool,
}

/// Retention policy applied by the background cleanup task.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ArtifactRetentionConfig {
    /// Maximum number of artifacts retained per the backend's view.
    #[serde(
        rename = "retention_max_artifacts",
        deserialize_with = "de::usize::deserialize"
    )]
    pub max_artifacts: usize,

    /// Maximum age of any artifact - older blobs are pruned.
    #[serde(
        rename = "retention_max_age",
        deserialize_with = "de::duration::deserialize"
    )]
    pub max_age: Duration,

    /// Frequency of the retention loop.
    #[serde(
        rename = "retention_cleanup_interval",
        deserialize_with = "de::duration::deserialize"
    )]
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
            enable_usage_metadata: true,
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

#[cfg(test)]
mod tests {
    use super::*;

    fn load(vars: &[(&str, &str)]) -> ArtifactsConfig {
        let owned = vars
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect::<Vec<_>>();
        envy::prefixed("ARTIFACTS_")
            .from_iter::<_, ArtifactsConfig>(owned)
            .expect("ArtifactsConfig should load from ARTIFACTS_* vars")
    }

    #[test]
    fn artifacts_config_loads_full_env_surface() {
        let cfg = load(&[
            ("ARTIFACTS_ENABLE", "true"),
            ("ARTIFACTS_SERVER_HOST", "127.0.0.1"),
            ("ARTIFACTS_SERVER_PORT", "9099"),
            ("ARTIFACTS_STORAGE_PROVIDER", "minio"),
            ("ARTIFACTS_STORAGE_BASE_PATH", "/data/artifacts"),
            ("ARTIFACTS_STORAGE_BASE_URL", "http://server:8088"),
            ("ARTIFACTS_STORAGE_ENDPOINT", "http://minio:9000"),
            ("ARTIFACTS_STORAGE_USE_SSL", "true"),
            ("ARTIFACTS_RETENTION_MAX_ARTIFACTS", "7"),
            ("ARTIFACTS_RETENTION_MAX_AGE", "168h"),
            ("ARTIFACTS_RETENTION_CLEANUP_INTERVAL", "24h"),
        ]);

        assert!(cfg.enable);
        assert_eq!(cfg.server.host, "127.0.0.1");
        assert_eq!(cfg.server.port, 9099);
        assert_eq!(cfg.storage.provider, ArtifactsStorageProvider::Minio);
        assert_eq!(cfg.storage.base_path, "/data/artifacts");
        assert_eq!(cfg.storage.base_url, "http://server:8088");
        assert_eq!(cfg.storage.endpoint.as_deref(), Some("http://minio:9000"));
        assert!(cfg.storage.use_ssl);
        assert_eq!(cfg.retention.max_artifacts, 7);
        assert_eq!(cfg.retention.max_age, Duration::from_secs(168 * 60 * 60));
        assert_eq!(
            cfg.retention.cleanup_interval,
            Duration::from_secs(24 * 60 * 60)
        );
    }

    #[test]
    fn artifacts_config_partial_env_falls_back_to_defaults() {
        let cfg = load(&[("ARTIFACTS_ENABLE", "1")]);
        assert!(cfg.enable);
        // Unset leaves keep ArtifactsConfig defaults (Go-matching).
        assert_eq!(cfg.server.port, 8081);
        assert_eq!(cfg.storage.provider, ArtifactsStorageProvider::Filesystem);
        assert_eq!(cfg.storage.base_path, "./artifacts");
        assert_eq!(cfg.retention.max_artifacts, 5);
        assert_eq!(cfg.retention.max_age, Duration::from_secs(7 * 24 * 60 * 60));
    }

    #[test]
    fn artifacts_config_empty_env_is_disabled_default() {
        let cfg = load(&[]);
        assert!(!cfg.enable);
        assert_eq!(cfg.server.port, ArtifactsServerConfig::default().port);
        assert_eq!(cfg.storage.provider, ArtifactsStorageProvider::default());
    }

    #[test]
    fn duration_helper_accepts_suffixes_and_bare_seconds() {
        #[derive(Deserialize)]
        struct Wrap {
            #[serde(deserialize_with = "de::duration::deserialize")]
            d: Duration,
        }
        let cases = [
            ("30s", Duration::from_secs(30)),
            ("15m", Duration::from_secs(15 * 60)),
            ("2h", Duration::from_secs(2 * 60 * 60)),
            ("7d", Duration::from_secs(7 * 24 * 60 * 60)),
            ("90", Duration::from_secs(90)),
        ];
        for (input, expect) in cases {
            let w: Wrap = serde_json::from_value(serde_json::json!({ "d": input }))
                .unwrap_or_else(|e| panic!("parse {input:?}: {e}"));
            assert_eq!(w.d, expect, "input {input:?}");
        }

        // A bare integer (seconds) and the native {secs, nanos} form both work.
        let native: Wrap = serde_json::from_value(serde_json::json!({ "d": 5 })).unwrap();
        assert_eq!(native.d, Duration::from_secs(5));
        let structured: Wrap =
            serde_json::from_value(serde_json::json!({ "d": { "secs": 3, "nanos": 0 } })).unwrap();
        assert_eq!(structured.d, Duration::from_secs(3));
    }

    #[test]
    fn duration_helper_rejects_unknown_suffix() {
        #[derive(Deserialize)]
        struct Wrap {
            #[serde(deserialize_with = "de::duration::deserialize")]
            #[allow(dead_code)]
            d: Duration,
        }
        let err = serde_json::from_value::<Wrap>(serde_json::json!({ "d": "5w" }));
        assert!(err.is_err(), "`5w` should be rejected");
    }

    #[test]
    fn config_artifacts_is_not_loaded_from_a2a_prefix() {
        // artifacts_config is `#[serde(skip)]` on Config, so an A2A_ load
        // never touches it - it stays at the programmatic default.
        let config: Config = envy::prefixed("A2A_")
            .from_iter::<_, Config>(vec![(
                "A2A_ARTIFACTS_ENABLE".to_string(),
                "true".to_string(),
            )])
            .expect("Config should load");
        assert!(!config.artifacts_config.enable);
    }
}

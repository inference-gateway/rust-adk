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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub enable: bool,
    pub issuer_url: String,
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueConfig {
    pub max_size: usize,
    pub timeout: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub tls_enable: bool,
    pub tls_cert_path: Option<String>,
    pub tls_key_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TelemetryConfig {
    pub enable: bool,
    pub endpoint: Option<String>,
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

        if let Ok(timeout) = std::env::var("AGENT_CLIENT_TIMEOUT") {
            if let Ok(timeout_secs) = timeout.parse::<u64>() {
                config.agent_config.timeout = Duration::from_secs(timeout_secs);
            }
        }

        if let Ok(max_retries) = std::env::var("AGENT_CLIENT_MAX_RETRIES") {
            if let Ok(retries) = max_retries.parse::<u32>() {
                config.agent_config.max_retries = retries;
            }
        }

        if let Ok(max_tokens) = std::env::var("AGENT_CLIENT_MAX_TOKENS") {
            if let Ok(tokens) = max_tokens.parse::<u32>() {
                config.agent_config.max_tokens = tokens;
            }
        }

        if let Ok(temperature) = std::env::var("AGENT_CLIENT_TEMPERATURE") {
            if let Ok(temp) = temperature.parse::<f32>() {
                config.agent_config.temperature = temp;
            }
        }

        if let Ok(system_prompt) = std::env::var("AGENT_CLIENT_SYSTEM_PROMPT") {
            config.agent_config.system_prompt = Some(system_prompt);
        }

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

        if let Ok(tls_enable) = std::env::var("SERVER_TLS_ENABLE") {
            if tls_enable.to_lowercase() == "true" {
                config.tls_config = Some(TlsConfig {
                    enable: true,
                    cert_path: std::env::var("SERVER_TLS_CERT_PATH").unwrap_or_default(),
                    key_path: std::env::var("SERVER_TLS_KEY_PATH").unwrap_or_default(),
                });
                config.server_config.tls_enable = true;
                config.server_config.tls_cert_path =
                    Some(config.tls_config.as_ref().unwrap().cert_path.clone());
                config.server_config.tls_key_path =
                    Some(config.tls_config.as_ref().unwrap().key_path.clone());
            }
        }

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

        Ok(config)
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

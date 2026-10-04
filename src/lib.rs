pub mod a2a_types;
pub mod client;
pub mod config;
pub mod server;
pub mod telemetry;

/// The A2A protocol version this ADK speaks, sent and checked as the `A2A-Version`
/// header (A2A spec 3.6).
pub const A2A_PROTOCOL_VERSION: &str = "1.0";

pub use client::{A2AClient, HealthStatus};
pub use config::{
    AgentConfig, ArtifactRetentionConfig, ArtifactsConfig, ArtifactsServerConfig,
    ArtifactsStorageConfig, ArtifactsStorageProvider, AuthConfig, CapabilitiesConfig, ClientConfig,
    Config, McpConfig, QueueConfig, QueueProvider, ServerConfig, TelemetryConfig, TlsConfig,
    TracesExporter,
};
#[cfg(feature = "minio")]
pub use server::MinioArtifactStorage;
#[cfg(feature = "redis")]
pub use server::RedisStorage;
pub use server::{
    A2AServer, A2AServerBuilder, Agent, AgentBuilder, AgentCardOverrides, ArtifactService,
    ArtifactStorage, ArtifactsServer, AsyncFunctionToolHandler, AuthError, AuthVerifier,
    AuthenticatedPrincipal, ClientCertPrincipal, DefaultArtifactService,
    DefaultBackgroundTaskHandler, DefaultStreamingTaskHandler, DefaultTaskManager, DiscoveredTool,
    FilesystemArtifactStorage, FunctionToolHandler, InMemoryStorage, LLMClient, McpClient,
    OidcJwtVerifier, OpenAICompatibleLLMClient, PeerCert, QueuedTask, Storage, StorageStats,
    StoredArtifactInfo, StreamEmitter, StreamableTaskHandler, TaskFilter, TaskHandler,
    TaskManagerRunner, ToolHandler, UsageTracker, create_storage, infer_mime_type,
    spawn_retention_task,
};
pub use server::{EXECUTION_STATS_METADATA_KEY, USAGE_EXTENSION_URI, USAGE_METADATA_KEY};

impl a2a_types::TaskState {
    /// A terminal task never changes state again; A2A v1.0 dropped the `final` flag,
    /// so this is the signal that ends a stream.
    pub fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::TaskStateCompleted
                | Self::TaskStateFailed
                | Self::TaskStateCanceled
                | Self::TaskStateRejected
        )
    }
}

impl a2a_types::Task {
    /// The task's context id, or "" when unset. The ADK always assigns one on tasks it
    /// creates; the field is optional only because A2A v1.0 made it so on the wire.
    pub fn context_id_str(&self) -> &str {
        self.context_id.as_deref().unwrap_or_default()
    }

    /// The task without the metadata keys of the extension identified by `uri`, and without
    /// metadata once none is left. A request that did not activate an extension gets this.
    pub fn without_extension(mut self, uri: &str) -> Self {
        let prefix = format!("{uri}/");
        if let Some(metadata) = self.metadata.as_mut() {
            metadata.0.retain(|key, _| !key.starts_with(&prefix));
        }
        if self.metadata.as_ref().is_some_and(|m| m.0.is_empty()) {
            self.metadata = None;
        }
        self
    }
}

impl a2a_types::TaskStatus {
    /// A status is stamped with the moment it was recorded.
    pub(crate) fn now(state: a2a_types::TaskState, message: Option<a2a_types::Message>) -> Self {
        Self {
            message,
            state,
            timestamp: Some(a2a_types::Timestamp(chrono::Utc::now())),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_a2a_types_module_exists() {
        use crate::a2a_types::Message;
        let _type_exists = std::mem::size_of::<Message>();
    }

    #[test]
    fn test_a2a_types_serialization() {
        use crate::a2a_types::*;

        let message = Message {
            context_id: None,
            extensions: Vec::new(),
            message_id: "test-id".to_string(),
            metadata: None,
            parts: Vec::new(),
            reference_task_ids: Vec::new(),
            role: Role::RoleUser,
            task_id: None,
        };

        let serialized = serde_json::to_string(&message).expect("Should serialize");
        assert!(serialized.contains("\"messageId\":\"test-id\""));
        assert!(serialized.contains("\"role\":\"ROLE_USER\""));

        let _deserialized: Message = serde_json::from_str(&serialized).expect("Should deserialize");
    }

    #[test]
    fn without_extension_drops_only_that_extensions_keys() {
        use crate::a2a_types::Task;
        let task = |metadata: serde_json::Value| -> Task {
            serde_json::from_value(serde_json::json!({
                "id": "t1",
                "status": {"state": "TASK_STATE_COMPLETED"},
                "metadata": metadata,
            }))
            .expect("task parses")
        };
        let uri = "https://example.com/ext/usage/v1";

        let mixed = task(serde_json::json!({
            "https://example.com/ext/usage/v1/usage": 1,
            "https://example.com/ext/usage/v1-other/key": 2,
            "plain": 3,
        }))
        .without_extension(uri);
        let keys: Vec<_> = mixed
            .metadata
            .expect("metadata kept")
            .0
            .into_iter()
            .map(|(k, _)| k)
            .collect();
        assert_eq!(
            keys,
            ["https://example.com/ext/usage/v1-other/key", "plain"]
        );

        let only_extension = task(serde_json::json!({"https://example.com/ext/usage/v1/usage": 1}))
            .without_extension(uri);
        assert!(only_extension.metadata.is_none());
    }
}

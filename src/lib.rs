pub mod a2a_types;
pub mod client;
pub mod config;
pub mod server;
pub mod telemetry;

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
}

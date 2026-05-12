pub mod a2a_types;
pub mod client;
pub mod config;
pub mod server;

pub use client::{A2AClient, HealthStatus};
pub use config::{
    AgentConfig, AuthConfig, CapabilitiesConfig, ClientConfig, Config, QueueConfig, QueueProvider,
    ServerConfig, TelemetryConfig, TlsConfig,
};
#[cfg(feature = "redis")]
pub use server::RedisStorage;
pub use server::{
    A2AServer, A2AServerBuilder, Agent, AgentBuilder, AgentCardOverrides, AsyncFunctionToolHandler,
    AuthError, AuthVerifier, AuthenticatedPrincipal, ClientCertPrincipal,
    DefaultBackgroundTaskHandler, DefaultStreamingTaskHandler, DefaultTaskManager,
    FunctionToolHandler, InMemoryStorage, LLMClient, OidcJwtVerifier, OpenAICompatibleLLMClient,
    PeerCert, QueuedTask, Storage, StorageStats, StreamEmitter, StreamableTaskHandler, TaskFilter,
    TaskHandler, TaskManagerRunner, ToolHandler, create_storage,
};

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

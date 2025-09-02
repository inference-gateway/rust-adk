pub mod a2a_types;
pub mod client;
pub mod config;
pub mod server;
pub mod task_handler;

pub use client::{A2AClient, HealthStatus};
pub use config::{AgentConfig, ClientConfig, Config};
pub use server::{A2AServer, A2AServerBuilder, Agent, AgentBuilder, AgentCardOverrides};
pub use task_handler::{
    BackgroundTaskHandlerConfig, BackgroundTaskQueue, DefaultBackgroundTaskHandler, 
    ManagedTask, QueuedTask, TaskHandler, TaskQueueStats
};

#[cfg(test)]
mod tests {
    #[test]
    fn test_a2a_types_module_exists() {
        use crate::a2a_types::JsonrpcMessage;
        let _type_exists = std::mem::size_of::<JsonrpcMessage>();
    }

    #[test]
    fn test_a2a_types_serialization() {
        use crate::a2a_types::*;

        let message = JsonrpcMessage {
            jsonrpc: "2.0".to_string(),
            id: Some(JsonrpcMessageId::String("test-id".to_string())),
        };

        let serialized = serde_json::to_string(&message).expect("Should serialize");
        assert!(serialized.contains("\"jsonrpc\":\"2.0\""));
        assert!(serialized.contains("\"id\":\"test-id\""));

        let _deserialized: JsonrpcMessage =
            serde_json::from_str(&serialized).expect("Should deserialize");
    }
}

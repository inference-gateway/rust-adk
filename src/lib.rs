pub mod a2a_types;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_a2a_types_module_exists() {
        // Basic test to ensure the a2a_types module is accessible
        // If this compiles, the module is successfully integrated

        // Test that we can access basic types from the module
        use crate::a2a_types::JsonrpcMessage;
        let _type_exists = std::mem::size_of::<JsonrpcMessage>();
        // If this compiles, the module and types are accessible
    }

    #[test]
    fn test_a2a_types_serialization() {
        // Test that we can create and serialize/deserialize A2A types
        use crate::a2a_types::*;

        // Test basic JSON-RPC message structure
        let message = JsonrpcMessage {
            jsonrpc: "2.0".to_string(),
            id: Some(JsonrpcMessageId::String("test-id".to_string())),
        };

        // Test serialization
        let serialized = serde_json::to_string(&message).expect("Should serialize");
        assert!(serialized.contains("\"jsonrpc\":\"2.0\""));
        assert!(serialized.contains("\"id\":\"test-id\""));

        // Test deserialization
        let _deserialized: JsonrpcMessage =
            serde_json::from_str(&serialized).expect("Should deserialize");
    }
}

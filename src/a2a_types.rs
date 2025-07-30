#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]
#![allow(irrefutable_let_patterns)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::unit_arg)]

#[doc = r" Error types."]
pub mod error {
    #[doc = r" Error from a `TryFrom` or `FromStr` implementation."]
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
#[doc = "A discriminated union of all standard JSON-RPC and A2A-specific error types."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A discriminated union of all standard JSON-RPC and A2A-specific error types.\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/JSONParseError\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/InvalidRequestError\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/MethodNotFoundError\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/InvalidParamsError\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/InternalError\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/TaskNotFoundError\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/TaskNotCancelableError\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/PushNotificationNotSupportedError\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/UnsupportedOperationError\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ContentTypeNotSupportedError\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/InvalidAgentResponseError\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct A2aError {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_0: ::std::option::Option<JsonParseError>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_1: ::std::option::Option<InvalidRequestError>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_2: ::std::option::Option<MethodNotFoundError>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_3: ::std::option::Option<InvalidParamsError>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_4: ::std::option::Option<InternalError>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_5: ::std::option::Option<TaskNotFoundError>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_6: ::std::option::Option<TaskNotCancelableError>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_7: ::std::option::Option<PushNotificationNotSupportedError>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_8: ::std::option::Option<UnsupportedOperationError>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_9: ::std::option::Option<ContentTypeNotSupportedError>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_10: ::std::option::Option<InvalidAgentResponseError>,
}
impl ::std::convert::From<&A2aError> for A2aError {
    fn from(value: &A2aError) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for A2aError {
    fn default() -> Self {
        Self {
            subtype_0: Default::default(),
            subtype_1: Default::default(),
            subtype_2: Default::default(),
            subtype_3: Default::default(),
            subtype_4: Default::default(),
            subtype_5: Default::default(),
            subtype_6: Default::default(),
            subtype_7: Default::default(),
            subtype_8: Default::default(),
            subtype_9: Default::default(),
            subtype_10: Default::default(),
        }
    }
}
impl A2aError {
    pub fn builder() -> builder::A2aError {
        Default::default()
    }
}
#[doc = "A discriminated union representing all possible JSON-RPC 2.0 requests supported by the A2A specification."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A discriminated union representing all possible JSON-RPC 2.0 requests supported by the A2A specification.\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/SendMessageRequest\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/SendStreamingMessageRequest\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/GetTaskRequest\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/CancelTaskRequest\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/SetTaskPushNotificationConfigRequest\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/GetTaskPushNotificationConfigRequest\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/TaskResubscriptionRequest\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ListTaskPushNotificationConfigRequest\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/DeleteTaskPushNotificationConfigRequest\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum A2aRequest {
    SendMessageRequest(SendMessageRequest),
    SendStreamingMessageRequest(SendStreamingMessageRequest),
    GetTaskRequest(GetTaskRequest),
    CancelTaskRequest(CancelTaskRequest),
    SetTaskPushNotificationConfigRequest(SetTaskPushNotificationConfigRequest),
    GetTaskPushNotificationConfigRequest(GetTaskPushNotificationConfigRequest),
    TaskResubscriptionRequest(TaskResubscriptionRequest),
    ListTaskPushNotificationConfigRequest(ListTaskPushNotificationConfigRequest),
    DeleteTaskPushNotificationConfigRequest(DeleteTaskPushNotificationConfigRequest),
}
impl ::std::convert::From<&Self> for A2aRequest {
    fn from(value: &A2aRequest) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<SendMessageRequest> for A2aRequest {
    fn from(value: SendMessageRequest) -> Self {
        Self::SendMessageRequest(value)
    }
}
impl ::std::convert::From<SendStreamingMessageRequest> for A2aRequest {
    fn from(value: SendStreamingMessageRequest) -> Self {
        Self::SendStreamingMessageRequest(value)
    }
}
impl ::std::convert::From<GetTaskRequest> for A2aRequest {
    fn from(value: GetTaskRequest) -> Self {
        Self::GetTaskRequest(value)
    }
}
impl ::std::convert::From<CancelTaskRequest> for A2aRequest {
    fn from(value: CancelTaskRequest) -> Self {
        Self::CancelTaskRequest(value)
    }
}
impl ::std::convert::From<SetTaskPushNotificationConfigRequest> for A2aRequest {
    fn from(value: SetTaskPushNotificationConfigRequest) -> Self {
        Self::SetTaskPushNotificationConfigRequest(value)
    }
}
impl ::std::convert::From<GetTaskPushNotificationConfigRequest> for A2aRequest {
    fn from(value: GetTaskPushNotificationConfigRequest) -> Self {
        Self::GetTaskPushNotificationConfigRequest(value)
    }
}
impl ::std::convert::From<TaskResubscriptionRequest> for A2aRequest {
    fn from(value: TaskResubscriptionRequest) -> Self {
        Self::TaskResubscriptionRequest(value)
    }
}
impl ::std::convert::From<ListTaskPushNotificationConfigRequest> for A2aRequest {
    fn from(value: ListTaskPushNotificationConfigRequest) -> Self {
        Self::ListTaskPushNotificationConfigRequest(value)
    }
}
impl ::std::convert::From<DeleteTaskPushNotificationConfigRequest> for A2aRequest {
    fn from(value: DeleteTaskPushNotificationConfigRequest) -> Self {
        Self::DeleteTaskPushNotificationConfigRequest(value)
    }
}
#[doc = "Defines optional capabilities supported by an agent."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Defines optional capabilities supported by an agent.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"A list of protocol extensions supported by the agent.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/AgentExtension\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"pushNotifications\": {"]
#[doc = "      \"description\": \"Indicates if the agent supports sending push notifications for asynchronous task updates.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"stateTransitionHistory\": {"]
#[doc = "      \"description\": \"Indicates if the agent provides a history of state transitions for a task.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"streaming\": {"]
#[doc = "      \"description\": \"Indicates if the agent supports Server-Sent Events (SSE) for streaming responses.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct AgentCapabilities {
    #[doc = "A list of protocol extensions supported by the agent."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub extensions: ::std::vec::Vec<AgentExtension>,
    #[doc = "Indicates if the agent supports sending push notifications for asynchronous task updates."]
    #[serde(
        rename = "pushNotifications",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub push_notifications: ::std::option::Option<bool>,
    #[doc = "Indicates if the agent provides a history of state transitions for a task."]
    #[serde(
        rename = "stateTransitionHistory",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub state_transition_history: ::std::option::Option<bool>,
    #[doc = "Indicates if the agent supports Server-Sent Events (SSE) for streaming responses."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub streaming: ::std::option::Option<bool>,
}
impl ::std::convert::From<&AgentCapabilities> for AgentCapabilities {
    fn from(value: &AgentCapabilities) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for AgentCapabilities {
    fn default() -> Self {
        Self {
            extensions: Default::default(),
            push_notifications: Default::default(),
            state_transition_history: Default::default(),
            streaming: Default::default(),
        }
    }
}
impl AgentCapabilities {
    pub fn builder() -> builder::AgentCapabilities {
        Default::default()
    }
}
#[doc = "The AgentCard is a self-describing manifest for an agent. It provides essential\nmetadata including the agent's identity, capabilities, skills, supported\ncommunication methods, and security requirements."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The AgentCard is a self-describing manifest for an agent. It provides essential\\nmetadata including the agent's identity, capabilities, skills, supported\\ncommunication methods, and security requirements.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"capabilities\","]
#[doc = "    \"defaultInputModes\","]
#[doc = "    \"defaultOutputModes\","]
#[doc = "    \"description\","]
#[doc = "    \"name\","]
#[doc = "    \"protocolVersion\","]
#[doc = "    \"skills\","]
#[doc = "    \"url\","]
#[doc = "    \"version\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"additionalInterfaces\": {"]
#[doc = "      \"description\": \"A list of additional supported interfaces (transport and URL combinations).\\nThis allows agents to expose multiple transports, potentially at different URLs.\\n\\nBest practices:\\n- SHOULD include all supported transports for completeness\\n- SHOULD include an entry matching the main 'url' and 'preferredTransport'\\n- MAY reuse URLs if multiple transports are available at the same endpoint\\n- MUST accurately declare the transport available at each URL\\n\\nClients can select any interface from this list based on their transport capabilities\\nand preferences. This enables transport negotiation and fallback scenarios.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/AgentInterface\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"capabilities\": {"]
#[doc = "      \"description\": \"A declaration of optional capabilities supported by the agent.\","]
#[doc = "      \"$ref\": \"#/definitions/AgentCapabilities\""]
#[doc = "    },"]
#[doc = "    \"defaultInputModes\": {"]
#[doc = "      \"description\": \"Default set of supported input MIME types for all skills, which can be\\noverridden on a per-skill basis.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"defaultOutputModes\": {"]
#[doc = "      \"description\": \"Default set of supported output MIME types for all skills, which can be\\noverridden on a per-skill basis.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"A human-readable description of the agent, assisting users and other agents\\nin understanding its purpose.\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Agent that helps users with recipes and cooking.\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"documentationUrl\": {"]
#[doc = "      \"description\": \"An optional URL to the agent's documentation.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"iconUrl\": {"]
#[doc = "      \"description\": \"An optional URL to an icon for the agent.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"A human-readable name for the agent.\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"Recipe Agent\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"preferredTransport\": {"]
#[doc = "      \"description\": \"The transport protocol for the preferred endpoint (the main 'url' field).\\nIf not specified, defaults to 'JSONRPC'.\\n\\nIMPORTANT: The transport specified here MUST be available at the main 'url'.\\nThis creates a binding between the main URL and its supported transport protocol.\\nClients should prefer this transport and URL combination when both are supported.\","]
#[doc = "      \"default\": \"JSONRPC\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"JSONRPC\","]
#[doc = "        \"GRPC\","]
#[doc = "        \"HTTP+JSON\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"protocolVersion\": {"]
#[doc = "      \"description\": \"The version of the A2A protocol this agent supports.\","]
#[doc = "      \"default\": \"0.2.6\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"provider\": {"]
#[doc = "      \"description\": \"Information about the agent's service provider.\","]
#[doc = "      \"$ref\": \"#/definitions/AgentProvider\""]
#[doc = "    },"]
#[doc = "    \"security\": {"]
#[doc = "      \"description\": \"A list of security requirement objects that apply to all agent interactions. Each object\\nlists security schemes that can be used. Follows the OpenAPI 3.0 Security Requirement Object.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"additionalProperties\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"securitySchemes\": {"]
#[doc = "      \"description\": \"A declaration of the security schemes available to authorize requests. The key is the\\nscheme name. Follows the OpenAPI 3.0 Security Scheme Object.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#/definitions/SecurityScheme\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"signatures\": {"]
#[doc = "      \"description\": \"JSON Web Signatures computed for this AgentCard.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/AgentCardSignature\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"skills\": {"]
#[doc = "      \"description\": \"The set of skills, or distinct capabilities, that the agent can perform.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/AgentSkill\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"supportsAuthenticatedExtendedCard\": {"]
#[doc = "      \"description\": \"If true, the agent can provide an extended agent card with additional details\\nto authenticated users. Defaults to false.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"url\": {"]
#[doc = "      \"description\": \"The preferred endpoint URL for interacting with the agent.\\nThis URL MUST support the transport specified by 'preferredTransport'.\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"https://api.example.com/a2a/v1\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"version\": {"]
#[doc = "      \"description\": \"The agent's own version number. The format is defined by the provider.\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"1.0.0\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct AgentCard {
    #[doc = "A list of additional supported interfaces (transport and URL combinations).\nThis allows agents to expose multiple transports, potentially at different URLs.\n\nBest practices:\n- SHOULD include all supported transports for completeness\n- SHOULD include an entry matching the main 'url' and 'preferredTransport'\n- MAY reuse URLs if multiple transports are available at the same endpoint\n- MUST accurately declare the transport available at each URL\n\nClients can select any interface from this list based on their transport capabilities\nand preferences. This enables transport negotiation and fallback scenarios."]
    #[serde(
        rename = "additionalInterfaces",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub additional_interfaces: ::std::vec::Vec<AgentInterface>,
    #[doc = "A declaration of optional capabilities supported by the agent."]
    pub capabilities: AgentCapabilities,
    #[doc = "Default set of supported input MIME types for all skills, which can be\noverridden on a per-skill basis."]
    #[serde(rename = "defaultInputModes")]
    pub default_input_modes: ::std::vec::Vec<::std::string::String>,
    #[doc = "Default set of supported output MIME types for all skills, which can be\noverridden on a per-skill basis."]
    #[serde(rename = "defaultOutputModes")]
    pub default_output_modes: ::std::vec::Vec<::std::string::String>,
    #[doc = "A human-readable description of the agent, assisting users and other agents\nin understanding its purpose."]
    pub description: ::std::string::String,
    #[doc = "An optional URL to the agent's documentation."]
    #[serde(
        rename = "documentationUrl",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub documentation_url: ::std::option::Option<::std::string::String>,
    #[doc = "An optional URL to an icon for the agent."]
    #[serde(
        rename = "iconUrl",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub icon_url: ::std::option::Option<::std::string::String>,
    #[doc = "A human-readable name for the agent."]
    pub name: ::std::string::String,
    #[doc = "The transport protocol for the preferred endpoint (the main 'url' field).\nIf not specified, defaults to 'JSONRPC'.\n\nIMPORTANT: The transport specified here MUST be available at the main 'url'.\nThis creates a binding between the main URL and its supported transport protocol.\nClients should prefer this transport and URL combination when both are supported."]
    #[serde(
        rename = "preferredTransport",
        default = "defaults::agent_card_preferred_transport"
    )]
    pub preferred_transport: ::std::string::String,
    #[doc = "The version of the A2A protocol this agent supports."]
    #[serde(rename = "protocolVersion")]
    pub protocol_version: ::std::string::String,
    #[doc = "Information about the agent's service provider."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub provider: ::std::option::Option<AgentProvider>,
    #[doc = "A list of security requirement objects that apply to all agent interactions. Each object\nlists security schemes that can be used. Follows the OpenAPI 3.0 Security Requirement Object."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub security: ::std::vec::Vec<
        ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<::std::string::String>>,
    >,
    #[doc = "A declaration of the security schemes available to authorize requests. The key is the\nscheme name. Follows the OpenAPI 3.0 Security Scheme Object."]
    #[serde(
        rename = "securitySchemes",
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub security_schemes: ::std::collections::HashMap<::std::string::String, SecurityScheme>,
    #[doc = "JSON Web Signatures computed for this AgentCard."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub signatures: ::std::vec::Vec<AgentCardSignature>,
    #[doc = "The set of skills, or distinct capabilities, that the agent can perform."]
    pub skills: ::std::vec::Vec<AgentSkill>,
    #[doc = "If true, the agent can provide an extended agent card with additional details\nto authenticated users. Defaults to false."]
    #[serde(
        rename = "supportsAuthenticatedExtendedCard",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub supports_authenticated_extended_card: ::std::option::Option<bool>,
    #[doc = "The preferred endpoint URL for interacting with the agent.\nThis URL MUST support the transport specified by 'preferredTransport'."]
    pub url: ::std::string::String,
    #[doc = "The agent's own version number. The format is defined by the provider."]
    pub version: ::std::string::String,
}
impl ::std::convert::From<&AgentCard> for AgentCard {
    fn from(value: &AgentCard) -> Self {
        value.clone()
    }
}
impl AgentCard {
    pub fn builder() -> builder::AgentCard {
        Default::default()
    }
}
#[doc = "AgentCardSignature represents a JWS signature of an AgentCard.\nThis follows the JSON format of an RFC 7515 JSON Web Signature (JWS)."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"AgentCardSignature represents a JWS signature of an AgentCard.\\nThis follows the JSON format of an RFC 7515 JSON Web Signature (JWS).\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"protected\","]
#[doc = "    \"signature\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"header\": {"]
#[doc = "      \"description\": \"The unprotected JWS header values.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    },"]
#[doc = "    \"protected\": {"]
#[doc = "      \"description\": \"The protected JWS header for the signature. This is a Base64url-encoded\\nJSON object, as per RFC 7515.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"signature\": {"]
#[doc = "      \"description\": \"The computed signature, Base64url-encoded.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct AgentCardSignature {
    #[doc = "The unprotected JWS header values."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub header: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "The protected JWS header for the signature. This is a Base64url-encoded\nJSON object, as per RFC 7515."]
    pub protected: ::std::string::String,
    #[doc = "The computed signature, Base64url-encoded."]
    pub signature: ::std::string::String,
}
impl ::std::convert::From<&AgentCardSignature> for AgentCardSignature {
    fn from(value: &AgentCardSignature) -> Self {
        value.clone()
    }
}
impl AgentCardSignature {
    pub fn builder() -> builder::AgentCardSignature {
        Default::default()
    }
}
#[doc = "A declaration of a protocol extension supported by an Agent."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A declaration of a protocol extension supported by an Agent.\","]
#[doc = "  \"examples\": ["]
#[doc = "    {"]
#[doc = "      \"description\": \"Google OAuth 2.0 authentication\","]
#[doc = "      \"required\": false,"]
#[doc = "      \"uri\": \"https://developers.google.com/identity/protocols/oauth2\""]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"uri\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"A human-readable description of how this agent uses the extension.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"description\": \"Optional, extension-specific configuration parameters.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    },"]
#[doc = "    \"required\": {"]
#[doc = "      \"description\": \"If true, the client must understand and comply with the extension's requirements\\nto interact with the agent.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"uri\": {"]
#[doc = "      \"description\": \"The unique URI identifying the extension.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct AgentExtension {
    #[doc = "A human-readable description of how this agent uses the extension."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "Optional, extension-specific configuration parameters."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub params: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "If true, the client must understand and comply with the extension's requirements\nto interact with the agent."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub required: ::std::option::Option<bool>,
    #[doc = "The unique URI identifying the extension."]
    pub uri: ::std::string::String,
}
impl ::std::convert::From<&AgentExtension> for AgentExtension {
    fn from(value: &AgentExtension) -> Self {
        value.clone()
    }
}
impl AgentExtension {
    pub fn builder() -> builder::AgentExtension {
        Default::default()
    }
}
#[doc = "Declares a combination of a target URL and a transport protocol for interacting with the agent.\nThis allows agents to expose the same functionality over multiple transport mechanisms."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Declares a combination of a target URL and a transport protocol for interacting with the agent.\\nThis allows agents to expose the same functionality over multiple transport mechanisms.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"transport\","]
#[doc = "    \"url\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"transport\": {"]
#[doc = "      \"description\": \"The transport protocol supported at this URL.\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"JSONRPC\","]
#[doc = "        \"GRPC\","]
#[doc = "        \"HTTP+JSON\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"url\": {"]
#[doc = "      \"description\": \"The URL where this interface is available. Must be a valid absolute HTTPS URL in production.\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"https://api.example.com/a2a/v1\","]
#[doc = "        \"https://grpc.example.com/a2a\","]
#[doc = "        \"https://rest.example.com/v1\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct AgentInterface {
    #[doc = "The transport protocol supported at this URL."]
    pub transport: ::std::string::String,
    #[doc = "The URL where this interface is available. Must be a valid absolute HTTPS URL in production."]
    pub url: ::std::string::String,
}
impl ::std::convert::From<&AgentInterface> for AgentInterface {
    fn from(value: &AgentInterface) -> Self {
        value.clone()
    }
}
impl AgentInterface {
    pub fn builder() -> builder::AgentInterface {
        Default::default()
    }
}
#[doc = "Represents the service provider of an agent."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Represents the service provider of an agent.\","]
#[doc = "  \"examples\": ["]
#[doc = "    {"]
#[doc = "      \"organization\": \"Google\","]
#[doc = "      \"url\": \"https://ai.google.dev\""]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"organization\","]
#[doc = "    \"url\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"organization\": {"]
#[doc = "      \"description\": \"The name of the agent provider's organization.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"url\": {"]
#[doc = "      \"description\": \"A URL for the agent provider's website or relevant documentation.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct AgentProvider {
    #[doc = "The name of the agent provider's organization."]
    pub organization: ::std::string::String,
    #[doc = "A URL for the agent provider's website or relevant documentation."]
    pub url: ::std::string::String,
}
impl ::std::convert::From<&AgentProvider> for AgentProvider {
    fn from(value: &AgentProvider) -> Self {
        value.clone()
    }
}
impl AgentProvider {
    pub fn builder() -> builder::AgentProvider {
        Default::default()
    }
}
#[doc = "Represents a distinct capability or function that an agent can perform."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Represents a distinct capability or function that an agent can perform.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"description\","]
#[doc = "    \"id\","]
#[doc = "    \"name\","]
#[doc = "    \"tags\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"A detailed description of the skill, intended to help clients or users\\nunderstand its purpose and functionality.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"examples\": {"]
#[doc = "      \"description\": \"Example prompts or scenarios that this skill can handle. Provides a hint to\\nthe client on how to use the skill.\","]
#[doc = "      \"examples\": ["]
#[doc = "        ["]
#[doc = "          \"I need a recipe for bread\""]
#[doc = "        ]"]
#[doc = "      ],"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"description\": \"A unique identifier for the agent's skill.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"inputModes\": {"]
#[doc = "      \"description\": \"The set of supported input MIME types for this skill, overriding the agent's defaults.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"A human-readable name for the skill.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"outputModes\": {"]
#[doc = "      \"description\": \"The set of supported output MIME types for this skill, overriding the agent's defaults.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"tags\": {"]
#[doc = "      \"description\": \"A set of keywords describing the skill's capabilities.\","]
#[doc = "      \"examples\": ["]
#[doc = "        ["]
#[doc = "          \"cooking\","]
#[doc = "          \"customer support\","]
#[doc = "          \"billing\""]
#[doc = "        ]"]
#[doc = "      ],"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct AgentSkill {
    #[doc = "A detailed description of the skill, intended to help clients or users\nunderstand its purpose and functionality."]
    pub description: ::std::string::String,
    #[doc = "Example prompts or scenarios that this skill can handle. Provides a hint to\nthe client on how to use the skill."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub examples: ::std::vec::Vec<::std::string::String>,
    #[doc = "A unique identifier for the agent's skill."]
    pub id: ::std::string::String,
    #[doc = "The set of supported input MIME types for this skill, overriding the agent's defaults."]
    #[serde(
        rename = "inputModes",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub input_modes: ::std::vec::Vec<::std::string::String>,
    #[doc = "A human-readable name for the skill."]
    pub name: ::std::string::String,
    #[doc = "The set of supported output MIME types for this skill, overriding the agent's defaults."]
    #[serde(
        rename = "outputModes",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub output_modes: ::std::vec::Vec<::std::string::String>,
    #[doc = "A set of keywords describing the skill's capabilities."]
    pub tags: ::std::vec::Vec<::std::string::String>,
}
impl ::std::convert::From<&AgentSkill> for AgentSkill {
    fn from(value: &AgentSkill) -> Self {
        value.clone()
    }
}
impl AgentSkill {
    pub fn builder() -> builder::AgentSkill {
        Default::default()
    }
}
#[doc = "Defines a security scheme using an API key."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Defines a security scheme using an API key.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"in\","]
#[doc = "    \"name\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"An optional description for the security scheme.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"in\": {"]
#[doc = "      \"description\": \"The location of the API key.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"cookie\","]
#[doc = "        \"header\","]
#[doc = "        \"query\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"The name of the header, query, or cookie parameter to be used.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"description\": \"The type of the security scheme. Must be 'apiKey'.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"apiKey\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ApiKeySecurityScheme {
    #[doc = "An optional description for the security scheme."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "The location of the API key."]
    #[serde(rename = "in")]
    pub in_: ApiKeySecuritySchemeIn,
    #[doc = "The name of the header, query, or cookie parameter to be used."]
    pub name: ::std::string::String,
    #[doc = "The type of the security scheme. Must be 'apiKey'."]
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl ::std::convert::From<&ApiKeySecurityScheme> for ApiKeySecurityScheme {
    fn from(value: &ApiKeySecurityScheme) -> Self {
        value.clone()
    }
}
impl ApiKeySecurityScheme {
    pub fn builder() -> builder::ApiKeySecurityScheme {
        Default::default()
    }
}
#[doc = "The location of the API key."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The location of the API key.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"cookie\","]
#[doc = "    \"header\","]
#[doc = "    \"query\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ApiKeySecuritySchemeIn {
    #[serde(rename = "cookie")]
    Cookie,
    #[serde(rename = "header")]
    Header,
    #[serde(rename = "query")]
    Query,
}
impl ::std::convert::From<&Self> for ApiKeySecuritySchemeIn {
    fn from(value: &ApiKeySecuritySchemeIn) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for ApiKeySecuritySchemeIn {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Cookie => write!(f, "cookie"),
            Self::Header => write!(f, "header"),
            Self::Query => write!(f, "query"),
        }
    }
}
impl ::std::str::FromStr for ApiKeySecuritySchemeIn {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "cookie" => Ok(Self::Cookie),
            "header" => Ok(Self::Header),
            "query" => Ok(Self::Query),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ApiKeySecuritySchemeIn {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ApiKeySecuritySchemeIn {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ApiKeySecuritySchemeIn {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Represents a file, data structure, or other resource generated by an agent during a task."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Represents a file, data structure, or other resource generated by an agent during a task.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"artifactId\","]
#[doc = "    \"parts\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"artifactId\": {"]
#[doc = "      \"description\": \"A unique identifier for the artifact within the scope of the task.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"An optional, human-readable description of the artifact.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"The URIs of extensions that are relevant to this artifact.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"metadata\": {"]
#[doc = "      \"description\": \"Optional metadata for extensions. The key is an extension-specific identifier.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"An optional, human-readable name for the artifact.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"parts\": {"]
#[doc = "      \"description\": \"An array of content parts that make up the artifact.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Part\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Artifact {
    #[doc = "A unique identifier for the artifact within the scope of the task."]
    #[serde(rename = "artifactId")]
    pub artifact_id: ::std::string::String,
    #[doc = "An optional, human-readable description of the artifact."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "The URIs of extensions that are relevant to this artifact."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub extensions: ::std::vec::Vec<::std::string::String>,
    #[doc = "Optional metadata for extensions. The key is an extension-specific identifier."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "An optional, human-readable name for the artifact."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "An array of content parts that make up the artifact."]
    pub parts: ::std::vec::Vec<Part>,
}
impl ::std::convert::From<&Artifact> for Artifact {
    fn from(value: &Artifact) -> Self {
        value.clone()
    }
}
impl Artifact {
    pub fn builder() -> builder::Artifact {
        Default::default()
    }
}
#[doc = "Defines configuration details for the OAuth 2.0 Authorization Code flow."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Defines configuration details for the OAuth 2.0 Authorization Code flow.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"authorizationUrl\","]
#[doc = "    \"scopes\","]
#[doc = "    \"tokenUrl\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"authorizationUrl\": {"]
#[doc = "      \"description\": \"The authorization URL to be used for this flow.\\nThis MUST be a URL and use TLS.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"refreshUrl\": {"]
#[doc = "      \"description\": \"The URL to be used for obtaining refresh tokens.\\nThis MUST be a URL and use TLS.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"scopes\": {"]
#[doc = "      \"description\": \"The available scopes for the OAuth2 security scheme. A map between the scope\\nname and a short description for it.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"tokenUrl\": {"]
#[doc = "      \"description\": \"The token URL to be used for this flow.\\nThis MUST be a URL and use TLS.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct AuthorizationCodeOAuthFlow {
    #[doc = "The authorization URL to be used for this flow.\nThis MUST be a URL and use TLS."]
    #[serde(rename = "authorizationUrl")]
    pub authorization_url: ::std::string::String,
    #[doc = "The URL to be used for obtaining refresh tokens.\nThis MUST be a URL and use TLS."]
    #[serde(
        rename = "refreshUrl",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub refresh_url: ::std::option::Option<::std::string::String>,
    #[doc = "The available scopes for the OAuth2 security scheme. A map between the scope\nname and a short description for it."]
    pub scopes: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    #[doc = "The token URL to be used for this flow.\nThis MUST be a URL and use TLS."]
    #[serde(rename = "tokenUrl")]
    pub token_url: ::std::string::String,
}
impl ::std::convert::From<&AuthorizationCodeOAuthFlow> for AuthorizationCodeOAuthFlow {
    fn from(value: &AuthorizationCodeOAuthFlow) -> Self {
        value.clone()
    }
}
impl AuthorizationCodeOAuthFlow {
    pub fn builder() -> builder::AuthorizationCodeOAuthFlow {
        Default::default()
    }
}
#[doc = "Represents a JSON-RPC request for the `tasks/cancel` method."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Represents a JSON-RPC request for the `tasks/cancel` method.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"jsonrpc\","]
#[doc = "    \"method\","]
#[doc = "    \"params\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"description\": \"The identifier for this request.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"integer\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"jsonrpc\": {"]
#[doc = "      \"description\": \"The version of the JSON-RPC protocol. MUST be exactly \\\"2.0\\\".\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"2.0\""]
#[doc = "    },"]
#[doc = "    \"method\": {"]
#[doc = "      \"description\": \"The method name. Must be 'tasks/cancel'.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"tasks/cancel\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"description\": \"The parameters identifying the task to cancel.\","]
#[doc = "      \"$ref\": \"#/definitions/TaskIdParams\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CancelTaskRequest {
    #[doc = "The identifier for this request."]
    pub id: CancelTaskRequestId,
    #[doc = "The version of the JSON-RPC protocol. MUST be exactly \"2.0\"."]
    pub jsonrpc: ::std::string::String,
    #[doc = "The method name. Must be 'tasks/cancel'."]
    pub method: ::std::string::String,
    #[doc = "The parameters identifying the task to cancel."]
    pub params: TaskIdParams,
}
impl ::std::convert::From<&CancelTaskRequest> for CancelTaskRequest {
    fn from(value: &CancelTaskRequest) -> Self {
        value.clone()
    }
}
impl CancelTaskRequest {
    pub fn builder() -> builder::CancelTaskRequest {
        Default::default()
    }
}
#[doc = "The identifier for this request."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The identifier for this request.\","]
#[doc = "  \"type\": ["]
#[doc = "    \"string\","]
#[doc = "    \"integer\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CancelTaskRequestId {
    String(::std::string::String),
    Integer(i64),
}
impl ::std::convert::From<&Self> for CancelTaskRequestId {
    fn from(value: &CancelTaskRequestId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for CancelTaskRequestId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Integer(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl ::std::convert::TryFrom<&str> for CancelTaskRequestId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CancelTaskRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CancelTaskRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::fmt::Display for CancelTaskRequestId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::String(x) => x.fmt(f),
            Self::Integer(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<i64> for CancelTaskRequestId {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
#[doc = "Represents a JSON-RPC response for the `tasks/cancel` method."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Represents a JSON-RPC response for the `tasks/cancel` method.\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/JSONRPCErrorResponse\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/CancelTaskSuccessResponse\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CancelTaskResponse {
    JsonrpcErrorResponse(JsonrpcErrorResponse),
    CancelTaskSuccessResponse(CancelTaskSuccessResponse),
}
impl ::std::convert::From<&Self> for CancelTaskResponse {
    fn from(value: &CancelTaskResponse) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<JsonrpcErrorResponse> for CancelTaskResponse {
    fn from(value: JsonrpcErrorResponse) -> Self {
        Self::JsonrpcErrorResponse(value)
    }
}
impl ::std::convert::From<CancelTaskSuccessResponse> for CancelTaskResponse {
    fn from(value: CancelTaskSuccessResponse) -> Self {
        Self::CancelTaskSuccessResponse(value)
    }
}
#[doc = "Represents a successful JSON-RPC response for the `tasks/cancel` method."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Represents a successful JSON-RPC response for the `tasks/cancel` method.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"jsonrpc\","]
#[doc = "    \"result\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"description\": \"The identifier established by the client.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"jsonrpc\": {"]
#[doc = "      \"description\": \"The version of the JSON-RPC protocol. MUST be exactly \\\"2.0\\\".\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"2.0\""]
#[doc = "    },"]
#[doc = "    \"result\": {"]
#[doc = "      \"description\": \"The result, containing the final state of the canceled Task object.\","]
#[doc = "      \"$ref\": \"#/definitions/Task\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CancelTaskSuccessResponse {
    #[doc = "The identifier established by the client."]
    pub id: CancelTaskSuccessResponseId,
    #[doc = "The version of the JSON-RPC protocol. MUST be exactly \"2.0\"."]
    pub jsonrpc: ::std::string::String,
    #[doc = "The result, containing the final state of the canceled Task object."]
    pub result: Task,
}
impl ::std::convert::From<&CancelTaskSuccessResponse> for CancelTaskSuccessResponse {
    fn from(value: &CancelTaskSuccessResponse) -> Self {
        value.clone()
    }
}
impl CancelTaskSuccessResponse {
    pub fn builder() -> builder::CancelTaskSuccessResponse {
        Default::default()
    }
}
#[doc = "The identifier established by the client."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The identifier established by the client.\","]
#[doc = "  \"type\": ["]
#[doc = "    \"string\","]
#[doc = "    \"integer\","]
#[doc = "    \"null\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CancelTaskSuccessResponseId {
    Null,
    String(::std::string::String),
    Integer(i64),
}
impl ::std::convert::From<&Self> for CancelTaskSuccessResponseId {
    fn from(value: &CancelTaskSuccessResponseId) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<i64> for CancelTaskSuccessResponseId {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
#[doc = "Defines configuration details for the OAuth 2.0 Client Credentials flow."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Defines configuration details for the OAuth 2.0 Client Credentials flow.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"scopes\","]
#[doc = "    \"tokenUrl\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"refreshUrl\": {"]
#[doc = "      \"description\": \"The URL to be used for obtaining refresh tokens. This MUST be a URL.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"scopes\": {"]
#[doc = "      \"description\": \"The available scopes for the OAuth2 security scheme. A map between the scope\\nname and a short description for it.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"tokenUrl\": {"]
#[doc = "      \"description\": \"The token URL to be used for this flow. This MUST be a URL.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ClientCredentialsOAuthFlow {
    #[doc = "The URL to be used for obtaining refresh tokens. This MUST be a URL."]
    #[serde(
        rename = "refreshUrl",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub refresh_url: ::std::option::Option<::std::string::String>,
    #[doc = "The available scopes for the OAuth2 security scheme. A map between the scope\nname and a short description for it."]
    pub scopes: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    #[doc = "The token URL to be used for this flow. This MUST be a URL."]
    #[serde(rename = "tokenUrl")]
    pub token_url: ::std::string::String,
}
impl ::std::convert::From<&ClientCredentialsOAuthFlow> for ClientCredentialsOAuthFlow {
    fn from(value: &ClientCredentialsOAuthFlow) -> Self {
        value.clone()
    }
}
impl ClientCredentialsOAuthFlow {
    pub fn builder() -> builder::ClientCredentialsOAuthFlow {
        Default::default()
    }
}
#[doc = "An A2A-specific error indicating an incompatibility between the requested\ncontent types and the agent's capabilities."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"An A2A-specific error indicating an incompatibility between the requested\\ncontent types and the agent's capabilities.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"code\","]
#[doc = "    \"message\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"code\": {"]
#[doc = "      \"description\": \"The error code for an unsupported content type.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"const\": -32005"]
#[doc = "    },"]
#[doc = "    \"data\": {"]
#[doc = "      \"description\": \"A primitive or structured value containing additional information about the error.\\nThis may be omitted.\""]
#[doc = "    },"]
#[doc = "    \"message\": {"]
#[doc = "      \"description\": \"The error message.\","]
#[doc = "      \"default\": \"Incompatible content types\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ContentTypeNotSupportedError {
    #[doc = "The error code for an unsupported content type."]
    pub code: i64,
    #[doc = "A primitive or structured value containing additional information about the error.\nThis may be omitted."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<::serde_json::Value>,
    #[doc = "The error message."]
    pub message: ::std::string::String,
}
impl ::std::convert::From<&ContentTypeNotSupportedError> for ContentTypeNotSupportedError {
    fn from(value: &ContentTypeNotSupportedError) -> Self {
        value.clone()
    }
}
impl ContentTypeNotSupportedError {
    pub fn builder() -> builder::ContentTypeNotSupportedError {
        Default::default()
    }
}
#[doc = "Represents a structured data segment (e.g., JSON) within a message or artifact."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Represents a structured data segment (e.g., JSON) within a message or artifact.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"data\","]
#[doc = "    \"kind\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"data\": {"]
#[doc = "      \"description\": \"The structured data content.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"description\": \"The type of this part, used as a discriminator. Always 'data'.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"data\""]
#[doc = "    },"]
#[doc = "    \"metadata\": {"]
#[doc = "      \"description\": \"Optional metadata associated with this part.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct DataPart {
    #[doc = "The structured data content."]
    pub data: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "The type of this part, used as a discriminator. Always 'data'."]
    pub kind: ::std::string::String,
    #[doc = "Optional metadata associated with this part."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl ::std::convert::From<&DataPart> for DataPart {
    fn from(value: &DataPart) -> Self {
        value.clone()
    }
}
impl DataPart {
    pub fn builder() -> builder::DataPart {
        Default::default()
    }
}
#[doc = "Defines parameters for deleting a specific push notification configuration for a task."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Defines parameters for deleting a specific push notification configuration for a task.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"pushNotificationConfigId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"description\": \"The unique identifier of the task.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"metadata\": {"]
#[doc = "      \"description\": \"Optional metadata associated with the request.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    },"]
#[doc = "    \"pushNotificationConfigId\": {"]
#[doc = "      \"description\": \"The ID of the push notification configuration to delete.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct DeleteTaskPushNotificationConfigParams {
    #[doc = "The unique identifier of the task."]
    pub id: ::std::string::String,
    #[doc = "Optional metadata associated with the request."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "The ID of the push notification configuration to delete."]
    #[serde(rename = "pushNotificationConfigId")]
    pub push_notification_config_id: ::std::string::String,
}
impl ::std::convert::From<&DeleteTaskPushNotificationConfigParams>
    for DeleteTaskPushNotificationConfigParams
{
    fn from(value: &DeleteTaskPushNotificationConfigParams) -> Self {
        value.clone()
    }
}
impl DeleteTaskPushNotificationConfigParams {
    pub fn builder() -> builder::DeleteTaskPushNotificationConfigParams {
        Default::default()
    }
}
#[doc = "Represents a JSON-RPC request for the `tasks/pushNotificationConfig/delete` method."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Represents a JSON-RPC request for the `tasks/pushNotificationConfig/delete` method.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"jsonrpc\","]
#[doc = "    \"method\","]
#[doc = "    \"params\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"description\": \"The identifier for this request.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"integer\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"jsonrpc\": {"]
#[doc = "      \"description\": \"The version of the JSON-RPC protocol. MUST be exactly \\\"2.0\\\".\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"2.0\""]
#[doc = "    },"]
#[doc = "    \"method\": {"]
#[doc = "      \"description\": \"The method name. Must be 'tasks/pushNotificationConfig/delete'.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"tasks/pushNotificationConfig/delete\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"description\": \"The parameters identifying the push notification configuration to delete.\","]
#[doc = "      \"$ref\": \"#/definitions/DeleteTaskPushNotificationConfigParams\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct DeleteTaskPushNotificationConfigRequest {
    #[doc = "The identifier for this request."]
    pub id: DeleteTaskPushNotificationConfigRequestId,
    #[doc = "The version of the JSON-RPC protocol. MUST be exactly \"2.0\"."]
    pub jsonrpc: ::std::string::String,
    #[doc = "The method name. Must be 'tasks/pushNotificationConfig/delete'."]
    pub method: ::std::string::String,
    #[doc = "The parameters identifying the push notification configuration to delete."]
    pub params: DeleteTaskPushNotificationConfigParams,
}
impl ::std::convert::From<&DeleteTaskPushNotificationConfigRequest>
    for DeleteTaskPushNotificationConfigRequest
{
    fn from(value: &DeleteTaskPushNotificationConfigRequest) -> Self {
        value.clone()
    }
}
impl DeleteTaskPushNotificationConfigRequest {
    pub fn builder() -> builder::DeleteTaskPushNotificationConfigRequest {
        Default::default()
    }
}
#[doc = "The identifier for this request."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The identifier for this request.\","]
#[doc = "  \"type\": ["]
#[doc = "    \"string\","]
#[doc = "    \"integer\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum DeleteTaskPushNotificationConfigRequestId {
    String(::std::string::String),
    Integer(i64),
}
impl ::std::convert::From<&Self> for DeleteTaskPushNotificationConfigRequestId {
    fn from(value: &DeleteTaskPushNotificationConfigRequestId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for DeleteTaskPushNotificationConfigRequestId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Integer(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl ::std::convert::TryFrom<&str> for DeleteTaskPushNotificationConfigRequestId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for DeleteTaskPushNotificationConfigRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DeleteTaskPushNotificationConfigRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::fmt::Display for DeleteTaskPushNotificationConfigRequestId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::String(x) => x.fmt(f),
            Self::Integer(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<i64> for DeleteTaskPushNotificationConfigRequestId {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
#[doc = "Represents a JSON-RPC response for the `tasks/pushNotificationConfig/delete` method."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Represents a JSON-RPC response for the `tasks/pushNotificationConfig/delete` method.\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/JSONRPCErrorResponse\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/DeleteTaskPushNotificationConfigSuccessResponse\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum DeleteTaskPushNotificationConfigResponse {
    JsonrpcErrorResponse(JsonrpcErrorResponse),
    DeleteTaskPushNotificationConfigSuccessResponse(
        DeleteTaskPushNotificationConfigSuccessResponse,
    ),
}
impl ::std::convert::From<&Self> for DeleteTaskPushNotificationConfigResponse {
    fn from(value: &DeleteTaskPushNotificationConfigResponse) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<JsonrpcErrorResponse> for DeleteTaskPushNotificationConfigResponse {
    fn from(value: JsonrpcErrorResponse) -> Self {
        Self::JsonrpcErrorResponse(value)
    }
}
impl ::std::convert::From<DeleteTaskPushNotificationConfigSuccessResponse>
    for DeleteTaskPushNotificationConfigResponse
{
    fn from(value: DeleteTaskPushNotificationConfigSuccessResponse) -> Self {
        Self::DeleteTaskPushNotificationConfigSuccessResponse(value)
    }
}
#[doc = "Represents a successful JSON-RPC response for the `tasks/pushNotificationConfig/delete` method."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Represents a successful JSON-RPC response for the `tasks/pushNotificationConfig/delete` method.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"jsonrpc\","]
#[doc = "    \"result\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"description\": \"The identifier established by the client.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"jsonrpc\": {"]
#[doc = "      \"description\": \"The version of the JSON-RPC protocol. MUST be exactly \\\"2.0\\\".\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"2.0\""]
#[doc = "    },"]
#[doc = "    \"result\": {"]
#[doc = "      \"description\": \"The result is null on successful deletion.\","]
#[doc = "      \"type\": \"null\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct DeleteTaskPushNotificationConfigSuccessResponse {
    #[doc = "The identifier established by the client."]
    pub id: DeleteTaskPushNotificationConfigSuccessResponseId,
    #[doc = "The version of the JSON-RPC protocol. MUST be exactly \"2.0\"."]
    pub jsonrpc: ::std::string::String,
    #[doc = "The result is null on successful deletion."]
    pub result: (),
}
impl ::std::convert::From<&DeleteTaskPushNotificationConfigSuccessResponse>
    for DeleteTaskPushNotificationConfigSuccessResponse
{
    fn from(value: &DeleteTaskPushNotificationConfigSuccessResponse) -> Self {
        value.clone()
    }
}
impl DeleteTaskPushNotificationConfigSuccessResponse {
    pub fn builder() -> builder::DeleteTaskPushNotificationConfigSuccessResponse {
        Default::default()
    }
}
#[doc = "The identifier established by the client."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The identifier established by the client.\","]
#[doc = "  \"type\": ["]
#[doc = "    \"string\","]
#[doc = "    \"integer\","]
#[doc = "    \"null\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum DeleteTaskPushNotificationConfigSuccessResponseId {
    Null,
    String(::std::string::String),
    Integer(i64),
}
impl ::std::convert::From<&Self> for DeleteTaskPushNotificationConfigSuccessResponseId {
    fn from(value: &DeleteTaskPushNotificationConfigSuccessResponseId) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<i64> for DeleteTaskPushNotificationConfigSuccessResponseId {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
#[doc = "Defines base properties for a file."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Defines base properties for a file.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"mimeType\": {"]
#[doc = "      \"description\": \"The MIME type of the file (e.g., \\\"application/pdf\\\").\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"An optional name for the file (e.g., \\\"document.pdf\\\").\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct FileBase {
    #[doc = "The MIME type of the file (e.g., \"application/pdf\")."]
    #[serde(
        rename = "mimeType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub mime_type: ::std::option::Option<::std::string::String>,
    #[doc = "An optional name for the file (e.g., \"document.pdf\")."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&FileBase> for FileBase {
    fn from(value: &FileBase) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for FileBase {
    fn default() -> Self {
        Self {
            mime_type: Default::default(),
            name: Default::default(),
        }
    }
}
impl FileBase {
    pub fn builder() -> builder::FileBase {
        Default::default()
    }
}
#[doc = "Represents a file segment within a message or artifact. The file content can be\nprovided either directly as bytes or as a URI."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Represents a file segment within a message or artifact. The file content can be\\nprovided either directly as bytes or as a URI.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"file\","]
#[doc = "    \"kind\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"file\": {"]
#[doc = "      \"description\": \"The file content, represented as either a URI or as base64-encoded bytes.\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/FileWithBytes\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/FileWithUri\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"description\": \"The type of this part, used as a discriminator. Always 'file'.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"file\""]
#[doc = "    },"]
#[doc = "    \"metadata\": {"]
#[doc = "      \"description\": \"Optional metadata associated with this part.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct FilePart {
    #[doc = "The file content, represented as either a URI or as base64-encoded bytes."]
    pub file: FilePartFile,
    #[doc = "The type of this part, used as a discriminator. Always 'file'."]
    pub kind: ::std::string::String,
    #[doc = "Optional metadata associated with this part."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl ::std::convert::From<&FilePart> for FilePart {
    fn from(value: &FilePart) -> Self {
        value.clone()
    }
}
impl FilePart {
    pub fn builder() -> builder::FilePart {
        Default::default()
    }
}
#[doc = "The file content, represented as either a URI or as base64-encoded bytes."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The file content, represented as either a URI or as base64-encoded bytes.\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/FileWithBytes\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/FileWithUri\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum FilePartFile {
    Bytes(FileWithBytes),
    Uri(FileWithUri),
}
impl ::std::convert::From<&Self> for FilePartFile {
    fn from(value: &FilePartFile) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<FileWithBytes> for FilePartFile {
    fn from(value: FileWithBytes) -> Self {
        Self::Bytes(value)
    }
}
impl ::std::convert::From<FileWithUri> for FilePartFile {
    fn from(value: FileWithUri) -> Self {
        Self::Uri(value)
    }
}
#[doc = "Represents a file with its content provided directly as a base64-encoded string."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Represents a file with its content provided directly as a base64-encoded string.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"bytes\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"bytes\": {"]
#[doc = "      \"description\": \"The base64-encoded content of the file.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"mimeType\": {"]
#[doc = "      \"description\": \"The MIME type of the file (e.g., \\\"application/pdf\\\").\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"An optional name for the file (e.g., \\\"document.pdf\\\").\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct FileWithBytes {
    #[doc = "The base64-encoded content of the file."]
    pub bytes: ::std::string::String,
    #[doc = "The MIME type of the file (e.g., \"application/pdf\")."]
    #[serde(
        rename = "mimeType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub mime_type: ::std::option::Option<::std::string::String>,
    #[doc = "An optional name for the file (e.g., \"document.pdf\")."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&FileWithBytes> for FileWithBytes {
    fn from(value: &FileWithBytes) -> Self {
        value.clone()
    }
}
impl FileWithBytes {
    pub fn builder() -> builder::FileWithBytes {
        Default::default()
    }
}
#[doc = "Represents a file with its content located at a specific URI."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Represents a file with its content located at a specific URI.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"uri\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"mimeType\": {"]
#[doc = "      \"description\": \"The MIME type of the file (e.g., \\\"application/pdf\\\").\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"An optional name for the file (e.g., \\\"document.pdf\\\").\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"uri\": {"]
#[doc = "      \"description\": \"A URL pointing to the file's content.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct FileWithUri {
    #[doc = "The MIME type of the file (e.g., \"application/pdf\")."]
    #[serde(
        rename = "mimeType",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub mime_type: ::std::option::Option<::std::string::String>,
    #[doc = "An optional name for the file (e.g., \"document.pdf\")."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "A URL pointing to the file's content."]
    pub uri: ::std::string::String,
}
impl ::std::convert::From<&FileWithUri> for FileWithUri {
    fn from(value: &FileWithUri) -> Self {
        value.clone()
    }
}
impl FileWithUri {
    pub fn builder() -> builder::FileWithUri {
        Default::default()
    }
}
#[doc = "Defines parameters for fetching a specific push notification configuration for a task."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Defines parameters for fetching a specific push notification configuration for a task.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"description\": \"The unique identifier of the task.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"metadata\": {"]
#[doc = "      \"description\": \"Optional metadata associated with the request.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    },"]
#[doc = "    \"pushNotificationConfigId\": {"]
#[doc = "      \"description\": \"The ID of the push notification configuration to retrieve.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct GetTaskPushNotificationConfigParams {
    #[doc = "The unique identifier of the task."]
    pub id: ::std::string::String,
    #[doc = "Optional metadata associated with the request."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "The ID of the push notification configuration to retrieve."]
    #[serde(
        rename = "pushNotificationConfigId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub push_notification_config_id: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&GetTaskPushNotificationConfigParams>
    for GetTaskPushNotificationConfigParams
{
    fn from(value: &GetTaskPushNotificationConfigParams) -> Self {
        value.clone()
    }
}
impl GetTaskPushNotificationConfigParams {
    pub fn builder() -> builder::GetTaskPushNotificationConfigParams {
        Default::default()
    }
}
#[doc = "Represents a JSON-RPC request for the `tasks/pushNotificationConfig/get` method."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Represents a JSON-RPC request for the `tasks/pushNotificationConfig/get` method.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"jsonrpc\","]
#[doc = "    \"method\","]
#[doc = "    \"params\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"description\": \"The identifier for this request.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"integer\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"jsonrpc\": {"]
#[doc = "      \"description\": \"The version of the JSON-RPC protocol. MUST be exactly \\\"2.0\\\".\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"2.0\""]
#[doc = "    },"]
#[doc = "    \"method\": {"]
#[doc = "      \"description\": \"The method name. Must be 'tasks/pushNotificationConfig/get'.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"tasks/pushNotificationConfig/get\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"description\": \"The parameters for getting a push notification configuration.\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/TaskIdParams\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/GetTaskPushNotificationConfigParams\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct GetTaskPushNotificationConfigRequest {
    #[doc = "The identifier for this request."]
    pub id: GetTaskPushNotificationConfigRequestId,
    #[doc = "The version of the JSON-RPC protocol. MUST be exactly \"2.0\"."]
    pub jsonrpc: ::std::string::String,
    #[doc = "The method name. Must be 'tasks/pushNotificationConfig/get'."]
    pub method: ::std::string::String,
    #[doc = "The parameters for getting a push notification configuration."]
    pub params: GetTaskPushNotificationConfigRequestParams,
}
impl ::std::convert::From<&GetTaskPushNotificationConfigRequest>
    for GetTaskPushNotificationConfigRequest
{
    fn from(value: &GetTaskPushNotificationConfigRequest) -> Self {
        value.clone()
    }
}
impl GetTaskPushNotificationConfigRequest {
    pub fn builder() -> builder::GetTaskPushNotificationConfigRequest {
        Default::default()
    }
}
#[doc = "The identifier for this request."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The identifier for this request.\","]
#[doc = "  \"type\": ["]
#[doc = "    \"string\","]
#[doc = "    \"integer\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum GetTaskPushNotificationConfigRequestId {
    String(::std::string::String),
    Integer(i64),
}
impl ::std::convert::From<&Self> for GetTaskPushNotificationConfigRequestId {
    fn from(value: &GetTaskPushNotificationConfigRequestId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for GetTaskPushNotificationConfigRequestId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Integer(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl ::std::convert::TryFrom<&str> for GetTaskPushNotificationConfigRequestId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for GetTaskPushNotificationConfigRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for GetTaskPushNotificationConfigRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::fmt::Display for GetTaskPushNotificationConfigRequestId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::String(x) => x.fmt(f),
            Self::Integer(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<i64> for GetTaskPushNotificationConfigRequestId {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
#[doc = "The parameters for getting a push notification configuration."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The parameters for getting a push notification configuration.\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/TaskIdParams\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/GetTaskPushNotificationConfigParams\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct GetTaskPushNotificationConfigRequestParams {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_0: ::std::option::Option<TaskIdParams>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_1: ::std::option::Option<GetTaskPushNotificationConfigParams>,
}
impl ::std::convert::From<&GetTaskPushNotificationConfigRequestParams>
    for GetTaskPushNotificationConfigRequestParams
{
    fn from(value: &GetTaskPushNotificationConfigRequestParams) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for GetTaskPushNotificationConfigRequestParams {
    fn default() -> Self {
        Self {
            subtype_0: Default::default(),
            subtype_1: Default::default(),
        }
    }
}
impl GetTaskPushNotificationConfigRequestParams {
    pub fn builder() -> builder::GetTaskPushNotificationConfigRequestParams {
        Default::default()
    }
}
#[doc = "Represents a JSON-RPC response for the `tasks/pushNotificationConfig/get` method."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Represents a JSON-RPC response for the `tasks/pushNotificationConfig/get` method.\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/JSONRPCErrorResponse\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/GetTaskPushNotificationConfigSuccessResponse\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum GetTaskPushNotificationConfigResponse {
    JsonrpcErrorResponse(JsonrpcErrorResponse),
    GetTaskPushNotificationConfigSuccessResponse(GetTaskPushNotificationConfigSuccessResponse),
}
impl ::std::convert::From<&Self> for GetTaskPushNotificationConfigResponse {
    fn from(value: &GetTaskPushNotificationConfigResponse) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<JsonrpcErrorResponse> for GetTaskPushNotificationConfigResponse {
    fn from(value: JsonrpcErrorResponse) -> Self {
        Self::JsonrpcErrorResponse(value)
    }
}
impl ::std::convert::From<GetTaskPushNotificationConfigSuccessResponse>
    for GetTaskPushNotificationConfigResponse
{
    fn from(value: GetTaskPushNotificationConfigSuccessResponse) -> Self {
        Self::GetTaskPushNotificationConfigSuccessResponse(value)
    }
}
#[doc = "Represents a successful JSON-RPC response for the `tasks/pushNotificationConfig/get` method."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Represents a successful JSON-RPC response for the `tasks/pushNotificationConfig/get` method.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"jsonrpc\","]
#[doc = "    \"result\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"description\": \"The identifier established by the client.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"jsonrpc\": {"]
#[doc = "      \"description\": \"The version of the JSON-RPC protocol. MUST be exactly \\\"2.0\\\".\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"2.0\""]
#[doc = "    },"]
#[doc = "    \"result\": {"]
#[doc = "      \"description\": \"The result, containing the requested push notification configuration.\","]
#[doc = "      \"$ref\": \"#/definitions/TaskPushNotificationConfig\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct GetTaskPushNotificationConfigSuccessResponse {
    #[doc = "The identifier established by the client."]
    pub id: GetTaskPushNotificationConfigSuccessResponseId,
    #[doc = "The version of the JSON-RPC protocol. MUST be exactly \"2.0\"."]
    pub jsonrpc: ::std::string::String,
    #[doc = "The result, containing the requested push notification configuration."]
    pub result: TaskPushNotificationConfig,
}
impl ::std::convert::From<&GetTaskPushNotificationConfigSuccessResponse>
    for GetTaskPushNotificationConfigSuccessResponse
{
    fn from(value: &GetTaskPushNotificationConfigSuccessResponse) -> Self {
        value.clone()
    }
}
impl GetTaskPushNotificationConfigSuccessResponse {
    pub fn builder() -> builder::GetTaskPushNotificationConfigSuccessResponse {
        Default::default()
    }
}
#[doc = "The identifier established by the client."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The identifier established by the client.\","]
#[doc = "  \"type\": ["]
#[doc = "    \"string\","]
#[doc = "    \"integer\","]
#[doc = "    \"null\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum GetTaskPushNotificationConfigSuccessResponseId {
    Null,
    String(::std::string::String),
    Integer(i64),
}
impl ::std::convert::From<&Self> for GetTaskPushNotificationConfigSuccessResponseId {
    fn from(value: &GetTaskPushNotificationConfigSuccessResponseId) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<i64> for GetTaskPushNotificationConfigSuccessResponseId {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
#[doc = "Represents a JSON-RPC request for the `tasks/get` method."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Represents a JSON-RPC request for the `tasks/get` method.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"jsonrpc\","]
#[doc = "    \"method\","]
#[doc = "    \"params\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"description\": \"The identifier for this request.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"integer\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"jsonrpc\": {"]
#[doc = "      \"description\": \"The version of the JSON-RPC protocol. MUST be exactly \\\"2.0\\\".\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"2.0\""]
#[doc = "    },"]
#[doc = "    \"method\": {"]
#[doc = "      \"description\": \"The method name. Must be 'tasks/get'.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"tasks/get\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"description\": \"The parameters for querying a task.\","]
#[doc = "      \"$ref\": \"#/definitions/TaskQueryParams\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct GetTaskRequest {
    #[doc = "The identifier for this request."]
    pub id: GetTaskRequestId,
    #[doc = "The version of the JSON-RPC protocol. MUST be exactly \"2.0\"."]
    pub jsonrpc: ::std::string::String,
    #[doc = "The method name. Must be 'tasks/get'."]
    pub method: ::std::string::String,
    #[doc = "The parameters for querying a task."]
    pub params: TaskQueryParams,
}
impl ::std::convert::From<&GetTaskRequest> for GetTaskRequest {
    fn from(value: &GetTaskRequest) -> Self {
        value.clone()
    }
}
impl GetTaskRequest {
    pub fn builder() -> builder::GetTaskRequest {
        Default::default()
    }
}
#[doc = "The identifier for this request."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The identifier for this request.\","]
#[doc = "  \"type\": ["]
#[doc = "    \"string\","]
#[doc = "    \"integer\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum GetTaskRequestId {
    String(::std::string::String),
    Integer(i64),
}
impl ::std::convert::From<&Self> for GetTaskRequestId {
    fn from(value: &GetTaskRequestId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for GetTaskRequestId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Integer(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl ::std::convert::TryFrom<&str> for GetTaskRequestId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for GetTaskRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for GetTaskRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::fmt::Display for GetTaskRequestId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::String(x) => x.fmt(f),
            Self::Integer(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<i64> for GetTaskRequestId {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
#[doc = "Represents a JSON-RPC response for the `tasks/get` method."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Represents a JSON-RPC response for the `tasks/get` method.\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/JSONRPCErrorResponse\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/GetTaskSuccessResponse\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum GetTaskResponse {
    JsonrpcErrorResponse(JsonrpcErrorResponse),
    GetTaskSuccessResponse(GetTaskSuccessResponse),
}
impl ::std::convert::From<&Self> for GetTaskResponse {
    fn from(value: &GetTaskResponse) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<JsonrpcErrorResponse> for GetTaskResponse {
    fn from(value: JsonrpcErrorResponse) -> Self {
        Self::JsonrpcErrorResponse(value)
    }
}
impl ::std::convert::From<GetTaskSuccessResponse> for GetTaskResponse {
    fn from(value: GetTaskSuccessResponse) -> Self {
        Self::GetTaskSuccessResponse(value)
    }
}
#[doc = "Represents a successful JSON-RPC response for the `tasks/get` method."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Represents a successful JSON-RPC response for the `tasks/get` method.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"jsonrpc\","]
#[doc = "    \"result\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"description\": \"The identifier established by the client.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"jsonrpc\": {"]
#[doc = "      \"description\": \"The version of the JSON-RPC protocol. MUST be exactly \\\"2.0\\\".\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"2.0\""]
#[doc = "    },"]
#[doc = "    \"result\": {"]
#[doc = "      \"description\": \"The result, containing the requested Task object.\","]
#[doc = "      \"$ref\": \"#/definitions/Task\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct GetTaskSuccessResponse {
    #[doc = "The identifier established by the client."]
    pub id: GetTaskSuccessResponseId,
    #[doc = "The version of the JSON-RPC protocol. MUST be exactly \"2.0\"."]
    pub jsonrpc: ::std::string::String,
    #[doc = "The result, containing the requested Task object."]
    pub result: Task,
}
impl ::std::convert::From<&GetTaskSuccessResponse> for GetTaskSuccessResponse {
    fn from(value: &GetTaskSuccessResponse) -> Self {
        value.clone()
    }
}
impl GetTaskSuccessResponse {
    pub fn builder() -> builder::GetTaskSuccessResponse {
        Default::default()
    }
}
#[doc = "The identifier established by the client."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The identifier established by the client.\","]
#[doc = "  \"type\": ["]
#[doc = "    \"string\","]
#[doc = "    \"integer\","]
#[doc = "    \"null\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum GetTaskSuccessResponseId {
    Null,
    String(::std::string::String),
    Integer(i64),
}
impl ::std::convert::From<&Self> for GetTaskSuccessResponseId {
    fn from(value: &GetTaskSuccessResponseId) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<i64> for GetTaskSuccessResponseId {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
#[doc = "Defines a security scheme using HTTP authentication."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Defines a security scheme using HTTP authentication.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"scheme\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"bearerFormat\": {"]
#[doc = "      \"description\": \"A hint to the client to identify how the bearer token is formatted (e.g., \\\"JWT\\\").\\nThis is primarily for documentation purposes.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"An optional description for the security scheme.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"scheme\": {"]
#[doc = "      \"description\": \"The name of the HTTP Authentication scheme to be used in the Authorization header,\\nas defined in RFC7235 (e.g., \\\"Bearer\\\").\\nThis value should be registered in the IANA Authentication Scheme registry.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"description\": \"The type of the security scheme. Must be 'http'.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"http\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct HttpAuthSecurityScheme {
    #[doc = "A hint to the client to identify how the bearer token is formatted (e.g., \"JWT\").\nThis is primarily for documentation purposes."]
    #[serde(
        rename = "bearerFormat",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub bearer_format: ::std::option::Option<::std::string::String>,
    #[doc = "An optional description for the security scheme."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "The name of the HTTP Authentication scheme to be used in the Authorization header,\nas defined in RFC7235 (e.g., \"Bearer\").\nThis value should be registered in the IANA Authentication Scheme registry."]
    pub scheme: ::std::string::String,
    #[doc = "The type of the security scheme. Must be 'http'."]
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl ::std::convert::From<&HttpAuthSecurityScheme> for HttpAuthSecurityScheme {
    fn from(value: &HttpAuthSecurityScheme) -> Self {
        value.clone()
    }
}
impl HttpAuthSecurityScheme {
    pub fn builder() -> builder::HttpAuthSecurityScheme {
        Default::default()
    }
}
#[doc = "Defines configuration details for the OAuth 2.0 Implicit flow."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Defines configuration details for the OAuth 2.0 Implicit flow.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"authorizationUrl\","]
#[doc = "    \"scopes\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"authorizationUrl\": {"]
#[doc = "      \"description\": \"The authorization URL to be used for this flow. This MUST be a URL.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"refreshUrl\": {"]
#[doc = "      \"description\": \"The URL to be used for obtaining refresh tokens. This MUST be a URL.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"scopes\": {"]
#[doc = "      \"description\": \"The available scopes for the OAuth2 security scheme. A map between the scope\\nname and a short description for it.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ImplicitOAuthFlow {
    #[doc = "The authorization URL to be used for this flow. This MUST be a URL."]
    #[serde(rename = "authorizationUrl")]
    pub authorization_url: ::std::string::String,
    #[doc = "The URL to be used for obtaining refresh tokens. This MUST be a URL."]
    #[serde(
        rename = "refreshUrl",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub refresh_url: ::std::option::Option<::std::string::String>,
    #[doc = "The available scopes for the OAuth2 security scheme. A map between the scope\nname and a short description for it."]
    pub scopes: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
}
impl ::std::convert::From<&ImplicitOAuthFlow> for ImplicitOAuthFlow {
    fn from(value: &ImplicitOAuthFlow) -> Self {
        value.clone()
    }
}
impl ImplicitOAuthFlow {
    pub fn builder() -> builder::ImplicitOAuthFlow {
        Default::default()
    }
}
#[doc = "An error indicating an internal error on the server."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"An error indicating an internal error on the server.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"code\","]
#[doc = "    \"message\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"code\": {"]
#[doc = "      \"description\": \"The error code for an internal server error.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"const\": -32603"]
#[doc = "    },"]
#[doc = "    \"data\": {"]
#[doc = "      \"description\": \"A primitive or structured value containing additional information about the error.\\nThis may be omitted.\""]
#[doc = "    },"]
#[doc = "    \"message\": {"]
#[doc = "      \"description\": \"The error message.\","]
#[doc = "      \"default\": \"Internal error\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct InternalError {
    #[doc = "The error code for an internal server error."]
    pub code: i64,
    #[doc = "A primitive or structured value containing additional information about the error.\nThis may be omitted."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<::serde_json::Value>,
    #[doc = "The error message."]
    pub message: ::std::string::String,
}
impl ::std::convert::From<&InternalError> for InternalError {
    fn from(value: &InternalError) -> Self {
        value.clone()
    }
}
impl InternalError {
    pub fn builder() -> builder::InternalError {
        Default::default()
    }
}
#[doc = "An A2A-specific error indicating that the agent returned a response that\ndoes not conform to the specification for the current method."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"An A2A-specific error indicating that the agent returned a response that\\ndoes not conform to the specification for the current method.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"code\","]
#[doc = "    \"message\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"code\": {"]
#[doc = "      \"description\": \"The error code for an invalid agent response.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"const\": -32006"]
#[doc = "    },"]
#[doc = "    \"data\": {"]
#[doc = "      \"description\": \"A primitive or structured value containing additional information about the error.\\nThis may be omitted.\""]
#[doc = "    },"]
#[doc = "    \"message\": {"]
#[doc = "      \"description\": \"The error message.\","]
#[doc = "      \"default\": \"Invalid agent response\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct InvalidAgentResponseError {
    #[doc = "The error code for an invalid agent response."]
    pub code: i64,
    #[doc = "A primitive or structured value containing additional information about the error.\nThis may be omitted."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<::serde_json::Value>,
    #[doc = "The error message."]
    pub message: ::std::string::String,
}
impl ::std::convert::From<&InvalidAgentResponseError> for InvalidAgentResponseError {
    fn from(value: &InvalidAgentResponseError) -> Self {
        value.clone()
    }
}
impl InvalidAgentResponseError {
    pub fn builder() -> builder::InvalidAgentResponseError {
        Default::default()
    }
}
#[doc = "An error indicating that the method parameters are invalid."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"An error indicating that the method parameters are invalid.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"code\","]
#[doc = "    \"message\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"code\": {"]
#[doc = "      \"description\": \"The error code for an invalid parameters error.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"const\": -32602"]
#[doc = "    },"]
#[doc = "    \"data\": {"]
#[doc = "      \"description\": \"A primitive or structured value containing additional information about the error.\\nThis may be omitted.\""]
#[doc = "    },"]
#[doc = "    \"message\": {"]
#[doc = "      \"description\": \"The error message.\","]
#[doc = "      \"default\": \"Invalid parameters\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct InvalidParamsError {
    #[doc = "The error code for an invalid parameters error."]
    pub code: i64,
    #[doc = "A primitive or structured value containing additional information about the error.\nThis may be omitted."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<::serde_json::Value>,
    #[doc = "The error message."]
    pub message: ::std::string::String,
}
impl ::std::convert::From<&InvalidParamsError> for InvalidParamsError {
    fn from(value: &InvalidParamsError) -> Self {
        value.clone()
    }
}
impl InvalidParamsError {
    pub fn builder() -> builder::InvalidParamsError {
        Default::default()
    }
}
#[doc = "An error indicating that the JSON sent is not a valid Request object."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"An error indicating that the JSON sent is not a valid Request object.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"code\","]
#[doc = "    \"message\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"code\": {"]
#[doc = "      \"description\": \"The error code for an invalid request.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"const\": -32600"]
#[doc = "    },"]
#[doc = "    \"data\": {"]
#[doc = "      \"description\": \"A primitive or structured value containing additional information about the error.\\nThis may be omitted.\""]
#[doc = "    },"]
#[doc = "    \"message\": {"]
#[doc = "      \"description\": \"The error message.\","]
#[doc = "      \"default\": \"Request payload validation error\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct InvalidRequestError {
    #[doc = "The error code for an invalid request."]
    pub code: i64,
    #[doc = "A primitive or structured value containing additional information about the error.\nThis may be omitted."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<::serde_json::Value>,
    #[doc = "The error message."]
    pub message: ::std::string::String,
}
impl ::std::convert::From<&InvalidRequestError> for InvalidRequestError {
    fn from(value: &InvalidRequestError) -> Self {
        value.clone()
    }
}
impl InvalidRequestError {
    pub fn builder() -> builder::InvalidRequestError {
        Default::default()
    }
}
#[doc = "An error indicating that the server received invalid JSON."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"An error indicating that the server received invalid JSON.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"code\","]
#[doc = "    \"message\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"code\": {"]
#[doc = "      \"description\": \"The error code for a JSON parse error.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"const\": -32700"]
#[doc = "    },"]
#[doc = "    \"data\": {"]
#[doc = "      \"description\": \"A primitive or structured value containing additional information about the error.\\nThis may be omitted.\""]
#[doc = "    },"]
#[doc = "    \"message\": {"]
#[doc = "      \"description\": \"The error message.\","]
#[doc = "      \"default\": \"Invalid JSON payload\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct JsonParseError {
    #[doc = "The error code for a JSON parse error."]
    pub code: i64,
    #[doc = "A primitive or structured value containing additional information about the error.\nThis may be omitted."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<::serde_json::Value>,
    #[doc = "The error message."]
    pub message: ::std::string::String,
}
impl ::std::convert::From<&JsonParseError> for JsonParseError {
    fn from(value: &JsonParseError) -> Self {
        value.clone()
    }
}
impl JsonParseError {
    pub fn builder() -> builder::JsonParseError {
        Default::default()
    }
}
#[doc = "Represents a JSON-RPC 2.0 Error object, included in an error response."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Represents a JSON-RPC 2.0 Error object, included in an error response.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"code\","]
#[doc = "    \"message\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"code\": {"]
#[doc = "      \"description\": \"A number that indicates the error type that occurred.\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"data\": {"]
#[doc = "      \"description\": \"A primitive or structured value containing additional information about the error.\\nThis may be omitted.\""]
#[doc = "    },"]
#[doc = "    \"message\": {"]
#[doc = "      \"description\": \"A string providing a short description of the error.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct JsonrpcError {
    #[doc = "A number that indicates the error type that occurred."]
    pub code: i64,
    #[doc = "A primitive or structured value containing additional information about the error.\nThis may be omitted."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<::serde_json::Value>,
    #[doc = "A string providing a short description of the error."]
    pub message: ::std::string::String,
}
impl ::std::convert::From<&JsonrpcError> for JsonrpcError {
    fn from(value: &JsonrpcError) -> Self {
        value.clone()
    }
}
impl JsonrpcError {
    pub fn builder() -> builder::JsonrpcError {
        Default::default()
    }
}
#[doc = "Represents a JSON-RPC 2.0 Error Response object."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Represents a JSON-RPC 2.0 Error Response object.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"error\","]
#[doc = "    \"id\","]
#[doc = "    \"jsonrpc\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"error\": {"]
#[doc = "      \"description\": \"An object describing the error that occurred.\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/JSONRPCError\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/JSONParseError\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/InvalidRequestError\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/MethodNotFoundError\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/InvalidParamsError\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/InternalError\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/TaskNotFoundError\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/TaskNotCancelableError\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/PushNotificationNotSupportedError\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/UnsupportedOperationError\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/ContentTypeNotSupportedError\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/InvalidAgentResponseError\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"description\": \"The identifier established by the client.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"jsonrpc\": {"]
#[doc = "      \"description\": \"The version of the JSON-RPC protocol. MUST be exactly \\\"2.0\\\".\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"2.0\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct JsonrpcErrorResponse {
    #[doc = "An object describing the error that occurred."]
    pub error: JsonrpcErrorResponseError,
    #[doc = "The identifier established by the client."]
    pub id: JsonrpcErrorResponseId,
    #[doc = "The version of the JSON-RPC protocol. MUST be exactly \"2.0\"."]
    pub jsonrpc: ::std::string::String,
}
impl ::std::convert::From<&JsonrpcErrorResponse> for JsonrpcErrorResponse {
    fn from(value: &JsonrpcErrorResponse) -> Self {
        value.clone()
    }
}
impl JsonrpcErrorResponse {
    pub fn builder() -> builder::JsonrpcErrorResponse {
        Default::default()
    }
}
#[doc = "An object describing the error that occurred."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"An object describing the error that occurred.\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/JSONRPCError\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/JSONParseError\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/InvalidRequestError\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/MethodNotFoundError\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/InvalidParamsError\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/InternalError\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/TaskNotFoundError\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/TaskNotCancelableError\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/PushNotificationNotSupportedError\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/UnsupportedOperationError\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ContentTypeNotSupportedError\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/InvalidAgentResponseError\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct JsonrpcErrorResponseError {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_0: ::std::option::Option<JsonrpcError>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_1: ::std::option::Option<JsonParseError>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_2: ::std::option::Option<InvalidRequestError>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_3: ::std::option::Option<MethodNotFoundError>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_4: ::std::option::Option<InvalidParamsError>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_5: ::std::option::Option<InternalError>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_6: ::std::option::Option<TaskNotFoundError>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_7: ::std::option::Option<TaskNotCancelableError>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_8: ::std::option::Option<PushNotificationNotSupportedError>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_9: ::std::option::Option<UnsupportedOperationError>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_10: ::std::option::Option<ContentTypeNotSupportedError>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_11: ::std::option::Option<InvalidAgentResponseError>,
}
impl ::std::convert::From<&JsonrpcErrorResponseError> for JsonrpcErrorResponseError {
    fn from(value: &JsonrpcErrorResponseError) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for JsonrpcErrorResponseError {
    fn default() -> Self {
        Self {
            subtype_0: Default::default(),
            subtype_1: Default::default(),
            subtype_2: Default::default(),
            subtype_3: Default::default(),
            subtype_4: Default::default(),
            subtype_5: Default::default(),
            subtype_6: Default::default(),
            subtype_7: Default::default(),
            subtype_8: Default::default(),
            subtype_9: Default::default(),
            subtype_10: Default::default(),
            subtype_11: Default::default(),
        }
    }
}
impl JsonrpcErrorResponseError {
    pub fn builder() -> builder::JsonrpcErrorResponseError {
        Default::default()
    }
}
#[doc = "The identifier established by the client."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The identifier established by the client.\","]
#[doc = "  \"type\": ["]
#[doc = "    \"string\","]
#[doc = "    \"integer\","]
#[doc = "    \"null\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum JsonrpcErrorResponseId {
    Null,
    String(::std::string::String),
    Integer(i64),
}
impl ::std::convert::From<&Self> for JsonrpcErrorResponseId {
    fn from(value: &JsonrpcErrorResponseId) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<i64> for JsonrpcErrorResponseId {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
#[doc = "Defines the base structure for any JSON-RPC 2.0 request, response, or notification."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Defines the base structure for any JSON-RPC 2.0 request, response, or notification.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"jsonrpc\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"description\": \"A unique identifier established by the client. It must be a String, a Number, or null.\\nThe server must reply with the same value in the response. This property is omitted for notifications.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"jsonrpc\": {"]
#[doc = "      \"description\": \"The version of the JSON-RPC protocol. MUST be exactly \\\"2.0\\\".\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"2.0\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct JsonrpcMessage {
    #[doc = "A unique identifier established by the client. It must be a String, a Number, or null.\nThe server must reply with the same value in the response. This property is omitted for notifications."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<JsonrpcMessageId>,
    #[doc = "The version of the JSON-RPC protocol. MUST be exactly \"2.0\"."]
    pub jsonrpc: ::std::string::String,
}
impl ::std::convert::From<&JsonrpcMessage> for JsonrpcMessage {
    fn from(value: &JsonrpcMessage) -> Self {
        value.clone()
    }
}
impl JsonrpcMessage {
    pub fn builder() -> builder::JsonrpcMessage {
        Default::default()
    }
}
#[doc = "A unique identifier established by the client. It must be a String, a Number, or null.\nThe server must reply with the same value in the response. This property is omitted for notifications."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A unique identifier established by the client. It must be a String, a Number, or null.\\nThe server must reply with the same value in the response. This property is omitted for notifications.\","]
#[doc = "  \"type\": ["]
#[doc = "    \"string\","]
#[doc = "    \"integer\","]
#[doc = "    \"null\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum JsonrpcMessageId {
    Null,
    String(::std::string::String),
    Integer(i64),
}
impl ::std::convert::From<&Self> for JsonrpcMessageId {
    fn from(value: &JsonrpcMessageId) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<i64> for JsonrpcMessageId {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
#[doc = "Represents a JSON-RPC 2.0 Request object."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Represents a JSON-RPC 2.0 Request object.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"jsonrpc\","]
#[doc = "    \"method\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"description\": \"A unique identifier established by the client. It must be a String, a Number, or null.\\nThe server must reply with the same value in the response. This property is omitted for notifications.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"jsonrpc\": {"]
#[doc = "      \"description\": \"The version of the JSON-RPC protocol. MUST be exactly \\\"2.0\\\".\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"2.0\""]
#[doc = "    },"]
#[doc = "    \"method\": {"]
#[doc = "      \"description\": \"A string containing the name of the method to be invoked.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"description\": \"A structured value holding the parameter values to be used during the method invocation.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct JsonrpcRequest {
    #[doc = "A unique identifier established by the client. It must be a String, a Number, or null.\nThe server must reply with the same value in the response. This property is omitted for notifications."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<JsonrpcRequestId>,
    #[doc = "The version of the JSON-RPC protocol. MUST be exactly \"2.0\"."]
    pub jsonrpc: ::std::string::String,
    #[doc = "A string containing the name of the method to be invoked."]
    pub method: ::std::string::String,
    #[doc = "A structured value holding the parameter values to be used during the method invocation."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub params: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl ::std::convert::From<&JsonrpcRequest> for JsonrpcRequest {
    fn from(value: &JsonrpcRequest) -> Self {
        value.clone()
    }
}
impl JsonrpcRequest {
    pub fn builder() -> builder::JsonrpcRequest {
        Default::default()
    }
}
#[doc = "A unique identifier established by the client. It must be a String, a Number, or null.\nThe server must reply with the same value in the response. This property is omitted for notifications."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A unique identifier established by the client. It must be a String, a Number, or null.\\nThe server must reply with the same value in the response. This property is omitted for notifications.\","]
#[doc = "  \"type\": ["]
#[doc = "    \"string\","]
#[doc = "    \"integer\","]
#[doc = "    \"null\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum JsonrpcRequestId {
    Null,
    String(::std::string::String),
    Integer(i64),
}
impl ::std::convert::From<&Self> for JsonrpcRequestId {
    fn from(value: &JsonrpcRequestId) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<i64> for JsonrpcRequestId {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
#[doc = "A discriminated union representing all possible JSON-RPC 2.0 responses\nfor the A2A specification methods."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A discriminated union representing all possible JSON-RPC 2.0 responses\\nfor the A2A specification methods.\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/JSONRPCErrorResponse\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/SendMessageSuccessResponse\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/SendStreamingMessageSuccessResponse\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/GetTaskSuccessResponse\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/CancelTaskSuccessResponse\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/SetTaskPushNotificationConfigSuccessResponse\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/GetTaskPushNotificationConfigSuccessResponse\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ListTaskPushNotificationConfigSuccessResponse\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/DeleteTaskPushNotificationConfigSuccessResponse\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct JsonrpcResponse {
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_0: ::std::option::Option<JsonrpcErrorResponse>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_1: ::std::option::Option<SendMessageSuccessResponse>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_2: ::std::option::Option<SendStreamingMessageSuccessResponse>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_3: ::std::option::Option<GetTaskSuccessResponse>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_4: ::std::option::Option<CancelTaskSuccessResponse>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_5: ::std::option::Option<SetTaskPushNotificationConfigSuccessResponse>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_6: ::std::option::Option<GetTaskPushNotificationConfigSuccessResponse>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_7: ::std::option::Option<ListTaskPushNotificationConfigSuccessResponse>,
    #[serde(
        flatten,
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub subtype_8: ::std::option::Option<DeleteTaskPushNotificationConfigSuccessResponse>,
}
impl ::std::convert::From<&JsonrpcResponse> for JsonrpcResponse {
    fn from(value: &JsonrpcResponse) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for JsonrpcResponse {
    fn default() -> Self {
        Self {
            subtype_0: Default::default(),
            subtype_1: Default::default(),
            subtype_2: Default::default(),
            subtype_3: Default::default(),
            subtype_4: Default::default(),
            subtype_5: Default::default(),
            subtype_6: Default::default(),
            subtype_7: Default::default(),
            subtype_8: Default::default(),
        }
    }
}
impl JsonrpcResponse {
    pub fn builder() -> builder::JsonrpcResponse {
        Default::default()
    }
}
#[doc = "Represents a successful JSON-RPC 2.0 Response object."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Represents a successful JSON-RPC 2.0 Response object.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"jsonrpc\","]
#[doc = "    \"result\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"description\": \"The identifier established by the client.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"jsonrpc\": {"]
#[doc = "      \"description\": \"The version of the JSON-RPC protocol. MUST be exactly \\\"2.0\\\".\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"2.0\""]
#[doc = "    },"]
#[doc = "    \"result\": {"]
#[doc = "      \"description\": \"The value of this member is determined by the method invoked on the Server.\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct JsonrpcSuccessResponse {
    #[doc = "The identifier established by the client."]
    pub id: JsonrpcSuccessResponseId,
    #[doc = "The version of the JSON-RPC protocol. MUST be exactly \"2.0\"."]
    pub jsonrpc: ::std::string::String,
    #[doc = "The value of this member is determined by the method invoked on the Server."]
    pub result: ::serde_json::Value,
}
impl ::std::convert::From<&JsonrpcSuccessResponse> for JsonrpcSuccessResponse {
    fn from(value: &JsonrpcSuccessResponse) -> Self {
        value.clone()
    }
}
impl JsonrpcSuccessResponse {
    pub fn builder() -> builder::JsonrpcSuccessResponse {
        Default::default()
    }
}
#[doc = "The identifier established by the client."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The identifier established by the client.\","]
#[doc = "  \"type\": ["]
#[doc = "    \"string\","]
#[doc = "    \"integer\","]
#[doc = "    \"null\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum JsonrpcSuccessResponseId {
    Null,
    String(::std::string::String),
    Integer(i64),
}
impl ::std::convert::From<&Self> for JsonrpcSuccessResponseId {
    fn from(value: &JsonrpcSuccessResponseId) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<i64> for JsonrpcSuccessResponseId {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
#[doc = "Defines parameters for listing all push notification configurations associated with a task."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Defines parameters for listing all push notification configurations associated with a task.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"description\": \"The unique identifier of the task.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"metadata\": {"]
#[doc = "      \"description\": \"Optional metadata associated with the request.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ListTaskPushNotificationConfigParams {
    #[doc = "The unique identifier of the task."]
    pub id: ::std::string::String,
    #[doc = "Optional metadata associated with the request."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl ::std::convert::From<&ListTaskPushNotificationConfigParams>
    for ListTaskPushNotificationConfigParams
{
    fn from(value: &ListTaskPushNotificationConfigParams) -> Self {
        value.clone()
    }
}
impl ListTaskPushNotificationConfigParams {
    pub fn builder() -> builder::ListTaskPushNotificationConfigParams {
        Default::default()
    }
}
#[doc = "Represents a JSON-RPC request for the `tasks/pushNotificationConfig/list` method."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Represents a JSON-RPC request for the `tasks/pushNotificationConfig/list` method.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"jsonrpc\","]
#[doc = "    \"method\","]
#[doc = "    \"params\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"description\": \"The identifier for this request.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"integer\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"jsonrpc\": {"]
#[doc = "      \"description\": \"The version of the JSON-RPC protocol. MUST be exactly \\\"2.0\\\".\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"2.0\""]
#[doc = "    },"]
#[doc = "    \"method\": {"]
#[doc = "      \"description\": \"The method name. Must be 'tasks/pushNotificationConfig/list'.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"tasks/pushNotificationConfig/list\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"description\": \"The parameters identifying the task whose configurations are to be listed.\","]
#[doc = "      \"$ref\": \"#/definitions/ListTaskPushNotificationConfigParams\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ListTaskPushNotificationConfigRequest {
    #[doc = "The identifier for this request."]
    pub id: ListTaskPushNotificationConfigRequestId,
    #[doc = "The version of the JSON-RPC protocol. MUST be exactly \"2.0\"."]
    pub jsonrpc: ::std::string::String,
    #[doc = "The method name. Must be 'tasks/pushNotificationConfig/list'."]
    pub method: ::std::string::String,
    #[doc = "The parameters identifying the task whose configurations are to be listed."]
    pub params: ListTaskPushNotificationConfigParams,
}
impl ::std::convert::From<&ListTaskPushNotificationConfigRequest>
    for ListTaskPushNotificationConfigRequest
{
    fn from(value: &ListTaskPushNotificationConfigRequest) -> Self {
        value.clone()
    }
}
impl ListTaskPushNotificationConfigRequest {
    pub fn builder() -> builder::ListTaskPushNotificationConfigRequest {
        Default::default()
    }
}
#[doc = "The identifier for this request."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The identifier for this request.\","]
#[doc = "  \"type\": ["]
#[doc = "    \"string\","]
#[doc = "    \"integer\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ListTaskPushNotificationConfigRequestId {
    String(::std::string::String),
    Integer(i64),
}
impl ::std::convert::From<&Self> for ListTaskPushNotificationConfigRequestId {
    fn from(value: &ListTaskPushNotificationConfigRequestId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for ListTaskPushNotificationConfigRequestId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Integer(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl ::std::convert::TryFrom<&str> for ListTaskPushNotificationConfigRequestId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ListTaskPushNotificationConfigRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ListTaskPushNotificationConfigRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::fmt::Display for ListTaskPushNotificationConfigRequestId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::String(x) => x.fmt(f),
            Self::Integer(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<i64> for ListTaskPushNotificationConfigRequestId {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
#[doc = "Represents a JSON-RPC response for the `tasks/pushNotificationConfig/list` method."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Represents a JSON-RPC response for the `tasks/pushNotificationConfig/list` method.\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/JSONRPCErrorResponse\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/ListTaskPushNotificationConfigSuccessResponse\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ListTaskPushNotificationConfigResponse {
    JsonrpcErrorResponse(JsonrpcErrorResponse),
    ListTaskPushNotificationConfigSuccessResponse(ListTaskPushNotificationConfigSuccessResponse),
}
impl ::std::convert::From<&Self> for ListTaskPushNotificationConfigResponse {
    fn from(value: &ListTaskPushNotificationConfigResponse) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<JsonrpcErrorResponse> for ListTaskPushNotificationConfigResponse {
    fn from(value: JsonrpcErrorResponse) -> Self {
        Self::JsonrpcErrorResponse(value)
    }
}
impl ::std::convert::From<ListTaskPushNotificationConfigSuccessResponse>
    for ListTaskPushNotificationConfigResponse
{
    fn from(value: ListTaskPushNotificationConfigSuccessResponse) -> Self {
        Self::ListTaskPushNotificationConfigSuccessResponse(value)
    }
}
#[doc = "Represents a successful JSON-RPC response for the `tasks/pushNotificationConfig/list` method."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Represents a successful JSON-RPC response for the `tasks/pushNotificationConfig/list` method.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"jsonrpc\","]
#[doc = "    \"result\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"description\": \"The identifier established by the client.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"jsonrpc\": {"]
#[doc = "      \"description\": \"The version of the JSON-RPC protocol. MUST be exactly \\\"2.0\\\".\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"2.0\""]
#[doc = "    },"]
#[doc = "    \"result\": {"]
#[doc = "      \"description\": \"The result, containing an array of all push notification configurations for the task.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/TaskPushNotificationConfig\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ListTaskPushNotificationConfigSuccessResponse {
    #[doc = "The identifier established by the client."]
    pub id: ListTaskPushNotificationConfigSuccessResponseId,
    #[doc = "The version of the JSON-RPC protocol. MUST be exactly \"2.0\"."]
    pub jsonrpc: ::std::string::String,
    #[doc = "The result, containing an array of all push notification configurations for the task."]
    pub result: ::std::vec::Vec<TaskPushNotificationConfig>,
}
impl ::std::convert::From<&ListTaskPushNotificationConfigSuccessResponse>
    for ListTaskPushNotificationConfigSuccessResponse
{
    fn from(value: &ListTaskPushNotificationConfigSuccessResponse) -> Self {
        value.clone()
    }
}
impl ListTaskPushNotificationConfigSuccessResponse {
    pub fn builder() -> builder::ListTaskPushNotificationConfigSuccessResponse {
        Default::default()
    }
}
#[doc = "The identifier established by the client."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The identifier established by the client.\","]
#[doc = "  \"type\": ["]
#[doc = "    \"string\","]
#[doc = "    \"integer\","]
#[doc = "    \"null\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ListTaskPushNotificationConfigSuccessResponseId {
    Null,
    String(::std::string::String),
    Integer(i64),
}
impl ::std::convert::From<&Self> for ListTaskPushNotificationConfigSuccessResponseId {
    fn from(value: &ListTaskPushNotificationConfigSuccessResponseId) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<i64> for ListTaskPushNotificationConfigSuccessResponseId {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
#[doc = "Represents a single message in the conversation between a user and an agent."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Represents a single message in the conversation between a user and an agent.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"kind\","]
#[doc = "    \"messageId\","]
#[doc = "    \"parts\","]
#[doc = "    \"role\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"contextId\": {"]
#[doc = "      \"description\": \"The context identifier for this message, used to group related interactions.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"The URIs of extensions that are relevant to this message.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"description\": \"The type of this object, used as a discriminator. Always 'message' for a Message.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"message\""]
#[doc = "    },"]
#[doc = "    \"messageId\": {"]
#[doc = "      \"description\": \"A unique identifier for the message, typically a UUID, generated by the sender.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"metadata\": {"]
#[doc = "      \"description\": \"Optional metadata for extensions. The key is an extension-specific identifier.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    },"]
#[doc = "    \"parts\": {"]
#[doc = "      \"description\": \"An array of content parts that form the message body. A message can be\\ncomposed of multiple parts of different types (e.g., text and files).\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Part\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"referenceTaskIds\": {"]
#[doc = "      \"description\": \"A list of other task IDs that this message references for additional context.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"role\": {"]
#[doc = "      \"description\": \"Identifies the sender of the message. `user` for the client, `agent` for the service.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"agent\","]
#[doc = "        \"user\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"taskId\": {"]
#[doc = "      \"description\": \"The identifier of the task this message is part of. Can be omitted for the first message of a new task.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Message {
    #[doc = "The context identifier for this message, used to group related interactions."]
    #[serde(
        rename = "contextId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub context_id: ::std::option::Option<::std::string::String>,
    #[doc = "The URIs of extensions that are relevant to this message."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub extensions: ::std::vec::Vec<::std::string::String>,
    #[doc = "The type of this object, used as a discriminator. Always 'message' for a Message."]
    pub kind: ::std::string::String,
    #[doc = "A unique identifier for the message, typically a UUID, generated by the sender."]
    #[serde(rename = "messageId")]
    pub message_id: ::std::string::String,
    #[doc = "Optional metadata for extensions. The key is an extension-specific identifier."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "An array of content parts that form the message body. A message can be\ncomposed of multiple parts of different types (e.g., text and files)."]
    pub parts: ::std::vec::Vec<Part>,
    #[doc = "A list of other task IDs that this message references for additional context."]
    #[serde(
        rename = "referenceTaskIds",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub reference_task_ids: ::std::vec::Vec<::std::string::String>,
    #[doc = "Identifies the sender of the message. `user` for the client, `agent` for the service."]
    pub role: MessageRole,
    #[doc = "The identifier of the task this message is part of. Can be omitted for the first message of a new task."]
    #[serde(
        rename = "taskId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub task_id: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&Message> for Message {
    fn from(value: &Message) -> Self {
        value.clone()
    }
}
impl Message {
    pub fn builder() -> builder::Message {
        Default::default()
    }
}
#[doc = "Identifies the sender of the message. `user` for the client, `agent` for the service."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Identifies the sender of the message. `user` for the client, `agent` for the service.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"agent\","]
#[doc = "    \"user\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum MessageRole {
    #[serde(rename = "agent")]
    Agent,
    #[serde(rename = "user")]
    User,
}
impl ::std::convert::From<&Self> for MessageRole {
    fn from(value: &MessageRole) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for MessageRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Agent => write!(f, "agent"),
            Self::User => write!(f, "user"),
        }
    }
}
impl ::std::str::FromStr for MessageRole {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "agent" => Ok(Self::Agent),
            "user" => Ok(Self::User),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MessageRole {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for MessageRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MessageRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Defines configuration options for a `message/send` or `message/stream` request."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Defines configuration options for a `message/send` or `message/stream` request.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"acceptedOutputModes\": {"]
#[doc = "      \"description\": \"A list of output MIME types the client is prepared to accept in the response.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"blocking\": {"]
#[doc = "      \"description\": \"If true, the client will wait for the task to complete. The server may reject this if the task is long-running.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"historyLength\": {"]
#[doc = "      \"description\": \"The number of most recent messages from the task's history to retrieve in the response.\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"pushNotificationConfig\": {"]
#[doc = "      \"description\": \"Configuration for the agent to send push notifications for updates after the initial response.\","]
#[doc = "      \"$ref\": \"#/definitions/PushNotificationConfig\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct MessageSendConfiguration {
    #[doc = "A list of output MIME types the client is prepared to accept in the response."]
    #[serde(
        rename = "acceptedOutputModes",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub accepted_output_modes: ::std::vec::Vec<::std::string::String>,
    #[doc = "If true, the client will wait for the task to complete. The server may reject this if the task is long-running."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub blocking: ::std::option::Option<bool>,
    #[doc = "The number of most recent messages from the task's history to retrieve in the response."]
    #[serde(
        rename = "historyLength",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub history_length: ::std::option::Option<i64>,
    #[doc = "Configuration for the agent to send push notifications for updates after the initial response."]
    #[serde(
        rename = "pushNotificationConfig",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub push_notification_config: ::std::option::Option<PushNotificationConfig>,
}
impl ::std::convert::From<&MessageSendConfiguration> for MessageSendConfiguration {
    fn from(value: &MessageSendConfiguration) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for MessageSendConfiguration {
    fn default() -> Self {
        Self {
            accepted_output_modes: Default::default(),
            blocking: Default::default(),
            history_length: Default::default(),
            push_notification_config: Default::default(),
        }
    }
}
impl MessageSendConfiguration {
    pub fn builder() -> builder::MessageSendConfiguration {
        Default::default()
    }
}
#[doc = "Defines the parameters for a request to send a message to an agent. This can be used\nto create a new task, continue an existing one, or restart a task."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Defines the parameters for a request to send a message to an agent. This can be used\\nto create a new task, continue an existing one, or restart a task.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"message\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"configuration\": {"]
#[doc = "      \"description\": \"Optional configuration for the send request.\","]
#[doc = "      \"$ref\": \"#/definitions/MessageSendConfiguration\""]
#[doc = "    },"]
#[doc = "    \"message\": {"]
#[doc = "      \"description\": \"The message object being sent to the agent.\","]
#[doc = "      \"$ref\": \"#/definitions/Message\""]
#[doc = "    },"]
#[doc = "    \"metadata\": {"]
#[doc = "      \"description\": \"Optional metadata for extensions.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct MessageSendParams {
    #[doc = "Optional configuration for the send request."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub configuration: ::std::option::Option<MessageSendConfiguration>,
    #[doc = "The message object being sent to the agent."]
    pub message: Message,
    #[doc = "Optional metadata for extensions."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl ::std::convert::From<&MessageSendParams> for MessageSendParams {
    fn from(value: &MessageSendParams) -> Self {
        value.clone()
    }
}
impl MessageSendParams {
    pub fn builder() -> builder::MessageSendParams {
        Default::default()
    }
}
#[doc = "An error indicating that the requested method does not exist or is not available."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"An error indicating that the requested method does not exist or is not available.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"code\","]
#[doc = "    \"message\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"code\": {"]
#[doc = "      \"description\": \"The error code for a method not found error.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"const\": -32601"]
#[doc = "    },"]
#[doc = "    \"data\": {"]
#[doc = "      \"description\": \"A primitive or structured value containing additional information about the error.\\nThis may be omitted.\""]
#[doc = "    },"]
#[doc = "    \"message\": {"]
#[doc = "      \"description\": \"The error message.\","]
#[doc = "      \"default\": \"Method not found\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct MethodNotFoundError {
    #[doc = "The error code for a method not found error."]
    pub code: i64,
    #[doc = "A primitive or structured value containing additional information about the error.\nThis may be omitted."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<::serde_json::Value>,
    #[doc = "The error message."]
    pub message: ::std::string::String,
}
impl ::std::convert::From<&MethodNotFoundError> for MethodNotFoundError {
    fn from(value: &MethodNotFoundError) -> Self {
        value.clone()
    }
}
impl MethodNotFoundError {
    pub fn builder() -> builder::MethodNotFoundError {
        Default::default()
    }
}
#[doc = "Defines a security scheme using OAuth 2.0."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Defines a security scheme using OAuth 2.0.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"flows\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"An optional description for the security scheme.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"flows\": {"]
#[doc = "      \"description\": \"An object containing configuration information for the supported OAuth 2.0 flows.\","]
#[doc = "      \"$ref\": \"#/definitions/OAuthFlows\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"description\": \"The type of the security scheme. Must be 'oauth2'.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"oauth2\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct OAuth2SecurityScheme {
    #[doc = "An optional description for the security scheme."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "An object containing configuration information for the supported OAuth 2.0 flows."]
    pub flows: OAuthFlows,
    #[doc = "The type of the security scheme. Must be 'oauth2'."]
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl ::std::convert::From<&OAuth2SecurityScheme> for OAuth2SecurityScheme {
    fn from(value: &OAuth2SecurityScheme) -> Self {
        value.clone()
    }
}
impl OAuth2SecurityScheme {
    pub fn builder() -> builder::OAuth2SecurityScheme {
        Default::default()
    }
}
#[doc = "Defines the configuration for the supported OAuth 2.0 flows."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Defines the configuration for the supported OAuth 2.0 flows.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"authorizationCode\": {"]
#[doc = "      \"description\": \"Configuration for the OAuth Authorization Code flow. Previously called accessCode in OpenAPI 2.0.\","]
#[doc = "      \"$ref\": \"#/definitions/AuthorizationCodeOAuthFlow\""]
#[doc = "    },"]
#[doc = "    \"clientCredentials\": {"]
#[doc = "      \"description\": \"Configuration for the OAuth Client Credentials flow. Previously called application in OpenAPI 2.0.\","]
#[doc = "      \"$ref\": \"#/definitions/ClientCredentialsOAuthFlow\""]
#[doc = "    },"]
#[doc = "    \"implicit\": {"]
#[doc = "      \"description\": \"Configuration for the OAuth Implicit flow.\","]
#[doc = "      \"$ref\": \"#/definitions/ImplicitOAuthFlow\""]
#[doc = "    },"]
#[doc = "    \"password\": {"]
#[doc = "      \"description\": \"Configuration for the OAuth Resource Owner Password flow.\","]
#[doc = "      \"$ref\": \"#/definitions/PasswordOAuthFlow\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct OAuthFlows {
    #[doc = "Configuration for the OAuth Authorization Code flow. Previously called accessCode in OpenAPI 2.0."]
    #[serde(
        rename = "authorizationCode",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub authorization_code: ::std::option::Option<AuthorizationCodeOAuthFlow>,
    #[doc = "Configuration for the OAuth Client Credentials flow. Previously called application in OpenAPI 2.0."]
    #[serde(
        rename = "clientCredentials",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub client_credentials: ::std::option::Option<ClientCredentialsOAuthFlow>,
    #[doc = "Configuration for the OAuth Implicit flow."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub implicit: ::std::option::Option<ImplicitOAuthFlow>,
    #[doc = "Configuration for the OAuth Resource Owner Password flow."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub password: ::std::option::Option<PasswordOAuthFlow>,
}
impl ::std::convert::From<&OAuthFlows> for OAuthFlows {
    fn from(value: &OAuthFlows) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for OAuthFlows {
    fn default() -> Self {
        Self {
            authorization_code: Default::default(),
            client_credentials: Default::default(),
            implicit: Default::default(),
            password: Default::default(),
        }
    }
}
impl OAuthFlows {
    pub fn builder() -> builder::OAuthFlows {
        Default::default()
    }
}
#[doc = "Defines a security scheme using OpenID Connect."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Defines a security scheme using OpenID Connect.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"openIdConnectUrl\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"An optional description for the security scheme.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"openIdConnectUrl\": {"]
#[doc = "      \"description\": \"The OpenID Connect Discovery URL for the OIDC provider's metadata.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"description\": \"The type of the security scheme. Must be 'openIdConnect'.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"openIdConnect\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct OpenIdConnectSecurityScheme {
    #[doc = "An optional description for the security scheme."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "The OpenID Connect Discovery URL for the OIDC provider's metadata."]
    #[serde(rename = "openIdConnectUrl")]
    pub open_id_connect_url: ::std::string::String,
    #[doc = "The type of the security scheme. Must be 'openIdConnect'."]
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl ::std::convert::From<&OpenIdConnectSecurityScheme> for OpenIdConnectSecurityScheme {
    fn from(value: &OpenIdConnectSecurityScheme) -> Self {
        value.clone()
    }
}
impl OpenIdConnectSecurityScheme {
    pub fn builder() -> builder::OpenIdConnectSecurityScheme {
        Default::default()
    }
}
#[doc = "A discriminated union representing a part of a message or artifact, which can\nbe text, a file, or structured data."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A discriminated union representing a part of a message or artifact, which can\\nbe text, a file, or structured data.\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/TextPart\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/FilePart\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/DataPart\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Part {
    TextPart(TextPart),
    FilePart(FilePart),
    DataPart(DataPart),
}
impl ::std::convert::From<&Self> for Part {
    fn from(value: &Part) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<TextPart> for Part {
    fn from(value: TextPart) -> Self {
        Self::TextPart(value)
    }
}
impl ::std::convert::From<FilePart> for Part {
    fn from(value: FilePart) -> Self {
        Self::FilePart(value)
    }
}
impl ::std::convert::From<DataPart> for Part {
    fn from(value: DataPart) -> Self {
        Self::DataPart(value)
    }
}
#[doc = "Defines base properties common to all message or artifact parts."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Defines base properties common to all message or artifact parts.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"metadata\": {"]
#[doc = "      \"description\": \"Optional metadata associated with this part.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PartBase {
    #[doc = "Optional metadata associated with this part."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl ::std::convert::From<&PartBase> for PartBase {
    fn from(value: &PartBase) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for PartBase {
    fn default() -> Self {
        Self {
            metadata: Default::default(),
        }
    }
}
impl PartBase {
    pub fn builder() -> builder::PartBase {
        Default::default()
    }
}
#[doc = "Defines configuration details for the OAuth 2.0 Resource Owner Password flow."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Defines configuration details for the OAuth 2.0 Resource Owner Password flow.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"scopes\","]
#[doc = "    \"tokenUrl\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"refreshUrl\": {"]
#[doc = "      \"description\": \"The URL to be used for obtaining refresh tokens. This MUST be a URL.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"scopes\": {"]
#[doc = "      \"description\": \"The available scopes for the OAuth2 security scheme. A map between the scope\\nname and a short description for it.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"tokenUrl\": {"]
#[doc = "      \"description\": \"The token URL to be used for this flow. This MUST be a URL.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PasswordOAuthFlow {
    #[doc = "The URL to be used for obtaining refresh tokens. This MUST be a URL."]
    #[serde(
        rename = "refreshUrl",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub refresh_url: ::std::option::Option<::std::string::String>,
    #[doc = "The available scopes for the OAuth2 security scheme. A map between the scope\nname and a short description for it."]
    pub scopes: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    #[doc = "The token URL to be used for this flow. This MUST be a URL."]
    #[serde(rename = "tokenUrl")]
    pub token_url: ::std::string::String,
}
impl ::std::convert::From<&PasswordOAuthFlow> for PasswordOAuthFlow {
    fn from(value: &PasswordOAuthFlow) -> Self {
        value.clone()
    }
}
impl PasswordOAuthFlow {
    pub fn builder() -> builder::PasswordOAuthFlow {
        Default::default()
    }
}
#[doc = "Defines authentication details for a push notification endpoint."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Defines authentication details for a push notification endpoint.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"schemes\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"credentials\": {"]
#[doc = "      \"description\": \"Optional credentials required by the push notification endpoint.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"schemes\": {"]
#[doc = "      \"description\": \"A list of supported authentication schemes (e.g., 'Basic', 'Bearer').\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PushNotificationAuthenticationInfo {
    #[doc = "Optional credentials required by the push notification endpoint."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub credentials: ::std::option::Option<::std::string::String>,
    #[doc = "A list of supported authentication schemes (e.g., 'Basic', 'Bearer')."]
    pub schemes: ::std::vec::Vec<::std::string::String>,
}
impl ::std::convert::From<&PushNotificationAuthenticationInfo>
    for PushNotificationAuthenticationInfo
{
    fn from(value: &PushNotificationAuthenticationInfo) -> Self {
        value.clone()
    }
}
impl PushNotificationAuthenticationInfo {
    pub fn builder() -> builder::PushNotificationAuthenticationInfo {
        Default::default()
    }
}
#[doc = "Defines the configuration for setting up push notifications for task updates."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Defines the configuration for setting up push notifications for task updates.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"url\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"authentication\": {"]
#[doc = "      \"description\": \"Optional authentication details for the agent to use when calling the notification URL.\","]
#[doc = "      \"$ref\": \"#/definitions/PushNotificationAuthenticationInfo\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"description\": \"A unique ID for the push notification configuration, created by the server\\nto support multiple notification callbacks.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"token\": {"]
#[doc = "      \"description\": \"A unique token for this task or session to validate incoming push notifications.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"url\": {"]
#[doc = "      \"description\": \"The callback URL where the agent should send push notifications.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PushNotificationConfig {
    #[doc = "Optional authentication details for the agent to use when calling the notification URL."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub authentication: ::std::option::Option<PushNotificationAuthenticationInfo>,
    #[doc = "A unique ID for the push notification configuration, created by the server\nto support multiple notification callbacks."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[doc = "A unique token for this task or session to validate incoming push notifications."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub token: ::std::option::Option<::std::string::String>,
    #[doc = "The callback URL where the agent should send push notifications."]
    pub url: ::std::string::String,
}
impl ::std::convert::From<&PushNotificationConfig> for PushNotificationConfig {
    fn from(value: &PushNotificationConfig) -> Self {
        value.clone()
    }
}
impl PushNotificationConfig {
    pub fn builder() -> builder::PushNotificationConfig {
        Default::default()
    }
}
#[doc = "An A2A-specific error indicating that the agent does not support push notifications."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"An A2A-specific error indicating that the agent does not support push notifications.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"code\","]
#[doc = "    \"message\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"code\": {"]
#[doc = "      \"description\": \"The error code for when push notifications are not supported.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"const\": -32003"]
#[doc = "    },"]
#[doc = "    \"data\": {"]
#[doc = "      \"description\": \"A primitive or structured value containing additional information about the error.\\nThis may be omitted.\""]
#[doc = "    },"]
#[doc = "    \"message\": {"]
#[doc = "      \"description\": \"The error message.\","]
#[doc = "      \"default\": \"Push Notification is not supported\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PushNotificationNotSupportedError {
    #[doc = "The error code for when push notifications are not supported."]
    pub code: i64,
    #[doc = "A primitive or structured value containing additional information about the error.\nThis may be omitted."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<::serde_json::Value>,
    #[doc = "The error message."]
    pub message: ::std::string::String,
}
impl ::std::convert::From<&PushNotificationNotSupportedError>
    for PushNotificationNotSupportedError
{
    fn from(value: &PushNotificationNotSupportedError) -> Self {
        value.clone()
    }
}
impl PushNotificationNotSupportedError {
    pub fn builder() -> builder::PushNotificationNotSupportedError {
        Default::default()
    }
}
#[doc = "Defines a security scheme that can be used to secure an agent's endpoints.\nThis is a discriminated union type based on the OpenAPI 3.0 Security Scheme Object."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Defines a security scheme that can be used to secure an agent's endpoints.\\nThis is a discriminated union type based on the OpenAPI 3.0 Security Scheme Object.\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/APIKeySecurityScheme\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/HTTPAuthSecurityScheme\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/OAuth2SecurityScheme\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/OpenIdConnectSecurityScheme\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum SecurityScheme {
    ApiKeySecurityScheme(ApiKeySecurityScheme),
    HttpAuthSecurityScheme(HttpAuthSecurityScheme),
    OAuth2SecurityScheme(OAuth2SecurityScheme),
    OpenIdConnectSecurityScheme(OpenIdConnectSecurityScheme),
}
impl ::std::convert::From<&Self> for SecurityScheme {
    fn from(value: &SecurityScheme) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<ApiKeySecurityScheme> for SecurityScheme {
    fn from(value: ApiKeySecurityScheme) -> Self {
        Self::ApiKeySecurityScheme(value)
    }
}
impl ::std::convert::From<HttpAuthSecurityScheme> for SecurityScheme {
    fn from(value: HttpAuthSecurityScheme) -> Self {
        Self::HttpAuthSecurityScheme(value)
    }
}
impl ::std::convert::From<OAuth2SecurityScheme> for SecurityScheme {
    fn from(value: OAuth2SecurityScheme) -> Self {
        Self::OAuth2SecurityScheme(value)
    }
}
impl ::std::convert::From<OpenIdConnectSecurityScheme> for SecurityScheme {
    fn from(value: OpenIdConnectSecurityScheme) -> Self {
        Self::OpenIdConnectSecurityScheme(value)
    }
}
#[doc = "Defines base properties shared by all security scheme objects."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Defines base properties shared by all security scheme objects.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"An optional description for the security scheme.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SecuritySchemeBase {
    #[doc = "An optional description for the security scheme."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&SecuritySchemeBase> for SecuritySchemeBase {
    fn from(value: &SecuritySchemeBase) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SecuritySchemeBase {
    fn default() -> Self {
        Self {
            description: Default::default(),
        }
    }
}
impl SecuritySchemeBase {
    pub fn builder() -> builder::SecuritySchemeBase {
        Default::default()
    }
}
#[doc = "Represents a JSON-RPC request for the `message/send` method."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Represents a JSON-RPC request for the `message/send` method.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"jsonrpc\","]
#[doc = "    \"method\","]
#[doc = "    \"params\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"description\": \"The identifier for this request.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"integer\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"jsonrpc\": {"]
#[doc = "      \"description\": \"The version of the JSON-RPC protocol. MUST be exactly \\\"2.0\\\".\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"2.0\""]
#[doc = "    },"]
#[doc = "    \"method\": {"]
#[doc = "      \"description\": \"The method name. Must be 'message/send'.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"message/send\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"description\": \"The parameters for sending a message.\","]
#[doc = "      \"$ref\": \"#/definitions/MessageSendParams\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SendMessageRequest {
    #[doc = "The identifier for this request."]
    pub id: SendMessageRequestId,
    #[doc = "The version of the JSON-RPC protocol. MUST be exactly \"2.0\"."]
    pub jsonrpc: ::std::string::String,
    #[doc = "The method name. Must be 'message/send'."]
    pub method: ::std::string::String,
    #[doc = "The parameters for sending a message."]
    pub params: MessageSendParams,
}
impl ::std::convert::From<&SendMessageRequest> for SendMessageRequest {
    fn from(value: &SendMessageRequest) -> Self {
        value.clone()
    }
}
impl SendMessageRequest {
    pub fn builder() -> builder::SendMessageRequest {
        Default::default()
    }
}
#[doc = "The identifier for this request."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The identifier for this request.\","]
#[doc = "  \"type\": ["]
#[doc = "    \"string\","]
#[doc = "    \"integer\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum SendMessageRequestId {
    String(::std::string::String),
    Integer(i64),
}
impl ::std::convert::From<&Self> for SendMessageRequestId {
    fn from(value: &SendMessageRequestId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for SendMessageRequestId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Integer(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl ::std::convert::TryFrom<&str> for SendMessageRequestId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SendMessageRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SendMessageRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::fmt::Display for SendMessageRequestId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::String(x) => x.fmt(f),
            Self::Integer(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<i64> for SendMessageRequestId {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
#[doc = "Represents a JSON-RPC response for the `message/send` method."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Represents a JSON-RPC response for the `message/send` method.\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/JSONRPCErrorResponse\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/SendMessageSuccessResponse\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum SendMessageResponse {
    JsonrpcErrorResponse(JsonrpcErrorResponse),
    SendMessageSuccessResponse(SendMessageSuccessResponse),
}
impl ::std::convert::From<&Self> for SendMessageResponse {
    fn from(value: &SendMessageResponse) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<JsonrpcErrorResponse> for SendMessageResponse {
    fn from(value: JsonrpcErrorResponse) -> Self {
        Self::JsonrpcErrorResponse(value)
    }
}
impl ::std::convert::From<SendMessageSuccessResponse> for SendMessageResponse {
    fn from(value: SendMessageSuccessResponse) -> Self {
        Self::SendMessageSuccessResponse(value)
    }
}
#[doc = "Represents a successful JSON-RPC response for the `message/send` method."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Represents a successful JSON-RPC response for the `message/send` method.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"jsonrpc\","]
#[doc = "    \"result\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"description\": \"The identifier established by the client.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"jsonrpc\": {"]
#[doc = "      \"description\": \"The version of the JSON-RPC protocol. MUST be exactly \\\"2.0\\\".\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"2.0\""]
#[doc = "    },"]
#[doc = "    \"result\": {"]
#[doc = "      \"description\": \"The result, which can be a direct reply Message or the initial Task object.\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/Task\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/Message\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SendMessageSuccessResponse {
    #[doc = "The identifier established by the client."]
    pub id: SendMessageSuccessResponseId,
    #[doc = "The version of the JSON-RPC protocol. MUST be exactly \"2.0\"."]
    pub jsonrpc: ::std::string::String,
    #[doc = "The result, which can be a direct reply Message or the initial Task object."]
    pub result: SendMessageSuccessResponseResult,
}
impl ::std::convert::From<&SendMessageSuccessResponse> for SendMessageSuccessResponse {
    fn from(value: &SendMessageSuccessResponse) -> Self {
        value.clone()
    }
}
impl SendMessageSuccessResponse {
    pub fn builder() -> builder::SendMessageSuccessResponse {
        Default::default()
    }
}
#[doc = "The identifier established by the client."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The identifier established by the client.\","]
#[doc = "  \"type\": ["]
#[doc = "    \"string\","]
#[doc = "    \"integer\","]
#[doc = "    \"null\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum SendMessageSuccessResponseId {
    Null,
    String(::std::string::String),
    Integer(i64),
}
impl ::std::convert::From<&Self> for SendMessageSuccessResponseId {
    fn from(value: &SendMessageSuccessResponseId) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<i64> for SendMessageSuccessResponseId {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
#[doc = "The result, which can be a direct reply Message or the initial Task object."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The result, which can be a direct reply Message or the initial Task object.\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Task\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Message\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum SendMessageSuccessResponseResult {
    Task(Task),
    Message(Message),
}
impl ::std::convert::From<&Self> for SendMessageSuccessResponseResult {
    fn from(value: &SendMessageSuccessResponseResult) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<Task> for SendMessageSuccessResponseResult {
    fn from(value: Task) -> Self {
        Self::Task(value)
    }
}
impl ::std::convert::From<Message> for SendMessageSuccessResponseResult {
    fn from(value: Message) -> Self {
        Self::Message(value)
    }
}
#[doc = "Represents a JSON-RPC request for the `message/stream` method."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Represents a JSON-RPC request for the `message/stream` method.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"jsonrpc\","]
#[doc = "    \"method\","]
#[doc = "    \"params\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"description\": \"The identifier for this request.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"integer\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"jsonrpc\": {"]
#[doc = "      \"description\": \"The version of the JSON-RPC protocol. MUST be exactly \\\"2.0\\\".\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"2.0\""]
#[doc = "    },"]
#[doc = "    \"method\": {"]
#[doc = "      \"description\": \"The method name. Must be 'message/stream'.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"message/stream\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"description\": \"The parameters for sending a message.\","]
#[doc = "      \"$ref\": \"#/definitions/MessageSendParams\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SendStreamingMessageRequest {
    #[doc = "The identifier for this request."]
    pub id: SendStreamingMessageRequestId,
    #[doc = "The version of the JSON-RPC protocol. MUST be exactly \"2.0\"."]
    pub jsonrpc: ::std::string::String,
    #[doc = "The method name. Must be 'message/stream'."]
    pub method: ::std::string::String,
    #[doc = "The parameters for sending a message."]
    pub params: MessageSendParams,
}
impl ::std::convert::From<&SendStreamingMessageRequest> for SendStreamingMessageRequest {
    fn from(value: &SendStreamingMessageRequest) -> Self {
        value.clone()
    }
}
impl SendStreamingMessageRequest {
    pub fn builder() -> builder::SendStreamingMessageRequest {
        Default::default()
    }
}
#[doc = "The identifier for this request."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The identifier for this request.\","]
#[doc = "  \"type\": ["]
#[doc = "    \"string\","]
#[doc = "    \"integer\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum SendStreamingMessageRequestId {
    String(::std::string::String),
    Integer(i64),
}
impl ::std::convert::From<&Self> for SendStreamingMessageRequestId {
    fn from(value: &SendStreamingMessageRequestId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for SendStreamingMessageRequestId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Integer(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl ::std::convert::TryFrom<&str> for SendStreamingMessageRequestId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SendStreamingMessageRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SendStreamingMessageRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::fmt::Display for SendStreamingMessageRequestId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::String(x) => x.fmt(f),
            Self::Integer(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<i64> for SendStreamingMessageRequestId {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
#[doc = "Represents a JSON-RPC response for the `message/stream` method."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Represents a JSON-RPC response for the `message/stream` method.\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/JSONRPCErrorResponse\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/SendStreamingMessageSuccessResponse\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum SendStreamingMessageResponse {
    JsonrpcErrorResponse(JsonrpcErrorResponse),
    SendStreamingMessageSuccessResponse(SendStreamingMessageSuccessResponse),
}
impl ::std::convert::From<&Self> for SendStreamingMessageResponse {
    fn from(value: &SendStreamingMessageResponse) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<JsonrpcErrorResponse> for SendStreamingMessageResponse {
    fn from(value: JsonrpcErrorResponse) -> Self {
        Self::JsonrpcErrorResponse(value)
    }
}
impl ::std::convert::From<SendStreamingMessageSuccessResponse> for SendStreamingMessageResponse {
    fn from(value: SendStreamingMessageSuccessResponse) -> Self {
        Self::SendStreamingMessageSuccessResponse(value)
    }
}
#[doc = "Represents a successful JSON-RPC response for the `message/stream` method.\nThe server may send multiple response objects for a single request."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Represents a successful JSON-RPC response for the `message/stream` method.\\nThe server may send multiple response objects for a single request.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"jsonrpc\","]
#[doc = "    \"result\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"description\": \"The identifier established by the client.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"jsonrpc\": {"]
#[doc = "      \"description\": \"The version of the JSON-RPC protocol. MUST be exactly \\\"2.0\\\".\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"2.0\""]
#[doc = "    },"]
#[doc = "    \"result\": {"]
#[doc = "      \"description\": \"The result, which can be a Message, Task, or a streaming update event.\","]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/Task\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/Message\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/TaskStatusUpdateEvent\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/TaskArtifactUpdateEvent\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SendStreamingMessageSuccessResponse {
    #[doc = "The identifier established by the client."]
    pub id: SendStreamingMessageSuccessResponseId,
    #[doc = "The version of the JSON-RPC protocol. MUST be exactly \"2.0\"."]
    pub jsonrpc: ::std::string::String,
    #[doc = "The result, which can be a Message, Task, or a streaming update event."]
    pub result: SendStreamingMessageSuccessResponseResult,
}
impl ::std::convert::From<&SendStreamingMessageSuccessResponse>
    for SendStreamingMessageSuccessResponse
{
    fn from(value: &SendStreamingMessageSuccessResponse) -> Self {
        value.clone()
    }
}
impl SendStreamingMessageSuccessResponse {
    pub fn builder() -> builder::SendStreamingMessageSuccessResponse {
        Default::default()
    }
}
#[doc = "The identifier established by the client."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The identifier established by the client.\","]
#[doc = "  \"type\": ["]
#[doc = "    \"string\","]
#[doc = "    \"integer\","]
#[doc = "    \"null\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum SendStreamingMessageSuccessResponseId {
    Null,
    String(::std::string::String),
    Integer(i64),
}
impl ::std::convert::From<&Self> for SendStreamingMessageSuccessResponseId {
    fn from(value: &SendStreamingMessageSuccessResponseId) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<i64> for SendStreamingMessageSuccessResponseId {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
#[doc = "The result, which can be a Message, Task, or a streaming update event."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The result, which can be a Message, Task, or a streaming update event.\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Task\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/Message\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/TaskStatusUpdateEvent\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/TaskArtifactUpdateEvent\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum SendStreamingMessageSuccessResponseResult {
    Task(Task),
    Message(Message),
    TaskStatusUpdateEvent(TaskStatusUpdateEvent),
    TaskArtifactUpdateEvent(TaskArtifactUpdateEvent),
}
impl ::std::convert::From<&Self> for SendStreamingMessageSuccessResponseResult {
    fn from(value: &SendStreamingMessageSuccessResponseResult) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<Task> for SendStreamingMessageSuccessResponseResult {
    fn from(value: Task) -> Self {
        Self::Task(value)
    }
}
impl ::std::convert::From<Message> for SendStreamingMessageSuccessResponseResult {
    fn from(value: Message) -> Self {
        Self::Message(value)
    }
}
impl ::std::convert::From<TaskStatusUpdateEvent> for SendStreamingMessageSuccessResponseResult {
    fn from(value: TaskStatusUpdateEvent) -> Self {
        Self::TaskStatusUpdateEvent(value)
    }
}
impl ::std::convert::From<TaskArtifactUpdateEvent> for SendStreamingMessageSuccessResponseResult {
    fn from(value: TaskArtifactUpdateEvent) -> Self {
        Self::TaskArtifactUpdateEvent(value)
    }
}
#[doc = "Represents a JSON-RPC request for the `tasks/pushNotificationConfig/set` method."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Represents a JSON-RPC request for the `tasks/pushNotificationConfig/set` method.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"jsonrpc\","]
#[doc = "    \"method\","]
#[doc = "    \"params\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"description\": \"The identifier for this request.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"integer\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"jsonrpc\": {"]
#[doc = "      \"description\": \"The version of the JSON-RPC protocol. MUST be exactly \\\"2.0\\\".\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"2.0\""]
#[doc = "    },"]
#[doc = "    \"method\": {"]
#[doc = "      \"description\": \"The method name. Must be 'tasks/pushNotificationConfig/set'.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"tasks/pushNotificationConfig/set\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"description\": \"The parameters for setting the push notification configuration.\","]
#[doc = "      \"$ref\": \"#/definitions/TaskPushNotificationConfig\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SetTaskPushNotificationConfigRequest {
    #[doc = "The identifier for this request."]
    pub id: SetTaskPushNotificationConfigRequestId,
    #[doc = "The version of the JSON-RPC protocol. MUST be exactly \"2.0\"."]
    pub jsonrpc: ::std::string::String,
    #[doc = "The method name. Must be 'tasks/pushNotificationConfig/set'."]
    pub method: ::std::string::String,
    #[doc = "The parameters for setting the push notification configuration."]
    pub params: TaskPushNotificationConfig,
}
impl ::std::convert::From<&SetTaskPushNotificationConfigRequest>
    for SetTaskPushNotificationConfigRequest
{
    fn from(value: &SetTaskPushNotificationConfigRequest) -> Self {
        value.clone()
    }
}
impl SetTaskPushNotificationConfigRequest {
    pub fn builder() -> builder::SetTaskPushNotificationConfigRequest {
        Default::default()
    }
}
#[doc = "The identifier for this request."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The identifier for this request.\","]
#[doc = "  \"type\": ["]
#[doc = "    \"string\","]
#[doc = "    \"integer\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum SetTaskPushNotificationConfigRequestId {
    String(::std::string::String),
    Integer(i64),
}
impl ::std::convert::From<&Self> for SetTaskPushNotificationConfigRequestId {
    fn from(value: &SetTaskPushNotificationConfigRequestId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for SetTaskPushNotificationConfigRequestId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Integer(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl ::std::convert::TryFrom<&str> for SetTaskPushNotificationConfigRequestId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SetTaskPushNotificationConfigRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SetTaskPushNotificationConfigRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::fmt::Display for SetTaskPushNotificationConfigRequestId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::String(x) => x.fmt(f),
            Self::Integer(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<i64> for SetTaskPushNotificationConfigRequestId {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
#[doc = "Represents a JSON-RPC response for the `tasks/pushNotificationConfig/set` method."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Represents a JSON-RPC response for the `tasks/pushNotificationConfig/set` method.\","]
#[doc = "  \"anyOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/JSONRPCErrorResponse\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/SetTaskPushNotificationConfigSuccessResponse\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum SetTaskPushNotificationConfigResponse {
    JsonrpcErrorResponse(JsonrpcErrorResponse),
    SetTaskPushNotificationConfigSuccessResponse(SetTaskPushNotificationConfigSuccessResponse),
}
impl ::std::convert::From<&Self> for SetTaskPushNotificationConfigResponse {
    fn from(value: &SetTaskPushNotificationConfigResponse) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<JsonrpcErrorResponse> for SetTaskPushNotificationConfigResponse {
    fn from(value: JsonrpcErrorResponse) -> Self {
        Self::JsonrpcErrorResponse(value)
    }
}
impl ::std::convert::From<SetTaskPushNotificationConfigSuccessResponse>
    for SetTaskPushNotificationConfigResponse
{
    fn from(value: SetTaskPushNotificationConfigSuccessResponse) -> Self {
        Self::SetTaskPushNotificationConfigSuccessResponse(value)
    }
}
#[doc = "Represents a successful JSON-RPC response for the `tasks/pushNotificationConfig/set` method."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Represents a successful JSON-RPC response for the `tasks/pushNotificationConfig/set` method.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"jsonrpc\","]
#[doc = "    \"result\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"description\": \"The identifier established by the client.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"integer\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"jsonrpc\": {"]
#[doc = "      \"description\": \"The version of the JSON-RPC protocol. MUST be exactly \\\"2.0\\\".\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"2.0\""]
#[doc = "    },"]
#[doc = "    \"result\": {"]
#[doc = "      \"description\": \"The result, containing the configured push notification settings.\","]
#[doc = "      \"$ref\": \"#/definitions/TaskPushNotificationConfig\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct SetTaskPushNotificationConfigSuccessResponse {
    #[doc = "The identifier established by the client."]
    pub id: SetTaskPushNotificationConfigSuccessResponseId,
    #[doc = "The version of the JSON-RPC protocol. MUST be exactly \"2.0\"."]
    pub jsonrpc: ::std::string::String,
    #[doc = "The result, containing the configured push notification settings."]
    pub result: TaskPushNotificationConfig,
}
impl ::std::convert::From<&SetTaskPushNotificationConfigSuccessResponse>
    for SetTaskPushNotificationConfigSuccessResponse
{
    fn from(value: &SetTaskPushNotificationConfigSuccessResponse) -> Self {
        value.clone()
    }
}
impl SetTaskPushNotificationConfigSuccessResponse {
    pub fn builder() -> builder::SetTaskPushNotificationConfigSuccessResponse {
        Default::default()
    }
}
#[doc = "The identifier established by the client."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The identifier established by the client.\","]
#[doc = "  \"type\": ["]
#[doc = "    \"string\","]
#[doc = "    \"integer\","]
#[doc = "    \"null\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum SetTaskPushNotificationConfigSuccessResponseId {
    Null,
    String(::std::string::String),
    Integer(i64),
}
impl ::std::convert::From<&Self> for SetTaskPushNotificationConfigSuccessResponseId {
    fn from(value: &SetTaskPushNotificationConfigSuccessResponseId) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<i64> for SetTaskPushNotificationConfigSuccessResponseId {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
#[doc = "Represents a single, stateful operation or conversation between a client and an agent."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Represents a single, stateful operation or conversation between a client and an agent.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"contextId\","]
#[doc = "    \"id\","]
#[doc = "    \"kind\","]
#[doc = "    \"status\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"artifacts\": {"]
#[doc = "      \"description\": \"A collection of artifacts generated by the agent during the execution of the task.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Artifact\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"contextId\": {"]
#[doc = "      \"description\": \"A server-generated identifier for maintaining context across multiple related tasks or interactions.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"history\": {"]
#[doc = "      \"description\": \"An array of messages exchanged during the task, representing the conversation history.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Message\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"description\": \"A unique identifier for the task, generated by the client for a new task or provided by the agent.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"description\": \"The type of this object, used as a discriminator. Always 'task' for a Task.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"task\""]
#[doc = "    },"]
#[doc = "    \"metadata\": {"]
#[doc = "      \"description\": \"Optional metadata for extensions. The key is an extension-specific identifier.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"description\": \"The current status of the task, including its state and a descriptive message.\","]
#[doc = "      \"$ref\": \"#/definitions/TaskStatus\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct Task {
    #[doc = "A collection of artifacts generated by the agent during the execution of the task."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub artifacts: ::std::vec::Vec<Artifact>,
    #[doc = "A server-generated identifier for maintaining context across multiple related tasks or interactions."]
    #[serde(rename = "contextId")]
    pub context_id: ::std::string::String,
    #[doc = "An array of messages exchanged during the task, representing the conversation history."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub history: ::std::vec::Vec<Message>,
    #[doc = "A unique identifier for the task, generated by the client for a new task or provided by the agent."]
    pub id: ::std::string::String,
    #[doc = "The type of this object, used as a discriminator. Always 'task' for a Task."]
    pub kind: ::std::string::String,
    #[doc = "Optional metadata for extensions. The key is an extension-specific identifier."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "The current status of the task, including its state and a descriptive message."]
    pub status: TaskStatus,
}
impl ::std::convert::From<&Task> for Task {
    fn from(value: &Task) -> Self {
        value.clone()
    }
}
impl Task {
    pub fn builder() -> builder::Task {
        Default::default()
    }
}
#[doc = "An event sent by the agent to notify the client that an artifact has been\ngenerated or updated. This is typically used in streaming models."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"An event sent by the agent to notify the client that an artifact has been\\ngenerated or updated. This is typically used in streaming models.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"artifact\","]
#[doc = "    \"contextId\","]
#[doc = "    \"kind\","]
#[doc = "    \"taskId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"append\": {"]
#[doc = "      \"description\": \"If true, the content of this artifact should be appended to a previously sent artifact with the same ID.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"artifact\": {"]
#[doc = "      \"description\": \"The artifact that was generated or updated.\","]
#[doc = "      \"$ref\": \"#/definitions/Artifact\""]
#[doc = "    },"]
#[doc = "    \"contextId\": {"]
#[doc = "      \"description\": \"The context ID associated with the task.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"description\": \"The type of this event, used as a discriminator. Always 'artifact-update'.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"artifact-update\""]
#[doc = "    },"]
#[doc = "    \"lastChunk\": {"]
#[doc = "      \"description\": \"If true, this is the final chunk of the artifact.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"metadata\": {"]
#[doc = "      \"description\": \"Optional metadata for extensions.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    },"]
#[doc = "    \"taskId\": {"]
#[doc = "      \"description\": \"The ID of the task this artifact belongs to.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TaskArtifactUpdateEvent {
    #[doc = "If true, the content of this artifact should be appended to a previously sent artifact with the same ID."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub append: ::std::option::Option<bool>,
    #[doc = "The artifact that was generated or updated."]
    pub artifact: Artifact,
    #[doc = "The context ID associated with the task."]
    #[serde(rename = "contextId")]
    pub context_id: ::std::string::String,
    #[doc = "The type of this event, used as a discriminator. Always 'artifact-update'."]
    pub kind: ::std::string::String,
    #[doc = "If true, this is the final chunk of the artifact."]
    #[serde(
        rename = "lastChunk",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub last_chunk: ::std::option::Option<bool>,
    #[doc = "Optional metadata for extensions."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "The ID of the task this artifact belongs to."]
    #[serde(rename = "taskId")]
    pub task_id: ::std::string::String,
}
impl ::std::convert::From<&TaskArtifactUpdateEvent> for TaskArtifactUpdateEvent {
    fn from(value: &TaskArtifactUpdateEvent) -> Self {
        value.clone()
    }
}
impl TaskArtifactUpdateEvent {
    pub fn builder() -> builder::TaskArtifactUpdateEvent {
        Default::default()
    }
}
#[doc = "Defines parameters containing a task ID, used for simple task operations."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Defines parameters containing a task ID, used for simple task operations.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"description\": \"The unique identifier of the task.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"metadata\": {"]
#[doc = "      \"description\": \"Optional metadata associated with the request.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TaskIdParams {
    #[doc = "The unique identifier of the task."]
    pub id: ::std::string::String,
    #[doc = "Optional metadata associated with the request."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl ::std::convert::From<&TaskIdParams> for TaskIdParams {
    fn from(value: &TaskIdParams) -> Self {
        value.clone()
    }
}
impl TaskIdParams {
    pub fn builder() -> builder::TaskIdParams {
        Default::default()
    }
}
#[doc = "An A2A-specific error indicating that the task is in a state where it cannot be canceled."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"An A2A-specific error indicating that the task is in a state where it cannot be canceled.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"code\","]
#[doc = "    \"message\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"code\": {"]
#[doc = "      \"description\": \"The error code for a task that cannot be canceled.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"const\": -32002"]
#[doc = "    },"]
#[doc = "    \"data\": {"]
#[doc = "      \"description\": \"A primitive or structured value containing additional information about the error.\\nThis may be omitted.\""]
#[doc = "    },"]
#[doc = "    \"message\": {"]
#[doc = "      \"description\": \"The error message.\","]
#[doc = "      \"default\": \"Task cannot be canceled\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TaskNotCancelableError {
    #[doc = "The error code for a task that cannot be canceled."]
    pub code: i64,
    #[doc = "A primitive or structured value containing additional information about the error.\nThis may be omitted."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<::serde_json::Value>,
    #[doc = "The error message."]
    pub message: ::std::string::String,
}
impl ::std::convert::From<&TaskNotCancelableError> for TaskNotCancelableError {
    fn from(value: &TaskNotCancelableError) -> Self {
        value.clone()
    }
}
impl TaskNotCancelableError {
    pub fn builder() -> builder::TaskNotCancelableError {
        Default::default()
    }
}
#[doc = "An A2A-specific error indicating that the requested task ID was not found."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"An A2A-specific error indicating that the requested task ID was not found.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"code\","]
#[doc = "    \"message\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"code\": {"]
#[doc = "      \"description\": \"The error code for a task not found error.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"const\": -32001"]
#[doc = "    },"]
#[doc = "    \"data\": {"]
#[doc = "      \"description\": \"A primitive or structured value containing additional information about the error.\\nThis may be omitted.\""]
#[doc = "    },"]
#[doc = "    \"message\": {"]
#[doc = "      \"description\": \"The error message.\","]
#[doc = "      \"default\": \"Task not found\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TaskNotFoundError {
    #[doc = "The error code for a task not found error."]
    pub code: i64,
    #[doc = "A primitive or structured value containing additional information about the error.\nThis may be omitted."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<::serde_json::Value>,
    #[doc = "The error message."]
    pub message: ::std::string::String,
}
impl ::std::convert::From<&TaskNotFoundError> for TaskNotFoundError {
    fn from(value: &TaskNotFoundError) -> Self {
        value.clone()
    }
}
impl TaskNotFoundError {
    pub fn builder() -> builder::TaskNotFoundError {
        Default::default()
    }
}
#[doc = "A container associating a push notification configuration with a specific task."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"A container associating a push notification configuration with a specific task.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"pushNotificationConfig\","]
#[doc = "    \"taskId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"pushNotificationConfig\": {"]
#[doc = "      \"description\": \"The push notification configuration for this task.\","]
#[doc = "      \"$ref\": \"#/definitions/PushNotificationConfig\""]
#[doc = "    },"]
#[doc = "    \"taskId\": {"]
#[doc = "      \"description\": \"The ID of the task.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TaskPushNotificationConfig {
    #[doc = "The push notification configuration for this task."]
    #[serde(rename = "pushNotificationConfig")]
    pub push_notification_config: PushNotificationConfig,
    #[doc = "The ID of the task."]
    #[serde(rename = "taskId")]
    pub task_id: ::std::string::String,
}
impl ::std::convert::From<&TaskPushNotificationConfig> for TaskPushNotificationConfig {
    fn from(value: &TaskPushNotificationConfig) -> Self {
        value.clone()
    }
}
impl TaskPushNotificationConfig {
    pub fn builder() -> builder::TaskPushNotificationConfig {
        Default::default()
    }
}
#[doc = "Defines parameters for querying a task, with an option to limit history length."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Defines parameters for querying a task, with an option to limit history length.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"historyLength\": {"]
#[doc = "      \"description\": \"The number of most recent messages from the task's history to retrieve.\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"description\": \"The unique identifier of the task.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"metadata\": {"]
#[doc = "      \"description\": \"Optional metadata associated with the request.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TaskQueryParams {
    #[doc = "The number of most recent messages from the task's history to retrieve."]
    #[serde(
        rename = "historyLength",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub history_length: ::std::option::Option<i64>,
    #[doc = "The unique identifier of the task."]
    pub id: ::std::string::String,
    #[doc = "Optional metadata associated with the request."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl ::std::convert::From<&TaskQueryParams> for TaskQueryParams {
    fn from(value: &TaskQueryParams) -> Self {
        value.clone()
    }
}
impl TaskQueryParams {
    pub fn builder() -> builder::TaskQueryParams {
        Default::default()
    }
}
#[doc = "Represents a JSON-RPC request for the `tasks/resubscribe` method, used to resume a streaming connection."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Represents a JSON-RPC request for the `tasks/resubscribe` method, used to resume a streaming connection.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"jsonrpc\","]
#[doc = "    \"method\","]
#[doc = "    \"params\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"description\": \"The identifier for this request.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"integer\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"jsonrpc\": {"]
#[doc = "      \"description\": \"The version of the JSON-RPC protocol. MUST be exactly \\\"2.0\\\".\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"2.0\""]
#[doc = "    },"]
#[doc = "    \"method\": {"]
#[doc = "      \"description\": \"The method name. Must be 'tasks/resubscribe'.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"tasks/resubscribe\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"description\": \"The parameters identifying the task to resubscribe to.\","]
#[doc = "      \"$ref\": \"#/definitions/TaskIdParams\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TaskResubscriptionRequest {
    #[doc = "The identifier for this request."]
    pub id: TaskResubscriptionRequestId,
    #[doc = "The version of the JSON-RPC protocol. MUST be exactly \"2.0\"."]
    pub jsonrpc: ::std::string::String,
    #[doc = "The method name. Must be 'tasks/resubscribe'."]
    pub method: ::std::string::String,
    #[doc = "The parameters identifying the task to resubscribe to."]
    pub params: TaskIdParams,
}
impl ::std::convert::From<&TaskResubscriptionRequest> for TaskResubscriptionRequest {
    fn from(value: &TaskResubscriptionRequest) -> Self {
        value.clone()
    }
}
impl TaskResubscriptionRequest {
    pub fn builder() -> builder::TaskResubscriptionRequest {
        Default::default()
    }
}
#[doc = "The identifier for this request."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The identifier for this request.\","]
#[doc = "  \"type\": ["]
#[doc = "    \"string\","]
#[doc = "    \"integer\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum TaskResubscriptionRequestId {
    String(::std::string::String),
    Integer(i64),
}
impl ::std::convert::From<&Self> for TaskResubscriptionRequestId {
    fn from(value: &TaskResubscriptionRequestId) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for TaskResubscriptionRequestId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Integer(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl ::std::convert::TryFrom<&str> for TaskResubscriptionRequestId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TaskResubscriptionRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TaskResubscriptionRequestId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::fmt::Display for TaskResubscriptionRequestId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::String(x) => x.fmt(f),
            Self::Integer(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<i64> for TaskResubscriptionRequestId {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
#[doc = "Defines the lifecycle states of a Task."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Defines the lifecycle states of a Task.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"submitted\","]
#[doc = "    \"working\","]
#[doc = "    \"input-required\","]
#[doc = "    \"completed\","]
#[doc = "    \"canceled\","]
#[doc = "    \"failed\","]
#[doc = "    \"rejected\","]
#[doc = "    \"auth-required\","]
#[doc = "    \"unknown\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum TaskState {
    #[serde(rename = "submitted")]
    Submitted,
    #[serde(rename = "working")]
    Working,
    #[serde(rename = "input-required")]
    InputRequired,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "auth-required")]
    AuthRequired,
    #[serde(rename = "unknown")]
    Unknown,
}
impl ::std::convert::From<&Self> for TaskState {
    fn from(value: &TaskState) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for TaskState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Submitted => write!(f, "submitted"),
            Self::Working => write!(f, "working"),
            Self::InputRequired => write!(f, "input-required"),
            Self::Completed => write!(f, "completed"),
            Self::Canceled => write!(f, "canceled"),
            Self::Failed => write!(f, "failed"),
            Self::Rejected => write!(f, "rejected"),
            Self::AuthRequired => write!(f, "auth-required"),
            Self::Unknown => write!(f, "unknown"),
        }
    }
}
impl ::std::str::FromStr for TaskState {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "submitted" => Ok(Self::Submitted),
            "working" => Ok(Self::Working),
            "input-required" => Ok(Self::InputRequired),
            "completed" => Ok(Self::Completed),
            "canceled" => Ok(Self::Canceled),
            "failed" => Ok(Self::Failed),
            "rejected" => Ok(Self::Rejected),
            "auth-required" => Ok(Self::AuthRequired),
            "unknown" => Ok(Self::Unknown),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TaskState {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TaskState {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TaskState {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Represents the status of a task at a specific point in time."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Represents the status of a task at a specific point in time.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"state\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"message\": {"]
#[doc = "      \"description\": \"An optional, human-readable message providing more details about the current status.\","]
#[doc = "      \"$ref\": \"#/definitions/Message\""]
#[doc = "    },"]
#[doc = "    \"state\": {"]
#[doc = "      \"description\": \"The current state of the task's lifecycle.\","]
#[doc = "      \"$ref\": \"#/definitions/TaskState\""]
#[doc = "    },"]
#[doc = "    \"timestamp\": {"]
#[doc = "      \"description\": \"An ISO 8601 datetime string indicating when this status was recorded.\","]
#[doc = "      \"examples\": ["]
#[doc = "        \"2023-10-27T10:00:00Z\""]
#[doc = "      ],"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TaskStatus {
    #[doc = "An optional, human-readable message providing more details about the current status."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub message: ::std::option::Option<Message>,
    #[doc = "The current state of the task's lifecycle."]
    pub state: TaskState,
    #[doc = "An ISO 8601 datetime string indicating when this status was recorded."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub timestamp: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&TaskStatus> for TaskStatus {
    fn from(value: &TaskStatus) -> Self {
        value.clone()
    }
}
impl TaskStatus {
    pub fn builder() -> builder::TaskStatus {
        Default::default()
    }
}
#[doc = "An event sent by the agent to notify the client of a change in a task's status.\nThis is typically used in streaming or subscription models."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"An event sent by the agent to notify the client of a change in a task's status.\\nThis is typically used in streaming or subscription models.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"contextId\","]
#[doc = "    \"final\","]
#[doc = "    \"kind\","]
#[doc = "    \"status\","]
#[doc = "    \"taskId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"contextId\": {"]
#[doc = "      \"description\": \"The context ID associated with the task.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"final\": {"]
#[doc = "      \"description\": \"If true, this is the final event in the stream for this interaction.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"kind\": {"]
#[doc = "      \"description\": \"The type of this event, used as a discriminator. Always 'status-update'.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"status-update\""]
#[doc = "    },"]
#[doc = "    \"metadata\": {"]
#[doc = "      \"description\": \"Optional metadata for extensions.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"description\": \"The new status of the task.\","]
#[doc = "      \"$ref\": \"#/definitions/TaskStatus\""]
#[doc = "    },"]
#[doc = "    \"taskId\": {"]
#[doc = "      \"description\": \"The ID of the task that was updated.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TaskStatusUpdateEvent {
    #[doc = "The context ID associated with the task."]
    #[serde(rename = "contextId")]
    pub context_id: ::std::string::String,
    #[doc = "If true, this is the final event in the stream for this interaction."]
    #[serde(rename = "final")]
    pub final_: bool,
    #[doc = "The type of this event, used as a discriminator. Always 'status-update'."]
    pub kind: ::std::string::String,
    #[doc = "Optional metadata for extensions."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "The new status of the task."]
    pub status: TaskStatus,
    #[doc = "The ID of the task that was updated."]
    #[serde(rename = "taskId")]
    pub task_id: ::std::string::String,
}
impl ::std::convert::From<&TaskStatusUpdateEvent> for TaskStatusUpdateEvent {
    fn from(value: &TaskStatusUpdateEvent) -> Self {
        value.clone()
    }
}
impl TaskStatusUpdateEvent {
    pub fn builder() -> builder::TaskStatusUpdateEvent {
        Default::default()
    }
}
#[doc = "Represents a text segment within a message or artifact."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Represents a text segment within a message or artifact.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"kind\","]
#[doc = "    \"text\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"kind\": {"]
#[doc = "      \"description\": \"The type of this part, used as a discriminator. Always 'text'.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"text\""]
#[doc = "    },"]
#[doc = "    \"metadata\": {"]
#[doc = "      \"description\": \"Optional metadata associated with this part.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {}"]
#[doc = "    },"]
#[doc = "    \"text\": {"]
#[doc = "      \"description\": \"The string content of the text part.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TextPart {
    #[doc = "The type of this part, used as a discriminator. Always 'text'."]
    pub kind: ::std::string::String,
    #[doc = "Optional metadata associated with this part."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "The string content of the text part."]
    pub text: ::std::string::String,
}
impl ::std::convert::From<&TextPart> for TextPart {
    fn from(value: &TextPart) -> Self {
        value.clone()
    }
}
impl TextPart {
    pub fn builder() -> builder::TextPart {
        Default::default()
    }
}
#[doc = "Supported A2A transport protocols."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Supported A2A transport protocols.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"JSONRPC\","]
#[doc = "    \"GRPC\","]
#[doc = "    \"HTTP+JSON\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum TransportProtocol {
    #[serde(rename = "JSONRPC")]
    Jsonrpc,
    #[serde(rename = "GRPC")]
    Grpc,
    #[serde(rename = "HTTP+JSON")]
    HttpJson,
}
impl ::std::convert::From<&Self> for TransportProtocol {
    fn from(value: &TransportProtocol) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for TransportProtocol {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Jsonrpc => write!(f, "JSONRPC"),
            Self::Grpc => write!(f, "GRPC"),
            Self::HttpJson => write!(f, "HTTP+JSON"),
        }
    }
}
impl ::std::str::FromStr for TransportProtocol {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "JSONRPC" => Ok(Self::Jsonrpc),
            "GRPC" => Ok(Self::Grpc),
            "HTTP+JSON" => Ok(Self::HttpJson),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TransportProtocol {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TransportProtocol {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TransportProtocol {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "An A2A-specific error indicating that the requested operation is not supported by the agent."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"An A2A-specific error indicating that the requested operation is not supported by the agent.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"code\","]
#[doc = "    \"message\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"code\": {"]
#[doc = "      \"description\": \"The error code for an unsupported operation.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"const\": -32004"]
#[doc = "    },"]
#[doc = "    \"data\": {"]
#[doc = "      \"description\": \"A primitive or structured value containing additional information about the error.\\nThis may be omitted.\""]
#[doc = "    },"]
#[doc = "    \"message\": {"]
#[doc = "      \"description\": \"The error message.\","]
#[doc = "      \"default\": \"This operation is not supported\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct UnsupportedOperationError {
    #[doc = "The error code for an unsupported operation."]
    pub code: i64,
    #[doc = "A primitive or structured value containing additional information about the error.\nThis may be omitted."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<::serde_json::Value>,
    #[doc = "The error message."]
    pub message: ::std::string::String,
}
impl ::std::convert::From<&UnsupportedOperationError> for UnsupportedOperationError {
    fn from(value: &UnsupportedOperationError) -> Self {
        value.clone()
    }
}
impl UnsupportedOperationError {
    pub fn builder() -> builder::UnsupportedOperationError {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct A2aError {
        subtype_0: ::std::result::Result<
            ::std::option::Option<super::JsonParseError>,
            ::std::string::String,
        >,
        subtype_1: ::std::result::Result<
            ::std::option::Option<super::InvalidRequestError>,
            ::std::string::String,
        >,
        subtype_2: ::std::result::Result<
            ::std::option::Option<super::MethodNotFoundError>,
            ::std::string::String,
        >,
        subtype_3: ::std::result::Result<
            ::std::option::Option<super::InvalidParamsError>,
            ::std::string::String,
        >,
        subtype_4: ::std::result::Result<
            ::std::option::Option<super::InternalError>,
            ::std::string::String,
        >,
        subtype_5: ::std::result::Result<
            ::std::option::Option<super::TaskNotFoundError>,
            ::std::string::String,
        >,
        subtype_6: ::std::result::Result<
            ::std::option::Option<super::TaskNotCancelableError>,
            ::std::string::String,
        >,
        subtype_7: ::std::result::Result<
            ::std::option::Option<super::PushNotificationNotSupportedError>,
            ::std::string::String,
        >,
        subtype_8: ::std::result::Result<
            ::std::option::Option<super::UnsupportedOperationError>,
            ::std::string::String,
        >,
        subtype_9: ::std::result::Result<
            ::std::option::Option<super::ContentTypeNotSupportedError>,
            ::std::string::String,
        >,
        subtype_10: ::std::result::Result<
            ::std::option::Option<super::InvalidAgentResponseError>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for A2aError {
        fn default() -> Self {
            Self {
                subtype_0: Ok(Default::default()),
                subtype_1: Ok(Default::default()),
                subtype_2: Ok(Default::default()),
                subtype_3: Ok(Default::default()),
                subtype_4: Ok(Default::default()),
                subtype_5: Ok(Default::default()),
                subtype_6: Ok(Default::default()),
                subtype_7: Ok(Default::default()),
                subtype_8: Ok(Default::default()),
                subtype_9: Ok(Default::default()),
                subtype_10: Ok(Default::default()),
            }
        }
    }
    impl A2aError {
        pub fn subtype_0<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::JsonParseError>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_0 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_0: {}", e));
            self
        }
        pub fn subtype_1<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::InvalidRequestError>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_1: {}", e));
            self
        }
        pub fn subtype_2<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::MethodNotFoundError>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_2 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_2: {}", e));
            self
        }
        pub fn subtype_3<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::InvalidParamsError>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_3 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_3: {}", e));
            self
        }
        pub fn subtype_4<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::InternalError>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_4 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_4: {}", e));
            self
        }
        pub fn subtype_5<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TaskNotFoundError>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_5 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_5: {}", e));
            self
        }
        pub fn subtype_6<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TaskNotCancelableError>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_6 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_6: {}", e));
            self
        }
        pub fn subtype_7<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::option::Option<super::PushNotificationNotSupportedError>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_7 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_7: {}", e));
            self
        }
        pub fn subtype_8<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::UnsupportedOperationError>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_8 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_8: {}", e));
            self
        }
        pub fn subtype_9<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ContentTypeNotSupportedError>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_9 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_9: {}", e));
            self
        }
        pub fn subtype_10<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::InvalidAgentResponseError>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_10 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_10: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<A2aError> for super::A2aError {
        type Error = super::error::ConversionError;
        fn try_from(value: A2aError) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                subtype_0: value.subtype_0?,
                subtype_1: value.subtype_1?,
                subtype_2: value.subtype_2?,
                subtype_3: value.subtype_3?,
                subtype_4: value.subtype_4?,
                subtype_5: value.subtype_5?,
                subtype_6: value.subtype_6?,
                subtype_7: value.subtype_7?,
                subtype_8: value.subtype_8?,
                subtype_9: value.subtype_9?,
                subtype_10: value.subtype_10?,
            })
        }
    }
    impl ::std::convert::From<super::A2aError> for A2aError {
        fn from(value: super::A2aError) -> Self {
            Self {
                subtype_0: Ok(value.subtype_0),
                subtype_1: Ok(value.subtype_1),
                subtype_2: Ok(value.subtype_2),
                subtype_3: Ok(value.subtype_3),
                subtype_4: Ok(value.subtype_4),
                subtype_5: Ok(value.subtype_5),
                subtype_6: Ok(value.subtype_6),
                subtype_7: Ok(value.subtype_7),
                subtype_8: Ok(value.subtype_8),
                subtype_9: Ok(value.subtype_9),
                subtype_10: Ok(value.subtype_10),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AgentCapabilities {
        extensions:
            ::std::result::Result<::std::vec::Vec<super::AgentExtension>, ::std::string::String>,
        push_notifications:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        state_transition_history:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        streaming: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
    }
    impl ::std::default::Default for AgentCapabilities {
        fn default() -> Self {
            Self {
                extensions: Ok(Default::default()),
                push_notifications: Ok(Default::default()),
                state_transition_history: Ok(Default::default()),
                streaming: Ok(Default::default()),
            }
        }
    }
    impl AgentCapabilities {
        pub fn extensions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::AgentExtension>>,
            T::Error: ::std::fmt::Display,
        {
            self.extensions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extensions: {}", e));
            self
        }
        pub fn push_notifications<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.push_notifications = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for push_notifications: {}",
                    e
                )
            });
            self
        }
        pub fn state_transition_history<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.state_transition_history = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for state_transition_history: {}",
                    e
                )
            });
            self
        }
        pub fn streaming<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.streaming = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for streaming: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<AgentCapabilities> for super::AgentCapabilities {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AgentCapabilities,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                extensions: value.extensions?,
                push_notifications: value.push_notifications?,
                state_transition_history: value.state_transition_history?,
                streaming: value.streaming?,
            })
        }
    }
    impl ::std::convert::From<super::AgentCapabilities> for AgentCapabilities {
        fn from(value: super::AgentCapabilities) -> Self {
            Self {
                extensions: Ok(value.extensions),
                push_notifications: Ok(value.push_notifications),
                state_transition_history: Ok(value.state_transition_history),
                streaming: Ok(value.streaming),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AgentCard {
        additional_interfaces:
            ::std::result::Result<::std::vec::Vec<super::AgentInterface>, ::std::string::String>,
        capabilities: ::std::result::Result<super::AgentCapabilities, ::std::string::String>,
        default_input_modes:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        default_output_modes:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        description: ::std::result::Result<::std::string::String, ::std::string::String>,
        documentation_url: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        icon_url: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        preferred_transport: ::std::result::Result<::std::string::String, ::std::string::String>,
        protocol_version: ::std::result::Result<::std::string::String, ::std::string::String>,
        provider: ::std::result::Result<
            ::std::option::Option<super::AgentProvider>,
            ::std::string::String,
        >,
        security: ::std::result::Result<
            ::std::vec::Vec<
                ::std::collections::HashMap<
                    ::std::string::String,
                    ::std::vec::Vec<::std::string::String>,
                >,
            >,
            ::std::string::String,
        >,
        security_schemes: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, super::SecurityScheme>,
            ::std::string::String,
        >,
        signatures: ::std::result::Result<
            ::std::vec::Vec<super::AgentCardSignature>,
            ::std::string::String,
        >,
        skills: ::std::result::Result<::std::vec::Vec<super::AgentSkill>, ::std::string::String>,
        supports_authenticated_extended_card:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        url: ::std::result::Result<::std::string::String, ::std::string::String>,
        version: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for AgentCard {
        fn default() -> Self {
            Self {
                additional_interfaces: Ok(Default::default()),
                capabilities: Err("no value supplied for capabilities".to_string()),
                default_input_modes: Err("no value supplied for default_input_modes".to_string()),
                default_output_modes: Err("no value supplied for default_output_modes".to_string()),
                description: Err("no value supplied for description".to_string()),
                documentation_url: Ok(Default::default()),
                icon_url: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                preferred_transport: Ok(super::defaults::agent_card_preferred_transport()),
                protocol_version: Err("no value supplied for protocol_version".to_string()),
                provider: Ok(Default::default()),
                security: Ok(Default::default()),
                security_schemes: Ok(Default::default()),
                signatures: Ok(Default::default()),
                skills: Err("no value supplied for skills".to_string()),
                supports_authenticated_extended_card: Ok(Default::default()),
                url: Err("no value supplied for url".to_string()),
                version: Err("no value supplied for version".to_string()),
            }
        }
    }
    impl AgentCard {
        pub fn additional_interfaces<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::AgentInterface>>,
            T::Error: ::std::fmt::Display,
        {
            self.additional_interfaces = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for additional_interfaces: {}",
                    e
                )
            });
            self
        }
        pub fn capabilities<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AgentCapabilities>,
            T::Error: ::std::fmt::Display,
        {
            self.capabilities = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for capabilities: {}", e));
            self
        }
        pub fn default_input_modes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.default_input_modes = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for default_input_modes: {}",
                    e
                )
            });
            self
        }
        pub fn default_output_modes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.default_output_modes = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for default_output_modes: {}",
                    e
                )
            });
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn documentation_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.documentation_url = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for documentation_url: {}",
                    e
                )
            });
            self
        }
        pub fn icon_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.icon_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for icon_url: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn preferred_transport<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.preferred_transport = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for preferred_transport: {}",
                    e
                )
            });
            self
        }
        pub fn protocol_version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.protocol_version = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for protocol_version: {}",
                    e
                )
            });
            self
        }
        pub fn provider<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AgentProvider>>,
            T::Error: ::std::fmt::Display,
        {
            self.provider = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for provider: {}", e));
            self
        }
        pub fn security<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::vec::Vec<
                        ::std::collections::HashMap<
                            ::std::string::String,
                            ::std::vec::Vec<::std::string::String>,
                        >,
                    >,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.security = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for security: {}", e));
            self
        }
        pub fn security_schemes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::collections::HashMap<::std::string::String, super::SecurityScheme>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.security_schemes = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for security_schemes: {}",
                    e
                )
            });
            self
        }
        pub fn signatures<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::AgentCardSignature>>,
            T::Error: ::std::fmt::Display,
        {
            self.signatures = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for signatures: {}", e));
            self
        }
        pub fn skills<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::AgentSkill>>,
            T::Error: ::std::fmt::Display,
        {
            self.skills = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for skills: {}", e));
            self
        }
        pub fn supports_authenticated_extended_card<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.supports_authenticated_extended_card = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for supports_authenticated_extended_card: {}",
                    e
                )
            });
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {}", e));
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for version: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<AgentCard> for super::AgentCard {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AgentCard,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                additional_interfaces: value.additional_interfaces?,
                capabilities: value.capabilities?,
                default_input_modes: value.default_input_modes?,
                default_output_modes: value.default_output_modes?,
                description: value.description?,
                documentation_url: value.documentation_url?,
                icon_url: value.icon_url?,
                name: value.name?,
                preferred_transport: value.preferred_transport?,
                protocol_version: value.protocol_version?,
                provider: value.provider?,
                security: value.security?,
                security_schemes: value.security_schemes?,
                signatures: value.signatures?,
                skills: value.skills?,
                supports_authenticated_extended_card: value.supports_authenticated_extended_card?,
                url: value.url?,
                version: value.version?,
            })
        }
    }
    impl ::std::convert::From<super::AgentCard> for AgentCard {
        fn from(value: super::AgentCard) -> Self {
            Self {
                additional_interfaces: Ok(value.additional_interfaces),
                capabilities: Ok(value.capabilities),
                default_input_modes: Ok(value.default_input_modes),
                default_output_modes: Ok(value.default_output_modes),
                description: Ok(value.description),
                documentation_url: Ok(value.documentation_url),
                icon_url: Ok(value.icon_url),
                name: Ok(value.name),
                preferred_transport: Ok(value.preferred_transport),
                protocol_version: Ok(value.protocol_version),
                provider: Ok(value.provider),
                security: Ok(value.security),
                security_schemes: Ok(value.security_schemes),
                signatures: Ok(value.signatures),
                skills: Ok(value.skills),
                supports_authenticated_extended_card: Ok(value.supports_authenticated_extended_card),
                url: Ok(value.url),
                version: Ok(value.version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AgentCardSignature {
        header: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        protected: ::std::result::Result<::std::string::String, ::std::string::String>,
        signature: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for AgentCardSignature {
        fn default() -> Self {
            Self {
                header: Ok(Default::default()),
                protected: Err("no value supplied for protected".to_string()),
                signature: Err("no value supplied for signature".to_string()),
            }
        }
    }
    impl AgentCardSignature {
        pub fn header<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.header = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for header: {}", e));
            self
        }
        pub fn protected<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.protected = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for protected: {}", e));
            self
        }
        pub fn signature<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.signature = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for signature: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<AgentCardSignature> for super::AgentCardSignature {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AgentCardSignature,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                header: value.header?,
                protected: value.protected?,
                signature: value.signature?,
            })
        }
    }
    impl ::std::convert::From<super::AgentCardSignature> for AgentCardSignature {
        fn from(value: super::AgentCardSignature) -> Self {
            Self {
                header: Ok(value.header),
                protected: Ok(value.protected),
                signature: Ok(value.signature),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AgentExtension {
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        params: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        required: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        uri: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for AgentExtension {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                params: Ok(Default::default()),
                required: Ok(Default::default()),
                uri: Err("no value supplied for uri".to_string()),
            }
        }
    }
    impl AgentExtension {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {}", e));
            self
        }
        pub fn required<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.required = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for required: {}", e));
            self
        }
        pub fn uri<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.uri = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for uri: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<AgentExtension> for super::AgentExtension {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AgentExtension,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                params: value.params?,
                required: value.required?,
                uri: value.uri?,
            })
        }
    }
    impl ::std::convert::From<super::AgentExtension> for AgentExtension {
        fn from(value: super::AgentExtension) -> Self {
            Self {
                description: Ok(value.description),
                params: Ok(value.params),
                required: Ok(value.required),
                uri: Ok(value.uri),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AgentInterface {
        transport: ::std::result::Result<::std::string::String, ::std::string::String>,
        url: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for AgentInterface {
        fn default() -> Self {
            Self {
                transport: Err("no value supplied for transport".to_string()),
                url: Err("no value supplied for url".to_string()),
            }
        }
    }
    impl AgentInterface {
        pub fn transport<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.transport = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for transport: {}", e));
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<AgentInterface> for super::AgentInterface {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AgentInterface,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                transport: value.transport?,
                url: value.url?,
            })
        }
    }
    impl ::std::convert::From<super::AgentInterface> for AgentInterface {
        fn from(value: super::AgentInterface) -> Self {
            Self {
                transport: Ok(value.transport),
                url: Ok(value.url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AgentProvider {
        organization: ::std::result::Result<::std::string::String, ::std::string::String>,
        url: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for AgentProvider {
        fn default() -> Self {
            Self {
                organization: Err("no value supplied for organization".to_string()),
                url: Err("no value supplied for url".to_string()),
            }
        }
    }
    impl AgentProvider {
        pub fn organization<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.organization = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for organization: {}", e));
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<AgentProvider> for super::AgentProvider {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AgentProvider,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                organization: value.organization?,
                url: value.url?,
            })
        }
    }
    impl ::std::convert::From<super::AgentProvider> for AgentProvider {
        fn from(value: super::AgentProvider) -> Self {
            Self {
                organization: Ok(value.organization),
                url: Ok(value.url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AgentSkill {
        description: ::std::result::Result<::std::string::String, ::std::string::String>,
        examples:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        input_modes:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        output_modes:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        tags: ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
    }
    impl ::std::default::Default for AgentSkill {
        fn default() -> Self {
            Self {
                description: Err("no value supplied for description".to_string()),
                examples: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                input_modes: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                output_modes: Ok(Default::default()),
                tags: Err("no value supplied for tags".to_string()),
            }
        }
    }
    impl AgentSkill {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn examples<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.examples = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for examples: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn input_modes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.input_modes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for input_modes: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn output_modes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.output_modes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for output_modes: {}", e));
            self
        }
        pub fn tags<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.tags = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tags: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<AgentSkill> for super::AgentSkill {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AgentSkill,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                examples: value.examples?,
                id: value.id?,
                input_modes: value.input_modes?,
                name: value.name?,
                output_modes: value.output_modes?,
                tags: value.tags?,
            })
        }
    }
    impl ::std::convert::From<super::AgentSkill> for AgentSkill {
        fn from(value: super::AgentSkill) -> Self {
            Self {
                description: Ok(value.description),
                examples: Ok(value.examples),
                id: Ok(value.id),
                input_modes: Ok(value.input_modes),
                name: Ok(value.name),
                output_modes: Ok(value.output_modes),
                tags: Ok(value.tags),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ApiKeySecurityScheme {
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        in_: ::std::result::Result<super::ApiKeySecuritySchemeIn, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ApiKeySecurityScheme {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                in_: Err("no value supplied for in_".to_string()),
                name: Err("no value supplied for name".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl ApiKeySecurityScheme {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn in_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ApiKeySecuritySchemeIn>,
            T::Error: ::std::fmt::Display,
        {
            self.in_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for in_: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ApiKeySecurityScheme> for super::ApiKeySecurityScheme {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ApiKeySecurityScheme,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                in_: value.in_?,
                name: value.name?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::ApiKeySecurityScheme> for ApiKeySecurityScheme {
        fn from(value: super::ApiKeySecurityScheme) -> Self {
            Self {
                description: Ok(value.description),
                in_: Ok(value.in_),
                name: Ok(value.name),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Artifact {
        artifact_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        extensions:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        metadata: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        parts: ::std::result::Result<::std::vec::Vec<super::Part>, ::std::string::String>,
    }
    impl ::std::default::Default for Artifact {
        fn default() -> Self {
            Self {
                artifact_id: Err("no value supplied for artifact_id".to_string()),
                description: Ok(Default::default()),
                extensions: Ok(Default::default()),
                metadata: Ok(Default::default()),
                name: Ok(Default::default()),
                parts: Err("no value supplied for parts".to_string()),
            }
        }
    }
    impl Artifact {
        pub fn artifact_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.artifact_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for artifact_id: {}", e));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn extensions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.extensions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extensions: {}", e));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn parts<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Part>>,
            T::Error: ::std::fmt::Display,
        {
            self.parts = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for parts: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Artifact> for super::Artifact {
        type Error = super::error::ConversionError;
        fn try_from(value: Artifact) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                artifact_id: value.artifact_id?,
                description: value.description?,
                extensions: value.extensions?,
                metadata: value.metadata?,
                name: value.name?,
                parts: value.parts?,
            })
        }
    }
    impl ::std::convert::From<super::Artifact> for Artifact {
        fn from(value: super::Artifact) -> Self {
            Self {
                artifact_id: Ok(value.artifact_id),
                description: Ok(value.description),
                extensions: Ok(value.extensions),
                metadata: Ok(value.metadata),
                name: Ok(value.name),
                parts: Ok(value.parts),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuthorizationCodeOAuthFlow {
        authorization_url: ::std::result::Result<::std::string::String, ::std::string::String>,
        refresh_url: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        scopes: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            ::std::string::String,
        >,
        token_url: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for AuthorizationCodeOAuthFlow {
        fn default() -> Self {
            Self {
                authorization_url: Err("no value supplied for authorization_url".to_string()),
                refresh_url: Ok(Default::default()),
                scopes: Err("no value supplied for scopes".to_string()),
                token_url: Err("no value supplied for token_url".to_string()),
            }
        }
    }
    impl AuthorizationCodeOAuthFlow {
        pub fn authorization_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.authorization_url = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for authorization_url: {}",
                    e
                )
            });
            self
        }
        pub fn refresh_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.refresh_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for refresh_url: {}", e));
            self
        }
        pub fn scopes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::collections::HashMap<::std::string::String, ::std::string::String>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.scopes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for scopes: {}", e));
            self
        }
        pub fn token_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.token_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for token_url: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<AuthorizationCodeOAuthFlow> for super::AuthorizationCodeOAuthFlow {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuthorizationCodeOAuthFlow,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                authorization_url: value.authorization_url?,
                refresh_url: value.refresh_url?,
                scopes: value.scopes?,
                token_url: value.token_url?,
            })
        }
    }
    impl ::std::convert::From<super::AuthorizationCodeOAuthFlow> for AuthorizationCodeOAuthFlow {
        fn from(value: super::AuthorizationCodeOAuthFlow) -> Self {
            Self {
                authorization_url: Ok(value.authorization_url),
                refresh_url: Ok(value.refresh_url),
                scopes: Ok(value.scopes),
                token_url: Ok(value.token_url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CancelTaskRequest {
        id: ::std::result::Result<super::CancelTaskRequestId, ::std::string::String>,
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<super::TaskIdParams, ::std::string::String>,
    }
    impl ::std::default::Default for CancelTaskRequest {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                method: Err("no value supplied for method".to_string()),
                params: Err("no value supplied for params".to_string()),
            }
        }
    }
    impl CancelTaskRequest {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CancelTaskRequestId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {}", e));
            self
        }
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {}", e));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TaskIdParams>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<CancelTaskRequest> for super::CancelTaskRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CancelTaskRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                jsonrpc: value.jsonrpc?,
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::CancelTaskRequest> for CancelTaskRequest {
        fn from(value: super::CancelTaskRequest) -> Self {
            Self {
                id: Ok(value.id),
                jsonrpc: Ok(value.jsonrpc),
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CancelTaskSuccessResponse {
        id: ::std::result::Result<super::CancelTaskSuccessResponseId, ::std::string::String>,
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        result: ::std::result::Result<super::Task, ::std::string::String>,
    }
    impl ::std::default::Default for CancelTaskSuccessResponse {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                result: Err("no value supplied for result".to_string()),
            }
        }
    }
    impl CancelTaskSuccessResponse {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CancelTaskSuccessResponseId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {}", e));
            self
        }
        pub fn result<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Task>,
            T::Error: ::std::fmt::Display,
        {
            self.result = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for result: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<CancelTaskSuccessResponse> for super::CancelTaskSuccessResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CancelTaskSuccessResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                jsonrpc: value.jsonrpc?,
                result: value.result?,
            })
        }
    }
    impl ::std::convert::From<super::CancelTaskSuccessResponse> for CancelTaskSuccessResponse {
        fn from(value: super::CancelTaskSuccessResponse) -> Self {
            Self {
                id: Ok(value.id),
                jsonrpc: Ok(value.jsonrpc),
                result: Ok(value.result),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ClientCredentialsOAuthFlow {
        refresh_url: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        scopes: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            ::std::string::String,
        >,
        token_url: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ClientCredentialsOAuthFlow {
        fn default() -> Self {
            Self {
                refresh_url: Ok(Default::default()),
                scopes: Err("no value supplied for scopes".to_string()),
                token_url: Err("no value supplied for token_url".to_string()),
            }
        }
    }
    impl ClientCredentialsOAuthFlow {
        pub fn refresh_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.refresh_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for refresh_url: {}", e));
            self
        }
        pub fn scopes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::collections::HashMap<::std::string::String, ::std::string::String>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.scopes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for scopes: {}", e));
            self
        }
        pub fn token_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.token_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for token_url: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ClientCredentialsOAuthFlow> for super::ClientCredentialsOAuthFlow {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ClientCredentialsOAuthFlow,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                refresh_url: value.refresh_url?,
                scopes: value.scopes?,
                token_url: value.token_url?,
            })
        }
    }
    impl ::std::convert::From<super::ClientCredentialsOAuthFlow> for ClientCredentialsOAuthFlow {
        fn from(value: super::ClientCredentialsOAuthFlow) -> Self {
            Self {
                refresh_url: Ok(value.refresh_url),
                scopes: Ok(value.scopes),
                token_url: Ok(value.token_url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ContentTypeNotSupportedError {
        code: ::std::result::Result<i64, ::std::string::String>,
        data: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
        message: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ContentTypeNotSupportedError {
        fn default() -> Self {
            Self {
                code: Err("no value supplied for code".to_string()),
                data: Ok(Default::default()),
                message: Err("no value supplied for message".to_string()),
            }
        }
    }
    impl ContentTypeNotSupportedError {
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code: {}", e));
            self
        }
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {}", e));
            self
        }
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ContentTypeNotSupportedError> for super::ContentTypeNotSupportedError {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ContentTypeNotSupportedError,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                data: value.data?,
                message: value.message?,
            })
        }
    }
    impl ::std::convert::From<super::ContentTypeNotSupportedError> for ContentTypeNotSupportedError {
        fn from(value: super::ContentTypeNotSupportedError) -> Self {
            Self {
                code: Ok(value.code),
                data: Ok(value.data),
                message: Ok(value.message),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DataPart {
        data: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        kind: ::std::result::Result<::std::string::String, ::std::string::String>,
        metadata: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for DataPart {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
                kind: Err("no value supplied for kind".to_string()),
                metadata: Ok(Default::default()),
            }
        }
    }
    impl DataPart {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {}", e));
            self
        }
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<DataPart> for super::DataPart {
        type Error = super::error::ConversionError;
        fn try_from(value: DataPart) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                kind: value.kind?,
                metadata: value.metadata?,
            })
        }
    }
    impl ::std::convert::From<super::DataPart> for DataPart {
        fn from(value: super::DataPart) -> Self {
            Self {
                data: Ok(value.data),
                kind: Ok(value.kind),
                metadata: Ok(value.metadata),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeleteTaskPushNotificationConfigParams {
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        metadata: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        push_notification_config_id:
            ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for DeleteTaskPushNotificationConfigParams {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                metadata: Ok(Default::default()),
                push_notification_config_id: Err(
                    "no value supplied for push_notification_config_id".to_string(),
                ),
            }
        }
    }
    impl DeleteTaskPushNotificationConfigParams {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {}", e));
            self
        }
        pub fn push_notification_config_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.push_notification_config_id = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for push_notification_config_id: {}",
                    e
                )
            });
            self
        }
    }
    impl ::std::convert::TryFrom<DeleteTaskPushNotificationConfigParams>
        for super::DeleteTaskPushNotificationConfigParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeleteTaskPushNotificationConfigParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                metadata: value.metadata?,
                push_notification_config_id: value.push_notification_config_id?,
            })
        }
    }
    impl ::std::convert::From<super::DeleteTaskPushNotificationConfigParams>
        for DeleteTaskPushNotificationConfigParams
    {
        fn from(value: super::DeleteTaskPushNotificationConfigParams) -> Self {
            Self {
                id: Ok(value.id),
                metadata: Ok(value.metadata),
                push_notification_config_id: Ok(value.push_notification_config_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeleteTaskPushNotificationConfigRequest {
        id: ::std::result::Result<
            super::DeleteTaskPushNotificationConfigRequestId,
            ::std::string::String,
        >,
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<
            super::DeleteTaskPushNotificationConfigParams,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for DeleteTaskPushNotificationConfigRequest {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                method: Err("no value supplied for method".to_string()),
                params: Err("no value supplied for params".to_string()),
            }
        }
    }
    impl DeleteTaskPushNotificationConfigRequest {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeleteTaskPushNotificationConfigRequestId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {}", e));
            self
        }
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {}", e));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeleteTaskPushNotificationConfigParams>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<DeleteTaskPushNotificationConfigRequest>
        for super::DeleteTaskPushNotificationConfigRequest
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeleteTaskPushNotificationConfigRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                jsonrpc: value.jsonrpc?,
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::DeleteTaskPushNotificationConfigRequest>
        for DeleteTaskPushNotificationConfigRequest
    {
        fn from(value: super::DeleteTaskPushNotificationConfigRequest) -> Self {
            Self {
                id: Ok(value.id),
                jsonrpc: Ok(value.jsonrpc),
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeleteTaskPushNotificationConfigSuccessResponse {
        id: ::std::result::Result<
            super::DeleteTaskPushNotificationConfigSuccessResponseId,
            ::std::string::String,
        >,
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        result: ::std::result::Result<(), ::std::string::String>,
    }
    impl ::std::default::Default for DeleteTaskPushNotificationConfigSuccessResponse {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                result: Err("no value supplied for result".to_string()),
            }
        }
    }
    impl DeleteTaskPushNotificationConfigSuccessResponse {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DeleteTaskPushNotificationConfigSuccessResponseId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {}", e));
            self
        }
        pub fn result<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<()>,
            T::Error: ::std::fmt::Display,
        {
            self.result = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for result: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<DeleteTaskPushNotificationConfigSuccessResponse>
        for super::DeleteTaskPushNotificationConfigSuccessResponse
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeleteTaskPushNotificationConfigSuccessResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                jsonrpc: value.jsonrpc?,
                result: value.result?,
            })
        }
    }
    impl ::std::convert::From<super::DeleteTaskPushNotificationConfigSuccessResponse>
        for DeleteTaskPushNotificationConfigSuccessResponse
    {
        fn from(value: super::DeleteTaskPushNotificationConfigSuccessResponse) -> Self {
            Self {
                id: Ok(value.id),
                jsonrpc: Ok(value.jsonrpc),
                result: Ok(value.result),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FileBase {
        mime_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FileBase {
        fn default() -> Self {
            Self {
                mime_type: Ok(Default::default()),
                name: Ok(Default::default()),
            }
        }
    }
    impl FileBase {
        pub fn mime_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.mime_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mime_type: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<FileBase> for super::FileBase {
        type Error = super::error::ConversionError;
        fn try_from(value: FileBase) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                mime_type: value.mime_type?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::FileBase> for FileBase {
        fn from(value: super::FileBase) -> Self {
            Self {
                mime_type: Ok(value.mime_type),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FilePart {
        file: ::std::result::Result<super::FilePartFile, ::std::string::String>,
        kind: ::std::result::Result<::std::string::String, ::std::string::String>,
        metadata: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FilePart {
        fn default() -> Self {
            Self {
                file: Err("no value supplied for file".to_string()),
                kind: Err("no value supplied for kind".to_string()),
                metadata: Ok(Default::default()),
            }
        }
    }
    impl FilePart {
        pub fn file<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FilePartFile>,
            T::Error: ::std::fmt::Display,
        {
            self.file = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file: {}", e));
            self
        }
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<FilePart> for super::FilePart {
        type Error = super::error::ConversionError;
        fn try_from(value: FilePart) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                file: value.file?,
                kind: value.kind?,
                metadata: value.metadata?,
            })
        }
    }
    impl ::std::convert::From<super::FilePart> for FilePart {
        fn from(value: super::FilePart) -> Self {
            Self {
                file: Ok(value.file),
                kind: Ok(value.kind),
                metadata: Ok(value.metadata),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FileWithBytes {
        bytes: ::std::result::Result<::std::string::String, ::std::string::String>,
        mime_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for FileWithBytes {
        fn default() -> Self {
            Self {
                bytes: Err("no value supplied for bytes".to_string()),
                mime_type: Ok(Default::default()),
                name: Ok(Default::default()),
            }
        }
    }
    impl FileWithBytes {
        pub fn bytes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.bytes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bytes: {}", e));
            self
        }
        pub fn mime_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.mime_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mime_type: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<FileWithBytes> for super::FileWithBytes {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FileWithBytes,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                bytes: value.bytes?,
                mime_type: value.mime_type?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::FileWithBytes> for FileWithBytes {
        fn from(value: super::FileWithBytes) -> Self {
            Self {
                bytes: Ok(value.bytes),
                mime_type: Ok(value.mime_type),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FileWithUri {
        mime_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        uri: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for FileWithUri {
        fn default() -> Self {
            Self {
                mime_type: Ok(Default::default()),
                name: Ok(Default::default()),
                uri: Err("no value supplied for uri".to_string()),
            }
        }
    }
    impl FileWithUri {
        pub fn mime_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.mime_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mime_type: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn uri<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.uri = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for uri: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<FileWithUri> for super::FileWithUri {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FileWithUri,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                mime_type: value.mime_type?,
                name: value.name?,
                uri: value.uri?,
            })
        }
    }
    impl ::std::convert::From<super::FileWithUri> for FileWithUri {
        fn from(value: super::FileWithUri) -> Self {
            Self {
                mime_type: Ok(value.mime_type),
                name: Ok(value.name),
                uri: Ok(value.uri),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GetTaskPushNotificationConfigParams {
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        metadata: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        push_notification_config_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for GetTaskPushNotificationConfigParams {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                metadata: Ok(Default::default()),
                push_notification_config_id: Ok(Default::default()),
            }
        }
    }
    impl GetTaskPushNotificationConfigParams {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {}", e));
            self
        }
        pub fn push_notification_config_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.push_notification_config_id = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for push_notification_config_id: {}",
                    e
                )
            });
            self
        }
    }
    impl ::std::convert::TryFrom<GetTaskPushNotificationConfigParams>
        for super::GetTaskPushNotificationConfigParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GetTaskPushNotificationConfigParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                metadata: value.metadata?,
                push_notification_config_id: value.push_notification_config_id?,
            })
        }
    }
    impl ::std::convert::From<super::GetTaskPushNotificationConfigParams>
        for GetTaskPushNotificationConfigParams
    {
        fn from(value: super::GetTaskPushNotificationConfigParams) -> Self {
            Self {
                id: Ok(value.id),
                metadata: Ok(value.metadata),
                push_notification_config_id: Ok(value.push_notification_config_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GetTaskPushNotificationConfigRequest {
        id: ::std::result::Result<
            super::GetTaskPushNotificationConfigRequestId,
            ::std::string::String,
        >,
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<
            super::GetTaskPushNotificationConfigRequestParams,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for GetTaskPushNotificationConfigRequest {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                method: Err("no value supplied for method".to_string()),
                params: Err("no value supplied for params".to_string()),
            }
        }
    }
    impl GetTaskPushNotificationConfigRequest {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GetTaskPushNotificationConfigRequestId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {}", e));
            self
        }
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {}", e));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GetTaskPushNotificationConfigRequestParams>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<GetTaskPushNotificationConfigRequest>
        for super::GetTaskPushNotificationConfigRequest
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GetTaskPushNotificationConfigRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                jsonrpc: value.jsonrpc?,
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::GetTaskPushNotificationConfigRequest>
        for GetTaskPushNotificationConfigRequest
    {
        fn from(value: super::GetTaskPushNotificationConfigRequest) -> Self {
            Self {
                id: Ok(value.id),
                jsonrpc: Ok(value.jsonrpc),
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GetTaskPushNotificationConfigRequestParams {
        subtype_0: ::std::result::Result<
            ::std::option::Option<super::TaskIdParams>,
            ::std::string::String,
        >,
        subtype_1: ::std::result::Result<
            ::std::option::Option<super::GetTaskPushNotificationConfigParams>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for GetTaskPushNotificationConfigRequestParams {
        fn default() -> Self {
            Self {
                subtype_0: Ok(Default::default()),
                subtype_1: Ok(Default::default()),
            }
        }
    }
    impl GetTaskPushNotificationConfigRequestParams {
        pub fn subtype_0<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TaskIdParams>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_0 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_0: {}", e));
            self
        }
        pub fn subtype_1<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::option::Option<super::GetTaskPushNotificationConfigParams>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_1: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<GetTaskPushNotificationConfigRequestParams>
        for super::GetTaskPushNotificationConfigRequestParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GetTaskPushNotificationConfigRequestParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                subtype_0: value.subtype_0?,
                subtype_1: value.subtype_1?,
            })
        }
    }
    impl ::std::convert::From<super::GetTaskPushNotificationConfigRequestParams>
        for GetTaskPushNotificationConfigRequestParams
    {
        fn from(value: super::GetTaskPushNotificationConfigRequestParams) -> Self {
            Self {
                subtype_0: Ok(value.subtype_0),
                subtype_1: Ok(value.subtype_1),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GetTaskPushNotificationConfigSuccessResponse {
        id: ::std::result::Result<
            super::GetTaskPushNotificationConfigSuccessResponseId,
            ::std::string::String,
        >,
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        result: ::std::result::Result<super::TaskPushNotificationConfig, ::std::string::String>,
    }
    impl ::std::default::Default for GetTaskPushNotificationConfigSuccessResponse {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                result: Err("no value supplied for result".to_string()),
            }
        }
    }
    impl GetTaskPushNotificationConfigSuccessResponse {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GetTaskPushNotificationConfigSuccessResponseId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {}", e));
            self
        }
        pub fn result<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TaskPushNotificationConfig>,
            T::Error: ::std::fmt::Display,
        {
            self.result = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for result: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<GetTaskPushNotificationConfigSuccessResponse>
        for super::GetTaskPushNotificationConfigSuccessResponse
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GetTaskPushNotificationConfigSuccessResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                jsonrpc: value.jsonrpc?,
                result: value.result?,
            })
        }
    }
    impl ::std::convert::From<super::GetTaskPushNotificationConfigSuccessResponse>
        for GetTaskPushNotificationConfigSuccessResponse
    {
        fn from(value: super::GetTaskPushNotificationConfigSuccessResponse) -> Self {
            Self {
                id: Ok(value.id),
                jsonrpc: Ok(value.jsonrpc),
                result: Ok(value.result),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GetTaskRequest {
        id: ::std::result::Result<super::GetTaskRequestId, ::std::string::String>,
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<super::TaskQueryParams, ::std::string::String>,
    }
    impl ::std::default::Default for GetTaskRequest {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                method: Err("no value supplied for method".to_string()),
                params: Err("no value supplied for params".to_string()),
            }
        }
    }
    impl GetTaskRequest {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GetTaskRequestId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {}", e));
            self
        }
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {}", e));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TaskQueryParams>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<GetTaskRequest> for super::GetTaskRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GetTaskRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                jsonrpc: value.jsonrpc?,
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::GetTaskRequest> for GetTaskRequest {
        fn from(value: super::GetTaskRequest) -> Self {
            Self {
                id: Ok(value.id),
                jsonrpc: Ok(value.jsonrpc),
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GetTaskSuccessResponse {
        id: ::std::result::Result<super::GetTaskSuccessResponseId, ::std::string::String>,
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        result: ::std::result::Result<super::Task, ::std::string::String>,
    }
    impl ::std::default::Default for GetTaskSuccessResponse {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                result: Err("no value supplied for result".to_string()),
            }
        }
    }
    impl GetTaskSuccessResponse {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::GetTaskSuccessResponseId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {}", e));
            self
        }
        pub fn result<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Task>,
            T::Error: ::std::fmt::Display,
        {
            self.result = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for result: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<GetTaskSuccessResponse> for super::GetTaskSuccessResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GetTaskSuccessResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                jsonrpc: value.jsonrpc?,
                result: value.result?,
            })
        }
    }
    impl ::std::convert::From<super::GetTaskSuccessResponse> for GetTaskSuccessResponse {
        fn from(value: super::GetTaskSuccessResponse) -> Self {
            Self {
                id: Ok(value.id),
                jsonrpc: Ok(value.jsonrpc),
                result: Ok(value.result),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct HttpAuthSecurityScheme {
        bearer_format: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        scheme: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for HttpAuthSecurityScheme {
        fn default() -> Self {
            Self {
                bearer_format: Ok(Default::default()),
                description: Ok(Default::default()),
                scheme: Err("no value supplied for scheme".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl HttpAuthSecurityScheme {
        pub fn bearer_format<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.bearer_format = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bearer_format: {}", e));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn scheme<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.scheme = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for scheme: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<HttpAuthSecurityScheme> for super::HttpAuthSecurityScheme {
        type Error = super::error::ConversionError;
        fn try_from(
            value: HttpAuthSecurityScheme,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                bearer_format: value.bearer_format?,
                description: value.description?,
                scheme: value.scheme?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::HttpAuthSecurityScheme> for HttpAuthSecurityScheme {
        fn from(value: super::HttpAuthSecurityScheme) -> Self {
            Self {
                bearer_format: Ok(value.bearer_format),
                description: Ok(value.description),
                scheme: Ok(value.scheme),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ImplicitOAuthFlow {
        authorization_url: ::std::result::Result<::std::string::String, ::std::string::String>,
        refresh_url: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        scopes: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ImplicitOAuthFlow {
        fn default() -> Self {
            Self {
                authorization_url: Err("no value supplied for authorization_url".to_string()),
                refresh_url: Ok(Default::default()),
                scopes: Err("no value supplied for scopes".to_string()),
            }
        }
    }
    impl ImplicitOAuthFlow {
        pub fn authorization_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.authorization_url = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for authorization_url: {}",
                    e
                )
            });
            self
        }
        pub fn refresh_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.refresh_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for refresh_url: {}", e));
            self
        }
        pub fn scopes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::collections::HashMap<::std::string::String, ::std::string::String>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.scopes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for scopes: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ImplicitOAuthFlow> for super::ImplicitOAuthFlow {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ImplicitOAuthFlow,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                authorization_url: value.authorization_url?,
                refresh_url: value.refresh_url?,
                scopes: value.scopes?,
            })
        }
    }
    impl ::std::convert::From<super::ImplicitOAuthFlow> for ImplicitOAuthFlow {
        fn from(value: super::ImplicitOAuthFlow) -> Self {
            Self {
                authorization_url: Ok(value.authorization_url),
                refresh_url: Ok(value.refresh_url),
                scopes: Ok(value.scopes),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct InternalError {
        code: ::std::result::Result<i64, ::std::string::String>,
        data: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
        message: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for InternalError {
        fn default() -> Self {
            Self {
                code: Err("no value supplied for code".to_string()),
                data: Ok(Default::default()),
                message: Err("no value supplied for message".to_string()),
            }
        }
    }
    impl InternalError {
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code: {}", e));
            self
        }
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {}", e));
            self
        }
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<InternalError> for super::InternalError {
        type Error = super::error::ConversionError;
        fn try_from(
            value: InternalError,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                data: value.data?,
                message: value.message?,
            })
        }
    }
    impl ::std::convert::From<super::InternalError> for InternalError {
        fn from(value: super::InternalError) -> Self {
            Self {
                code: Ok(value.code),
                data: Ok(value.data),
                message: Ok(value.message),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct InvalidAgentResponseError {
        code: ::std::result::Result<i64, ::std::string::String>,
        data: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
        message: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for InvalidAgentResponseError {
        fn default() -> Self {
            Self {
                code: Err("no value supplied for code".to_string()),
                data: Ok(Default::default()),
                message: Err("no value supplied for message".to_string()),
            }
        }
    }
    impl InvalidAgentResponseError {
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code: {}", e));
            self
        }
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {}", e));
            self
        }
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<InvalidAgentResponseError> for super::InvalidAgentResponseError {
        type Error = super::error::ConversionError;
        fn try_from(
            value: InvalidAgentResponseError,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                data: value.data?,
                message: value.message?,
            })
        }
    }
    impl ::std::convert::From<super::InvalidAgentResponseError> for InvalidAgentResponseError {
        fn from(value: super::InvalidAgentResponseError) -> Self {
            Self {
                code: Ok(value.code),
                data: Ok(value.data),
                message: Ok(value.message),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct InvalidParamsError {
        code: ::std::result::Result<i64, ::std::string::String>,
        data: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
        message: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for InvalidParamsError {
        fn default() -> Self {
            Self {
                code: Err("no value supplied for code".to_string()),
                data: Ok(Default::default()),
                message: Err("no value supplied for message".to_string()),
            }
        }
    }
    impl InvalidParamsError {
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code: {}", e));
            self
        }
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {}", e));
            self
        }
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<InvalidParamsError> for super::InvalidParamsError {
        type Error = super::error::ConversionError;
        fn try_from(
            value: InvalidParamsError,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                data: value.data?,
                message: value.message?,
            })
        }
    }
    impl ::std::convert::From<super::InvalidParamsError> for InvalidParamsError {
        fn from(value: super::InvalidParamsError) -> Self {
            Self {
                code: Ok(value.code),
                data: Ok(value.data),
                message: Ok(value.message),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct InvalidRequestError {
        code: ::std::result::Result<i64, ::std::string::String>,
        data: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
        message: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for InvalidRequestError {
        fn default() -> Self {
            Self {
                code: Err("no value supplied for code".to_string()),
                data: Ok(Default::default()),
                message: Err("no value supplied for message".to_string()),
            }
        }
    }
    impl InvalidRequestError {
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code: {}", e));
            self
        }
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {}", e));
            self
        }
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<InvalidRequestError> for super::InvalidRequestError {
        type Error = super::error::ConversionError;
        fn try_from(
            value: InvalidRequestError,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                data: value.data?,
                message: value.message?,
            })
        }
    }
    impl ::std::convert::From<super::InvalidRequestError> for InvalidRequestError {
        fn from(value: super::InvalidRequestError) -> Self {
            Self {
                code: Ok(value.code),
                data: Ok(value.data),
                message: Ok(value.message),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct JsonParseError {
        code: ::std::result::Result<i64, ::std::string::String>,
        data: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
        message: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for JsonParseError {
        fn default() -> Self {
            Self {
                code: Err("no value supplied for code".to_string()),
                data: Ok(Default::default()),
                message: Err("no value supplied for message".to_string()),
            }
        }
    }
    impl JsonParseError {
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code: {}", e));
            self
        }
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {}", e));
            self
        }
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<JsonParseError> for super::JsonParseError {
        type Error = super::error::ConversionError;
        fn try_from(
            value: JsonParseError,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                data: value.data?,
                message: value.message?,
            })
        }
    }
    impl ::std::convert::From<super::JsonParseError> for JsonParseError {
        fn from(value: super::JsonParseError) -> Self {
            Self {
                code: Ok(value.code),
                data: Ok(value.data),
                message: Ok(value.message),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct JsonrpcError {
        code: ::std::result::Result<i64, ::std::string::String>,
        data: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
        message: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for JsonrpcError {
        fn default() -> Self {
            Self {
                code: Err("no value supplied for code".to_string()),
                data: Ok(Default::default()),
                message: Err("no value supplied for message".to_string()),
            }
        }
    }
    impl JsonrpcError {
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code: {}", e));
            self
        }
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {}", e));
            self
        }
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<JsonrpcError> for super::JsonrpcError {
        type Error = super::error::ConversionError;
        fn try_from(
            value: JsonrpcError,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                data: value.data?,
                message: value.message?,
            })
        }
    }
    impl ::std::convert::From<super::JsonrpcError> for JsonrpcError {
        fn from(value: super::JsonrpcError) -> Self {
            Self {
                code: Ok(value.code),
                data: Ok(value.data),
                message: Ok(value.message),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct JsonrpcErrorResponse {
        error: ::std::result::Result<super::JsonrpcErrorResponseError, ::std::string::String>,
        id: ::std::result::Result<super::JsonrpcErrorResponseId, ::std::string::String>,
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for JsonrpcErrorResponse {
        fn default() -> Self {
            Self {
                error: Err("no value supplied for error".to_string()),
                id: Err("no value supplied for id".to_string()),
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
            }
        }
    }
    impl JsonrpcErrorResponse {
        pub fn error<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::JsonrpcErrorResponseError>,
            T::Error: ::std::fmt::Display,
        {
            self.error = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for error: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::JsonrpcErrorResponseId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<JsonrpcErrorResponse> for super::JsonrpcErrorResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: JsonrpcErrorResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                error: value.error?,
                id: value.id?,
                jsonrpc: value.jsonrpc?,
            })
        }
    }
    impl ::std::convert::From<super::JsonrpcErrorResponse> for JsonrpcErrorResponse {
        fn from(value: super::JsonrpcErrorResponse) -> Self {
            Self {
                error: Ok(value.error),
                id: Ok(value.id),
                jsonrpc: Ok(value.jsonrpc),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct JsonrpcErrorResponseError {
        subtype_0: ::std::result::Result<
            ::std::option::Option<super::JsonrpcError>,
            ::std::string::String,
        >,
        subtype_1: ::std::result::Result<
            ::std::option::Option<super::JsonParseError>,
            ::std::string::String,
        >,
        subtype_2: ::std::result::Result<
            ::std::option::Option<super::InvalidRequestError>,
            ::std::string::String,
        >,
        subtype_3: ::std::result::Result<
            ::std::option::Option<super::MethodNotFoundError>,
            ::std::string::String,
        >,
        subtype_4: ::std::result::Result<
            ::std::option::Option<super::InvalidParamsError>,
            ::std::string::String,
        >,
        subtype_5: ::std::result::Result<
            ::std::option::Option<super::InternalError>,
            ::std::string::String,
        >,
        subtype_6: ::std::result::Result<
            ::std::option::Option<super::TaskNotFoundError>,
            ::std::string::String,
        >,
        subtype_7: ::std::result::Result<
            ::std::option::Option<super::TaskNotCancelableError>,
            ::std::string::String,
        >,
        subtype_8: ::std::result::Result<
            ::std::option::Option<super::PushNotificationNotSupportedError>,
            ::std::string::String,
        >,
        subtype_9: ::std::result::Result<
            ::std::option::Option<super::UnsupportedOperationError>,
            ::std::string::String,
        >,
        subtype_10: ::std::result::Result<
            ::std::option::Option<super::ContentTypeNotSupportedError>,
            ::std::string::String,
        >,
        subtype_11: ::std::result::Result<
            ::std::option::Option<super::InvalidAgentResponseError>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for JsonrpcErrorResponseError {
        fn default() -> Self {
            Self {
                subtype_0: Ok(Default::default()),
                subtype_1: Ok(Default::default()),
                subtype_2: Ok(Default::default()),
                subtype_3: Ok(Default::default()),
                subtype_4: Ok(Default::default()),
                subtype_5: Ok(Default::default()),
                subtype_6: Ok(Default::default()),
                subtype_7: Ok(Default::default()),
                subtype_8: Ok(Default::default()),
                subtype_9: Ok(Default::default()),
                subtype_10: Ok(Default::default()),
                subtype_11: Ok(Default::default()),
            }
        }
    }
    impl JsonrpcErrorResponseError {
        pub fn subtype_0<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::JsonrpcError>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_0 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_0: {}", e));
            self
        }
        pub fn subtype_1<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::JsonParseError>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_1: {}", e));
            self
        }
        pub fn subtype_2<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::InvalidRequestError>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_2 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_2: {}", e));
            self
        }
        pub fn subtype_3<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::MethodNotFoundError>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_3 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_3: {}", e));
            self
        }
        pub fn subtype_4<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::InvalidParamsError>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_4 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_4: {}", e));
            self
        }
        pub fn subtype_5<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::InternalError>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_5 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_5: {}", e));
            self
        }
        pub fn subtype_6<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TaskNotFoundError>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_6 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_6: {}", e));
            self
        }
        pub fn subtype_7<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TaskNotCancelableError>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_7 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_7: {}", e));
            self
        }
        pub fn subtype_8<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::option::Option<super::PushNotificationNotSupportedError>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_8 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_8: {}", e));
            self
        }
        pub fn subtype_9<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::UnsupportedOperationError>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_9 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_9: {}", e));
            self
        }
        pub fn subtype_10<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ContentTypeNotSupportedError>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_10 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_10: {}", e));
            self
        }
        pub fn subtype_11<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::InvalidAgentResponseError>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_11 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_11: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<JsonrpcErrorResponseError> for super::JsonrpcErrorResponseError {
        type Error = super::error::ConversionError;
        fn try_from(
            value: JsonrpcErrorResponseError,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                subtype_0: value.subtype_0?,
                subtype_1: value.subtype_1?,
                subtype_2: value.subtype_2?,
                subtype_3: value.subtype_3?,
                subtype_4: value.subtype_4?,
                subtype_5: value.subtype_5?,
                subtype_6: value.subtype_6?,
                subtype_7: value.subtype_7?,
                subtype_8: value.subtype_8?,
                subtype_9: value.subtype_9?,
                subtype_10: value.subtype_10?,
                subtype_11: value.subtype_11?,
            })
        }
    }
    impl ::std::convert::From<super::JsonrpcErrorResponseError> for JsonrpcErrorResponseError {
        fn from(value: super::JsonrpcErrorResponseError) -> Self {
            Self {
                subtype_0: Ok(value.subtype_0),
                subtype_1: Ok(value.subtype_1),
                subtype_2: Ok(value.subtype_2),
                subtype_3: Ok(value.subtype_3),
                subtype_4: Ok(value.subtype_4),
                subtype_5: Ok(value.subtype_5),
                subtype_6: Ok(value.subtype_6),
                subtype_7: Ok(value.subtype_7),
                subtype_8: Ok(value.subtype_8),
                subtype_9: Ok(value.subtype_9),
                subtype_10: Ok(value.subtype_10),
                subtype_11: Ok(value.subtype_11),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct JsonrpcMessage {
        id: ::std::result::Result<
            ::std::option::Option<super::JsonrpcMessageId>,
            ::std::string::String,
        >,
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for JsonrpcMessage {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
            }
        }
    }
    impl JsonrpcMessage {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::JsonrpcMessageId>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<JsonrpcMessage> for super::JsonrpcMessage {
        type Error = super::error::ConversionError;
        fn try_from(
            value: JsonrpcMessage,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                jsonrpc: value.jsonrpc?,
            })
        }
    }
    impl ::std::convert::From<super::JsonrpcMessage> for JsonrpcMessage {
        fn from(value: super::JsonrpcMessage) -> Self {
            Self {
                id: Ok(value.id),
                jsonrpc: Ok(value.jsonrpc),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct JsonrpcRequest {
        id: ::std::result::Result<
            ::std::option::Option<super::JsonrpcRequestId>,
            ::std::string::String,
        >,
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for JsonrpcRequest {
        fn default() -> Self {
            Self {
                id: Ok(Default::default()),
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                method: Err("no value supplied for method".to_string()),
                params: Ok(Default::default()),
            }
        }
    }
    impl JsonrpcRequest {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::JsonrpcRequestId>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {}", e));
            self
        }
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {}", e));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<JsonrpcRequest> for super::JsonrpcRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: JsonrpcRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                jsonrpc: value.jsonrpc?,
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::JsonrpcRequest> for JsonrpcRequest {
        fn from(value: super::JsonrpcRequest) -> Self {
            Self {
                id: Ok(value.id),
                jsonrpc: Ok(value.jsonrpc),
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct JsonrpcResponse {
        subtype_0: ::std::result::Result<
            ::std::option::Option<super::JsonrpcErrorResponse>,
            ::std::string::String,
        >,
        subtype_1: ::std::result::Result<
            ::std::option::Option<super::SendMessageSuccessResponse>,
            ::std::string::String,
        >,
        subtype_2: ::std::result::Result<
            ::std::option::Option<super::SendStreamingMessageSuccessResponse>,
            ::std::string::String,
        >,
        subtype_3: ::std::result::Result<
            ::std::option::Option<super::GetTaskSuccessResponse>,
            ::std::string::String,
        >,
        subtype_4: ::std::result::Result<
            ::std::option::Option<super::CancelTaskSuccessResponse>,
            ::std::string::String,
        >,
        subtype_5: ::std::result::Result<
            ::std::option::Option<super::SetTaskPushNotificationConfigSuccessResponse>,
            ::std::string::String,
        >,
        subtype_6: ::std::result::Result<
            ::std::option::Option<super::GetTaskPushNotificationConfigSuccessResponse>,
            ::std::string::String,
        >,
        subtype_7: ::std::result::Result<
            ::std::option::Option<super::ListTaskPushNotificationConfigSuccessResponse>,
            ::std::string::String,
        >,
        subtype_8: ::std::result::Result<
            ::std::option::Option<super::DeleteTaskPushNotificationConfigSuccessResponse>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for JsonrpcResponse {
        fn default() -> Self {
            Self {
                subtype_0: Ok(Default::default()),
                subtype_1: Ok(Default::default()),
                subtype_2: Ok(Default::default()),
                subtype_3: Ok(Default::default()),
                subtype_4: Ok(Default::default()),
                subtype_5: Ok(Default::default()),
                subtype_6: Ok(Default::default()),
                subtype_7: Ok(Default::default()),
                subtype_8: Ok(Default::default()),
            }
        }
    }
    impl JsonrpcResponse {
        pub fn subtype_0<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::JsonrpcErrorResponse>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_0 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_0: {}", e));
            self
        }
        pub fn subtype_1<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SendMessageSuccessResponse>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_1: {}", e));
            self
        }
        pub fn subtype_2<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::option::Option<super::SendStreamingMessageSuccessResponse>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_2 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_2: {}", e));
            self
        }
        pub fn subtype_3<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::GetTaskSuccessResponse>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_3 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_3: {}", e));
            self
        }
        pub fn subtype_4<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CancelTaskSuccessResponse>>,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_4 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_4: {}", e));
            self
        }
        pub fn subtype_5<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::option::Option<super::SetTaskPushNotificationConfigSuccessResponse>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_5 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_5: {}", e));
            self
        }
        pub fn subtype_6<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::option::Option<super::GetTaskPushNotificationConfigSuccessResponse>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_6 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_6: {}", e));
            self
        }
        pub fn subtype_7<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::option::Option<super::ListTaskPushNotificationConfigSuccessResponse>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_7 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_7: {}", e));
            self
        }
        pub fn subtype_8<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::option::Option<super::DeleteTaskPushNotificationConfigSuccessResponse>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.subtype_8 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_8: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<JsonrpcResponse> for super::JsonrpcResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: JsonrpcResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                subtype_0: value.subtype_0?,
                subtype_1: value.subtype_1?,
                subtype_2: value.subtype_2?,
                subtype_3: value.subtype_3?,
                subtype_4: value.subtype_4?,
                subtype_5: value.subtype_5?,
                subtype_6: value.subtype_6?,
                subtype_7: value.subtype_7?,
                subtype_8: value.subtype_8?,
            })
        }
    }
    impl ::std::convert::From<super::JsonrpcResponse> for JsonrpcResponse {
        fn from(value: super::JsonrpcResponse) -> Self {
            Self {
                subtype_0: Ok(value.subtype_0),
                subtype_1: Ok(value.subtype_1),
                subtype_2: Ok(value.subtype_2),
                subtype_3: Ok(value.subtype_3),
                subtype_4: Ok(value.subtype_4),
                subtype_5: Ok(value.subtype_5),
                subtype_6: Ok(value.subtype_6),
                subtype_7: Ok(value.subtype_7),
                subtype_8: Ok(value.subtype_8),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct JsonrpcSuccessResponse {
        id: ::std::result::Result<super::JsonrpcSuccessResponseId, ::std::string::String>,
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        result: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for JsonrpcSuccessResponse {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                result: Err("no value supplied for result".to_string()),
            }
        }
    }
    impl JsonrpcSuccessResponse {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::JsonrpcSuccessResponseId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {}", e));
            self
        }
        pub fn result<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.result = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for result: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<JsonrpcSuccessResponse> for super::JsonrpcSuccessResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: JsonrpcSuccessResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                jsonrpc: value.jsonrpc?,
                result: value.result?,
            })
        }
    }
    impl ::std::convert::From<super::JsonrpcSuccessResponse> for JsonrpcSuccessResponse {
        fn from(value: super::JsonrpcSuccessResponse) -> Self {
            Self {
                id: Ok(value.id),
                jsonrpc: Ok(value.jsonrpc),
                result: Ok(value.result),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListTaskPushNotificationConfigParams {
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        metadata: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ListTaskPushNotificationConfigParams {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                metadata: Ok(Default::default()),
            }
        }
    }
    impl ListTaskPushNotificationConfigParams {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ListTaskPushNotificationConfigParams>
        for super::ListTaskPushNotificationConfigParams
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListTaskPushNotificationConfigParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                metadata: value.metadata?,
            })
        }
    }
    impl ::std::convert::From<super::ListTaskPushNotificationConfigParams>
        for ListTaskPushNotificationConfigParams
    {
        fn from(value: super::ListTaskPushNotificationConfigParams) -> Self {
            Self {
                id: Ok(value.id),
                metadata: Ok(value.metadata),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListTaskPushNotificationConfigRequest {
        id: ::std::result::Result<
            super::ListTaskPushNotificationConfigRequestId,
            ::std::string::String,
        >,
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<
            super::ListTaskPushNotificationConfigParams,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ListTaskPushNotificationConfigRequest {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                method: Err("no value supplied for method".to_string()),
                params: Err("no value supplied for params".to_string()),
            }
        }
    }
    impl ListTaskPushNotificationConfigRequest {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListTaskPushNotificationConfigRequestId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {}", e));
            self
        }
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {}", e));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListTaskPushNotificationConfigParams>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ListTaskPushNotificationConfigRequest>
        for super::ListTaskPushNotificationConfigRequest
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListTaskPushNotificationConfigRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                jsonrpc: value.jsonrpc?,
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::ListTaskPushNotificationConfigRequest>
        for ListTaskPushNotificationConfigRequest
    {
        fn from(value: super::ListTaskPushNotificationConfigRequest) -> Self {
            Self {
                id: Ok(value.id),
                jsonrpc: Ok(value.jsonrpc),
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListTaskPushNotificationConfigSuccessResponse {
        id: ::std::result::Result<
            super::ListTaskPushNotificationConfigSuccessResponseId,
            ::std::string::String,
        >,
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        result: ::std::result::Result<
            ::std::vec::Vec<super::TaskPushNotificationConfig>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ListTaskPushNotificationConfigSuccessResponse {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                result: Err("no value supplied for result".to_string()),
            }
        }
    }
    impl ListTaskPushNotificationConfigSuccessResponse {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ListTaskPushNotificationConfigSuccessResponseId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {}", e));
            self
        }
        pub fn result<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::TaskPushNotificationConfig>>,
            T::Error: ::std::fmt::Display,
        {
            self.result = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for result: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ListTaskPushNotificationConfigSuccessResponse>
        for super::ListTaskPushNotificationConfigSuccessResponse
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListTaskPushNotificationConfigSuccessResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                jsonrpc: value.jsonrpc?,
                result: value.result?,
            })
        }
    }
    impl ::std::convert::From<super::ListTaskPushNotificationConfigSuccessResponse>
        for ListTaskPushNotificationConfigSuccessResponse
    {
        fn from(value: super::ListTaskPushNotificationConfigSuccessResponse) -> Self {
            Self {
                id: Ok(value.id),
                jsonrpc: Ok(value.jsonrpc),
                result: Ok(value.result),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Message {
        context_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        extensions:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        kind: ::std::result::Result<::std::string::String, ::std::string::String>,
        message_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        metadata: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        parts: ::std::result::Result<::std::vec::Vec<super::Part>, ::std::string::String>,
        reference_task_ids:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        role: ::std::result::Result<super::MessageRole, ::std::string::String>,
        task_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Message {
        fn default() -> Self {
            Self {
                context_id: Ok(Default::default()),
                extensions: Ok(Default::default()),
                kind: Err("no value supplied for kind".to_string()),
                message_id: Err("no value supplied for message_id".to_string()),
                metadata: Ok(Default::default()),
                parts: Err("no value supplied for parts".to_string()),
                reference_task_ids: Ok(Default::default()),
                role: Err("no value supplied for role".to_string()),
                task_id: Ok(Default::default()),
            }
        }
    }
    impl Message {
        pub fn context_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.context_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for context_id: {}", e));
            self
        }
        pub fn extensions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.extensions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extensions: {}", e));
            self
        }
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
        pub fn message_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.message_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message_id: {}", e));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {}", e));
            self
        }
        pub fn parts<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Part>>,
            T::Error: ::std::fmt::Display,
        {
            self.parts = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for parts: {}", e));
            self
        }
        pub fn reference_task_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.reference_task_ids = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for reference_task_ids: {}",
                    e
                )
            });
            self
        }
        pub fn role<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MessageRole>,
            T::Error: ::std::fmt::Display,
        {
            self.role = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role: {}", e));
            self
        }
        pub fn task_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.task_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for task_id: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Message> for super::Message {
        type Error = super::error::ConversionError;
        fn try_from(value: Message) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                context_id: value.context_id?,
                extensions: value.extensions?,
                kind: value.kind?,
                message_id: value.message_id?,
                metadata: value.metadata?,
                parts: value.parts?,
                reference_task_ids: value.reference_task_ids?,
                role: value.role?,
                task_id: value.task_id?,
            })
        }
    }
    impl ::std::convert::From<super::Message> for Message {
        fn from(value: super::Message) -> Self {
            Self {
                context_id: Ok(value.context_id),
                extensions: Ok(value.extensions),
                kind: Ok(value.kind),
                message_id: Ok(value.message_id),
                metadata: Ok(value.metadata),
                parts: Ok(value.parts),
                reference_task_ids: Ok(value.reference_task_ids),
                role: Ok(value.role),
                task_id: Ok(value.task_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MessageSendConfiguration {
        accepted_output_modes:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        blocking: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        history_length: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        push_notification_config: ::std::result::Result<
            ::std::option::Option<super::PushNotificationConfig>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for MessageSendConfiguration {
        fn default() -> Self {
            Self {
                accepted_output_modes: Ok(Default::default()),
                blocking: Ok(Default::default()),
                history_length: Ok(Default::default()),
                push_notification_config: Ok(Default::default()),
            }
        }
    }
    impl MessageSendConfiguration {
        pub fn accepted_output_modes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.accepted_output_modes = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for accepted_output_modes: {}",
                    e
                )
            });
            self
        }
        pub fn blocking<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.blocking = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for blocking: {}", e));
            self
        }
        pub fn history_length<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.history_length = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for history_length: {}", e));
            self
        }
        pub fn push_notification_config<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PushNotificationConfig>>,
            T::Error: ::std::fmt::Display,
        {
            self.push_notification_config = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for push_notification_config: {}",
                    e
                )
            });
            self
        }
    }
    impl ::std::convert::TryFrom<MessageSendConfiguration> for super::MessageSendConfiguration {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MessageSendConfiguration,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                accepted_output_modes: value.accepted_output_modes?,
                blocking: value.blocking?,
                history_length: value.history_length?,
                push_notification_config: value.push_notification_config?,
            })
        }
    }
    impl ::std::convert::From<super::MessageSendConfiguration> for MessageSendConfiguration {
        fn from(value: super::MessageSendConfiguration) -> Self {
            Self {
                accepted_output_modes: Ok(value.accepted_output_modes),
                blocking: Ok(value.blocking),
                history_length: Ok(value.history_length),
                push_notification_config: Ok(value.push_notification_config),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MessageSendParams {
        configuration: ::std::result::Result<
            ::std::option::Option<super::MessageSendConfiguration>,
            ::std::string::String,
        >,
        message: ::std::result::Result<super::Message, ::std::string::String>,
        metadata: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for MessageSendParams {
        fn default() -> Self {
            Self {
                configuration: Ok(Default::default()),
                message: Err("no value supplied for message".to_string()),
                metadata: Ok(Default::default()),
            }
        }
    }
    impl MessageSendParams {
        pub fn configuration<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::MessageSendConfiguration>>,
            T::Error: ::std::fmt::Display,
        {
            self.configuration = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for configuration: {}", e));
            self
        }
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Message>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {}", e));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<MessageSendParams> for super::MessageSendParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MessageSendParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                configuration: value.configuration?,
                message: value.message?,
                metadata: value.metadata?,
            })
        }
    }
    impl ::std::convert::From<super::MessageSendParams> for MessageSendParams {
        fn from(value: super::MessageSendParams) -> Self {
            Self {
                configuration: Ok(value.configuration),
                message: Ok(value.message),
                metadata: Ok(value.metadata),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MethodNotFoundError {
        code: ::std::result::Result<i64, ::std::string::String>,
        data: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
        message: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for MethodNotFoundError {
        fn default() -> Self {
            Self {
                code: Err("no value supplied for code".to_string()),
                data: Ok(Default::default()),
                message: Err("no value supplied for message".to_string()),
            }
        }
    }
    impl MethodNotFoundError {
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code: {}", e));
            self
        }
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {}", e));
            self
        }
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<MethodNotFoundError> for super::MethodNotFoundError {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MethodNotFoundError,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                data: value.data?,
                message: value.message?,
            })
        }
    }
    impl ::std::convert::From<super::MethodNotFoundError> for MethodNotFoundError {
        fn from(value: super::MethodNotFoundError) -> Self {
            Self {
                code: Ok(value.code),
                data: Ok(value.data),
                message: Ok(value.message),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OAuth2SecurityScheme {
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        flows: ::std::result::Result<super::OAuthFlows, ::std::string::String>,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for OAuth2SecurityScheme {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                flows: Err("no value supplied for flows".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl OAuth2SecurityScheme {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn flows<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::OAuthFlows>,
            T::Error: ::std::fmt::Display,
        {
            self.flows = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for flows: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<OAuth2SecurityScheme> for super::OAuth2SecurityScheme {
        type Error = super::error::ConversionError;
        fn try_from(
            value: OAuth2SecurityScheme,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                flows: value.flows?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::OAuth2SecurityScheme> for OAuth2SecurityScheme {
        fn from(value: super::OAuth2SecurityScheme) -> Self {
            Self {
                description: Ok(value.description),
                flows: Ok(value.flows),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OAuthFlows {
        authorization_code: ::std::result::Result<
            ::std::option::Option<super::AuthorizationCodeOAuthFlow>,
            ::std::string::String,
        >,
        client_credentials: ::std::result::Result<
            ::std::option::Option<super::ClientCredentialsOAuthFlow>,
            ::std::string::String,
        >,
        implicit: ::std::result::Result<
            ::std::option::Option<super::ImplicitOAuthFlow>,
            ::std::string::String,
        >,
        password: ::std::result::Result<
            ::std::option::Option<super::PasswordOAuthFlow>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for OAuthFlows {
        fn default() -> Self {
            Self {
                authorization_code: Ok(Default::default()),
                client_credentials: Ok(Default::default()),
                implicit: Ok(Default::default()),
                password: Ok(Default::default()),
            }
        }
    }
    impl OAuthFlows {
        pub fn authorization_code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AuthorizationCodeOAuthFlow>>,
            T::Error: ::std::fmt::Display,
        {
            self.authorization_code = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for authorization_code: {}",
                    e
                )
            });
            self
        }
        pub fn client_credentials<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ClientCredentialsOAuthFlow>>,
            T::Error: ::std::fmt::Display,
        {
            self.client_credentials = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for client_credentials: {}",
                    e
                )
            });
            self
        }
        pub fn implicit<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ImplicitOAuthFlow>>,
            T::Error: ::std::fmt::Display,
        {
            self.implicit = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for implicit: {}", e));
            self
        }
        pub fn password<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PasswordOAuthFlow>>,
            T::Error: ::std::fmt::Display,
        {
            self.password = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for password: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<OAuthFlows> for super::OAuthFlows {
        type Error = super::error::ConversionError;
        fn try_from(
            value: OAuthFlows,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                authorization_code: value.authorization_code?,
                client_credentials: value.client_credentials?,
                implicit: value.implicit?,
                password: value.password?,
            })
        }
    }
    impl ::std::convert::From<super::OAuthFlows> for OAuthFlows {
        fn from(value: super::OAuthFlows) -> Self {
            Self {
                authorization_code: Ok(value.authorization_code),
                client_credentials: Ok(value.client_credentials),
                implicit: Ok(value.implicit),
                password: Ok(value.password),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OpenIdConnectSecurityScheme {
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        open_id_connect_url: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for OpenIdConnectSecurityScheme {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                open_id_connect_url: Err("no value supplied for open_id_connect_url".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl OpenIdConnectSecurityScheme {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn open_id_connect_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.open_id_connect_url = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for open_id_connect_url: {}",
                    e
                )
            });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<OpenIdConnectSecurityScheme> for super::OpenIdConnectSecurityScheme {
        type Error = super::error::ConversionError;
        fn try_from(
            value: OpenIdConnectSecurityScheme,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                open_id_connect_url: value.open_id_connect_url?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::OpenIdConnectSecurityScheme> for OpenIdConnectSecurityScheme {
        fn from(value: super::OpenIdConnectSecurityScheme) -> Self {
            Self {
                description: Ok(value.description),
                open_id_connect_url: Ok(value.open_id_connect_url),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PartBase {
        metadata: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PartBase {
        fn default() -> Self {
            Self {
                metadata: Ok(Default::default()),
            }
        }
    }
    impl PartBase {
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<PartBase> for super::PartBase {
        type Error = super::error::ConversionError;
        fn try_from(value: PartBase) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                metadata: value.metadata?,
            })
        }
    }
    impl ::std::convert::From<super::PartBase> for PartBase {
        fn from(value: super::PartBase) -> Self {
            Self {
                metadata: Ok(value.metadata),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PasswordOAuthFlow {
        refresh_url: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        scopes: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            ::std::string::String,
        >,
        token_url: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for PasswordOAuthFlow {
        fn default() -> Self {
            Self {
                refresh_url: Ok(Default::default()),
                scopes: Err("no value supplied for scopes".to_string()),
                token_url: Err("no value supplied for token_url".to_string()),
            }
        }
    }
    impl PasswordOAuthFlow {
        pub fn refresh_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.refresh_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for refresh_url: {}", e));
            self
        }
        pub fn scopes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::collections::HashMap<::std::string::String, ::std::string::String>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.scopes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for scopes: {}", e));
            self
        }
        pub fn token_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.token_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for token_url: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<PasswordOAuthFlow> for super::PasswordOAuthFlow {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PasswordOAuthFlow,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                refresh_url: value.refresh_url?,
                scopes: value.scopes?,
                token_url: value.token_url?,
            })
        }
    }
    impl ::std::convert::From<super::PasswordOAuthFlow> for PasswordOAuthFlow {
        fn from(value: super::PasswordOAuthFlow) -> Self {
            Self {
                refresh_url: Ok(value.refresh_url),
                scopes: Ok(value.scopes),
                token_url: Ok(value.token_url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PushNotificationAuthenticationInfo {
        credentials: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        schemes:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
    }
    impl ::std::default::Default for PushNotificationAuthenticationInfo {
        fn default() -> Self {
            Self {
                credentials: Ok(Default::default()),
                schemes: Err("no value supplied for schemes".to_string()),
            }
        }
    }
    impl PushNotificationAuthenticationInfo {
        pub fn credentials<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.credentials = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for credentials: {}", e));
            self
        }
        pub fn schemes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.schemes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for schemes: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<PushNotificationAuthenticationInfo>
        for super::PushNotificationAuthenticationInfo
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PushNotificationAuthenticationInfo,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                credentials: value.credentials?,
                schemes: value.schemes?,
            })
        }
    }
    impl ::std::convert::From<super::PushNotificationAuthenticationInfo>
        for PushNotificationAuthenticationInfo
    {
        fn from(value: super::PushNotificationAuthenticationInfo) -> Self {
            Self {
                credentials: Ok(value.credentials),
                schemes: Ok(value.schemes),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PushNotificationConfig {
        authentication: ::std::result::Result<
            ::std::option::Option<super::PushNotificationAuthenticationInfo>,
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        token: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        url: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for PushNotificationConfig {
        fn default() -> Self {
            Self {
                authentication: Ok(Default::default()),
                id: Ok(Default::default()),
                token: Ok(Default::default()),
                url: Err("no value supplied for url".to_string()),
            }
        }
    }
    impl PushNotificationConfig {
        pub fn authentication<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::option::Option<super::PushNotificationAuthenticationInfo>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.authentication = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for authentication: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn token<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.token = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for token: {}", e));
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<PushNotificationConfig> for super::PushNotificationConfig {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PushNotificationConfig,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                authentication: value.authentication?,
                id: value.id?,
                token: value.token?,
                url: value.url?,
            })
        }
    }
    impl ::std::convert::From<super::PushNotificationConfig> for PushNotificationConfig {
        fn from(value: super::PushNotificationConfig) -> Self {
            Self {
                authentication: Ok(value.authentication),
                id: Ok(value.id),
                token: Ok(value.token),
                url: Ok(value.url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PushNotificationNotSupportedError {
        code: ::std::result::Result<i64, ::std::string::String>,
        data: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
        message: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for PushNotificationNotSupportedError {
        fn default() -> Self {
            Self {
                code: Err("no value supplied for code".to_string()),
                data: Ok(Default::default()),
                message: Err("no value supplied for message".to_string()),
            }
        }
    }
    impl PushNotificationNotSupportedError {
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code: {}", e));
            self
        }
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {}", e));
            self
        }
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<PushNotificationNotSupportedError>
        for super::PushNotificationNotSupportedError
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PushNotificationNotSupportedError,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                data: value.data?,
                message: value.message?,
            })
        }
    }
    impl ::std::convert::From<super::PushNotificationNotSupportedError>
        for PushNotificationNotSupportedError
    {
        fn from(value: super::PushNotificationNotSupportedError) -> Self {
            Self {
                code: Ok(value.code),
                data: Ok(value.data),
                message: Ok(value.message),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SecuritySchemeBase {
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for SecuritySchemeBase {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
            }
        }
    }
    impl SecuritySchemeBase {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<SecuritySchemeBase> for super::SecuritySchemeBase {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SecuritySchemeBase,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
            })
        }
    }
    impl ::std::convert::From<super::SecuritySchemeBase> for SecuritySchemeBase {
        fn from(value: super::SecuritySchemeBase) -> Self {
            Self {
                description: Ok(value.description),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SendMessageRequest {
        id: ::std::result::Result<super::SendMessageRequestId, ::std::string::String>,
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<super::MessageSendParams, ::std::string::String>,
    }
    impl ::std::default::Default for SendMessageRequest {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                method: Err("no value supplied for method".to_string()),
                params: Err("no value supplied for params".to_string()),
            }
        }
    }
    impl SendMessageRequest {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SendMessageRequestId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {}", e));
            self
        }
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {}", e));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MessageSendParams>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<SendMessageRequest> for super::SendMessageRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SendMessageRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                jsonrpc: value.jsonrpc?,
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::SendMessageRequest> for SendMessageRequest {
        fn from(value: super::SendMessageRequest) -> Self {
            Self {
                id: Ok(value.id),
                jsonrpc: Ok(value.jsonrpc),
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SendMessageSuccessResponse {
        id: ::std::result::Result<super::SendMessageSuccessResponseId, ::std::string::String>,
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        result:
            ::std::result::Result<super::SendMessageSuccessResponseResult, ::std::string::String>,
    }
    impl ::std::default::Default for SendMessageSuccessResponse {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                result: Err("no value supplied for result".to_string()),
            }
        }
    }
    impl SendMessageSuccessResponse {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SendMessageSuccessResponseId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {}", e));
            self
        }
        pub fn result<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SendMessageSuccessResponseResult>,
            T::Error: ::std::fmt::Display,
        {
            self.result = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for result: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<SendMessageSuccessResponse> for super::SendMessageSuccessResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SendMessageSuccessResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                jsonrpc: value.jsonrpc?,
                result: value.result?,
            })
        }
    }
    impl ::std::convert::From<super::SendMessageSuccessResponse> for SendMessageSuccessResponse {
        fn from(value: super::SendMessageSuccessResponse) -> Self {
            Self {
                id: Ok(value.id),
                jsonrpc: Ok(value.jsonrpc),
                result: Ok(value.result),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SendStreamingMessageRequest {
        id: ::std::result::Result<super::SendStreamingMessageRequestId, ::std::string::String>,
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<super::MessageSendParams, ::std::string::String>,
    }
    impl ::std::default::Default for SendStreamingMessageRequest {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                method: Err("no value supplied for method".to_string()),
                params: Err("no value supplied for params".to_string()),
            }
        }
    }
    impl SendStreamingMessageRequest {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SendStreamingMessageRequestId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {}", e));
            self
        }
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {}", e));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MessageSendParams>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<SendStreamingMessageRequest> for super::SendStreamingMessageRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SendStreamingMessageRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                jsonrpc: value.jsonrpc?,
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::SendStreamingMessageRequest> for SendStreamingMessageRequest {
        fn from(value: super::SendStreamingMessageRequest) -> Self {
            Self {
                id: Ok(value.id),
                jsonrpc: Ok(value.jsonrpc),
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SendStreamingMessageSuccessResponse {
        id: ::std::result::Result<
            super::SendStreamingMessageSuccessResponseId,
            ::std::string::String,
        >,
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        result: ::std::result::Result<
            super::SendStreamingMessageSuccessResponseResult,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for SendStreamingMessageSuccessResponse {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                result: Err("no value supplied for result".to_string()),
            }
        }
    }
    impl SendStreamingMessageSuccessResponse {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SendStreamingMessageSuccessResponseId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {}", e));
            self
        }
        pub fn result<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SendStreamingMessageSuccessResponseResult>,
            T::Error: ::std::fmt::Display,
        {
            self.result = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for result: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<SendStreamingMessageSuccessResponse>
        for super::SendStreamingMessageSuccessResponse
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SendStreamingMessageSuccessResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                jsonrpc: value.jsonrpc?,
                result: value.result?,
            })
        }
    }
    impl ::std::convert::From<super::SendStreamingMessageSuccessResponse>
        for SendStreamingMessageSuccessResponse
    {
        fn from(value: super::SendStreamingMessageSuccessResponse) -> Self {
            Self {
                id: Ok(value.id),
                jsonrpc: Ok(value.jsonrpc),
                result: Ok(value.result),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SetTaskPushNotificationConfigRequest {
        id: ::std::result::Result<
            super::SetTaskPushNotificationConfigRequestId,
            ::std::string::String,
        >,
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<super::TaskPushNotificationConfig, ::std::string::String>,
    }
    impl ::std::default::Default for SetTaskPushNotificationConfigRequest {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                method: Err("no value supplied for method".to_string()),
                params: Err("no value supplied for params".to_string()),
            }
        }
    }
    impl SetTaskPushNotificationConfigRequest {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SetTaskPushNotificationConfigRequestId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {}", e));
            self
        }
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {}", e));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TaskPushNotificationConfig>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<SetTaskPushNotificationConfigRequest>
        for super::SetTaskPushNotificationConfigRequest
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SetTaskPushNotificationConfigRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                jsonrpc: value.jsonrpc?,
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::SetTaskPushNotificationConfigRequest>
        for SetTaskPushNotificationConfigRequest
    {
        fn from(value: super::SetTaskPushNotificationConfigRequest) -> Self {
            Self {
                id: Ok(value.id),
                jsonrpc: Ok(value.jsonrpc),
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SetTaskPushNotificationConfigSuccessResponse {
        id: ::std::result::Result<
            super::SetTaskPushNotificationConfigSuccessResponseId,
            ::std::string::String,
        >,
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        result: ::std::result::Result<super::TaskPushNotificationConfig, ::std::string::String>,
    }
    impl ::std::default::Default for SetTaskPushNotificationConfigSuccessResponse {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                result: Err("no value supplied for result".to_string()),
            }
        }
    }
    impl SetTaskPushNotificationConfigSuccessResponse {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SetTaskPushNotificationConfigSuccessResponseId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {}", e));
            self
        }
        pub fn result<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TaskPushNotificationConfig>,
            T::Error: ::std::fmt::Display,
        {
            self.result = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for result: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<SetTaskPushNotificationConfigSuccessResponse>
        for super::SetTaskPushNotificationConfigSuccessResponse
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SetTaskPushNotificationConfigSuccessResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                jsonrpc: value.jsonrpc?,
                result: value.result?,
            })
        }
    }
    impl ::std::convert::From<super::SetTaskPushNotificationConfigSuccessResponse>
        for SetTaskPushNotificationConfigSuccessResponse
    {
        fn from(value: super::SetTaskPushNotificationConfigSuccessResponse) -> Self {
            Self {
                id: Ok(value.id),
                jsonrpc: Ok(value.jsonrpc),
                result: Ok(value.result),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Task {
        artifacts: ::std::result::Result<::std::vec::Vec<super::Artifact>, ::std::string::String>,
        context_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        history: ::std::result::Result<::std::vec::Vec<super::Message>, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        kind: ::std::result::Result<::std::string::String, ::std::string::String>,
        metadata: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        status: ::std::result::Result<super::TaskStatus, ::std::string::String>,
    }
    impl ::std::default::Default for Task {
        fn default() -> Self {
            Self {
                artifacts: Ok(Default::default()),
                context_id: Err("no value supplied for context_id".to_string()),
                history: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                kind: Err("no value supplied for kind".to_string()),
                metadata: Ok(Default::default()),
                status: Err("no value supplied for status".to_string()),
            }
        }
    }
    impl Task {
        pub fn artifacts<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Artifact>>,
            T::Error: ::std::fmt::Display,
        {
            self.artifacts = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for artifacts: {}", e));
            self
        }
        pub fn context_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.context_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for context_id: {}", e));
            self
        }
        pub fn history<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Message>>,
            T::Error: ::std::fmt::Display,
        {
            self.history = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for history: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {}", e));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TaskStatus>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<Task> for super::Task {
        type Error = super::error::ConversionError;
        fn try_from(value: Task) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                artifacts: value.artifacts?,
                context_id: value.context_id?,
                history: value.history?,
                id: value.id?,
                kind: value.kind?,
                metadata: value.metadata?,
                status: value.status?,
            })
        }
    }
    impl ::std::convert::From<super::Task> for Task {
        fn from(value: super::Task) -> Self {
            Self {
                artifacts: Ok(value.artifacts),
                context_id: Ok(value.context_id),
                history: Ok(value.history),
                id: Ok(value.id),
                kind: Ok(value.kind),
                metadata: Ok(value.metadata),
                status: Ok(value.status),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TaskArtifactUpdateEvent {
        append: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        artifact: ::std::result::Result<super::Artifact, ::std::string::String>,
        context_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        kind: ::std::result::Result<::std::string::String, ::std::string::String>,
        last_chunk: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        metadata: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        task_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for TaskArtifactUpdateEvent {
        fn default() -> Self {
            Self {
                append: Ok(Default::default()),
                artifact: Err("no value supplied for artifact".to_string()),
                context_id: Err("no value supplied for context_id".to_string()),
                kind: Err("no value supplied for kind".to_string()),
                last_chunk: Ok(Default::default()),
                metadata: Ok(Default::default()),
                task_id: Err("no value supplied for task_id".to_string()),
            }
        }
    }
    impl TaskArtifactUpdateEvent {
        pub fn append<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.append = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for append: {}", e));
            self
        }
        pub fn artifact<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Artifact>,
            T::Error: ::std::fmt::Display,
        {
            self.artifact = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for artifact: {}", e));
            self
        }
        pub fn context_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.context_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for context_id: {}", e));
            self
        }
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
        pub fn last_chunk<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.last_chunk = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for last_chunk: {}", e));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {}", e));
            self
        }
        pub fn task_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.task_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for task_id: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<TaskArtifactUpdateEvent> for super::TaskArtifactUpdateEvent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TaskArtifactUpdateEvent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                append: value.append?,
                artifact: value.artifact?,
                context_id: value.context_id?,
                kind: value.kind?,
                last_chunk: value.last_chunk?,
                metadata: value.metadata?,
                task_id: value.task_id?,
            })
        }
    }
    impl ::std::convert::From<super::TaskArtifactUpdateEvent> for TaskArtifactUpdateEvent {
        fn from(value: super::TaskArtifactUpdateEvent) -> Self {
            Self {
                append: Ok(value.append),
                artifact: Ok(value.artifact),
                context_id: Ok(value.context_id),
                kind: Ok(value.kind),
                last_chunk: Ok(value.last_chunk),
                metadata: Ok(value.metadata),
                task_id: Ok(value.task_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TaskIdParams {
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        metadata: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for TaskIdParams {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                metadata: Ok(Default::default()),
            }
        }
    }
    impl TaskIdParams {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<TaskIdParams> for super::TaskIdParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TaskIdParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                metadata: value.metadata?,
            })
        }
    }
    impl ::std::convert::From<super::TaskIdParams> for TaskIdParams {
        fn from(value: super::TaskIdParams) -> Self {
            Self {
                id: Ok(value.id),
                metadata: Ok(value.metadata),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TaskNotCancelableError {
        code: ::std::result::Result<i64, ::std::string::String>,
        data: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
        message: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for TaskNotCancelableError {
        fn default() -> Self {
            Self {
                code: Err("no value supplied for code".to_string()),
                data: Ok(Default::default()),
                message: Err("no value supplied for message".to_string()),
            }
        }
    }
    impl TaskNotCancelableError {
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code: {}", e));
            self
        }
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {}", e));
            self
        }
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<TaskNotCancelableError> for super::TaskNotCancelableError {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TaskNotCancelableError,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                data: value.data?,
                message: value.message?,
            })
        }
    }
    impl ::std::convert::From<super::TaskNotCancelableError> for TaskNotCancelableError {
        fn from(value: super::TaskNotCancelableError) -> Self {
            Self {
                code: Ok(value.code),
                data: Ok(value.data),
                message: Ok(value.message),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TaskNotFoundError {
        code: ::std::result::Result<i64, ::std::string::String>,
        data: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
        message: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for TaskNotFoundError {
        fn default() -> Self {
            Self {
                code: Err("no value supplied for code".to_string()),
                data: Ok(Default::default()),
                message: Err("no value supplied for message".to_string()),
            }
        }
    }
    impl TaskNotFoundError {
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code: {}", e));
            self
        }
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {}", e));
            self
        }
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<TaskNotFoundError> for super::TaskNotFoundError {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TaskNotFoundError,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                data: value.data?,
                message: value.message?,
            })
        }
    }
    impl ::std::convert::From<super::TaskNotFoundError> for TaskNotFoundError {
        fn from(value: super::TaskNotFoundError) -> Self {
            Self {
                code: Ok(value.code),
                data: Ok(value.data),
                message: Ok(value.message),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TaskPushNotificationConfig {
        push_notification_config:
            ::std::result::Result<super::PushNotificationConfig, ::std::string::String>,
        task_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for TaskPushNotificationConfig {
        fn default() -> Self {
            Self {
                push_notification_config: Err(
                    "no value supplied for push_notification_config".to_string()
                ),
                task_id: Err("no value supplied for task_id".to_string()),
            }
        }
    }
    impl TaskPushNotificationConfig {
        pub fn push_notification_config<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PushNotificationConfig>,
            T::Error: ::std::fmt::Display,
        {
            self.push_notification_config = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for push_notification_config: {}",
                    e
                )
            });
            self
        }
        pub fn task_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.task_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for task_id: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<TaskPushNotificationConfig> for super::TaskPushNotificationConfig {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TaskPushNotificationConfig,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                push_notification_config: value.push_notification_config?,
                task_id: value.task_id?,
            })
        }
    }
    impl ::std::convert::From<super::TaskPushNotificationConfig> for TaskPushNotificationConfig {
        fn from(value: super::TaskPushNotificationConfig) -> Self {
            Self {
                push_notification_config: Ok(value.push_notification_config),
                task_id: Ok(value.task_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TaskQueryParams {
        history_length: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        metadata: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for TaskQueryParams {
        fn default() -> Self {
            Self {
                history_length: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                metadata: Ok(Default::default()),
            }
        }
    }
    impl TaskQueryParams {
        pub fn history_length<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.history_length = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for history_length: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<TaskQueryParams> for super::TaskQueryParams {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TaskQueryParams,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                history_length: value.history_length?,
                id: value.id?,
                metadata: value.metadata?,
            })
        }
    }
    impl ::std::convert::From<super::TaskQueryParams> for TaskQueryParams {
        fn from(value: super::TaskQueryParams) -> Self {
            Self {
                history_length: Ok(value.history_length),
                id: Ok(value.id),
                metadata: Ok(value.metadata),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TaskResubscriptionRequest {
        id: ::std::result::Result<super::TaskResubscriptionRequestId, ::std::string::String>,
        jsonrpc: ::std::result::Result<::std::string::String, ::std::string::String>,
        method: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<super::TaskIdParams, ::std::string::String>,
    }
    impl ::std::default::Default for TaskResubscriptionRequest {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                jsonrpc: Err("no value supplied for jsonrpc".to_string()),
                method: Err("no value supplied for method".to_string()),
                params: Err("no value supplied for params".to_string()),
            }
        }
    }
    impl TaskResubscriptionRequest {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TaskResubscriptionRequestId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn jsonrpc<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.jsonrpc = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for jsonrpc: {}", e));
            self
        }
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {}", e));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TaskIdParams>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<TaskResubscriptionRequest> for super::TaskResubscriptionRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TaskResubscriptionRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                jsonrpc: value.jsonrpc?,
                method: value.method?,
                params: value.params?,
            })
        }
    }
    impl ::std::convert::From<super::TaskResubscriptionRequest> for TaskResubscriptionRequest {
        fn from(value: super::TaskResubscriptionRequest) -> Self {
            Self {
                id: Ok(value.id),
                jsonrpc: Ok(value.jsonrpc),
                method: Ok(value.method),
                params: Ok(value.params),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TaskStatus {
        message:
            ::std::result::Result<::std::option::Option<super::Message>, ::std::string::String>,
        state: ::std::result::Result<super::TaskState, ::std::string::String>,
        timestamp: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for TaskStatus {
        fn default() -> Self {
            Self {
                message: Ok(Default::default()),
                state: Err("no value supplied for state".to_string()),
                timestamp: Ok(Default::default()),
            }
        }
    }
    impl TaskStatus {
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Message>>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {}", e));
            self
        }
        pub fn state<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TaskState>,
            T::Error: ::std::fmt::Display,
        {
            self.state = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for state: {}", e));
            self
        }
        pub fn timestamp<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.timestamp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for timestamp: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<TaskStatus> for super::TaskStatus {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TaskStatus,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                message: value.message?,
                state: value.state?,
                timestamp: value.timestamp?,
            })
        }
    }
    impl ::std::convert::From<super::TaskStatus> for TaskStatus {
        fn from(value: super::TaskStatus) -> Self {
            Self {
                message: Ok(value.message),
                state: Ok(value.state),
                timestamp: Ok(value.timestamp),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TaskStatusUpdateEvent {
        context_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        final_: ::std::result::Result<bool, ::std::string::String>,
        kind: ::std::result::Result<::std::string::String, ::std::string::String>,
        metadata: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        status: ::std::result::Result<super::TaskStatus, ::std::string::String>,
        task_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for TaskStatusUpdateEvent {
        fn default() -> Self {
            Self {
                context_id: Err("no value supplied for context_id".to_string()),
                final_: Err("no value supplied for final_".to_string()),
                kind: Err("no value supplied for kind".to_string()),
                metadata: Ok(Default::default()),
                status: Err("no value supplied for status".to_string()),
                task_id: Err("no value supplied for task_id".to_string()),
            }
        }
    }
    impl TaskStatusUpdateEvent {
        pub fn context_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.context_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for context_id: {}", e));
            self
        }
        pub fn final_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.final_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for final_: {}", e));
            self
        }
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {}", e));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TaskStatus>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {}", e));
            self
        }
        pub fn task_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.task_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for task_id: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<TaskStatusUpdateEvent> for super::TaskStatusUpdateEvent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TaskStatusUpdateEvent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                context_id: value.context_id?,
                final_: value.final_?,
                kind: value.kind?,
                metadata: value.metadata?,
                status: value.status?,
                task_id: value.task_id?,
            })
        }
    }
    impl ::std::convert::From<super::TaskStatusUpdateEvent> for TaskStatusUpdateEvent {
        fn from(value: super::TaskStatusUpdateEvent) -> Self {
            Self {
                context_id: Ok(value.context_id),
                final_: Ok(value.final_),
                kind: Ok(value.kind),
                metadata: Ok(value.metadata),
                status: Ok(value.status),
                task_id: Ok(value.task_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TextPart {
        kind: ::std::result::Result<::std::string::String, ::std::string::String>,
        metadata: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        text: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for TextPart {
        fn default() -> Self {
            Self {
                kind: Err("no value supplied for kind".to_string()),
                metadata: Ok(Default::default()),
                text: Err("no value supplied for text".to_string()),
            }
        }
    }
    impl TextPart {
        pub fn kind<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.kind = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kind: {}", e));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {}", e));
            self
        }
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<TextPart> for super::TextPart {
        type Error = super::error::ConversionError;
        fn try_from(value: TextPart) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                kind: value.kind?,
                metadata: value.metadata?,
                text: value.text?,
            })
        }
    }
    impl ::std::convert::From<super::TextPart> for TextPart {
        fn from(value: super::TextPart) -> Self {
            Self {
                kind: Ok(value.kind),
                metadata: Ok(value.metadata),
                text: Ok(value.text),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UnsupportedOperationError {
        code: ::std::result::Result<i64, ::std::string::String>,
        data: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
        message: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for UnsupportedOperationError {
        fn default() -> Self {
            Self {
                code: Err("no value supplied for code".to_string()),
                data: Ok(Default::default()),
                message: Err("no value supplied for message".to_string()),
            }
        }
    }
    impl UnsupportedOperationError {
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code: {}", e));
            self
        }
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {}", e));
            self
        }
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<UnsupportedOperationError> for super::UnsupportedOperationError {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UnsupportedOperationError,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                data: value.data?,
                message: value.message?,
            })
        }
    }
    impl ::std::convert::From<super::UnsupportedOperationError> for UnsupportedOperationError {
        fn from(value: super::UnsupportedOperationError) -> Self {
            Self {
                code: Ok(value.code),
                data: Ok(value.data),
                message: Ok(value.message),
            }
        }
    }
}
#[doc = r" Generation of default values for serde."]
pub mod defaults {
    pub(super) fn agent_card_preferred_transport() -> ::std::string::String {
        "JSONRPC".to_string()
    }
}

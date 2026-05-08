#![allow(irrefutable_let_patterns)]
#![allow(clippy::unit_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::enum_variant_names)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::uninlined_format_args)]
#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

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
#[doc = "Defines optional capabilities supported by an agent."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Agent Capabilities\","]
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
#[doc = "      \"description\": \"Indicates if the agent supports streaming responses.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
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
    #[doc = "Indicates if the agent supports streaming responses."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub streaming: ::std::option::Option<bool>,
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
#[doc = "AgentCard is a self-describing manifest for an agent. It provides essential\n metadata including the agent's identity, capabilities, skills, supported\n communication methods, and security requirements.\n Next ID: 20"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Agent Card\","]
#[doc = "  \"description\": \"AgentCard is a self-describing manifest for an agent. It provides essential\\n metadata including the agent's identity, capabilities, skills, supported\\n communication methods, and security requirements.\\n Next ID: 20\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"capabilities\","]
#[doc = "    \"defaultInputModes\","]
#[doc = "    \"defaultOutputModes\","]
#[doc = "    \"description\","]
#[doc = "    \"name\","]
#[doc = "    \"protocolVersion\","]
#[doc = "    \"skills\","]
#[doc = "    \"version\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"additionalInterfaces\": {"]
#[doc = "      \"description\": \"DEPRECATED: Use 'supported_interfaces' instead.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/AgentInterface\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"capabilities\": {"]
#[doc = "      \"description\": \"A2A Capability set supported by the agent.\","]
#[doc = "      \"$ref\": \"#/definitions/AgentCapabilities\""]
#[doc = "    },"]
#[doc = "    \"defaultInputModes\": {"]
#[doc = "      \"description\": \"protolint:enable REPEATED_FIELD_NAMES_PLURALIZED\\n The set of interaction modes that the agent supports across all skills.\\n This can be overridden per skill. Defined as media types.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"defaultOutputModes\": {"]
#[doc = "      \"description\": \"The media types supported as outputs from this agent.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"A human-readable description of the agent, assisting users and other agents\\n in understanding its purpose.\\n Example: \\\"Agent that helps users with recipes and cooking.\\\"\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"documentationUrl\": {"]
#[doc = "      \"description\": \"A url to provide additional documentation about the agent.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"iconUrl\": {"]
#[doc = "      \"description\": \"An optional URL to an icon for the agent.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"A human readable name for the agent.\\n Example: \\\"Recipe Agent\\\"\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"preferredTransport\": {"]
#[doc = "      \"description\": \"DEPRECATED: Use 'supported_interfaces' instead.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"protocolVersion\": {"]
#[doc = "      \"description\": \"The version of the A2A protocol this agent supports.\\n Default: \\\"1.0\\\"\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"provider\": {"]
#[doc = "      \"description\": \"The service provider of the agent.\","]
#[doc = "      \"$ref\": \"#/definitions/AgentProvider\""]
#[doc = "    },"]
#[doc = "    \"security\": {"]
#[doc = "      \"description\": \"protolint:disable REPEATED_FIELD_NAMES_PLURALIZED\\n Security requirements for contacting the agent.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Security\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"securitySchemes\": {"]
#[doc = "      \"description\": \"The security scheme details used for authenticating with this agent.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#/definitions/SecurityScheme\""]
#[doc = "      },"]
#[doc = "      \"propertyNames\": {"]
#[doc = "        \"type\": \"string\""]
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
#[doc = "      \"description\": \"Skills represent an ability of an agent. It is largely\\n a descriptive concept but represents a more focused set of behaviors that the\\n agent is likely to succeed at.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/AgentSkill\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"supportedInterfaces\": {"]
#[doc = "      \"description\": \"Ordered list of supported interfaces. First entry is preferred.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/AgentInterface\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"supportsExtendedAgentCard\": {"]
#[doc = "      \"description\": \"Whether the agent supports providing an extended agent card when authenticated.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"url\": {"]
#[doc = "      \"description\": \"DEPRECATED: Use 'supported_interfaces' instead.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"version\": {"]
#[doc = "      \"description\": \"The version of the agent.\\n Example: \\\"1.0.0\\\"\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct AgentCard {
    #[doc = "DEPRECATED: Use 'supported_interfaces' instead."]
    #[serde(
        rename = "additionalInterfaces",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub additional_interfaces: ::std::vec::Vec<AgentInterface>,
    #[doc = "A2A Capability set supported by the agent."]
    pub capabilities: AgentCapabilities,
    #[doc = "protolint:enable REPEATED_FIELD_NAMES_PLURALIZED\n The set of interaction modes that the agent supports across all skills.\n This can be overridden per skill. Defined as media types."]
    #[serde(rename = "defaultInputModes")]
    pub default_input_modes: ::std::vec::Vec<::std::string::String>,
    #[doc = "The media types supported as outputs from this agent."]
    #[serde(rename = "defaultOutputModes")]
    pub default_output_modes: ::std::vec::Vec<::std::string::String>,
    #[doc = "A human-readable description of the agent, assisting users and other agents\n in understanding its purpose.\n Example: \"Agent that helps users with recipes and cooking.\""]
    pub description: ::std::string::String,
    #[doc = "A url to provide additional documentation about the agent."]
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
    #[doc = "A human readable name for the agent.\n Example: \"Recipe Agent\""]
    pub name: ::std::string::String,
    #[doc = "DEPRECATED: Use 'supported_interfaces' instead."]
    #[serde(
        rename = "preferredTransport",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub preferred_transport: ::std::option::Option<::std::string::String>,
    #[doc = "The version of the A2A protocol this agent supports.\n Default: \"1.0\""]
    #[serde(rename = "protocolVersion")]
    pub protocol_version: ::std::string::String,
    #[doc = "The service provider of the agent."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub provider: ::std::option::Option<AgentProvider>,
    #[doc = "protolint:disable REPEATED_FIELD_NAMES_PLURALIZED\n Security requirements for contacting the agent."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub security: ::std::vec::Vec<Security>,
    #[doc = "The security scheme details used for authenticating with this agent."]
    #[serde(
        rename = "securitySchemes",
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub security_schemes: ::std::collections::HashMap<::std::string::String, SecurityScheme>,
    #[doc = "JSON Web Signatures computed for this AgentCard."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub signatures: ::std::vec::Vec<AgentCardSignature>,
    #[doc = "Skills represent an ability of an agent. It is largely\n a descriptive concept but represents a more focused set of behaviors that the\n agent is likely to succeed at."]
    pub skills: ::std::vec::Vec<AgentSkill>,
    #[doc = "Ordered list of supported interfaces. First entry is preferred."]
    #[serde(
        rename = "supportedInterfaces",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub supported_interfaces: ::std::vec::Vec<AgentInterface>,
    #[doc = "Whether the agent supports providing an extended agent card when authenticated."]
    #[serde(
        rename = "supportsExtendedAgentCard",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub supports_extended_agent_card: ::std::option::Option<bool>,
    #[doc = "DEPRECATED: Use 'supported_interfaces' instead."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<::std::string::String>,
    #[doc = "The version of the agent.\n Example: \"1.0.0\""]
    pub version: ::std::string::String,
}
impl AgentCard {
    pub fn builder() -> builder::AgentCard {
        Default::default()
    }
}
#[doc = "AgentCardSignature represents a JWS signature of an AgentCard.\n This follows the JSON format of an RFC 7515 JSON Web Signature (JWS)."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Agent Card Signature\","]
#[doc = "  \"description\": \"AgentCardSignature represents a JWS signature of an AgentCard.\\n This follows the JSON format of an RFC 7515 JSON Web Signature (JWS).\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"protected\","]
#[doc = "    \"signature\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"header\": {"]
#[doc = "      \"description\": \"The unprotected JWS header values.\","]
#[doc = "      \"$ref\": \"#/definitions/Struct\""]
#[doc = "    },"]
#[doc = "    \"protected\": {"]
#[doc = "      \"description\": \"The protected JWS header for the signature. This is always a\\n base64url-encoded JSON object. Required.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"signature\": {"]
#[doc = "      \"description\": \"The computed signature, base64url-encoded. Required.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct AgentCardSignature {
    #[doc = "The unprotected JWS header values."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub header: ::std::option::Option<Struct>,
    #[doc = "The protected JWS header for the signature. This is always a\n base64url-encoded JSON object. Required."]
    pub protected: ::std::string::String,
    #[doc = "The computed signature, base64url-encoded. Required."]
    pub signature: ::std::string::String,
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
#[doc = "  \"title\": \"Agent Extension\","]
#[doc = "  \"description\": \"A declaration of a protocol extension supported by an Agent.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"description\","]
#[doc = "    \"required\","]
#[doc = "    \"uri\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"A human-readable description of how this agent uses the extension.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"params\": {"]
#[doc = "      \"description\": \"Optional, extension-specific configuration parameters.\","]
#[doc = "      \"$ref\": \"#/definitions/Struct\""]
#[doc = "    },"]
#[doc = "    \"required\": {"]
#[doc = "      \"description\": \"If true, the client must understand and comply with the extension's requirements.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"uri\": {"]
#[doc = "      \"description\": \"The unique URI identifying the extension.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct AgentExtension {
    #[doc = "A human-readable description of how this agent uses the extension."]
    pub description: ::std::string::String,
    #[doc = "Optional, extension-specific configuration parameters."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub params: ::std::option::Option<Struct>,
    #[doc = "If true, the client must understand and comply with the extension's requirements."]
    pub required: bool,
    #[doc = "The unique URI identifying the extension."]
    pub uri: ::std::string::String,
}
impl AgentExtension {
    pub fn builder() -> builder::AgentExtension {
        Default::default()
    }
}
#[doc = "Declares a combination of a target URL and a transport protocol for interacting with the agent.\n This allows agents to expose the same functionality over multiple protocol binding mechanisms."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Agent Interface\","]
#[doc = "  \"description\": \"Declares a combination of a target URL and a transport protocol for interacting with the agent.\\n This allows agents to expose the same functionality over multiple protocol binding mechanisms.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"protocolBinding\","]
#[doc = "    \"url\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"protocolBinding\": {"]
#[doc = "      \"description\": \"The protocol binding supported at this URL. This is an open form string, to be\\n easily extended for other protocol bindings. The core ones officially\\n supported are `JSONRPC`, `GRPC` and `HTTP+JSON`.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"tenant\": {"]
#[doc = "      \"description\": \"Tenant to be set in the request when calling the agent.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"url\": {"]
#[doc = "      \"description\": \"The URL where this interface is available. Must be a valid absolute HTTPS URL in production.\\n Example: \\\"https://api.example.com/a2a/v1\\\", \\\"https://grpc.example.com/a2a\\\"\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct AgentInterface {
    #[doc = "The protocol binding supported at this URL. This is an open form string, to be\n easily extended for other protocol bindings. The core ones officially\n supported are `JSONRPC`, `GRPC` and `HTTP+JSON`."]
    #[serde(rename = "protocolBinding")]
    pub protocol_binding: ::std::string::String,
    #[doc = "Tenant to be set in the request when calling the agent."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tenant: ::std::option::Option<::std::string::String>,
    #[doc = "The URL where this interface is available. Must be a valid absolute HTTPS URL in production.\n Example: \"https://api.example.com/a2a/v1\", \"https://grpc.example.com/a2a\""]
    pub url: ::std::string::String,
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
#[doc = "  \"title\": \"Agent Provider\","]
#[doc = "  \"description\": \"Represents the service provider of an agent.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"organization\","]
#[doc = "    \"url\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"organization\": {"]
#[doc = "      \"description\": \"The name of the agent provider's organization.\\n Example: \\\"Google\\\"\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"url\": {"]
#[doc = "      \"description\": \"A URL for the agent provider's website or relevant documentation.\\n Example: \\\"https://ai.google.dev\\\"\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct AgentProvider {
    #[doc = "The name of the agent provider's organization.\n Example: \"Google\""]
    pub organization: ::std::string::String,
    #[doc = "A URL for the agent provider's website or relevant documentation.\n Example: \"https://ai.google.dev\""]
    pub url: ::std::string::String,
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
#[doc = "  \"title\": \"Agent Skill\","]
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
#[doc = "      \"description\": \"A detailed description of the skill.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"examples\": {"]
#[doc = "      \"description\": \"Example prompts or scenarios that this skill can handle.\","]
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
#[doc = "      \"description\": \"The set of supported input media types for this skill, overriding the agent's defaults.\","]
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
#[doc = "      \"description\": \"The set of supported output media types for this skill, overriding the agent's defaults.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"security\": {"]
#[doc = "      \"description\": \"protolint:disable REPEATED_FIELD_NAMES_PLURALIZED\\n Security schemes necessary for this skill.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Security\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"tags\": {"]
#[doc = "      \"description\": \"A set of keywords describing the skill's capabilities.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct AgentSkill {
    #[doc = "A detailed description of the skill."]
    pub description: ::std::string::String,
    #[doc = "Example prompts or scenarios that this skill can handle."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub examples: ::std::vec::Vec<::std::string::String>,
    #[doc = "A unique identifier for the agent's skill."]
    pub id: ::std::string::String,
    #[doc = "The set of supported input media types for this skill, overriding the agent's defaults."]
    #[serde(
        rename = "inputModes",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub input_modes: ::std::vec::Vec<::std::string::String>,
    #[doc = "A human-readable name for the skill."]
    pub name: ::std::string::String,
    #[doc = "The set of supported output media types for this skill, overriding the agent's defaults."]
    #[serde(
        rename = "outputModes",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub output_modes: ::std::vec::Vec<::std::string::String>,
    #[doc = "protolint:disable REPEATED_FIELD_NAMES_PLURALIZED\n Security schemes necessary for this skill."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub security: ::std::vec::Vec<Security>,
    #[doc = "A set of keywords describing the skill's capabilities."]
    pub tags: ::std::vec::Vec<::std::string::String>,
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
#[doc = "  \"title\": \"API Key Security Scheme\","]
#[doc = "  \"description\": \"Defines a security scheme using an API key.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"location\","]
#[doc = "    \"name\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"An optional description for the security scheme.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"location\": {"]
#[doc = "      \"description\": \"The location of the API key. Valid values are \\\"query\\\", \\\"header\\\", or \\\"cookie\\\".\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"The name of the header, query, or cookie parameter to be used.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ApiKeySecurityScheme {
    #[doc = "An optional description for the security scheme."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "The location of the API key. Valid values are \"query\", \"header\", or \"cookie\"."]
    pub location: ::std::string::String,
    #[doc = "The name of the header, query, or cookie parameter to be used."]
    pub name: ::std::string::String,
}
impl ApiKeySecurityScheme {
    pub fn builder() -> builder::ApiKeySecurityScheme {
        Default::default()
    }
}
#[doc = "Artifacts represent task outputs."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Artifact\","]
#[doc = "  \"description\": \"Artifacts represent task outputs.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"artifactId\","]
#[doc = "    \"parts\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"artifactId\": {"]
#[doc = "      \"description\": \"Unique identifier (e.g. UUID) for the artifact. It must be at least unique\\n within a task.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"A human readable description of the artifact, optional.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"The URIs of extensions that are present or contributed to this Artifact.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"metadata\": {"]
#[doc = "      \"description\": \"Optional metadata included with the artifact.\","]
#[doc = "      \"$ref\": \"#/definitions/Struct\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"A human readable name for the artifact.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"parts\": {"]
#[doc = "      \"description\": \"The content of the artifact. Must contain at least one part.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Part\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Artifact {
    #[doc = "Unique identifier (e.g. UUID) for the artifact. It must be at least unique\n within a task."]
    #[serde(rename = "artifactId")]
    pub artifact_id: ::std::string::String,
    #[doc = "A human readable description of the artifact, optional."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "The URIs of extensions that are present or contributed to this Artifact."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub extensions: ::std::vec::Vec<::std::string::String>,
    #[doc = "Optional metadata included with the artifact."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<Struct>,
    #[doc = "A human readable name for the artifact."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[doc = "The content of the artifact. Must contain at least one part."]
    pub parts: ::std::vec::Vec<Part>,
}
impl Artifact {
    pub fn builder() -> builder::Artifact {
        Default::default()
    }
}
#[doc = "Defines authentication details, used for push notifications."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Authentication Info\","]
#[doc = "  \"description\": \"Defines authentication details, used for push notifications.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"schemes\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"credentials\": {"]
#[doc = "      \"description\": \"Optional credentials\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"schemes\": {"]
#[doc = "      \"description\": \"A list of supported authentication schemes (e.g., 'Basic', 'Bearer').\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct AuthenticationInfo {
    #[doc = "Optional credentials"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub credentials: ::std::option::Option<::std::string::String>,
    #[doc = "A list of supported authentication schemes (e.g., 'Basic', 'Bearer')."]
    pub schemes: ::std::vec::Vec<::std::string::String>,
}
impl AuthenticationInfo {
    pub fn builder() -> builder::AuthenticationInfo {
        Default::default()
    }
}
#[doc = "Defines configuration details for the OAuth 2.0 Authorization Code flow."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Authorization CodeO Auth Flow\","]
#[doc = "  \"description\": \"Defines configuration details for the OAuth 2.0 Authorization Code flow.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"authorizationUrl\","]
#[doc = "    \"scopes\","]
#[doc = "    \"tokenUrl\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"authorizationUrl\": {"]
#[doc = "      \"description\": \"The authorization URL to be used for this flow.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"refreshUrl\": {"]
#[doc = "      \"description\": \"The URL to be used for obtaining refresh tokens.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"scopes\": {"]
#[doc = "      \"description\": \"The available scopes for the OAuth2 security scheme.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"propertyNames\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"tokenUrl\": {"]
#[doc = "      \"description\": \"The token URL to be used for this flow.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct AuthorizationCodeOAuthFlow {
    #[doc = "The authorization URL to be used for this flow."]
    #[serde(rename = "authorizationUrl")]
    pub authorization_url: ::std::string::String,
    #[doc = "The URL to be used for obtaining refresh tokens."]
    #[serde(
        rename = "refreshUrl",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub refresh_url: ::std::option::Option<::std::string::String>,
    #[doc = "The available scopes for the OAuth2 security scheme."]
    pub scopes: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    #[doc = "The token URL to be used for this flow."]
    #[serde(rename = "tokenUrl")]
    pub token_url: ::std::string::String,
}
impl AuthorizationCodeOAuthFlow {
    pub fn builder() -> builder::AuthorizationCodeOAuthFlow {
        Default::default()
    }
}
#[doc = "Represents a request for the `tasks/cancel` method."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Cancel Task Request\","]
#[doc = "  \"description\": \"Represents a request for the `tasks/cancel` method.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"tenant\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"The resource name of the task to cancel.\\n Format: tasks/{task_id}\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"tenant\": {"]
#[doc = "      \"description\": \"Optional tenant, provided as a path parameter.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct CancelTaskRequest {
    #[doc = "The resource name of the task to cancel.\n Format: tasks/{task_id}"]
    pub name: ::std::string::String,
    #[doc = "Optional tenant, provided as a path parameter."]
    pub tenant: ::std::string::String,
}
impl CancelTaskRequest {
    pub fn builder() -> builder::CancelTaskRequest {
        Default::default()
    }
}
#[doc = "Defines configuration details for the OAuth 2.0 Client Credentials flow."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Client CredentialsO Auth Flow\","]
#[doc = "  \"description\": \"Defines configuration details for the OAuth 2.0 Client Credentials flow.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"scopes\","]
#[doc = "    \"tokenUrl\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"refreshUrl\": {"]
#[doc = "      \"description\": \"The URL to be used for obtaining refresh tokens.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"scopes\": {"]
#[doc = "      \"description\": \"The available scopes for the OAuth2 security scheme.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"propertyNames\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"tokenUrl\": {"]
#[doc = "      \"description\": \"The token URL to be used for this flow.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ClientCredentialsOAuthFlow {
    #[doc = "The URL to be used for obtaining refresh tokens."]
    #[serde(
        rename = "refreshUrl",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub refresh_url: ::std::option::Option<::std::string::String>,
    #[doc = "The available scopes for the OAuth2 security scheme."]
    pub scopes: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    #[doc = "The token URL to be used for this flow."]
    #[serde(rename = "tokenUrl")]
    pub token_url: ::std::string::String,
}
impl ClientCredentialsOAuthFlow {
    pub fn builder() -> builder::ClientCredentialsOAuthFlow {
        Default::default()
    }
}
#[doc = "DataPart represents a structured blob."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Data Part\","]
#[doc = "  \"description\": \"DataPart represents a structured blob.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"data\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"data\": {"]
#[doc = "      \"description\": \"A JSON object containing arbitrary data.\","]
#[doc = "      \"$ref\": \"#/definitions/Struct\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct DataPart {
    #[doc = "A JSON object containing arbitrary data."]
    pub data: Struct,
}
impl DataPart {
    pub fn builder() -> builder::DataPart {
        Default::default()
    }
}
#[doc = "Represents a request for the `tasks/pushNotificationConfig/delete` method."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Delete Task Push Notification Config Request\","]
#[doc = "  \"description\": \"Represents a request for the `tasks/pushNotificationConfig/delete` method.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"tenant\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"The resource name of the config to delete.\\n Format: tasks/{task_id}/pushNotificationConfigs/{config_id}\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"tenant\": {"]
#[doc = "      \"description\": \"Optional tenant, provided as a path parameter.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct DeleteTaskPushNotificationConfigRequest {
    #[doc = "The resource name of the config to delete.\n Format: tasks/{task_id}/pushNotificationConfigs/{config_id}"]
    pub name: ::std::string::String,
    #[doc = "Optional tenant, provided as a path parameter."]
    pub tenant: ::std::string::String,
}
impl DeleteTaskPushNotificationConfigRequest {
    pub fn builder() -> builder::DeleteTaskPushNotificationConfigRequest {
        Default::default()
    }
}
#[doc = "FilePart represents the different ways files can be provided. If files are\n small, directly feeding the bytes is supported via file_with_bytes. If the\n file is large, the agent should read the content as appropriate directly\n from the file_with_uri source."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"File Part\","]
#[doc = "  \"description\": \"FilePart represents the different ways files can be provided. If files are\\n small, directly feeding the bytes is supported via file_with_bytes. If the\\n file is large, the agent should read the content as appropriate directly\\n from the file_with_uri source.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"mediaType\","]
#[doc = "    \"name\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"fileWithBytes\": {"]
#[doc = "      \"description\": \"The base64-encoded content of the file.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^[A-Za-z0-9+/]*={0,2}$\""]
#[doc = "    },"]
#[doc = "    \"fileWithUri\": {"]
#[doc = "      \"description\": \"A URL pointing to the file's content.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"mediaType\": {"]
#[doc = "      \"description\": \"The media type of the file (e.g., \\\"application/pdf\\\").\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"An optional name for the file (e.g., \\\"document.pdf\\\").\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct FilePart {
    #[doc = "The base64-encoded content of the file."]
    #[serde(
        rename = "fileWithBytes",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub file_with_bytes: ::std::option::Option<FilePartFileWithBytes>,
    #[doc = "A URL pointing to the file's content."]
    #[serde(
        rename = "fileWithUri",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub file_with_uri: ::std::option::Option<::std::string::String>,
    #[doc = "The media type of the file (e.g., \"application/pdf\")."]
    #[serde(rename = "mediaType")]
    pub media_type: ::std::string::String,
    #[doc = "An optional name for the file (e.g., \"document.pdf\")."]
    pub name: ::std::string::String,
}
impl FilePart {
    pub fn builder() -> builder::FilePart {
        Default::default()
    }
}
#[doc = "The base64-encoded content of the file."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The base64-encoded content of the file.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^[A-Za-z0-9+/]*={0,2}$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct FilePartFileWithBytes(::std::string::String);
impl ::std::ops::Deref for FilePartFileWithBytes {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FilePartFileWithBytes> for ::std::string::String {
    fn from(value: FilePartFileWithBytes) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for FilePartFileWithBytes {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("^[A-Za-z0-9+/]*={0,2}$").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[A-Za-z0-9+/]*={0,2}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for FilePartFileWithBytes {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FilePartFileWithBytes {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FilePartFileWithBytes {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FilePartFileWithBytes {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "`GetExtendedAgentCardRequest`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Get Extended Agent Card Request\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"tenant\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"tenant\": {"]
#[doc = "      \"description\": \"Optional tenant, provided as a path parameter.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct GetExtendedAgentCardRequest {
    #[doc = "Optional tenant, provided as a path parameter."]
    pub tenant: ::std::string::String,
}
impl GetExtendedAgentCardRequest {
    pub fn builder() -> builder::GetExtendedAgentCardRequest {
        Default::default()
    }
}
#[doc = "`GetTaskPushNotificationConfigRequest`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Get Task Push Notification Config Request\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"tenant\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"The resource name of the config to retrieve.\\n Format: tasks/{task_id}/pushNotificationConfigs/{config_id}\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"tenant\": {"]
#[doc = "      \"description\": \"Optional tenant, provided as a path parameter.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct GetTaskPushNotificationConfigRequest {
    #[doc = "The resource name of the config to retrieve.\n Format: tasks/{task_id}/pushNotificationConfigs/{config_id}"]
    pub name: ::std::string::String,
    #[doc = "Optional tenant, provided as a path parameter."]
    pub tenant: ::std::string::String,
}
impl GetTaskPushNotificationConfigRequest {
    pub fn builder() -> builder::GetTaskPushNotificationConfigRequest {
        Default::default()
    }
}
#[doc = "Represents a request for the `tasks/get` method."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Get Task Request\","]
#[doc = "  \"description\": \"Represents a request for the `tasks/get` method.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"historyLength\": {"]
#[doc = "      \"description\": \"The maximum number of messages to include in the history.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"maximum\": 2147483647.0,"]
#[doc = "      \"minimum\": -2147483648.0"]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"The resource name of the task.\\n Format: tasks/{task_id}\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"tenant\": {"]
#[doc = "      \"description\": \"Optional tenant, provided as a path parameter.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct GetTaskRequest {
    #[doc = "The maximum number of messages to include in the history."]
    #[serde(
        rename = "historyLength",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub history_length: ::std::option::Option<i32>,
    #[doc = "The resource name of the task.\n Format: tasks/{task_id}"]
    pub name: ::std::string::String,
    #[doc = "Optional tenant, provided as a path parameter."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tenant: ::std::option::Option<::std::string::String>,
}
impl GetTaskRequest {
    pub fn builder() -> builder::GetTaskRequest {
        Default::default()
    }
}
#[doc = "Defines a security scheme using HTTP authentication."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"HTTP Auth Security Scheme\","]
#[doc = "  \"description\": \"Defines a security scheme using HTTP authentication.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"scheme\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"bearerFormat\": {"]
#[doc = "      \"description\": \"A hint to the client to identify how the bearer token is formatted (e.g., \\\"JWT\\\").\\n This is primarily for documentation purposes.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"An optional description for the security scheme.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"scheme\": {"]
#[doc = "      \"description\": \"The name of the HTTP Authentication scheme to be used in the Authorization header,\\n as defined in RFC7235 (e.g., \\\"Bearer\\\").\\n This value should be registered in the IANA Authentication Scheme registry.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct HttpAuthSecurityScheme {
    #[doc = "A hint to the client to identify how the bearer token is formatted (e.g., \"JWT\").\n This is primarily for documentation purposes."]
    #[serde(
        rename = "bearerFormat",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub bearer_format: ::std::option::Option<::std::string::String>,
    #[doc = "An optional description for the security scheme."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "The name of the HTTP Authentication scheme to be used in the Authorization header,\n as defined in RFC7235 (e.g., \"Bearer\").\n This value should be registered in the IANA Authentication Scheme registry."]
    pub scheme: ::std::string::String,
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
#[doc = "  \"title\": \"ImplicitO Auth Flow\","]
#[doc = "  \"description\": \"Defines configuration details for the OAuth 2.0 Implicit flow.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"authorizationUrl\","]
#[doc = "    \"scopes\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"authorizationUrl\": {"]
#[doc = "      \"description\": \"The authorization URL to be used for this flow.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"refreshUrl\": {"]
#[doc = "      \"description\": \"The URL to be used for obtaining refresh tokens.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"scopes\": {"]
#[doc = "      \"description\": \"The available scopes for the OAuth2 security scheme.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"propertyNames\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ImplicitOAuthFlow {
    #[doc = "The authorization URL to be used for this flow."]
    #[serde(rename = "authorizationUrl")]
    pub authorization_url: ::std::string::String,
    #[doc = "The URL to be used for obtaining refresh tokens."]
    #[serde(
        rename = "refreshUrl",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub refresh_url: ::std::option::Option<::std::string::String>,
    #[doc = "The available scopes for the OAuth2 security scheme."]
    pub scopes: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
}
impl ImplicitOAuthFlow {
    pub fn builder() -> builder::ImplicitOAuthFlow {
        Default::default()
    }
}
#[doc = "`ListTaskPushNotificationConfigRequest`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"List Task Push Notification Config Request\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"pageSize\","]
#[doc = "    \"pageToken\","]
#[doc = "    \"parent\","]
#[doc = "    \"tenant\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"pageSize\": {"]
#[doc = "      \"description\": \"The maximum number of configurations to return.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"maximum\": 2147483647.0,"]
#[doc = "      \"minimum\": -2147483648.0"]
#[doc = "    },"]
#[doc = "    \"pageToken\": {"]
#[doc = "      \"description\": \"A page token received from a previous ListTaskPushNotificationConfigRequest call.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"parent\": {"]
#[doc = "      \"description\": \"The parent task resource.\\n Format: tasks/{task_id}\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"tenant\": {"]
#[doc = "      \"description\": \"Optional tenant, provided as a path parameter.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ListTaskPushNotificationConfigRequest {
    #[doc = "The maximum number of configurations to return."]
    #[serde(rename = "pageSize")]
    pub page_size: i32,
    #[doc = "A page token received from a previous ListTaskPushNotificationConfigRequest call."]
    #[serde(rename = "pageToken")]
    pub page_token: ::std::string::String,
    #[doc = "The parent task resource.\n Format: tasks/{task_id}"]
    pub parent: ::std::string::String,
    #[doc = "Optional tenant, provided as a path parameter."]
    pub tenant: ::std::string::String,
}
impl ListTaskPushNotificationConfigRequest {
    pub fn builder() -> builder::ListTaskPushNotificationConfigRequest {
        Default::default()
    }
}
#[doc = "Represents a successful response for the `tasks/pushNotificationConfig/list`\n method."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"List Task Push Notification Config Response\","]
#[doc = "  \"description\": \"Represents a successful response for the `tasks/pushNotificationConfig/list`\\n method.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"nextPageToken\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"configs\": {"]
#[doc = "      \"description\": \"The list of push notification configurations.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/TaskPushNotificationConfig\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"nextPageToken\": {"]
#[doc = "      \"description\": \"A token, which can be sent as `page_token` to retrieve the next page.\\n If this field is omitted, there are no subsequent pages.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ListTaskPushNotificationConfigResponse {
    #[doc = "The list of push notification configurations."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub configs: ::std::vec::Vec<TaskPushNotificationConfig>,
    #[doc = "A token, which can be sent as `page_token` to retrieve the next page.\n If this field is omitted, there are no subsequent pages."]
    #[serde(rename = "nextPageToken")]
    pub next_page_token: ::std::string::String,
}
impl ListTaskPushNotificationConfigResponse {
    pub fn builder() -> builder::ListTaskPushNotificationConfigResponse {
        Default::default()
    }
}
#[doc = "Parameters for listing tasks with optional filtering criteria."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"List Tasks Request\","]
#[doc = "  \"description\": \"Parameters for listing tasks with optional filtering criteria.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"contextId\","]
#[doc = "    \"lastUpdatedAfter\","]
#[doc = "    \"pageToken\","]
#[doc = "    \"status\","]
#[doc = "    \"tenant\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"contextId\": {"]
#[doc = "      \"description\": \"Filter tasks by context ID to get tasks from a specific conversation or session.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"historyLength\": {"]
#[doc = "      \"description\": \"The maximum number of messages to include in each task's history.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"maximum\": 2147483647.0,"]
#[doc = "      \"minimum\": -2147483648.0"]
#[doc = "    },"]
#[doc = "    \"includeArtifacts\": {"]
#[doc = "      \"description\": \"Whether to include artifacts in the returned tasks.\\n Defaults to false to reduce payload size.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"lastUpdatedAfter\": {"]
#[doc = "      \"description\": \"Filter tasks updated after this timestamp (milliseconds since epoch).\\n Only tasks with a last updated time greater than or equal to this value will be returned.\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"pageSize\": {"]
#[doc = "      \"description\": \"Maximum number of tasks to return. Must be between 1 and 100.\\n Defaults to 50 if not specified.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"maximum\": 2147483647.0,"]
#[doc = "      \"minimum\": -2147483648.0"]
#[doc = "    },"]
#[doc = "    \"pageToken\": {"]
#[doc = "      \"description\": \"Token for pagination. Use the next_page_token from a previous ListTasksResponse.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"title\": \"Task State\","]
#[doc = "      \"description\": \"Filter tasks by their current status state.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"TASK_STATE_UNSPECIFIED\","]
#[doc = "        \"TASK_STATE_SUBMITTED\","]
#[doc = "        \"TASK_STATE_WORKING\","]
#[doc = "        \"TASK_STATE_COMPLETED\","]
#[doc = "        \"TASK_STATE_FAILED\","]
#[doc = "        \"TASK_STATE_CANCELLED\","]
#[doc = "        \"TASK_STATE_INPUT_REQUIRED\","]
#[doc = "        \"TASK_STATE_REJECTED\","]
#[doc = "        \"TASK_STATE_AUTH_REQUIRED\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"tenant\": {"]
#[doc = "      \"description\": \"Optional tenant, provided as a path parameter.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ListTasksRequest {
    #[doc = "Filter tasks by context ID to get tasks from a specific conversation or session."]
    #[serde(rename = "contextId")]
    pub context_id: ::std::string::String,
    #[doc = "The maximum number of messages to include in each task's history."]
    #[serde(
        rename = "historyLength",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub history_length: ::std::option::Option<i32>,
    #[doc = "Whether to include artifacts in the returned tasks.\n Defaults to false to reduce payload size."]
    #[serde(
        rename = "includeArtifacts",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub include_artifacts: ::std::option::Option<bool>,
    #[doc = "Filter tasks updated after this timestamp (milliseconds since epoch).\n Only tasks with a last updated time greater than or equal to this value will be returned."]
    #[serde(rename = "lastUpdatedAfter")]
    pub last_updated_after: i64,
    #[doc = "Maximum number of tasks to return. Must be between 1 and 100.\n Defaults to 50 if not specified."]
    #[serde(
        rename = "pageSize",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub page_size: ::std::option::Option<i32>,
    #[doc = "Token for pagination. Use the next_page_token from a previous ListTasksResponse."]
    #[serde(rename = "pageToken")]
    pub page_token: ::std::string::String,
    #[doc = "Filter tasks by their current status state."]
    pub status: TaskState,
    #[doc = "Optional tenant, provided as a path parameter."]
    pub tenant: ::std::string::String,
}
impl ListTasksRequest {
    pub fn builder() -> builder::ListTasksRequest {
        Default::default()
    }
}
#[doc = "Result object for tasks/list method containing an array of tasks and pagination information."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"List Tasks Response\","]
#[doc = "  \"description\": \"Result object for tasks/list method containing an array of tasks and pagination information.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"nextPageToken\","]
#[doc = "    \"pageSize\","]
#[doc = "    \"tasks\","]
#[doc = "    \"totalSize\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"nextPageToken\": {"]
#[doc = "      \"description\": \"Token for retrieving the next page. Empty string if no more results.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"pageSize\": {"]
#[doc = "      \"description\": \"The size of page requested.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"maximum\": 2147483647.0,"]
#[doc = "      \"minimum\": -2147483648.0"]
#[doc = "    },"]
#[doc = "    \"tasks\": {"]
#[doc = "      \"description\": \"Array of tasks matching the specified criteria.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Task\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"totalSize\": {"]
#[doc = "      \"description\": \"Total number of tasks available (before pagination).\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"maximum\": 2147483647.0,"]
#[doc = "      \"minimum\": -2147483648.0"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ListTasksResponse {
    #[doc = "Token for retrieving the next page. Empty string if no more results."]
    #[serde(rename = "nextPageToken")]
    pub next_page_token: ::std::string::String,
    #[doc = "The size of page requested."]
    #[serde(rename = "pageSize")]
    pub page_size: i32,
    #[doc = "Array of tasks matching the specified criteria."]
    pub tasks: ::std::vec::Vec<Task>,
    #[doc = "Total number of tasks available (before pagination)."]
    #[serde(rename = "totalSize")]
    pub total_size: i32,
}
impl ListTasksResponse {
    pub fn builder() -> builder::ListTasksResponse {
        Default::default()
    }
}
#[doc = "Message is one unit of communication between client and server. It is\n associated with a context and optionally a task. Since the server is\n responsible for the context definition, it must always provide a context_id\n in its messages. The client can optionally provide the context_id if it\n knows the context to associate the message to. Similarly for task_id,\n except the server decides if a task is created and whether to include the\n task_id."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Message\","]
#[doc = "  \"description\": \"Message is one unit of communication between client and server. It is\\n associated with a context and optionally a task. Since the server is\\n responsible for the context definition, it must always provide a context_id\\n in its messages. The client can optionally provide the context_id if it\\n knows the context to associate the message to. Similarly for task_id,\\n except the server decides if a task is created and whether to include the\\n task_id.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"messageId\","]
#[doc = "    \"parts\","]
#[doc = "    \"role\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"contextId\": {"]
#[doc = "      \"description\": \"The context id of the message. This is optional and if set, the message\\n will be associated with the given context.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"extensions\": {"]
#[doc = "      \"description\": \"The URIs of extensions that are present or contributed to this Message.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"messageId\": {"]
#[doc = "      \"description\": \"The unique identifier (e.g. UUID) of the message. This is required and\\n created by the message creator.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"metadata\": {"]
#[doc = "      \"description\": \"protolint:enable REPEATED_FIELD_NAMES_PLURALIZED\\n Any optional metadata to provide along with the message.\","]
#[doc = "      \"$ref\": \"#/definitions/Struct\""]
#[doc = "    },"]
#[doc = "    \"parts\": {"]
#[doc = "      \"description\": \"protolint:disable REPEATED_FIELD_NAMES_PLURALIZED\\n Parts is the container of the message content.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Part\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"referenceTaskIds\": {"]
#[doc = "      \"description\": \"A list of task IDs that this message references for additional context.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"role\": {"]
#[doc = "      \"title\": \"Role\","]
#[doc = "      \"description\": \"Identifies the sender of the message.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"ROLE_UNSPECIFIED\","]
#[doc = "        \"ROLE_USER\","]
#[doc = "        \"ROLE_AGENT\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"taskId\": {"]
#[doc = "      \"description\": \"The task id of the message. This is optional and if set, the message\\n will be associated with the given task.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Message {
    #[doc = "The context id of the message. This is optional and if set, the message\n will be associated with the given context."]
    #[serde(
        rename = "contextId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub context_id: ::std::option::Option<::std::string::String>,
    #[doc = "The URIs of extensions that are present or contributed to this Message."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub extensions: ::std::vec::Vec<::std::string::String>,
    #[doc = "The unique identifier (e.g. UUID) of the message. This is required and\n created by the message creator."]
    #[serde(rename = "messageId")]
    pub message_id: ::std::string::String,
    #[doc = "protolint:enable REPEATED_FIELD_NAMES_PLURALIZED\n Any optional metadata to provide along with the message."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<Struct>,
    #[doc = "protolint:disable REPEATED_FIELD_NAMES_PLURALIZED\n Parts is the container of the message content."]
    pub parts: ::std::vec::Vec<Part>,
    #[doc = "A list of task IDs that this message references for additional context."]
    #[serde(
        rename = "referenceTaskIds",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub reference_task_ids: ::std::vec::Vec<::std::string::String>,
    #[doc = "Identifies the sender of the message."]
    pub role: Role,
    #[doc = "The task id of the message. This is optional and if set, the message\n will be associated with the given task."]
    #[serde(
        rename = "taskId",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub task_id: ::std::option::Option<::std::string::String>,
}
impl Message {
    pub fn builder() -> builder::Message {
        Default::default()
    }
}
#[doc = "Defines a security scheme using mTLS authentication."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Mutual Tls Security Scheme\","]
#[doc = "  \"description\": \"Defines a security scheme using mTLS authentication.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"description\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"An optional description for the security scheme.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MutualTlsSecurityScheme {
    #[doc = "An optional description for the security scheme."]
    pub description: ::std::string::String,
}
impl MutualTlsSecurityScheme {
    pub fn builder() -> builder::MutualTlsSecurityScheme {
        Default::default()
    }
}
#[doc = "Defines a security scheme using OAuth 2.0."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"O Auth2 Security Scheme\","]
#[doc = "  \"description\": \"Defines a security scheme using OAuth 2.0.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"flows\""]
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
#[doc = "    \"oauth2MetadataUrl\": {"]
#[doc = "      \"description\": \"URL to the oauth2 authorization server metadata\\n RFC8414 (https://datatracker.ietf.org/doc/html/rfc8414). TLS is required.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct OAuth2SecurityScheme {
    #[doc = "An optional description for the security scheme."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "An object containing configuration information for the supported OAuth 2.0 flows."]
    pub flows: OAuthFlows,
    #[doc = "URL to the oauth2 authorization server metadata\n RFC8414 (https://datatracker.ietf.org/doc/html/rfc8414). TLS is required."]
    #[serde(
        rename = "oauth2MetadataUrl",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub oauth2_metadata_url: ::std::option::Option<::std::string::String>,
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
#[doc = "  \"title\": \"O Auth Flows\","]
#[doc = "  \"description\": \"Defines the configuration for the supported OAuth 2.0 flows.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"authorizationCode\": {"]
#[doc = "      \"description\": \"Configuration for the OAuth Authorization Code flow.\","]
#[doc = "      \"$ref\": \"#/definitions/AuthorizationCodeOAuthFlow\""]
#[doc = "    },"]
#[doc = "    \"clientCredentials\": {"]
#[doc = "      \"description\": \"Configuration for the OAuth Client Credentials flow.\","]
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
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct OAuthFlows {
    #[doc = "Configuration for the OAuth Authorization Code flow."]
    #[serde(
        rename = "authorizationCode",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub authorization_code: ::std::option::Option<AuthorizationCodeOAuthFlow>,
    #[doc = "Configuration for the OAuth Client Credentials flow."]
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
#[doc = "  \"title\": \"Open Id Connect Security Scheme\","]
#[doc = "  \"description\": \"Defines a security scheme using OpenID Connect.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"openIdConnectUrl\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"An optional description for the security scheme.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"openIdConnectUrl\": {"]
#[doc = "      \"description\": \"The OpenID Connect Discovery URL for the OIDC provider's metadata.\\n See: https://openid.net/specs/openid-connect-discovery-1_0.html\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct OpenIdConnectSecurityScheme {
    #[doc = "An optional description for the security scheme."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "The OpenID Connect Discovery URL for the OIDC provider's metadata.\n See: https://openid.net/specs/openid-connect-discovery-1_0.html"]
    #[serde(rename = "openIdConnectUrl")]
    pub open_id_connect_url: ::std::string::String,
}
impl OpenIdConnectSecurityScheme {
    pub fn builder() -> builder::OpenIdConnectSecurityScheme {
        Default::default()
    }
}
#[doc = "Part represents a container for a section of communication content.\n Parts can be purely textual, some sort of file (image, video, etc) or\n a structured data blob (i.e. JSON)."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Part\","]
#[doc = "  \"description\": \"Part represents a container for a section of communication content.\\n Parts can be purely textual, some sort of file (image, video, etc) or\\n a structured data blob (i.e. JSON).\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"data\": {"]
#[doc = "      \"description\": \"The structured data content.\","]
#[doc = "      \"$ref\": \"#/definitions/DataPart\""]
#[doc = "    },"]
#[doc = "    \"file\": {"]
#[doc = "      \"description\": \"The file content, represented as either a URI or as base64-encoded bytes.\","]
#[doc = "      \"$ref\": \"#/definitions/FilePart\""]
#[doc = "    },"]
#[doc = "    \"metadata\": {"]
#[doc = "      \"description\": \"Optional metadata associated with this part.\","]
#[doc = "      \"$ref\": \"#/definitions/Struct\""]
#[doc = "    },"]
#[doc = "    \"text\": {"]
#[doc = "      \"description\": \"The string content of the text part.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Part {
    #[doc = "The structured data content."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<DataPart>,
    #[doc = "The file content, represented as either a URI or as base64-encoded bytes."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub file: ::std::option::Option<FilePart>,
    #[doc = "Optional metadata associated with this part."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<Struct>,
    #[doc = "The string content of the text part."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub text: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for Part {
    fn default() -> Self {
        Self {
            data: Default::default(),
            file: Default::default(),
            metadata: Default::default(),
            text: Default::default(),
        }
    }
}
impl Part {
    pub fn builder() -> builder::Part {
        Default::default()
    }
}
#[doc = "Defines configuration details for the OAuth 2.0 Resource Owner Password flow."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"PasswordO Auth Flow\","]
#[doc = "  \"description\": \"Defines configuration details for the OAuth 2.0 Resource Owner Password flow.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"scopes\","]
#[doc = "    \"tokenUrl\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"refreshUrl\": {"]
#[doc = "      \"description\": \"The URL to be used for obtaining refresh tokens.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"scopes\": {"]
#[doc = "      \"description\": \"The available scopes for the OAuth2 security scheme.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"propertyNames\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"tokenUrl\": {"]
#[doc = "      \"description\": \"The token URL to be used for this flow.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct PasswordOAuthFlow {
    #[doc = "The URL to be used for obtaining refresh tokens."]
    #[serde(
        rename = "refreshUrl",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub refresh_url: ::std::option::Option<::std::string::String>,
    #[doc = "The available scopes for the OAuth2 security scheme."]
    pub scopes: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    #[doc = "The token URL to be used for this flow."]
    #[serde(rename = "tokenUrl")]
    pub token_url: ::std::string::String,
}
impl PasswordOAuthFlow {
    pub fn builder() -> builder::PasswordOAuthFlow {
        Default::default()
    }
}
#[doc = "Configuration for setting up push notifications for task updates."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Push Notification Config\","]
#[doc = "  \"description\": \"Configuration for setting up push notifications for task updates.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"url\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"authentication\": {"]
#[doc = "      \"description\": \"Information about the authentication to sent with the notification\","]
#[doc = "      \"$ref\": \"#/definitions/AuthenticationInfo\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"description\": \"A unique identifier (e.g. UUID) for this push notification.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"token\": {"]
#[doc = "      \"description\": \"Token unique for this task/session\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"url\": {"]
#[doc = "      \"description\": \"Url to send the notification too\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct PushNotificationConfig {
    #[doc = "Information about the authentication to sent with the notification"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub authentication: ::std::option::Option<AuthenticationInfo>,
    #[doc = "A unique identifier (e.g. UUID) for this push notification."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[doc = "Token unique for this task/session"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub token: ::std::option::Option<::std::string::String>,
    #[doc = "Url to send the notification too"]
    pub url: ::std::string::String,
}
impl PushNotificationConfig {
    pub fn builder() -> builder::PushNotificationConfig {
        Default::default()
    }
}
#[doc = "Identifies the sender of the message."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Role\","]
#[doc = "  \"description\": \"Identifies the sender of the message.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"ROLE_UNSPECIFIED\","]
#[doc = "    \"ROLE_USER\","]
#[doc = "    \"ROLE_AGENT\""]
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
pub enum Role {
    #[serde(rename = "ROLE_UNSPECIFIED")]
    RoleUnspecified,
    #[serde(rename = "ROLE_USER")]
    RoleUser,
    #[serde(rename = "ROLE_AGENT")]
    RoleAgent,
}
impl ::std::fmt::Display for Role {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::RoleUnspecified => f.write_str("ROLE_UNSPECIFIED"),
            Self::RoleUser => f.write_str("ROLE_USER"),
            Self::RoleAgent => f.write_str("ROLE_AGENT"),
        }
    }
}
impl ::std::str::FromStr for Role {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "ROLE_UNSPECIFIED" => Ok(Self::RoleUnspecified),
            "ROLE_USER" => Ok(Self::RoleUser),
            "ROLE_AGENT" => Ok(Self::RoleAgent),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for Role {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Role {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Role {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`Security`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Security\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"schemes\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#/definitions/StringList\""]
#[doc = "      },"]
#[doc = "      \"propertyNames\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Security {
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub schemes: ::std::collections::HashMap<::std::string::String, StringList>,
}
impl ::std::default::Default for Security {
    fn default() -> Self {
        Self {
            schemes: Default::default(),
        }
    }
}
impl Security {
    pub fn builder() -> builder::Security {
        Default::default()
    }
}
#[doc = "Defines a security scheme that can be used to secure an agent's endpoints.\n This is a discriminated union type based on the OpenAPI 3.2 Security Scheme Object.\n See: https://spec.openapis.org/oas/v3.2.0.html#security-scheme-object"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Security Scheme\","]
#[doc = "  \"description\": \"Defines a security scheme that can be used to secure an agent's endpoints.\\n This is a discriminated union type based on the OpenAPI 3.2 Security Scheme Object.\\n See: https://spec.openapis.org/oas/v3.2.0.html#security-scheme-object\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"apiKeySecurityScheme\": {"]
#[doc = "      \"description\": \"API key-based authentication.\","]
#[doc = "      \"$ref\": \"#/definitions/APIKeySecurityScheme\""]
#[doc = "    },"]
#[doc = "    \"httpAuthSecurityScheme\": {"]
#[doc = "      \"description\": \"HTTP authentication (Basic, Bearer, etc.).\","]
#[doc = "      \"$ref\": \"#/definitions/HTTPAuthSecurityScheme\""]
#[doc = "    },"]
#[doc = "    \"mtlsSecurityScheme\": {"]
#[doc = "      \"description\": \"Mutual TLS authentication.\","]
#[doc = "      \"$ref\": \"#/definitions/MutualTlsSecurityScheme\""]
#[doc = "    },"]
#[doc = "    \"oauth2SecurityScheme\": {"]
#[doc = "      \"description\": \"OAuth 2.0 authentication.\","]
#[doc = "      \"$ref\": \"#/definitions/OAuth2SecurityScheme\""]
#[doc = "    },"]
#[doc = "    \"openIdConnectSecurityScheme\": {"]
#[doc = "      \"description\": \"OpenID Connect authentication.\","]
#[doc = "      \"$ref\": \"#/definitions/OpenIdConnectSecurityScheme\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SecurityScheme {
    #[doc = "API key-based authentication."]
    #[serde(
        rename = "apiKeySecurityScheme",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub api_key_security_scheme: ::std::option::Option<ApiKeySecurityScheme>,
    #[doc = "HTTP authentication (Basic, Bearer, etc.)."]
    #[serde(
        rename = "httpAuthSecurityScheme",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub http_auth_security_scheme: ::std::option::Option<HttpAuthSecurityScheme>,
    #[doc = "Mutual TLS authentication."]
    #[serde(
        rename = "mtlsSecurityScheme",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub mtls_security_scheme: ::std::option::Option<MutualTlsSecurityScheme>,
    #[doc = "OAuth 2.0 authentication."]
    #[serde(
        rename = "oauth2SecurityScheme",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub oauth2_security_scheme: ::std::option::Option<OAuth2SecurityScheme>,
    #[doc = "OpenID Connect authentication."]
    #[serde(
        rename = "openIdConnectSecurityScheme",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub open_id_connect_security_scheme: ::std::option::Option<OpenIdConnectSecurityScheme>,
}
impl ::std::default::Default for SecurityScheme {
    fn default() -> Self {
        Self {
            api_key_security_scheme: Default::default(),
            http_auth_security_scheme: Default::default(),
            mtls_security_scheme: Default::default(),
            oauth2_security_scheme: Default::default(),
            open_id_connect_security_scheme: Default::default(),
        }
    }
}
impl SecurityScheme {
    pub fn builder() -> builder::SecurityScheme {
        Default::default()
    }
}
#[doc = "Configuration of a send message request."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Send Message Configuration\","]
#[doc = "  \"description\": \"Configuration of a send message request.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"blocking\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"acceptedOutputModes\": {"]
#[doc = "      \"description\": \"A list of media types the client is prepared to accept for response parts. Agents SHOULD use this to tailor their output.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"blocking\": {"]
#[doc = "      \"description\": \"If true, the operation waits until the task reaches a terminal state before returning. Default is false.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"historyLength\": {"]
#[doc = "      \"description\": \"The maximum number of messages to include in the history.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"maximum\": 2147483647.0,"]
#[doc = "      \"minimum\": -2147483648.0"]
#[doc = "    },"]
#[doc = "    \"pushNotificationConfig\": {"]
#[doc = "      \"description\": \"Configuration for the agent to send push notifications for task updates.\","]
#[doc = "      \"$ref\": \"#/definitions/PushNotificationConfig\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SendMessageConfiguration {
    #[doc = "A list of media types the client is prepared to accept for response parts. Agents SHOULD use this to tailor their output."]
    #[serde(
        rename = "acceptedOutputModes",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub accepted_output_modes: ::std::vec::Vec<::std::string::String>,
    #[doc = "If true, the operation waits until the task reaches a terminal state before returning. Default is false."]
    pub blocking: bool,
    #[doc = "The maximum number of messages to include in the history."]
    #[serde(
        rename = "historyLength",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub history_length: ::std::option::Option<i32>,
    #[doc = "Configuration for the agent to send push notifications for task updates."]
    #[serde(
        rename = "pushNotificationConfig",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub push_notification_config: ::std::option::Option<PushNotificationConfig>,
}
impl SendMessageConfiguration {
    pub fn builder() -> builder::SendMessageConfiguration {
        Default::default()
    }
}
#[doc = "/////////// Request Messages ///////////\n Represents a request for the `message/send` method."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Send Message Request\","]
#[doc = "  \"description\": \"/////////// Request Messages ///////////\\n Represents a request for the `message/send` method.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"tenant\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"configuration\": {"]
#[doc = "      \"description\": \"Configuration for the send request.\","]
#[doc = "      \"$ref\": \"#/definitions/SendMessageConfiguration\""]
#[doc = "    },"]
#[doc = "    \"message\": {"]
#[doc = "      \"description\": \"The message to send to the agent.\","]
#[doc = "      \"$ref\": \"#/definitions/Message\""]
#[doc = "    },"]
#[doc = "    \"metadata\": {"]
#[doc = "      \"description\": \"A flexible key-value map for passing additional context or parameters.\","]
#[doc = "      \"$ref\": \"#/definitions/Struct\""]
#[doc = "    },"]
#[doc = "    \"tenant\": {"]
#[doc = "      \"description\": \"Optional tenant, provided as a path parameter.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SendMessageRequest {
    #[doc = "Configuration for the send request."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub configuration: ::std::option::Option<SendMessageConfiguration>,
    #[doc = "The message to send to the agent."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub message: ::std::option::Option<Message>,
    #[doc = "A flexible key-value map for passing additional context or parameters."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<Struct>,
    #[doc = "Optional tenant, provided as a path parameter."]
    pub tenant: ::std::string::String,
}
impl SendMessageRequest {
    pub fn builder() -> builder::SendMessageRequest {
        Default::default()
    }
}
#[doc = "////// Response Messages ///////////"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Send Message Response\","]
#[doc = "  \"description\": \"////// Response Messages ///////////\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"message\": {"]
#[doc = "      \"$ref\": \"#/definitions/Message\""]
#[doc = "    },"]
#[doc = "    \"task\": {"]
#[doc = "      \"$ref\": \"#/definitions/Task\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SendMessageResponse {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub message: ::std::option::Option<Message>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub task: ::std::option::Option<Task>,
}
impl ::std::default::Default for SendMessageResponse {
    fn default() -> Self {
        Self {
            message: Default::default(),
            task: Default::default(),
        }
    }
}
impl SendMessageResponse {
    pub fn builder() -> builder::SendMessageResponse {
        Default::default()
    }
}
#[doc = "Represents a request for the `tasks/pushNotificationConfig/set` method."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Set Task Push Notification Config Request\","]
#[doc = "  \"description\": \"Represents a request for the `tasks/pushNotificationConfig/set` method.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"config\","]
#[doc = "    \"configId\","]
#[doc = "    \"parent\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"config\": {"]
#[doc = "      \"description\": \"The configuration to create.\","]
#[doc = "      \"$ref\": \"#/definitions/TaskPushNotificationConfig\""]
#[doc = "    },"]
#[doc = "    \"configId\": {"]
#[doc = "      \"description\": \"The ID for the new config.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"parent\": {"]
#[doc = "      \"description\": \"The parent task resource for this config.\\n Format: tasks/{task_id}\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"tenant\": {"]
#[doc = "      \"description\": \"Optional tenant, provided as a path parameter.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SetTaskPushNotificationConfigRequest {
    #[doc = "The configuration to create."]
    pub config: TaskPushNotificationConfig,
    #[doc = "The ID for the new config."]
    #[serde(rename = "configId")]
    pub config_id: ::std::string::String,
    #[doc = "The parent task resource for this config.\n Format: tasks/{task_id}"]
    pub parent: ::std::string::String,
    #[doc = "Optional tenant, provided as a path parameter."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tenant: ::std::option::Option<::std::string::String>,
}
impl SetTaskPushNotificationConfigRequest {
    pub fn builder() -> builder::SetTaskPushNotificationConfigRequest {
        Default::default()
    }
}
#[doc = "A wrapper object used in streaming operations to encapsulate different types of response data."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Stream Response\","]
#[doc = "  \"description\": \"A wrapper object used in streaming operations to encapsulate different types of response data.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"artifactUpdate\": {"]
#[doc = "      \"description\": \"An event indicating a task artifact update.\","]
#[doc = "      \"$ref\": \"#/definitions/TaskArtifactUpdateEvent\""]
#[doc = "    },"]
#[doc = "    \"message\": {"]
#[doc = "      \"description\": \"A Message object containing a message from the agent.\","]
#[doc = "      \"$ref\": \"#/definitions/Message\""]
#[doc = "    },"]
#[doc = "    \"statusUpdate\": {"]
#[doc = "      \"description\": \"An event indicating a task status update.\","]
#[doc = "      \"$ref\": \"#/definitions/TaskStatusUpdateEvent\""]
#[doc = "    },"]
#[doc = "    \"task\": {"]
#[doc = "      \"description\": \"A Task object containing the current state of the task.\","]
#[doc = "      \"$ref\": \"#/definitions/Task\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct StreamResponse {
    #[doc = "An event indicating a task artifact update."]
    #[serde(
        rename = "artifactUpdate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub artifact_update: ::std::option::Option<TaskArtifactUpdateEvent>,
    #[doc = "A Message object containing a message from the agent."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub message: ::std::option::Option<Message>,
    #[doc = "An event indicating a task status update."]
    #[serde(
        rename = "statusUpdate",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub status_update: ::std::option::Option<TaskStatusUpdateEvent>,
    #[doc = "A Task object containing the current state of the task."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub task: ::std::option::Option<Task>,
}
impl ::std::default::Default for StreamResponse {
    fn default() -> Self {
        Self {
            artifact_update: Default::default(),
            message: Default::default(),
            status_update: Default::default(),
            task: Default::default(),
        }
    }
}
impl StreamResponse {
    pub fn builder() -> builder::StreamResponse {
        Default::default()
    }
}
#[doc = "protolint:disable REPEATED_FIELD_NAMES_PLURALIZED"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"String List\","]
#[doc = "  \"description\": \"protolint:disable REPEATED_FIELD_NAMES_PLURALIZED\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"list\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct StringList {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub list: ::std::vec::Vec<::std::string::String>,
}
impl ::std::default::Default for StringList {
    fn default() -> Self {
        Self {
            list: Default::default(),
        }
    }
}
impl StringList {
    pub fn builder() -> builder::StringList {
        Default::default()
    }
}
#[doc = "`Struct`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Struct\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Struct(pub ::serde_json::Map<::std::string::String, ::serde_json::Value>);
impl ::std::ops::Deref for Struct {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<Struct>
    for ::serde_json::Map<::std::string::String, ::serde_json::Value>
{
    fn from(value: Struct) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
    for Struct
{
    fn from(value: ::serde_json::Map<::std::string::String, ::serde_json::Value>) -> Self {
        Self(value)
    }
}
#[doc = "`SubscribeToTaskRequest`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Subscribe To Task Request\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"tenant\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"The resource name of the task to subscribe to.\\n Format: tasks/{task_id}\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"tenant\": {"]
#[doc = "      \"description\": \"Optional tenant, provided as a path parameter.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SubscribeToTaskRequest {
    #[doc = "The resource name of the task to subscribe to.\n Format: tasks/{task_id}"]
    pub name: ::std::string::String,
    #[doc = "Optional tenant, provided as a path parameter."]
    pub tenant: ::std::string::String,
}
impl SubscribeToTaskRequest {
    pub fn builder() -> builder::SubscribeToTaskRequest {
        Default::default()
    }
}
#[doc = "Task is the core unit of action for A2A. It has a current status\n and when results are created for the task they are stored in the\n artifact. If there are multiple turns for a task, these are stored in\n history."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Task\","]
#[doc = "  \"description\": \"Task is the core unit of action for A2A. It has a current status\\n and when results are created for the task they are stored in the\\n artifact. If there are multiple turns for a task, these are stored in\\n history.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"contextId\","]
#[doc = "    \"id\","]
#[doc = "    \"status\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"artifacts\": {"]
#[doc = "      \"description\": \"A set of output artifacts for a Task.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Artifact\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"contextId\": {"]
#[doc = "      \"description\": \"Unique identifier (e.g. UUID) for the contextual collection of interactions\\n (tasks and messages). Created by the A2A server.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"history\": {"]
#[doc = "      \"description\": \"protolint:disable REPEATED_FIELD_NAMES_PLURALIZED\\n The history of interactions from a task.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/Message\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"description\": \"Unique identifier (e.g. UUID) for the task, generated by the server for a\\n new task.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"metadata\": {"]
#[doc = "      \"description\": \"protolint:enable REPEATED_FIELD_NAMES_PLURALIZED\\n A key/value object to store custom metadata about a task.\","]
#[doc = "      \"$ref\": \"#/definitions/Struct\""]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"description\": \"The current status of a Task, including state and a message.\","]
#[doc = "      \"$ref\": \"#/definitions/TaskStatus\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Task {
    #[doc = "A set of output artifacts for a Task."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub artifacts: ::std::vec::Vec<Artifact>,
    #[doc = "Unique identifier (e.g. UUID) for the contextual collection of interactions\n (tasks and messages). Created by the A2A server."]
    #[serde(rename = "contextId")]
    pub context_id: ::std::string::String,
    #[doc = "protolint:disable REPEATED_FIELD_NAMES_PLURALIZED\n The history of interactions from a task."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub history: ::std::vec::Vec<Message>,
    #[doc = "Unique identifier (e.g. UUID) for the task, generated by the server for a\n new task."]
    pub id: ::std::string::String,
    #[doc = "protolint:enable REPEATED_FIELD_NAMES_PLURALIZED\n A key/value object to store custom metadata about a task."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<Struct>,
    #[doc = "The current status of a Task, including state and a message."]
    pub status: TaskStatus,
}
impl Task {
    pub fn builder() -> builder::Task {
        Default::default()
    }
}
#[doc = "TaskArtifactUpdateEvent represents a task delta where an artifact has\n been generated."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Task Artifact Update Event\","]
#[doc = "  \"description\": \"TaskArtifactUpdateEvent represents a task delta where an artifact has\\n been generated.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"artifact\","]
#[doc = "    \"contextId\","]
#[doc = "    \"taskId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"append\": {"]
#[doc = "      \"description\": \"If true, the content of this artifact should be appended to a previously\\n sent artifact with the same ID.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"artifact\": {"]
#[doc = "      \"description\": \"The artifact that was generated or updated.\","]
#[doc = "      \"$ref\": \"#/definitions/Artifact\""]
#[doc = "    },"]
#[doc = "    \"contextId\": {"]
#[doc = "      \"description\": \"The id of the context that this task belongs to.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"lastChunk\": {"]
#[doc = "      \"description\": \"If true, this is the final chunk of the artifact.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"metadata\": {"]
#[doc = "      \"description\": \"Optional metadata associated with the artifact update.\","]
#[doc = "      \"$ref\": \"#/definitions/Struct\""]
#[doc = "    },"]
#[doc = "    \"taskId\": {"]
#[doc = "      \"description\": \"The id of the task for this artifact.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TaskArtifactUpdateEvent {
    #[doc = "If true, the content of this artifact should be appended to a previously\n sent artifact with the same ID."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub append: ::std::option::Option<bool>,
    #[doc = "The artifact that was generated or updated."]
    pub artifact: Artifact,
    #[doc = "The id of the context that this task belongs to."]
    #[serde(rename = "contextId")]
    pub context_id: ::std::string::String,
    #[doc = "If true, this is the final chunk of the artifact."]
    #[serde(
        rename = "lastChunk",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub last_chunk: ::std::option::Option<bool>,
    #[doc = "Optional metadata associated with the artifact update."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<Struct>,
    #[doc = "The id of the task for this artifact."]
    #[serde(rename = "taskId")]
    pub task_id: ::std::string::String,
}
impl TaskArtifactUpdateEvent {
    pub fn builder() -> builder::TaskArtifactUpdateEvent {
        Default::default()
    }
}
#[doc = "A container associating a push notification configuration with a specific\n task."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Task Push Notification Config\","]
#[doc = "  \"description\": \"A container associating a push notification configuration with a specific\\n task.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"pushNotificationConfig\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"The resource name of the config.\\n Format: tasks/{task_id}/pushNotificationConfigs/{config_id}\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"pushNotificationConfig\": {"]
#[doc = "      \"description\": \"The push notification configuration details.\","]
#[doc = "      \"$ref\": \"#/definitions/PushNotificationConfig\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TaskPushNotificationConfig {
    #[doc = "The resource name of the config.\n Format: tasks/{task_id}/pushNotificationConfigs/{config_id}"]
    pub name: ::std::string::String,
    #[doc = "The push notification configuration details."]
    #[serde(rename = "pushNotificationConfig")]
    pub push_notification_config: PushNotificationConfig,
}
impl TaskPushNotificationConfig {
    pub fn builder() -> builder::TaskPushNotificationConfig {
        Default::default()
    }
}
#[doc = "Filter tasks by their current status state."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Task State\","]
#[doc = "  \"description\": \"Filter tasks by their current status state.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"TASK_STATE_UNSPECIFIED\","]
#[doc = "    \"TASK_STATE_SUBMITTED\","]
#[doc = "    \"TASK_STATE_WORKING\","]
#[doc = "    \"TASK_STATE_COMPLETED\","]
#[doc = "    \"TASK_STATE_FAILED\","]
#[doc = "    \"TASK_STATE_CANCELLED\","]
#[doc = "    \"TASK_STATE_INPUT_REQUIRED\","]
#[doc = "    \"TASK_STATE_REJECTED\","]
#[doc = "    \"TASK_STATE_AUTH_REQUIRED\""]
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
    #[serde(rename = "TASK_STATE_UNSPECIFIED")]
    TaskStateUnspecified,
    #[serde(rename = "TASK_STATE_SUBMITTED")]
    TaskStateSubmitted,
    #[serde(rename = "TASK_STATE_WORKING")]
    TaskStateWorking,
    #[serde(rename = "TASK_STATE_COMPLETED")]
    TaskStateCompleted,
    #[serde(rename = "TASK_STATE_FAILED")]
    TaskStateFailed,
    #[serde(rename = "TASK_STATE_CANCELLED")]
    TaskStateCancelled,
    #[serde(rename = "TASK_STATE_INPUT_REQUIRED")]
    TaskStateInputRequired,
    #[serde(rename = "TASK_STATE_REJECTED")]
    TaskStateRejected,
    #[serde(rename = "TASK_STATE_AUTH_REQUIRED")]
    TaskStateAuthRequired,
}
impl ::std::fmt::Display for TaskState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::TaskStateUnspecified => f.write_str("TASK_STATE_UNSPECIFIED"),
            Self::TaskStateSubmitted => f.write_str("TASK_STATE_SUBMITTED"),
            Self::TaskStateWorking => f.write_str("TASK_STATE_WORKING"),
            Self::TaskStateCompleted => f.write_str("TASK_STATE_COMPLETED"),
            Self::TaskStateFailed => f.write_str("TASK_STATE_FAILED"),
            Self::TaskStateCancelled => f.write_str("TASK_STATE_CANCELLED"),
            Self::TaskStateInputRequired => f.write_str("TASK_STATE_INPUT_REQUIRED"),
            Self::TaskStateRejected => f.write_str("TASK_STATE_REJECTED"),
            Self::TaskStateAuthRequired => f.write_str("TASK_STATE_AUTH_REQUIRED"),
        }
    }
}
impl ::std::str::FromStr for TaskState {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "TASK_STATE_UNSPECIFIED" => Ok(Self::TaskStateUnspecified),
            "TASK_STATE_SUBMITTED" => Ok(Self::TaskStateSubmitted),
            "TASK_STATE_WORKING" => Ok(Self::TaskStateWorking),
            "TASK_STATE_COMPLETED" => Ok(Self::TaskStateCompleted),
            "TASK_STATE_FAILED" => Ok(Self::TaskStateFailed),
            "TASK_STATE_CANCELLED" => Ok(Self::TaskStateCancelled),
            "TASK_STATE_INPUT_REQUIRED" => Ok(Self::TaskStateInputRequired),
            "TASK_STATE_REJECTED" => Ok(Self::TaskStateRejected),
            "TASK_STATE_AUTH_REQUIRED" => Ok(Self::TaskStateAuthRequired),
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
#[doc = "A container for the status of a task"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Task Status\","]
#[doc = "  \"description\": \"A container for the status of a task\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"state\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"message\": {"]
#[doc = "      \"description\": \"A message associated with the status.\","]
#[doc = "      \"$ref\": \"#/definitions/Message\""]
#[doc = "    },"]
#[doc = "    \"state\": {"]
#[doc = "      \"title\": \"Task State\","]
#[doc = "      \"description\": \"The current state of this task.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"TASK_STATE_UNSPECIFIED\","]
#[doc = "        \"TASK_STATE_SUBMITTED\","]
#[doc = "        \"TASK_STATE_WORKING\","]
#[doc = "        \"TASK_STATE_COMPLETED\","]
#[doc = "        \"TASK_STATE_FAILED\","]
#[doc = "        \"TASK_STATE_CANCELLED\","]
#[doc = "        \"TASK_STATE_INPUT_REQUIRED\","]
#[doc = "        \"TASK_STATE_REJECTED\","]
#[doc = "        \"TASK_STATE_AUTH_REQUIRED\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"timestamp\": {"]
#[doc = "      \"description\": \"ISO 8601 Timestamp when the status was recorded.\\n Example: \\\"2023-10-27T10:00:00Z\\\"\","]
#[doc = "      \"$ref\": \"#/definitions/Timestamp\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TaskStatus {
    #[doc = "A message associated with the status."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub message: ::std::option::Option<Message>,
    #[doc = "The current state of this task."]
    pub state: TaskState,
    #[doc = "ISO 8601 Timestamp when the status was recorded.\n Example: \"2023-10-27T10:00:00Z\""]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub timestamp: ::std::option::Option<Timestamp>,
}
impl TaskStatus {
    pub fn builder() -> builder::TaskStatus {
        Default::default()
    }
}
#[doc = "An event sent by the agent to notify the client of a change in a task's\n status."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Task Status Update Event\","]
#[doc = "  \"description\": \"An event sent by the agent to notify the client of a change in a task's\\n status.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"contextId\","]
#[doc = "    \"final\","]
#[doc = "    \"status\","]
#[doc = "    \"taskId\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"contextId\": {"]
#[doc = "      \"description\": \"The id of the context that the task belongs to\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"final\": {"]
#[doc = "      \"description\": \"If true, this is the final event in the stream for this interaction.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"metadata\": {"]
#[doc = "      \"description\": \"Optional metadata to associate with the task update.\","]
#[doc = "      \"$ref\": \"#/definitions/Struct\""]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"description\": \"The new status of the task.\","]
#[doc = "      \"$ref\": \"#/definitions/TaskStatus\""]
#[doc = "    },"]
#[doc = "    \"taskId\": {"]
#[doc = "      \"description\": \"The id of the task that is changed\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TaskStatusUpdateEvent {
    #[doc = "The id of the context that the task belongs to"]
    #[serde(rename = "contextId")]
    pub context_id: ::std::string::String,
    #[doc = "If true, this is the final event in the stream for this interaction."]
    #[serde(rename = "final")]
    pub final_: bool,
    #[doc = "Optional metadata to associate with the task update."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<Struct>,
    #[doc = "The new status of the task."]
    pub status: TaskStatus,
    #[doc = "The id of the task that is changed"]
    #[serde(rename = "taskId")]
    pub task_id: ::std::string::String,
}
impl TaskStatusUpdateEvent {
    pub fn builder() -> builder::TaskStatusUpdateEvent {
        Default::default()
    }
}
#[doc = "`Timestamp`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Timestamp\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"format\": \"date-time\","]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Timestamp(pub ::chrono::DateTime<::chrono::offset::Utc>);
impl ::std::ops::Deref for Timestamp {
    type Target = ::chrono::DateTime<::chrono::offset::Utc>;
    fn deref(&self) -> &::chrono::DateTime<::chrono::offset::Utc> {
        &self.0
    }
}
impl ::std::convert::From<Timestamp> for ::chrono::DateTime<::chrono::offset::Utc> {
    fn from(value: Timestamp) -> Self {
        value.0
    }
}
impl ::std::convert::From<::chrono::DateTime<::chrono::offset::Utc>> for Timestamp {
    fn from(value: ::chrono::DateTime<::chrono::offset::Utc>) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for Timestamp {
    type Err = <::chrono::DateTime<::chrono::offset::Utc> as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for Timestamp {
    type Error = <::chrono::DateTime<::chrono::offset::Utc> as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for Timestamp {
    type Error = <::chrono::DateTime<::chrono::offset::Utc> as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
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
                .map_err(|e| format!("error converting supplied value for extensions: {e}"));
            self
        }
        pub fn push_notifications<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.push_notifications = value.try_into().map_err(|e| {
                format!("error converting supplied value for push_notifications: {e}")
            });
            self
        }
        pub fn state_transition_history<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.state_transition_history = value.try_into().map_err(|e| {
                format!("error converting supplied value for state_transition_history: {e}")
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
                .map_err(|e| format!("error converting supplied value for streaming: {e}"));
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
        preferred_transport: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        protocol_version: ::std::result::Result<::std::string::String, ::std::string::String>,
        provider: ::std::result::Result<
            ::std::option::Option<super::AgentProvider>,
            ::std::string::String,
        >,
        security: ::std::result::Result<::std::vec::Vec<super::Security>, ::std::string::String>,
        security_schemes: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, super::SecurityScheme>,
            ::std::string::String,
        >,
        signatures: ::std::result::Result<
            ::std::vec::Vec<super::AgentCardSignature>,
            ::std::string::String,
        >,
        skills: ::std::result::Result<::std::vec::Vec<super::AgentSkill>, ::std::string::String>,
        supported_interfaces:
            ::std::result::Result<::std::vec::Vec<super::AgentInterface>, ::std::string::String>,
        supports_extended_agent_card:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        url: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
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
                preferred_transport: Ok(Default::default()),
                protocol_version: Err("no value supplied for protocol_version".to_string()),
                provider: Ok(Default::default()),
                security: Ok(Default::default()),
                security_schemes: Ok(Default::default()),
                signatures: Ok(Default::default()),
                skills: Err("no value supplied for skills".to_string()),
                supported_interfaces: Ok(Default::default()),
                supports_extended_agent_card: Ok(Default::default()),
                url: Ok(Default::default()),
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
                format!("error converting supplied value for additional_interfaces: {e}")
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
                .map_err(|e| format!("error converting supplied value for capabilities: {e}"));
            self
        }
        pub fn default_input_modes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.default_input_modes = value.try_into().map_err(|e| {
                format!("error converting supplied value for default_input_modes: {e}")
            });
            self
        }
        pub fn default_output_modes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.default_output_modes = value.try_into().map_err(|e| {
                format!("error converting supplied value for default_output_modes: {e}")
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
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn documentation_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.documentation_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for documentation_url: {e}"));
            self
        }
        pub fn icon_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.icon_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for icon_url: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn preferred_transport<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.preferred_transport = value.try_into().map_err(|e| {
                format!("error converting supplied value for preferred_transport: {e}")
            });
            self
        }
        pub fn protocol_version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.protocol_version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for protocol_version: {e}"));
            self
        }
        pub fn provider<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::AgentProvider>>,
            T::Error: ::std::fmt::Display,
        {
            self.provider = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for provider: {e}"));
            self
        }
        pub fn security<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Security>>,
            T::Error: ::std::fmt::Display,
        {
            self.security = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for security: {e}"));
            self
        }
        pub fn security_schemes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::collections::HashMap<::std::string::String, super::SecurityScheme>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.security_schemes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for security_schemes: {e}"));
            self
        }
        pub fn signatures<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::AgentCardSignature>>,
            T::Error: ::std::fmt::Display,
        {
            self.signatures = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for signatures: {e}"));
            self
        }
        pub fn skills<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::AgentSkill>>,
            T::Error: ::std::fmt::Display,
        {
            self.skills = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for skills: {e}"));
            self
        }
        pub fn supported_interfaces<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::AgentInterface>>,
            T::Error: ::std::fmt::Display,
        {
            self.supported_interfaces = value.try_into().map_err(|e| {
                format!("error converting supplied value for supported_interfaces: {e}")
            });
            self
        }
        pub fn supports_extended_agent_card<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.supports_extended_agent_card = value.try_into().map_err(|e| {
                format!("error converting supplied value for supports_extended_agent_card: {e}")
            });
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {e}"));
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for version: {e}"));
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
                supported_interfaces: value.supported_interfaces?,
                supports_extended_agent_card: value.supports_extended_agent_card?,
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
                supported_interfaces: Ok(value.supported_interfaces),
                supports_extended_agent_card: Ok(value.supports_extended_agent_card),
                url: Ok(value.url),
                version: Ok(value.version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AgentCardSignature {
        header: ::std::result::Result<::std::option::Option<super::Struct>, ::std::string::String>,
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
            T: ::std::convert::TryInto<::std::option::Option<super::Struct>>,
            T::Error: ::std::fmt::Display,
        {
            self.header = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for header: {e}"));
            self
        }
        pub fn protected<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.protected = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for protected: {e}"));
            self
        }
        pub fn signature<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.signature = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for signature: {e}"));
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
        description: ::std::result::Result<::std::string::String, ::std::string::String>,
        params: ::std::result::Result<::std::option::Option<super::Struct>, ::std::string::String>,
        required: ::std::result::Result<bool, ::std::string::String>,
        uri: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for AgentExtension {
        fn default() -> Self {
            Self {
                description: Err("no value supplied for description".to_string()),
                params: Ok(Default::default()),
                required: Err("no value supplied for required".to_string()),
                uri: Err("no value supplied for uri".to_string()),
            }
        }
    }
    impl AgentExtension {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn params<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Struct>>,
            T::Error: ::std::fmt::Display,
        {
            self.params = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for params: {e}"));
            self
        }
        pub fn required<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.required = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for required: {e}"));
            self
        }
        pub fn uri<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.uri = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for uri: {e}"));
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
        protocol_binding: ::std::result::Result<::std::string::String, ::std::string::String>,
        tenant: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        url: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for AgentInterface {
        fn default() -> Self {
            Self {
                protocol_binding: Err("no value supplied for protocol_binding".to_string()),
                tenant: Ok(Default::default()),
                url: Err("no value supplied for url".to_string()),
            }
        }
    }
    impl AgentInterface {
        pub fn protocol_binding<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.protocol_binding = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for protocol_binding: {e}"));
            self
        }
        pub fn tenant<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.tenant = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tenant: {e}"));
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AgentInterface> for super::AgentInterface {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AgentInterface,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                protocol_binding: value.protocol_binding?,
                tenant: value.tenant?,
                url: value.url?,
            })
        }
    }
    impl ::std::convert::From<super::AgentInterface> for AgentInterface {
        fn from(value: super::AgentInterface) -> Self {
            Self {
                protocol_binding: Ok(value.protocol_binding),
                tenant: Ok(value.tenant),
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
                .map_err(|e| format!("error converting supplied value for organization: {e}"));
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {e}"));
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
        security: ::std::result::Result<::std::vec::Vec<super::Security>, ::std::string::String>,
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
                security: Ok(Default::default()),
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
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn examples<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.examples = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for examples: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn input_modes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.input_modes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for input_modes: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn output_modes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.output_modes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for output_modes: {e}"));
            self
        }
        pub fn security<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Security>>,
            T::Error: ::std::fmt::Display,
        {
            self.security = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for security: {e}"));
            self
        }
        pub fn tags<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.tags = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tags: {e}"));
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
                security: value.security?,
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
                security: Ok(value.security),
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
        location: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ApiKeySecurityScheme {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                location: Err("no value supplied for location".to_string()),
                name: Err("no value supplied for name".to_string()),
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
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn location<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.location = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for location: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
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
                location: value.location?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::ApiKeySecurityScheme> for ApiKeySecurityScheme {
        fn from(value: super::ApiKeySecurityScheme) -> Self {
            Self {
                description: Ok(value.description),
                location: Ok(value.location),
                name: Ok(value.name),
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
        metadata:
            ::std::result::Result<::std::option::Option<super::Struct>, ::std::string::String>,
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
                .map_err(|e| format!("error converting supplied value for artifact_id: {e}"));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn extensions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.extensions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extensions: {e}"));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Struct>>,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn parts<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Part>>,
            T::Error: ::std::fmt::Display,
        {
            self.parts = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for parts: {e}"));
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
    pub struct AuthenticationInfo {
        credentials: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        schemes:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
    }
    impl ::std::default::Default for AuthenticationInfo {
        fn default() -> Self {
            Self {
                credentials: Ok(Default::default()),
                schemes: Err("no value supplied for schemes".to_string()),
            }
        }
    }
    impl AuthenticationInfo {
        pub fn credentials<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.credentials = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for credentials: {e}"));
            self
        }
        pub fn schemes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.schemes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for schemes: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<AuthenticationInfo> for super::AuthenticationInfo {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuthenticationInfo,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                credentials: value.credentials?,
                schemes: value.schemes?,
            })
        }
    }
    impl ::std::convert::From<super::AuthenticationInfo> for AuthenticationInfo {
        fn from(value: super::AuthenticationInfo) -> Self {
            Self {
                credentials: Ok(value.credentials),
                schemes: Ok(value.schemes),
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
            self.authorization_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for authorization_url: {e}"));
            self
        }
        pub fn refresh_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.refresh_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for refresh_url: {e}"));
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
                .map_err(|e| format!("error converting supplied value for scopes: {e}"));
            self
        }
        pub fn token_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.token_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for token_url: {e}"));
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
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        tenant: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CancelTaskRequest {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                tenant: Err("no value supplied for tenant".to_string()),
            }
        }
    }
    impl CancelTaskRequest {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn tenant<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.tenant = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tenant: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CancelTaskRequest> for super::CancelTaskRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CancelTaskRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                tenant: value.tenant?,
            })
        }
    }
    impl ::std::convert::From<super::CancelTaskRequest> for CancelTaskRequest {
        fn from(value: super::CancelTaskRequest) -> Self {
            Self {
                name: Ok(value.name),
                tenant: Ok(value.tenant),
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
                .map_err(|e| format!("error converting supplied value for refresh_url: {e}"));
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
                .map_err(|e| format!("error converting supplied value for scopes: {e}"));
            self
        }
        pub fn token_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.token_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for token_url: {e}"));
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
    pub struct DataPart {
        data: ::std::result::Result<super::Struct, ::std::string::String>,
    }
    impl ::std::default::Default for DataPart {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
            }
        }
    }
    impl DataPart {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Struct>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<DataPart> for super::DataPart {
        type Error = super::error::ConversionError;
        fn try_from(value: DataPart) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { data: value.data? })
        }
    }
    impl ::std::convert::From<super::DataPart> for DataPart {
        fn from(value: super::DataPart) -> Self {
            Self {
                data: Ok(value.data),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeleteTaskPushNotificationConfigRequest {
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        tenant: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for DeleteTaskPushNotificationConfigRequest {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                tenant: Err("no value supplied for tenant".to_string()),
            }
        }
    }
    impl DeleteTaskPushNotificationConfigRequest {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn tenant<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.tenant = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tenant: {e}"));
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
                name: value.name?,
                tenant: value.tenant?,
            })
        }
    }
    impl ::std::convert::From<super::DeleteTaskPushNotificationConfigRequest>
        for DeleteTaskPushNotificationConfigRequest
    {
        fn from(value: super::DeleteTaskPushNotificationConfigRequest) -> Self {
            Self {
                name: Ok(value.name),
                tenant: Ok(value.tenant),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FilePart {
        file_with_bytes: ::std::result::Result<
            ::std::option::Option<super::FilePartFileWithBytes>,
            ::std::string::String,
        >,
        file_with_uri: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        media_type: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for FilePart {
        fn default() -> Self {
            Self {
                file_with_bytes: Ok(Default::default()),
                file_with_uri: Ok(Default::default()),
                media_type: Err("no value supplied for media_type".to_string()),
                name: Err("no value supplied for name".to_string()),
            }
        }
    }
    impl FilePart {
        pub fn file_with_bytes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::FilePartFileWithBytes>>,
            T::Error: ::std::fmt::Display,
        {
            self.file_with_bytes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_with_bytes: {e}"));
            self
        }
        pub fn file_with_uri<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.file_with_uri = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file_with_uri: {e}"));
            self
        }
        pub fn media_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.media_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for media_type: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<FilePart> for super::FilePart {
        type Error = super::error::ConversionError;
        fn try_from(value: FilePart) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                file_with_bytes: value.file_with_bytes?,
                file_with_uri: value.file_with_uri?,
                media_type: value.media_type?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::FilePart> for FilePart {
        fn from(value: super::FilePart) -> Self {
            Self {
                file_with_bytes: Ok(value.file_with_bytes),
                file_with_uri: Ok(value.file_with_uri),
                media_type: Ok(value.media_type),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GetExtendedAgentCardRequest {
        tenant: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for GetExtendedAgentCardRequest {
        fn default() -> Self {
            Self {
                tenant: Err("no value supplied for tenant".to_string()),
            }
        }
    }
    impl GetExtendedAgentCardRequest {
        pub fn tenant<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.tenant = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tenant: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GetExtendedAgentCardRequest> for super::GetExtendedAgentCardRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GetExtendedAgentCardRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                tenant: value.tenant?,
            })
        }
    }
    impl ::std::convert::From<super::GetExtendedAgentCardRequest> for GetExtendedAgentCardRequest {
        fn from(value: super::GetExtendedAgentCardRequest) -> Self {
            Self {
                tenant: Ok(value.tenant),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GetTaskPushNotificationConfigRequest {
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        tenant: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for GetTaskPushNotificationConfigRequest {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                tenant: Err("no value supplied for tenant".to_string()),
            }
        }
    }
    impl GetTaskPushNotificationConfigRequest {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn tenant<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.tenant = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tenant: {e}"));
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
                name: value.name?,
                tenant: value.tenant?,
            })
        }
    }
    impl ::std::convert::From<super::GetTaskPushNotificationConfigRequest>
        for GetTaskPushNotificationConfigRequest
    {
        fn from(value: super::GetTaskPushNotificationConfigRequest) -> Self {
            Self {
                name: Ok(value.name),
                tenant: Ok(value.tenant),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GetTaskRequest {
        history_length: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        tenant: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for GetTaskRequest {
        fn default() -> Self {
            Self {
                history_length: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                tenant: Ok(Default::default()),
            }
        }
    }
    impl GetTaskRequest {
        pub fn history_length<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.history_length = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for history_length: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn tenant<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.tenant = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tenant: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<GetTaskRequest> for super::GetTaskRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GetTaskRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                history_length: value.history_length?,
                name: value.name?,
                tenant: value.tenant?,
            })
        }
    }
    impl ::std::convert::From<super::GetTaskRequest> for GetTaskRequest {
        fn from(value: super::GetTaskRequest) -> Self {
            Self {
                history_length: Ok(value.history_length),
                name: Ok(value.name),
                tenant: Ok(value.tenant),
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
    }
    impl ::std::default::Default for HttpAuthSecurityScheme {
        fn default() -> Self {
            Self {
                bearer_format: Ok(Default::default()),
                description: Ok(Default::default()),
                scheme: Err("no value supplied for scheme".to_string()),
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
                .map_err(|e| format!("error converting supplied value for bearer_format: {e}"));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn scheme<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.scheme = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for scheme: {e}"));
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
            })
        }
    }
    impl ::std::convert::From<super::HttpAuthSecurityScheme> for HttpAuthSecurityScheme {
        fn from(value: super::HttpAuthSecurityScheme) -> Self {
            Self {
                bearer_format: Ok(value.bearer_format),
                description: Ok(value.description),
                scheme: Ok(value.scheme),
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
            self.authorization_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for authorization_url: {e}"));
            self
        }
        pub fn refresh_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.refresh_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for refresh_url: {e}"));
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
                .map_err(|e| format!("error converting supplied value for scopes: {e}"));
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
    pub struct ListTaskPushNotificationConfigRequest {
        page_size: ::std::result::Result<i32, ::std::string::String>,
        page_token: ::std::result::Result<::std::string::String, ::std::string::String>,
        parent: ::std::result::Result<::std::string::String, ::std::string::String>,
        tenant: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ListTaskPushNotificationConfigRequest {
        fn default() -> Self {
            Self {
                page_size: Err("no value supplied for page_size".to_string()),
                page_token: Err("no value supplied for page_token".to_string()),
                parent: Err("no value supplied for parent".to_string()),
                tenant: Err("no value supplied for tenant".to_string()),
            }
        }
    }
    impl ListTaskPushNotificationConfigRequest {
        pub fn page_size<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i32>,
            T::Error: ::std::fmt::Display,
        {
            self.page_size = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for page_size: {e}"));
            self
        }
        pub fn page_token<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.page_token = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for page_token: {e}"));
            self
        }
        pub fn parent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.parent = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for parent: {e}"));
            self
        }
        pub fn tenant<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.tenant = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tenant: {e}"));
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
                page_size: value.page_size?,
                page_token: value.page_token?,
                parent: value.parent?,
                tenant: value.tenant?,
            })
        }
    }
    impl ::std::convert::From<super::ListTaskPushNotificationConfigRequest>
        for ListTaskPushNotificationConfigRequest
    {
        fn from(value: super::ListTaskPushNotificationConfigRequest) -> Self {
            Self {
                page_size: Ok(value.page_size),
                page_token: Ok(value.page_token),
                parent: Ok(value.parent),
                tenant: Ok(value.tenant),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListTaskPushNotificationConfigResponse {
        configs: ::std::result::Result<
            ::std::vec::Vec<super::TaskPushNotificationConfig>,
            ::std::string::String,
        >,
        next_page_token: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ListTaskPushNotificationConfigResponse {
        fn default() -> Self {
            Self {
                configs: Ok(Default::default()),
                next_page_token: Err("no value supplied for next_page_token".to_string()),
            }
        }
    }
    impl ListTaskPushNotificationConfigResponse {
        pub fn configs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::TaskPushNotificationConfig>>,
            T::Error: ::std::fmt::Display,
        {
            self.configs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for configs: {e}"));
            self
        }
        pub fn next_page_token<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.next_page_token = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for next_page_token: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListTaskPushNotificationConfigResponse>
        for super::ListTaskPushNotificationConfigResponse
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListTaskPushNotificationConfigResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                configs: value.configs?,
                next_page_token: value.next_page_token?,
            })
        }
    }
    impl ::std::convert::From<super::ListTaskPushNotificationConfigResponse>
        for ListTaskPushNotificationConfigResponse
    {
        fn from(value: super::ListTaskPushNotificationConfigResponse) -> Self {
            Self {
                configs: Ok(value.configs),
                next_page_token: Ok(value.next_page_token),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListTasksRequest {
        context_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        history_length: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        include_artifacts:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        last_updated_after: ::std::result::Result<i64, ::std::string::String>,
        page_size: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        page_token: ::std::result::Result<::std::string::String, ::std::string::String>,
        status: ::std::result::Result<super::TaskState, ::std::string::String>,
        tenant: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ListTasksRequest {
        fn default() -> Self {
            Self {
                context_id: Err("no value supplied for context_id".to_string()),
                history_length: Ok(Default::default()),
                include_artifacts: Ok(Default::default()),
                last_updated_after: Err("no value supplied for last_updated_after".to_string()),
                page_size: Ok(Default::default()),
                page_token: Err("no value supplied for page_token".to_string()),
                status: Err("no value supplied for status".to_string()),
                tenant: Err("no value supplied for tenant".to_string()),
            }
        }
    }
    impl ListTasksRequest {
        pub fn context_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.context_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for context_id: {e}"));
            self
        }
        pub fn history_length<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.history_length = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for history_length: {e}"));
            self
        }
        pub fn include_artifacts<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.include_artifacts = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for include_artifacts: {e}"));
            self
        }
        pub fn last_updated_after<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.last_updated_after = value.try_into().map_err(|e| {
                format!("error converting supplied value for last_updated_after: {e}")
            });
            self
        }
        pub fn page_size<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.page_size = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for page_size: {e}"));
            self
        }
        pub fn page_token<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.page_token = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for page_token: {e}"));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TaskState>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {e}"));
            self
        }
        pub fn tenant<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.tenant = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tenant: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListTasksRequest> for super::ListTasksRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListTasksRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                context_id: value.context_id?,
                history_length: value.history_length?,
                include_artifacts: value.include_artifacts?,
                last_updated_after: value.last_updated_after?,
                page_size: value.page_size?,
                page_token: value.page_token?,
                status: value.status?,
                tenant: value.tenant?,
            })
        }
    }
    impl ::std::convert::From<super::ListTasksRequest> for ListTasksRequest {
        fn from(value: super::ListTasksRequest) -> Self {
            Self {
                context_id: Ok(value.context_id),
                history_length: Ok(value.history_length),
                include_artifacts: Ok(value.include_artifacts),
                last_updated_after: Ok(value.last_updated_after),
                page_size: Ok(value.page_size),
                page_token: Ok(value.page_token),
                status: Ok(value.status),
                tenant: Ok(value.tenant),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListTasksResponse {
        next_page_token: ::std::result::Result<::std::string::String, ::std::string::String>,
        page_size: ::std::result::Result<i32, ::std::string::String>,
        tasks: ::std::result::Result<::std::vec::Vec<super::Task>, ::std::string::String>,
        total_size: ::std::result::Result<i32, ::std::string::String>,
    }
    impl ::std::default::Default for ListTasksResponse {
        fn default() -> Self {
            Self {
                next_page_token: Err("no value supplied for next_page_token".to_string()),
                page_size: Err("no value supplied for page_size".to_string()),
                tasks: Err("no value supplied for tasks".to_string()),
                total_size: Err("no value supplied for total_size".to_string()),
            }
        }
    }
    impl ListTasksResponse {
        pub fn next_page_token<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.next_page_token = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for next_page_token: {e}"));
            self
        }
        pub fn page_size<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i32>,
            T::Error: ::std::fmt::Display,
        {
            self.page_size = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for page_size: {e}"));
            self
        }
        pub fn tasks<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Task>>,
            T::Error: ::std::fmt::Display,
        {
            self.tasks = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tasks: {e}"));
            self
        }
        pub fn total_size<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i32>,
            T::Error: ::std::fmt::Display,
        {
            self.total_size = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for total_size: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListTasksResponse> for super::ListTasksResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListTasksResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                next_page_token: value.next_page_token?,
                page_size: value.page_size?,
                tasks: value.tasks?,
                total_size: value.total_size?,
            })
        }
    }
    impl ::std::convert::From<super::ListTasksResponse> for ListTasksResponse {
        fn from(value: super::ListTasksResponse) -> Self {
            Self {
                next_page_token: Ok(value.next_page_token),
                page_size: Ok(value.page_size),
                tasks: Ok(value.tasks),
                total_size: Ok(value.total_size),
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
        message_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        metadata:
            ::std::result::Result<::std::option::Option<super::Struct>, ::std::string::String>,
        parts: ::std::result::Result<::std::vec::Vec<super::Part>, ::std::string::String>,
        reference_task_ids:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        role: ::std::result::Result<super::Role, ::std::string::String>,
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
                .map_err(|e| format!("error converting supplied value for context_id: {e}"));
            self
        }
        pub fn extensions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.extensions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extensions: {e}"));
            self
        }
        pub fn message_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.message_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message_id: {e}"));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Struct>>,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {e}"));
            self
        }
        pub fn parts<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Part>>,
            T::Error: ::std::fmt::Display,
        {
            self.parts = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for parts: {e}"));
            self
        }
        pub fn reference_task_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.reference_task_ids = value.try_into().map_err(|e| {
                format!("error converting supplied value for reference_task_ids: {e}")
            });
            self
        }
        pub fn role<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Role>,
            T::Error: ::std::fmt::Display,
        {
            self.role = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role: {e}"));
            self
        }
        pub fn task_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.task_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for task_id: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Message> for super::Message {
        type Error = super::error::ConversionError;
        fn try_from(value: Message) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                context_id: value.context_id?,
                extensions: value.extensions?,
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
    pub struct MutualTlsSecurityScheme {
        description: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for MutualTlsSecurityScheme {
        fn default() -> Self {
            Self {
                description: Err("no value supplied for description".to_string()),
            }
        }
    }
    impl MutualTlsSecurityScheme {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<MutualTlsSecurityScheme> for super::MutualTlsSecurityScheme {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MutualTlsSecurityScheme,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
            })
        }
    }
    impl ::std::convert::From<super::MutualTlsSecurityScheme> for MutualTlsSecurityScheme {
        fn from(value: super::MutualTlsSecurityScheme) -> Self {
            Self {
                description: Ok(value.description),
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
        oauth2_metadata_url: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for OAuth2SecurityScheme {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                flows: Err("no value supplied for flows".to_string()),
                oauth2_metadata_url: Ok(Default::default()),
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
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn flows<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::OAuthFlows>,
            T::Error: ::std::fmt::Display,
        {
            self.flows = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for flows: {e}"));
            self
        }
        pub fn oauth2_metadata_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.oauth2_metadata_url = value.try_into().map_err(|e| {
                format!("error converting supplied value for oauth2_metadata_url: {e}")
            });
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
                oauth2_metadata_url: value.oauth2_metadata_url?,
            })
        }
    }
    impl ::std::convert::From<super::OAuth2SecurityScheme> for OAuth2SecurityScheme {
        fn from(value: super::OAuth2SecurityScheme) -> Self {
            Self {
                description: Ok(value.description),
                flows: Ok(value.flows),
                oauth2_metadata_url: Ok(value.oauth2_metadata_url),
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
                format!("error converting supplied value for authorization_code: {e}")
            });
            self
        }
        pub fn client_credentials<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ClientCredentialsOAuthFlow>>,
            T::Error: ::std::fmt::Display,
        {
            self.client_credentials = value.try_into().map_err(|e| {
                format!("error converting supplied value for client_credentials: {e}")
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
                .map_err(|e| format!("error converting supplied value for implicit: {e}"));
            self
        }
        pub fn password<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PasswordOAuthFlow>>,
            T::Error: ::std::fmt::Display,
        {
            self.password = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for password: {e}"));
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
    }
    impl ::std::default::Default for OpenIdConnectSecurityScheme {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                open_id_connect_url: Err("no value supplied for open_id_connect_url".to_string()),
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
                .map_err(|e| format!("error converting supplied value for description: {e}"));
            self
        }
        pub fn open_id_connect_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.open_id_connect_url = value.try_into().map_err(|e| {
                format!("error converting supplied value for open_id_connect_url: {e}")
            });
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
            })
        }
    }
    impl ::std::convert::From<super::OpenIdConnectSecurityScheme> for OpenIdConnectSecurityScheme {
        fn from(value: super::OpenIdConnectSecurityScheme) -> Self {
            Self {
                description: Ok(value.description),
                open_id_connect_url: Ok(value.open_id_connect_url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Part {
        data: ::std::result::Result<::std::option::Option<super::DataPart>, ::std::string::String>,
        file: ::std::result::Result<::std::option::Option<super::FilePart>, ::std::string::String>,
        metadata:
            ::std::result::Result<::std::option::Option<super::Struct>, ::std::string::String>,
        text: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Part {
        fn default() -> Self {
            Self {
                data: Ok(Default::default()),
                file: Ok(Default::default()),
                metadata: Ok(Default::default()),
                text: Ok(Default::default()),
            }
        }
    }
    impl Part {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::DataPart>>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
            self
        }
        pub fn file<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::FilePart>>,
            T::Error: ::std::fmt::Display,
        {
            self.file = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for file: {e}"));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Struct>>,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {e}"));
            self
        }
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Part> for super::Part {
        type Error = super::error::ConversionError;
        fn try_from(value: Part) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                file: value.file?,
                metadata: value.metadata?,
                text: value.text?,
            })
        }
    }
    impl ::std::convert::From<super::Part> for Part {
        fn from(value: super::Part) -> Self {
            Self {
                data: Ok(value.data),
                file: Ok(value.file),
                metadata: Ok(value.metadata),
                text: Ok(value.text),
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
                .map_err(|e| format!("error converting supplied value for refresh_url: {e}"));
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
                .map_err(|e| format!("error converting supplied value for scopes: {e}"));
            self
        }
        pub fn token_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.token_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for token_url: {e}"));
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
    pub struct PushNotificationConfig {
        authentication: ::std::result::Result<
            ::std::option::Option<super::AuthenticationInfo>,
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
            T: ::std::convert::TryInto<::std::option::Option<super::AuthenticationInfo>>,
            T::Error: ::std::fmt::Display,
        {
            self.authentication = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for authentication: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn token<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.token = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for token: {e}"));
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {e}"));
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
    pub struct Security {
        schemes: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, super::StringList>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Security {
        fn default() -> Self {
            Self {
                schemes: Ok(Default::default()),
            }
        }
    }
    impl Security {
        pub fn schemes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::collections::HashMap<::std::string::String, super::StringList>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.schemes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for schemes: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Security> for super::Security {
        type Error = super::error::ConversionError;
        fn try_from(value: Security) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                schemes: value.schemes?,
            })
        }
    }
    impl ::std::convert::From<super::Security> for Security {
        fn from(value: super::Security) -> Self {
            Self {
                schemes: Ok(value.schemes),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SecurityScheme {
        api_key_security_scheme: ::std::result::Result<
            ::std::option::Option<super::ApiKeySecurityScheme>,
            ::std::string::String,
        >,
        http_auth_security_scheme: ::std::result::Result<
            ::std::option::Option<super::HttpAuthSecurityScheme>,
            ::std::string::String,
        >,
        mtls_security_scheme: ::std::result::Result<
            ::std::option::Option<super::MutualTlsSecurityScheme>,
            ::std::string::String,
        >,
        oauth2_security_scheme: ::std::result::Result<
            ::std::option::Option<super::OAuth2SecurityScheme>,
            ::std::string::String,
        >,
        open_id_connect_security_scheme: ::std::result::Result<
            ::std::option::Option<super::OpenIdConnectSecurityScheme>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for SecurityScheme {
        fn default() -> Self {
            Self {
                api_key_security_scheme: Ok(Default::default()),
                http_auth_security_scheme: Ok(Default::default()),
                mtls_security_scheme: Ok(Default::default()),
                oauth2_security_scheme: Ok(Default::default()),
                open_id_connect_security_scheme: Ok(Default::default()),
            }
        }
    }
    impl SecurityScheme {
        pub fn api_key_security_scheme<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ApiKeySecurityScheme>>,
            T::Error: ::std::fmt::Display,
        {
            self.api_key_security_scheme = value.try_into().map_err(|e| {
                format!("error converting supplied value for api_key_security_scheme: {e}")
            });
            self
        }
        pub fn http_auth_security_scheme<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::HttpAuthSecurityScheme>>,
            T::Error: ::std::fmt::Display,
        {
            self.http_auth_security_scheme = value.try_into().map_err(|e| {
                format!("error converting supplied value for http_auth_security_scheme: {e}")
            });
            self
        }
        pub fn mtls_security_scheme<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::MutualTlsSecurityScheme>>,
            T::Error: ::std::fmt::Display,
        {
            self.mtls_security_scheme = value.try_into().map_err(|e| {
                format!("error converting supplied value for mtls_security_scheme: {e}")
            });
            self
        }
        pub fn oauth2_security_scheme<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::OAuth2SecurityScheme>>,
            T::Error: ::std::fmt::Display,
        {
            self.oauth2_security_scheme = value.try_into().map_err(|e| {
                format!("error converting supplied value for oauth2_security_scheme: {e}")
            });
            self
        }
        pub fn open_id_connect_security_scheme<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::OpenIdConnectSecurityScheme>>,
            T::Error: ::std::fmt::Display,
        {
            self.open_id_connect_security_scheme = value.try_into().map_err(|e| {
                format!("error converting supplied value for open_id_connect_security_scheme: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<SecurityScheme> for super::SecurityScheme {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SecurityScheme,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                api_key_security_scheme: value.api_key_security_scheme?,
                http_auth_security_scheme: value.http_auth_security_scheme?,
                mtls_security_scheme: value.mtls_security_scheme?,
                oauth2_security_scheme: value.oauth2_security_scheme?,
                open_id_connect_security_scheme: value.open_id_connect_security_scheme?,
            })
        }
    }
    impl ::std::convert::From<super::SecurityScheme> for SecurityScheme {
        fn from(value: super::SecurityScheme) -> Self {
            Self {
                api_key_security_scheme: Ok(value.api_key_security_scheme),
                http_auth_security_scheme: Ok(value.http_auth_security_scheme),
                mtls_security_scheme: Ok(value.mtls_security_scheme),
                oauth2_security_scheme: Ok(value.oauth2_security_scheme),
                open_id_connect_security_scheme: Ok(value.open_id_connect_security_scheme),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SendMessageConfiguration {
        accepted_output_modes:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        blocking: ::std::result::Result<bool, ::std::string::String>,
        history_length: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        push_notification_config: ::std::result::Result<
            ::std::option::Option<super::PushNotificationConfig>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for SendMessageConfiguration {
        fn default() -> Self {
            Self {
                accepted_output_modes: Ok(Default::default()),
                blocking: Err("no value supplied for blocking".to_string()),
                history_length: Ok(Default::default()),
                push_notification_config: Ok(Default::default()),
            }
        }
    }
    impl SendMessageConfiguration {
        pub fn accepted_output_modes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.accepted_output_modes = value.try_into().map_err(|e| {
                format!("error converting supplied value for accepted_output_modes: {e}")
            });
            self
        }
        pub fn blocking<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.blocking = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for blocking: {e}"));
            self
        }
        pub fn history_length<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i32>>,
            T::Error: ::std::fmt::Display,
        {
            self.history_length = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for history_length: {e}"));
            self
        }
        pub fn push_notification_config<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PushNotificationConfig>>,
            T::Error: ::std::fmt::Display,
        {
            self.push_notification_config = value.try_into().map_err(|e| {
                format!("error converting supplied value for push_notification_config: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<SendMessageConfiguration> for super::SendMessageConfiguration {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SendMessageConfiguration,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                accepted_output_modes: value.accepted_output_modes?,
                blocking: value.blocking?,
                history_length: value.history_length?,
                push_notification_config: value.push_notification_config?,
            })
        }
    }
    impl ::std::convert::From<super::SendMessageConfiguration> for SendMessageConfiguration {
        fn from(value: super::SendMessageConfiguration) -> Self {
            Self {
                accepted_output_modes: Ok(value.accepted_output_modes),
                blocking: Ok(value.blocking),
                history_length: Ok(value.history_length),
                push_notification_config: Ok(value.push_notification_config),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SendMessageRequest {
        configuration: ::std::result::Result<
            ::std::option::Option<super::SendMessageConfiguration>,
            ::std::string::String,
        >,
        message:
            ::std::result::Result<::std::option::Option<super::Message>, ::std::string::String>,
        metadata:
            ::std::result::Result<::std::option::Option<super::Struct>, ::std::string::String>,
        tenant: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for SendMessageRequest {
        fn default() -> Self {
            Self {
                configuration: Ok(Default::default()),
                message: Ok(Default::default()),
                metadata: Ok(Default::default()),
                tenant: Err("no value supplied for tenant".to_string()),
            }
        }
    }
    impl SendMessageRequest {
        pub fn configuration<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SendMessageConfiguration>>,
            T::Error: ::std::fmt::Display,
        {
            self.configuration = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for configuration: {e}"));
            self
        }
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Message>>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {e}"));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Struct>>,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {e}"));
            self
        }
        pub fn tenant<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.tenant = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tenant: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SendMessageRequest> for super::SendMessageRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SendMessageRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                configuration: value.configuration?,
                message: value.message?,
                metadata: value.metadata?,
                tenant: value.tenant?,
            })
        }
    }
    impl ::std::convert::From<super::SendMessageRequest> for SendMessageRequest {
        fn from(value: super::SendMessageRequest) -> Self {
            Self {
                configuration: Ok(value.configuration),
                message: Ok(value.message),
                metadata: Ok(value.metadata),
                tenant: Ok(value.tenant),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SendMessageResponse {
        message:
            ::std::result::Result<::std::option::Option<super::Message>, ::std::string::String>,
        task: ::std::result::Result<::std::option::Option<super::Task>, ::std::string::String>,
    }
    impl ::std::default::Default for SendMessageResponse {
        fn default() -> Self {
            Self {
                message: Ok(Default::default()),
                task: Ok(Default::default()),
            }
        }
    }
    impl SendMessageResponse {
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Message>>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {e}"));
            self
        }
        pub fn task<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Task>>,
            T::Error: ::std::fmt::Display,
        {
            self.task = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for task: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SendMessageResponse> for super::SendMessageResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SendMessageResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                message: value.message?,
                task: value.task?,
            })
        }
    }
    impl ::std::convert::From<super::SendMessageResponse> for SendMessageResponse {
        fn from(value: super::SendMessageResponse) -> Self {
            Self {
                message: Ok(value.message),
                task: Ok(value.task),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SetTaskPushNotificationConfigRequest {
        config: ::std::result::Result<super::TaskPushNotificationConfig, ::std::string::String>,
        config_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        parent: ::std::result::Result<::std::string::String, ::std::string::String>,
        tenant: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for SetTaskPushNotificationConfigRequest {
        fn default() -> Self {
            Self {
                config: Err("no value supplied for config".to_string()),
                config_id: Err("no value supplied for config_id".to_string()),
                parent: Err("no value supplied for parent".to_string()),
                tenant: Ok(Default::default()),
            }
        }
    }
    impl SetTaskPushNotificationConfigRequest {
        pub fn config<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TaskPushNotificationConfig>,
            T::Error: ::std::fmt::Display,
        {
            self.config = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for config: {e}"));
            self
        }
        pub fn config_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.config_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for config_id: {e}"));
            self
        }
        pub fn parent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.parent = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for parent: {e}"));
            self
        }
        pub fn tenant<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.tenant = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tenant: {e}"));
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
                config: value.config?,
                config_id: value.config_id?,
                parent: value.parent?,
                tenant: value.tenant?,
            })
        }
    }
    impl ::std::convert::From<super::SetTaskPushNotificationConfigRequest>
        for SetTaskPushNotificationConfigRequest
    {
        fn from(value: super::SetTaskPushNotificationConfigRequest) -> Self {
            Self {
                config: Ok(value.config),
                config_id: Ok(value.config_id),
                parent: Ok(value.parent),
                tenant: Ok(value.tenant),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StreamResponse {
        artifact_update: ::std::result::Result<
            ::std::option::Option<super::TaskArtifactUpdateEvent>,
            ::std::string::String,
        >,
        message:
            ::std::result::Result<::std::option::Option<super::Message>, ::std::string::String>,
        status_update: ::std::result::Result<
            ::std::option::Option<super::TaskStatusUpdateEvent>,
            ::std::string::String,
        >,
        task: ::std::result::Result<::std::option::Option<super::Task>, ::std::string::String>,
    }
    impl ::std::default::Default for StreamResponse {
        fn default() -> Self {
            Self {
                artifact_update: Ok(Default::default()),
                message: Ok(Default::default()),
                status_update: Ok(Default::default()),
                task: Ok(Default::default()),
            }
        }
    }
    impl StreamResponse {
        pub fn artifact_update<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TaskArtifactUpdateEvent>>,
            T::Error: ::std::fmt::Display,
        {
            self.artifact_update = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for artifact_update: {e}"));
            self
        }
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Message>>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {e}"));
            self
        }
        pub fn status_update<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TaskStatusUpdateEvent>>,
            T::Error: ::std::fmt::Display,
        {
            self.status_update = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status_update: {e}"));
            self
        }
        pub fn task<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Task>>,
            T::Error: ::std::fmt::Display,
        {
            self.task = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for task: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<StreamResponse> for super::StreamResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: StreamResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                artifact_update: value.artifact_update?,
                message: value.message?,
                status_update: value.status_update?,
                task: value.task?,
            })
        }
    }
    impl ::std::convert::From<super::StreamResponse> for StreamResponse {
        fn from(value: super::StreamResponse) -> Self {
            Self {
                artifact_update: Ok(value.artifact_update),
                message: Ok(value.message),
                status_update: Ok(value.status_update),
                task: Ok(value.task),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StringList {
        list: ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
    }
    impl ::std::default::Default for StringList {
        fn default() -> Self {
            Self {
                list: Ok(Default::default()),
            }
        }
    }
    impl StringList {
        pub fn list<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.list = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for list: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<StringList> for super::StringList {
        type Error = super::error::ConversionError;
        fn try_from(
            value: StringList,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { list: value.list? })
        }
    }
    impl ::std::convert::From<super::StringList> for StringList {
        fn from(value: super::StringList) -> Self {
            Self {
                list: Ok(value.list),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SubscribeToTaskRequest {
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        tenant: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for SubscribeToTaskRequest {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                tenant: Err("no value supplied for tenant".to_string()),
            }
        }
    }
    impl SubscribeToTaskRequest {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn tenant<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.tenant = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tenant: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SubscribeToTaskRequest> for super::SubscribeToTaskRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SubscribeToTaskRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                tenant: value.tenant?,
            })
        }
    }
    impl ::std::convert::From<super::SubscribeToTaskRequest> for SubscribeToTaskRequest {
        fn from(value: super::SubscribeToTaskRequest) -> Self {
            Self {
                name: Ok(value.name),
                tenant: Ok(value.tenant),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Task {
        artifacts: ::std::result::Result<::std::vec::Vec<super::Artifact>, ::std::string::String>,
        context_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        history: ::std::result::Result<::std::vec::Vec<super::Message>, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        metadata:
            ::std::result::Result<::std::option::Option<super::Struct>, ::std::string::String>,
        status: ::std::result::Result<super::TaskStatus, ::std::string::String>,
    }
    impl ::std::default::Default for Task {
        fn default() -> Self {
            Self {
                artifacts: Ok(Default::default()),
                context_id: Err("no value supplied for context_id".to_string()),
                history: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
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
                .map_err(|e| format!("error converting supplied value for artifacts: {e}"));
            self
        }
        pub fn context_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.context_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for context_id: {e}"));
            self
        }
        pub fn history<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Message>>,
            T::Error: ::std::fmt::Display,
        {
            self.history = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for history: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Struct>>,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {e}"));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TaskStatus>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {e}"));
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
        last_chunk: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        metadata:
            ::std::result::Result<::std::option::Option<super::Struct>, ::std::string::String>,
        task_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for TaskArtifactUpdateEvent {
        fn default() -> Self {
            Self {
                append: Ok(Default::default()),
                artifact: Err("no value supplied for artifact".to_string()),
                context_id: Err("no value supplied for context_id".to_string()),
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
                .map_err(|e| format!("error converting supplied value for append: {e}"));
            self
        }
        pub fn artifact<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Artifact>,
            T::Error: ::std::fmt::Display,
        {
            self.artifact = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for artifact: {e}"));
            self
        }
        pub fn context_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.context_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for context_id: {e}"));
            self
        }
        pub fn last_chunk<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.last_chunk = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for last_chunk: {e}"));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Struct>>,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {e}"));
            self
        }
        pub fn task_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.task_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for task_id: {e}"));
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
                last_chunk: Ok(value.last_chunk),
                metadata: Ok(value.metadata),
                task_id: Ok(value.task_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TaskPushNotificationConfig {
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        push_notification_config:
            ::std::result::Result<super::PushNotificationConfig, ::std::string::String>,
    }
    impl ::std::default::Default for TaskPushNotificationConfig {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                push_notification_config: Err(
                    "no value supplied for push_notification_config".to_string()
                ),
            }
        }
    }
    impl TaskPushNotificationConfig {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn push_notification_config<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::PushNotificationConfig>,
            T::Error: ::std::fmt::Display,
        {
            self.push_notification_config = value.try_into().map_err(|e| {
                format!("error converting supplied value for push_notification_config: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<TaskPushNotificationConfig> for super::TaskPushNotificationConfig {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TaskPushNotificationConfig,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                push_notification_config: value.push_notification_config?,
            })
        }
    }
    impl ::std::convert::From<super::TaskPushNotificationConfig> for TaskPushNotificationConfig {
        fn from(value: super::TaskPushNotificationConfig) -> Self {
            Self {
                name: Ok(value.name),
                push_notification_config: Ok(value.push_notification_config),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TaskStatus {
        message:
            ::std::result::Result<::std::option::Option<super::Message>, ::std::string::String>,
        state: ::std::result::Result<super::TaskState, ::std::string::String>,
        timestamp:
            ::std::result::Result<::std::option::Option<super::Timestamp>, ::std::string::String>,
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
                .map_err(|e| format!("error converting supplied value for message: {e}"));
            self
        }
        pub fn state<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TaskState>,
            T::Error: ::std::fmt::Display,
        {
            self.state = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for state: {e}"));
            self
        }
        pub fn timestamp<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Timestamp>>,
            T::Error: ::std::fmt::Display,
        {
            self.timestamp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for timestamp: {e}"));
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
        metadata:
            ::std::result::Result<::std::option::Option<super::Struct>, ::std::string::String>,
        status: ::std::result::Result<super::TaskStatus, ::std::string::String>,
        task_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for TaskStatusUpdateEvent {
        fn default() -> Self {
            Self {
                context_id: Err("no value supplied for context_id".to_string()),
                final_: Err("no value supplied for final_".to_string()),
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
                .map_err(|e| format!("error converting supplied value for context_id: {e}"));
            self
        }
        pub fn final_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.final_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for final_: {e}"));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Struct>>,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {e}"));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TaskStatus>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {e}"));
            self
        }
        pub fn task_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.task_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for task_id: {e}"));
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
                metadata: Ok(value.metadata),
                status: Ok(value.status),
                task_id: Ok(value.task_id),
            }
        }
    }
}

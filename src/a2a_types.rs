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

#[doc = "Defines optional capabilities supported by an agent."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct AgentCapabilities {
    #[doc = "Indicates if the agent supports providing an extended agent card when authenticated."]
    #[serde(
        rename = "extendedAgentCard",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub extended_agent_card: ::std::option::Option<bool>,
    #[doc = "A list of protocol extensions supported by the agent."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub extensions: ::std::vec::Vec<AgentExtension>,
    #[doc = "Indicates if the agent supports sending push notifications for asynchronous task updates."]
    #[serde(
        rename = "pushNotifications",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub push_notifications: ::std::option::Option<bool>,
    #[doc = "Indicates if the agent supports streaming responses."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub streaming: ::std::option::Option<bool>,
}
impl AgentCapabilities {
    pub fn builder() -> builder::AgentCapabilities {
        Default::default()
    }
}
#[doc = "A self-describing manifest for an agent. It provides essential\n metadata including the agent's identity, capabilities, skills, supported\n communication methods, and security requirements.\n Next ID: 20"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct AgentCard {
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
    #[doc = "A URL providing additional documentation about the agent."]
    #[serde(
        rename = "documentationUrl",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub documentation_url: ::std::option::Option<::std::string::String>,
    #[doc = "Optional. A URL to an icon for the agent."]
    #[serde(
        rename = "iconUrl",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub icon_url: ::std::option::Option<::std::string::String>,
    #[doc = "A human readable name for the agent.\n Example: \"Recipe Agent\""]
    pub name: ::std::string::String,
    #[doc = "The service provider of the agent."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub provider: ::std::option::Option<AgentProvider>,
    #[doc = "Security requirements for contacting the agent."]
    #[serde(
        rename = "securityRequirements",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub security_requirements: ::std::vec::Vec<SecurityRequirement>,
    #[doc = "The security scheme details used for authenticating with this agent."]
    #[serde(
        rename = "securitySchemes",
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub security_schemes: ::std::collections::HashMap<::std::string::String, SecurityScheme>,
    #[doc = "JSON Web Signatures computed for this `AgentCard`."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub signatures: ::std::vec::Vec<AgentCardSignature>,
    #[doc = "Skills represent the abilities of an agent.\n It is largely a descriptive concept but represents a more focused set of behaviors that the\n agent is likely to succeed at."]
    pub skills: ::std::vec::Vec<AgentSkill>,
    #[doc = "Ordered list of supported interfaces. The first entry is preferred."]
    #[serde(rename = "supportedInterfaces")]
    pub supported_interfaces: ::std::vec::Vec<AgentInterface>,
    #[doc = "The version of the agent.\n Example: \"1.0.0\""]
    pub version: ::std::string::String,
}
impl AgentCard {
    pub fn builder() -> builder::AgentCard {
        Default::default()
    }
}
#[doc = "AgentCardSignature represents a JWS signature of an AgentCard.\n This follows the JSON format of an RFC 7515 JSON Web Signature (JWS)."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct AgentCardSignature {
    #[doc = "The unprotected JWS header values."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub header: ::std::option::Option<Struct>,
    #[doc = "(-- api-linter: core::0140::reserved-words=disabled\n     aip.dev/not-precedent: Backwards compatibility --)\n Required. The protected JWS header for the signature. This is always a\n base64url-encoded JSON object."]
    pub protected: ::std::string::String,
    #[doc = "Required. The computed signature, base64url-encoded."]
    pub signature: ::std::string::String,
}
impl AgentCardSignature {
    pub fn builder() -> builder::AgentCardSignature {
        Default::default()
    }
}
#[doc = "A declaration of a protocol extension supported by an Agent."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct AgentExtension {
    #[doc = "A human-readable description of how this agent uses the extension."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "Optional. Extension-specific configuration parameters."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub params: ::std::option::Option<Struct>,
    #[doc = "If true, the client must understand and comply with the extension's requirements."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub required: ::std::option::Option<bool>,
    #[doc = "The unique URI identifying the extension."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub uri: ::std::option::Option<::std::string::String>,
}
impl AgentExtension {
    pub fn builder() -> builder::AgentExtension {
        Default::default()
    }
}
#[doc = "Declares a combination of a target URL, transport and protocol version for interacting with the agent.\n This allows agents to expose the same functionality over multiple protocol binding mechanisms."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct AgentInterface {
    #[doc = "The protocol binding supported at this URL. This is an open form string, to be\n easily extended for other protocol bindings. The core ones officially\n supported are `JSONRPC`, `GRPC` and `HTTP+JSON`."]
    #[serde(rename = "protocolBinding")]
    pub protocol_binding: ::std::string::String,
    #[doc = "The version of the A2A protocol this interface exposes.\n Use the latest supported minor version per major version.\n Examples: \"0.3\", \"1.0\""]
    #[serde(rename = "protocolVersion")]
    pub protocol_version: ::std::string::String,
    #[doc = "Optional. An opaque string used for routing requests to a specific agent\n or tenant when multiple agents are served behind a single A2A endpoint.\n When set, clients MUST include this value in the `tenant` field of all\n request messages sent to this interface. The server is responsible for\n interpreting the value and routing requests accordingly; the protocol\n does not define its format or semantics."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
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
    #[doc = "Security schemes necessary for this skill."]
    #[serde(
        rename = "securityRequirements",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub security_requirements: ::std::vec::Vec<SecurityRequirement>,
    #[doc = "A set of keywords describing the skill's capabilities."]
    pub tags: ::std::vec::Vec<::std::string::String>,
}
impl AgentSkill {
    pub fn builder() -> builder::AgentSkill {
        Default::default()
    }
}
#[doc = "Defines a security scheme using an API key."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ApiKeySecurityScheme {
    #[doc = "An optional description for the security scheme."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
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
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Artifact {
    #[doc = "Unique identifier (e.g. UUID) for the artifact. It must be unique within a task."]
    #[serde(rename = "artifactId")]
    pub artifact_id: ::std::string::String,
    #[doc = "Optional. A human readable description of the artifact."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "The URIs of extensions that are present or contributed to this Artifact."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub extensions: ::std::vec::Vec<::std::string::String>,
    #[doc = "Optional. Metadata included with the artifact."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<Struct>,
    #[doc = "A human readable name for the artifact."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
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
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct AuthenticationInfo {
    #[doc = "Push Notification credentials. Format depends on the scheme (e.g., token for Bearer)."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub credentials: ::std::option::Option<::std::string::String>,
    #[doc = "HTTP Authentication Scheme from the [IANA registry](https://www.iana.org/assignments/http-authschemes/).\n Examples: `Bearer`, `Basic`, `Digest`.\n Scheme names are case-insensitive per [RFC 9110 Section 11.1](https://www.rfc-editor.org/rfc/rfc9110#section-11.1)."]
    pub scheme: ::std::string::String,
}
impl AuthenticationInfo {
    pub fn builder() -> builder::AuthenticationInfo {
        Default::default()
    }
}
#[doc = "Defines configuration details for the OAuth 2.0 Authorization Code flow."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct AuthorizationCodeOAuthFlow {
    #[doc = "The authorization URL to be used for this flow."]
    #[serde(rename = "authorizationUrl")]
    pub authorization_url: ::std::string::String,
    #[doc = "Indicates if PKCE (RFC 7636) is required for this flow.\n PKCE should always be used for public clients and is recommended for all clients."]
    #[serde(
        rename = "pkceRequired",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub pkce_required: ::std::option::Option<bool>,
    #[doc = "The URL to be used for obtaining refresh tokens."]
    #[serde(
        rename = "refreshUrl",
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
#[doc = "Represents a request for the `CancelTask` method."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct CancelTaskRequest {
    #[doc = "The resource ID of the task to cancel."]
    pub id: ::std::string::String,
    #[doc = "A flexible key-value map for passing additional context or parameters."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<Struct>,
    #[doc = "Optional. Opaque routing identifier. Must match the `tenant` value from\n the selected `AgentInterface` in the Agent Card when that field is set."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tenant: ::std::option::Option<::std::string::String>,
}
impl CancelTaskRequest {
    pub fn builder() -> builder::CancelTaskRequest {
        Default::default()
    }
}
#[doc = "Defines configuration details for the OAuth 2.0 Client Credentials flow."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ClientCredentialsOAuthFlow {
    #[doc = "The URL to be used for obtaining refresh tokens."]
    #[serde(
        rename = "refreshUrl",
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
#[doc = "Represents a request for the `DeleteTaskPushNotificationConfig` method."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct DeleteTaskPushNotificationConfigRequest {
    #[doc = "The resource ID of the configuration to delete."]
    pub id: ::std::string::String,
    #[doc = "The parent task resource ID."]
    #[serde(rename = "taskId")]
    pub task_id: ::std::string::String,
    #[doc = "Optional. Opaque routing identifier. Must match the `tenant` value from\n the selected `AgentInterface` in the Agent Card when that field is set."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tenant: ::std::option::Option<::std::string::String>,
}
impl DeleteTaskPushNotificationConfigRequest {
    pub fn builder() -> builder::DeleteTaskPushNotificationConfigRequest {
        Default::default()
    }
}
#[doc = "Defines configuration details for the OAuth 2.0 Device Code flow (RFC 8628).\n This flow is designed for input-constrained devices such as IoT devices,\n and CLI tools where the user authenticates on a separate device."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct DeviceCodeOAuthFlow {
    #[doc = "The device authorization endpoint URL."]
    #[serde(rename = "deviceAuthorizationUrl")]
    pub device_authorization_url: ::std::string::String,
    #[doc = "The URL to be used for obtaining refresh tokens."]
    #[serde(
        rename = "refreshUrl",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub refresh_url: ::std::option::Option<::std::string::String>,
    #[doc = "The available scopes for the OAuth2 security scheme."]
    pub scopes: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    #[doc = "The token URL to be used for this flow."]
    #[serde(rename = "tokenUrl")]
    pub token_url: ::std::string::String,
}
impl DeviceCodeOAuthFlow {
    pub fn builder() -> builder::DeviceCodeOAuthFlow {
        Default::default()
    }
}
#[doc = "Represents a request for the `GetExtendedAgentCard` method."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct GetExtendedAgentCardRequest {
    #[doc = "Optional. Opaque routing identifier. Must match the `tenant` value from\n the selected `AgentInterface` in the Agent Card when that field is set."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tenant: ::std::option::Option<::std::string::String>,
}
impl GetExtendedAgentCardRequest {
    pub fn builder() -> builder::GetExtendedAgentCardRequest {
        Default::default()
    }
}
#[doc = "Represents a request for the `GetTaskPushNotificationConfig` method."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct GetTaskPushNotificationConfigRequest {
    #[doc = "The resource ID of the configuration to retrieve."]
    pub id: ::std::string::String,
    #[doc = "The parent task resource ID."]
    #[serde(rename = "taskId")]
    pub task_id: ::std::string::String,
    #[doc = "Optional. Opaque routing identifier. Must match the `tenant` value from\n the selected `AgentInterface` in the Agent Card when that field is set."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tenant: ::std::option::Option<::std::string::String>,
}
impl GetTaskPushNotificationConfigRequest {
    pub fn builder() -> builder::GetTaskPushNotificationConfigRequest {
        Default::default()
    }
}
#[doc = "Represents a request for the `GetTask` method."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct GetTaskRequest {
    #[doc = "The maximum number of most recent messages from the task's history to retrieve. An\n unset value means the client does not impose any limit. A value of zero is\n a request to not include any messages. The server MUST NOT return more\n messages than the provided value, but MAY apply a lower limit."]
    #[serde(
        rename = "historyLength",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub history_length: ::std::option::Option<i32>,
    #[doc = "The resource ID of the task to retrieve."]
    pub id: ::std::string::String,
    #[doc = "Optional. Opaque routing identifier. Must match the `tenant` value from\n the selected `AgentInterface` in the Agent Card when that field is set."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tenant: ::std::option::Option<::std::string::String>,
}
impl GetTaskRequest {
    pub fn builder() -> builder::GetTaskRequest {
        Default::default()
    }
}
#[doc = "Defines a security scheme using HTTP authentication."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct HttpAuthSecurityScheme {
    #[doc = "A hint to the client to identify how the bearer token is formatted (e.g., \"JWT\").\n Primarily for documentation purposes."]
    #[serde(
        rename = "bearerFormat",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub bearer_format: ::std::option::Option<::std::string::String>,
    #[doc = "An optional description for the security scheme."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "The name of the HTTP Authentication scheme to be used in the Authorization header,\n as defined in RFC7235 (e.g., \"Bearer\").\n This value should be registered in the IANA Authentication Scheme registry."]
    pub scheme: ::std::string::String,
}
impl HttpAuthSecurityScheme {
    pub fn builder() -> builder::HttpAuthSecurityScheme {
        Default::default()
    }
}
#[doc = "Deprecated: Use Authorization Code + PKCE instead."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct ImplicitOAuthFlow {
    #[doc = "The authorization URL to be used for this flow. This MUST be in the\n form of a URL. The OAuth2 standard requires the use of TLS"]
    #[serde(
        rename = "authorizationUrl",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub authorization_url: ::std::option::Option<::std::string::String>,
    #[doc = "The URL to be used for obtaining refresh tokens. This MUST be in the\n form of a URL. The OAuth2 standard requires the use of TLS."]
    #[serde(
        rename = "refreshUrl",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub refresh_url: ::std::option::Option<::std::string::String>,
    #[doc = "The available scopes for the OAuth2 security scheme. A map between the\n scope name and a short description for it. The map MAY be empty."]
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub scopes: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
}
impl ImplicitOAuthFlow {
    pub fn builder() -> builder::ImplicitOAuthFlow {
        Default::default()
    }
}
#[doc = "Represents a request for the `ListTaskPushNotificationConfigs` method."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ListTaskPushNotificationConfigsRequest {
    #[doc = "The maximum number of configurations to return."]
    #[serde(
        rename = "pageSize",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub page_size: ::std::option::Option<i32>,
    #[doc = "A page token received from a previous `ListTaskPushNotificationConfigsRequest` call."]
    #[serde(
        rename = "pageToken",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub page_token: ::std::option::Option<::std::string::String>,
    #[doc = "The parent task resource ID."]
    #[serde(rename = "taskId")]
    pub task_id: ::std::string::String,
    #[doc = "Optional. Opaque routing identifier. Must match the `tenant` value from\n the selected `AgentInterface` in the Agent Card when that field is set."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tenant: ::std::option::Option<::std::string::String>,
}
impl ListTaskPushNotificationConfigsRequest {
    pub fn builder() -> builder::ListTaskPushNotificationConfigsRequest {
        Default::default()
    }
}
#[doc = "Represents a successful response for the `ListTaskPushNotificationConfigs`\n method."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct ListTaskPushNotificationConfigsResponse {
    #[doc = "The list of push notification configurations."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub configs: ::std::vec::Vec<TaskPushNotificationConfig>,
    #[doc = "A token to retrieve the next page of results, or empty if there are no more results in the list."]
    #[serde(
        rename = "nextPageToken",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub next_page_token: ::std::option::Option<::std::string::String>,
}
impl ListTaskPushNotificationConfigsResponse {
    pub fn builder() -> builder::ListTaskPushNotificationConfigsResponse {
        Default::default()
    }
}
#[doc = "Parameters for listing tasks with optional filtering criteria."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct ListTasksRequest {
    #[doc = "Filter tasks by context ID to get tasks from a specific conversation or session."]
    #[serde(
        rename = "contextId",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub context_id: ::std::option::Option<::std::string::String>,
    #[doc = "The maximum number of messages to include in each task's history."]
    #[serde(
        rename = "historyLength",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub history_length: ::std::option::Option<i32>,
    #[doc = "Whether to include artifacts in the returned tasks.\n Defaults to false to reduce payload size."]
    #[serde(
        rename = "includeArtifacts",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub include_artifacts: ::std::option::Option<bool>,
    #[doc = "The maximum number of tasks to return. The service may return fewer than this value.\n If unspecified, at most 50 tasks will be returned.\n The minimum value is 1.\n The maximum value is 100."]
    #[serde(
        rename = "pageSize",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub page_size: ::std::option::Option<i32>,
    #[doc = "A page token, received from a previous `ListTasks` call.\n `ListTasksResponse.next_page_token`.\n Provide this to retrieve the subsequent page."]
    #[serde(
        rename = "pageToken",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub page_token: ::std::option::Option<::std::string::String>,
    #[doc = "Filter tasks by their current status state."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<TaskState>,
    #[doc = "Filter tasks which have a status updated after the provided timestamp in ISO 8601 format (e.g., \"2023-10-27T10:00:00Z\").\n Only tasks with a status timestamp time greater than or equal to this value will be returned."]
    #[serde(
        rename = "statusTimestampAfter",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub status_timestamp_after: ::std::option::Option<Timestamp>,
    #[doc = "Optional. Opaque routing identifier. Must match the `tenant` value from\n the selected `AgentInterface` in the Agent Card when that field is set."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tenant: ::std::option::Option<::std::string::String>,
}
impl ListTasksRequest {
    pub fn builder() -> builder::ListTasksRequest {
        Default::default()
    }
}
#[doc = "Result object for `ListTasks` method containing an array of tasks and pagination information."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ListTasksResponse {
    #[doc = "A token to retrieve the next page of results, or empty if there are no more results in the list."]
    #[serde(rename = "nextPageToken")]
    pub next_page_token: ::std::string::String,
    #[doc = "The page size used for this response."]
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
#[doc = "`Message` is one unit of communication between client and server. It can be\n associated with a context and/or a task. For server messages, `context_id` must\n be provided, and `task_id` only if a task was created. For client messages, both\n fields are optional, with the caveat that if both are provided, they have to\n match (the `context_id` has to be the one that is set on the task). If only\n `task_id` is provided, the server will infer `context_id` from it."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Message {
    #[doc = "Optional. The context id of the message. If set, the message will be associated with the given context."]
    #[serde(
        rename = "contextId",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub context_id: ::std::option::Option<::std::string::String>,
    #[doc = "The URIs of extensions that are present or contributed to this Message."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub extensions: ::std::vec::Vec<::std::string::String>,
    #[doc = "The unique identifier (e.g. UUID) of the message. This is created by the message creator."]
    #[serde(rename = "messageId")]
    pub message_id: ::std::string::String,
    #[doc = "Optional. Any metadata to provide along with the message."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<Struct>,
    #[doc = "Parts is the container of the message content."]
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
    #[doc = "Optional. The task id of the message. If set, the message will be associated with the given task."]
    #[serde(
        rename = "taskId",
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
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct MutualTlsSecurityScheme {
    #[doc = "An optional description for the security scheme."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
}
impl MutualTlsSecurityScheme {
    pub fn builder() -> builder::MutualTlsSecurityScheme {
        Default::default()
    }
}
#[doc = "Defines a security scheme using OAuth 2.0."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct OAuth2SecurityScheme {
    #[doc = "An optional description for the security scheme."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "An object containing configuration information for the supported OAuth 2.0 flows."]
    pub flows: OAuthFlows,
    #[doc = "URL to the OAuth2 authorization server metadata [RFC 8414](https://datatracker.ietf.org/doc/html/rfc8414).\n TLS is required."]
    #[serde(
        rename = "oauth2MetadataUrl",
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
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct OAuthFlows {
    #[doc = "Configuration for the OAuth Authorization Code flow."]
    #[serde(
        rename = "authorizationCode",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub authorization_code: ::std::option::Option<AuthorizationCodeOAuthFlow>,
    #[doc = "Configuration for the OAuth Client Credentials flow."]
    #[serde(
        rename = "clientCredentials",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub client_credentials: ::std::option::Option<ClientCredentialsOAuthFlow>,
    #[doc = "Configuration for the OAuth Device Code flow."]
    #[serde(
        rename = "deviceCode",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub device_code: ::std::option::Option<DeviceCodeOAuthFlow>,
    #[doc = "Deprecated: Use Authorization Code + PKCE instead."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub implicit: ::std::option::Option<ImplicitOAuthFlow>,
    #[doc = "Deprecated: Use Authorization Code + PKCE or Device Code."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub password: ::std::option::Option<PasswordOAuthFlow>,
}
impl OAuthFlows {
    pub fn builder() -> builder::OAuthFlows {
        Default::default()
    }
}
#[doc = "Defines a security scheme using OpenID Connect."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct OpenIdConnectSecurityScheme {
    #[doc = "An optional description for the security scheme."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[doc = "The [OpenID Connect Discovery URL](https://openid.net/specs/openid-connect-discovery-1_0.html) for the OIDC provider's metadata."]
    #[serde(rename = "openIdConnectUrl")]
    pub open_id_connect_url: ::std::string::String,
}
impl OpenIdConnectSecurityScheme {
    pub fn builder() -> builder::OpenIdConnectSecurityScheme {
        Default::default()
    }
}
#[doc = "`Part` represents a container for a section of communication content.\n Parts can be purely textual, some sort of file (image, video, etc) or\n a structured data blob (i.e. JSON)."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct Part {
    #[doc = "Arbitrary structured `data` as a JSON value (object, array, string, number, boolean, or null)."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<Value>,
    #[doc = "An optional `filename` for the file (e.g., \"document.pdf\")."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub filename: ::std::option::Option<::std::string::String>,
    #[doc = "The `media_type` (MIME type) of the part content (e.g., \"text/plain\", \"application/json\", \"image/png\").\n This field is available for all part types."]
    #[serde(
        rename = "mediaType",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub media_type: ::std::option::Option<::std::string::String>,
    #[doc = "Optional. metadata associated with this part."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<Struct>,
    #[doc = "The `raw` byte content of a file. In JSON serialization, this is encoded as a base64 string."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub raw: ::std::option::Option<PartRaw>,
    #[doc = "The string content of the `text` part."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub text: ::std::option::Option<::std::string::String>,
    #[doc = "A `url` pointing to the file's content."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub url: ::std::option::Option<::std::string::String>,
}
impl Part {
    pub fn builder() -> builder::Part {
        Default::default()
    }
}
#[doc = "The `raw` byte content of a file. In JSON serialization, this is encoded as a base64 string."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct PartRaw(::std::string::String);
impl ::std::ops::Deref for PartRaw {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<PartRaw> for ::std::string::String {
    fn from(value: PartRaw) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for PartRaw {
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
impl ::std::convert::TryFrom<&str> for PartRaw {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PartRaw {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for PartRaw {
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
#[doc = "Deprecated: Use Authorization Code + PKCE or Device Code."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct PasswordOAuthFlow {
    #[doc = "The URL to be used for obtaining refresh tokens. This MUST be in the\n form of a URL. The OAuth2 standard requires the use of TLS."]
    #[serde(
        rename = "refreshUrl",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub refresh_url: ::std::option::Option<::std::string::String>,
    #[doc = "The available scopes for the OAuth2 security scheme. A map between the\n scope name and a short description for it. The map MAY be empty."]
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub scopes: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    #[doc = "The token URL to be used for this flow. This MUST be in the form of a URL.\n The OAuth2 standard requires the use of TLS."]
    #[serde(
        rename = "tokenUrl",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub token_url: ::std::option::Option<::std::string::String>,
}
impl PasswordOAuthFlow {
    pub fn builder() -> builder::PasswordOAuthFlow {
        Default::default()
    }
}
#[doc = "Identifies the sender of the message."]
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
impl ::std::convert::TryFrom<::std::string::String> for Role {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Defines the security requirements for an agent."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct SecurityRequirement {
    #[doc = "A map of security schemes to the required scopes."]
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub schemes: ::std::collections::HashMap<::std::string::String, StringList>,
}
impl SecurityRequirement {
    pub fn builder() -> builder::SecurityRequirement {
        Default::default()
    }
}
#[doc = "Defines a security scheme that can be used to secure an agent's endpoints.\n This is a discriminated union type based on the OpenAPI 3.2 Security Scheme Object.\n See: https://spec.openapis.org/oas/v3.2.0.html#security-scheme-object"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct SecurityScheme {
    #[doc = "API key-based authentication."]
    #[serde(
        rename = "apiKeySecurityScheme",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub api_key_security_scheme: ::std::option::Option<ApiKeySecurityScheme>,
    #[doc = "HTTP authentication (Basic, Bearer, etc.)."]
    #[serde(
        rename = "httpAuthSecurityScheme",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub http_auth_security_scheme: ::std::option::Option<HttpAuthSecurityScheme>,
    #[doc = "Mutual TLS authentication."]
    #[serde(
        rename = "mtlsSecurityScheme",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub mtls_security_scheme: ::std::option::Option<MutualTlsSecurityScheme>,
    #[doc = "OAuth 2.0 authentication."]
    #[serde(
        rename = "oauth2SecurityScheme",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub oauth2_security_scheme: ::std::option::Option<OAuth2SecurityScheme>,
    #[doc = "OpenID Connect authentication."]
    #[serde(
        rename = "openIdConnectSecurityScheme",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub open_id_connect_security_scheme: ::std::option::Option<OpenIdConnectSecurityScheme>,
}
impl SecurityScheme {
    pub fn builder() -> builder::SecurityScheme {
        Default::default()
    }
}
#[doc = "Configuration of a send message request."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct SendMessageConfiguration {
    #[doc = "A list of media types the client is prepared to accept for response parts.\n Agents SHOULD use this to tailor their output."]
    #[serde(
        rename = "acceptedOutputModes",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub accepted_output_modes: ::std::vec::Vec<::std::string::String>,
    #[doc = "The maximum number of most recent messages from the task's history to retrieve in\n the response. An unset value means the client does not impose any limit. A\n value of zero is a request to not include any messages. The server MUST NOT\n return more messages than the provided value, but MAY apply a lower limit."]
    #[serde(
        rename = "historyLength",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub history_length: ::std::option::Option<i32>,
    #[doc = "If `true`, the operation returns immediately after creating the task,\n even if processing is still in progress.\n If `false` (default), the operation MUST wait until the task reaches a\n terminal (`COMPLETED`, `FAILED`, `CANCELED`, `REJECTED`) or interrupted\n (`INPUT_REQUIRED`, `AUTH_REQUIRED`) state before returning."]
    #[serde(
        rename = "returnImmediately",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub return_immediately: ::std::option::Option<bool>,
    #[doc = "Configuration for the agent to send push notifications for task updates.\n Task id should be empty when sending this configuration in a `SendMessage` request."]
    #[serde(
        rename = "taskPushNotificationConfig",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub task_push_notification_config: ::std::option::Option<TaskPushNotificationConfig>,
}
impl SendMessageConfiguration {
    pub fn builder() -> builder::SendMessageConfiguration {
        Default::default()
    }
}
#[doc = "Represents a request for the `SendMessage` method."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SendMessageRequest {
    #[doc = "Configuration for the send request."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub configuration: ::std::option::Option<SendMessageConfiguration>,
    #[doc = "The message to send to the agent."]
    pub message: Message,
    #[doc = "A flexible key-value map for passing additional context or parameters."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<Struct>,
    #[doc = "Optional. Opaque routing identifier. Must match the `tenant` value from\n the selected `AgentInterface` in the Agent Card when that field is set."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tenant: ::std::option::Option<::std::string::String>,
}
impl SendMessageRequest {
    pub fn builder() -> builder::SendMessageRequest {
        Default::default()
    }
}
#[doc = "Represents the response for the `SendMessage` method."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct SendMessageResponse {
    #[doc = "A message from the agent."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub message: ::std::option::Option<Message>,
    #[doc = "The task created or updated by the message."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub task: ::std::option::Option<Task>,
}
impl SendMessageResponse {
    pub fn builder() -> builder::SendMessageResponse {
        Default::default()
    }
}
#[doc = "A wrapper object used in streaming operations to encapsulate different types of response data."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct StreamResponse {
    #[doc = "An event indicating a task artifact update."]
    #[serde(
        rename = "artifactUpdate",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub artifact_update: ::std::option::Option<TaskArtifactUpdateEvent>,
    #[doc = "A Message object containing a message from the agent."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub message: ::std::option::Option<Message>,
    #[doc = "An event indicating a task status update."]
    #[serde(
        rename = "statusUpdate",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub status_update: ::std::option::Option<TaskStatusUpdateEvent>,
    #[doc = "A Task object containing the current state of the task."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub task: ::std::option::Option<Task>,
}
impl StreamResponse {
    pub fn builder() -> builder::StreamResponse {
        Default::default()
    }
}
#[doc = "protolint:disable REPEATED_FIELD_NAMES_PLURALIZED\n A list of strings."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct StringList {
    #[doc = "The individual string values."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub list: ::std::vec::Vec<::std::string::String>,
}
impl StringList {
    pub fn builder() -> builder::StringList {
        Default::default()
    }
}
#[doc = "`Struct`"]
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
#[doc = "Represents a request for the `SubscribeToTask` method."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SubscribeToTaskRequest {
    #[doc = "The resource ID of the task to subscribe to."]
    pub id: ::std::string::String,
    #[doc = "Optional. Opaque routing identifier. Must match the `tenant` value from\n the selected `AgentInterface` in the Agent Card when that field is set."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tenant: ::std::option::Option<::std::string::String>,
}
impl SubscribeToTaskRequest {
    pub fn builder() -> builder::SubscribeToTaskRequest {
        Default::default()
    }
}
#[doc = "`Task` is the core unit of action for A2A. It has a current status\n and when results are created for the task they are stored in the\n artifact. If there are multiple turns for a task, these are stored in\n history."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Task {
    #[doc = "A set of output artifacts for a `Task`."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub artifacts: ::std::vec::Vec<Artifact>,
    #[doc = "Unique identifier (e.g. UUID) for the contextual collection of interactions\n (tasks and messages)."]
    #[serde(
        rename = "contextId",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub context_id: ::std::option::Option<::std::string::String>,
    #[doc = "protolint:disable REPEATED_FIELD_NAMES_PLURALIZED\n The history of interactions from a `Task`."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub history: ::std::vec::Vec<Message>,
    #[doc = "Unique identifier (e.g. UUID) for the task, generated by the server for a\n new task."]
    pub id: ::std::string::String,
    #[doc = "protolint:enable REPEATED_FIELD_NAMES_PLURALIZED\n A key/value object to store custom metadata about a task."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<Struct>,
    #[doc = "The current status of a `Task`, including `state` and a `message`."]
    pub status: TaskStatus,
}
impl Task {
    pub fn builder() -> builder::Task {
        Default::default()
    }
}
#[doc = "A task delta where an artifact has been generated."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TaskArtifactUpdateEvent {
    #[doc = "If true, the content of this artifact should be appended to a previously\n sent artifact with the same ID."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub append: ::std::option::Option<bool>,
    #[doc = "The artifact that was generated or updated."]
    pub artifact: Artifact,
    #[doc = "The ID of the context that this task belongs to."]
    #[serde(rename = "contextId")]
    pub context_id: ::std::string::String,
    #[doc = "If true, this is the final chunk of the artifact."]
    #[serde(
        rename = "lastChunk",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub last_chunk: ::std::option::Option<bool>,
    #[doc = "Optional. Metadata associated with the artifact update."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<Struct>,
    #[doc = "The ID of the task for this artifact."]
    #[serde(rename = "taskId")]
    pub task_id: ::std::string::String,
}
impl TaskArtifactUpdateEvent {
    pub fn builder() -> builder::TaskArtifactUpdateEvent {
        Default::default()
    }
}
#[doc = "A container associating a push notification configuration with a specific task."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TaskPushNotificationConfig {
    #[doc = "Authentication information required to send the notification."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub authentication: ::std::option::Option<AuthenticationInfo>,
    #[doc = "The push notification configuration details.\n A unique identifier (e.g. UUID) for this push notification configuration."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
    #[doc = "The ID of the task this configuration is associated with."]
    #[serde(
        rename = "taskId",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub task_id: ::std::option::Option<::std::string::String>,
    #[doc = "Optional. Opaque routing identifier. Must match the `tenant` value from\n the selected `AgentInterface` in the Agent Card when that field is set."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub tenant: ::std::option::Option<::std::string::String>,
    #[doc = "A token unique for this task or session."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub token: ::std::option::Option<::std::string::String>,
    #[doc = "The URL where the notification should be sent."]
    pub url: ::std::string::String,
}
impl TaskPushNotificationConfig {
    pub fn builder() -> builder::TaskPushNotificationConfig {
        Default::default()
    }
}
#[doc = "Filter tasks by their current status state."]
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
    #[serde(rename = "TASK_STATE_CANCELED")]
    TaskStateCanceled,
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
            Self::TaskStateCanceled => f.write_str("TASK_STATE_CANCELED"),
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
            "TASK_STATE_CANCELED" => Ok(Self::TaskStateCanceled),
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
impl ::std::convert::TryFrom<::std::string::String> for TaskState {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A container for the status of a task"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TaskStatus {
    #[doc = "A message associated with the status."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub message: ::std::option::Option<Message>,
    #[doc = "The current state of this task."]
    pub state: TaskState,
    #[doc = "ISO 8601 Timestamp when the status was recorded.\n Example: \"2023-10-27T10:00:00Z\""]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub timestamp: ::std::option::Option<Timestamp>,
}
impl TaskStatus {
    pub fn builder() -> builder::TaskStatus {
        Default::default()
    }
}
#[doc = "An event sent by the agent to notify the client of a change in a task's status."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TaskStatusUpdateEvent {
    #[doc = "The ID of the context that the task belongs to."]
    #[serde(rename = "contextId")]
    pub context_id: ::std::string::String,
    #[doc = "Optional. Metadata associated with the task update."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<Struct>,
    #[doc = "The new status of the task."]
    pub status: TaskStatus,
    #[doc = "The ID of the task that has changed."]
    #[serde(rename = "taskId")]
    pub task_id: ::std::string::String,
}
impl TaskStatusUpdateEvent {
    pub fn builder() -> builder::TaskStatusUpdateEvent {
        Default::default()
    }
}
#[doc = "`Timestamp`"]
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
impl ::std::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
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
#[doc = "`Value`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Value(pub ::serde_json::Value);
impl ::std::ops::Deref for Value {
    type Target = ::serde_json::Value;
    fn deref(&self) -> &::serde_json::Value {
        &self.0
    }
}
impl ::std::convert::From<Value> for ::serde_json::Value {
    fn from(value: Value) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Value> for Value {
    fn from(value: ::serde_json::Value) -> Self {
        Self(value)
    }
}
#[doc = " Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct AgentCapabilities {
        extended_agent_card:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        extensions:
            ::std::result::Result<::std::vec::Vec<super::AgentExtension>, ::std::string::String>,
        push_notifications:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        streaming: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
    }
    impl ::std::default::Default for AgentCapabilities {
        fn default() -> Self {
            Self {
                extended_agent_card: Ok(Default::default()),
                extensions: Ok(Default::default()),
                push_notifications: Ok(Default::default()),
                streaming: Ok(Default::default()),
            }
        }
    }
    impl AgentCapabilities {
        pub fn extended_agent_card<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.extended_agent_card = value.try_into().map_err(|e| {
                format!("error converting supplied value for extended_agent_card: {e}")
            });
            self
        }
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
                extended_agent_card: value.extended_agent_card?,
                extensions: value.extensions?,
                push_notifications: value.push_notifications?,
                streaming: value.streaming?,
            })
        }
    }
    impl ::std::convert::From<super::AgentCapabilities> for AgentCapabilities {
        fn from(value: super::AgentCapabilities) -> Self {
            Self {
                extended_agent_card: Ok(value.extended_agent_card),
                extensions: Ok(value.extensions),
                push_notifications: Ok(value.push_notifications),
                streaming: Ok(value.streaming),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AgentCard {
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
        provider: ::std::result::Result<
            ::std::option::Option<super::AgentProvider>,
            ::std::string::String,
        >,
        security_requirements: ::std::result::Result<
            ::std::vec::Vec<super::SecurityRequirement>,
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
        supported_interfaces:
            ::std::result::Result<::std::vec::Vec<super::AgentInterface>, ::std::string::String>,
        version: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for AgentCard {
        fn default() -> Self {
            Self {
                capabilities: Err("no value supplied for capabilities".to_string()),
                default_input_modes: Err("no value supplied for default_input_modes".to_string()),
                default_output_modes: Err("no value supplied for default_output_modes".to_string()),
                description: Err("no value supplied for description".to_string()),
                documentation_url: Ok(Default::default()),
                icon_url: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                provider: Ok(Default::default()),
                security_requirements: Ok(Default::default()),
                security_schemes: Ok(Default::default()),
                signatures: Ok(Default::default()),
                skills: Err("no value supplied for skills".to_string()),
                supported_interfaces: Err("no value supplied for supported_interfaces".to_string()),
                version: Err("no value supplied for version".to_string()),
            }
        }
    }
    impl AgentCard {
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
        pub fn security_requirements<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::SecurityRequirement>>,
            T::Error: ::std::fmt::Display,
        {
            self.security_requirements = value.try_into().map_err(|e| {
                format!("error converting supplied value for security_requirements: {e}")
            });
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
                capabilities: value.capabilities?,
                default_input_modes: value.default_input_modes?,
                default_output_modes: value.default_output_modes?,
                description: value.description?,
                documentation_url: value.documentation_url?,
                icon_url: value.icon_url?,
                name: value.name?,
                provider: value.provider?,
                security_requirements: value.security_requirements?,
                security_schemes: value.security_schemes?,
                signatures: value.signatures?,
                skills: value.skills?,
                supported_interfaces: value.supported_interfaces?,
                version: value.version?,
            })
        }
    }
    impl ::std::convert::From<super::AgentCard> for AgentCard {
        fn from(value: super::AgentCard) -> Self {
            Self {
                capabilities: Ok(value.capabilities),
                default_input_modes: Ok(value.default_input_modes),
                default_output_modes: Ok(value.default_output_modes),
                description: Ok(value.description),
                documentation_url: Ok(value.documentation_url),
                icon_url: Ok(value.icon_url),
                name: Ok(value.name),
                provider: Ok(value.provider),
                security_requirements: Ok(value.security_requirements),
                security_schemes: Ok(value.security_schemes),
                signatures: Ok(value.signatures),
                skills: Ok(value.skills),
                supported_interfaces: Ok(value.supported_interfaces),
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
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        params: ::std::result::Result<::std::option::Option<super::Struct>, ::std::string::String>,
        required: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        uri: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AgentExtension {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                params: Ok(Default::default()),
                required: Ok(Default::default()),
                uri: Ok(Default::default()),
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
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.required = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for required: {e}"));
            self
        }
        pub fn uri<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
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
        protocol_version: ::std::result::Result<::std::string::String, ::std::string::String>,
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
                protocol_version: Err("no value supplied for protocol_version".to_string()),
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
                protocol_version: value.protocol_version?,
                tenant: value.tenant?,
                url: value.url?,
            })
        }
    }
    impl ::std::convert::From<super::AgentInterface> for AgentInterface {
        fn from(value: super::AgentInterface) -> Self {
            Self {
                protocol_binding: Ok(value.protocol_binding),
                protocol_version: Ok(value.protocol_version),
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
        security_requirements: ::std::result::Result<
            ::std::vec::Vec<super::SecurityRequirement>,
            ::std::string::String,
        >,
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
                security_requirements: Ok(Default::default()),
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
        pub fn security_requirements<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::SecurityRequirement>>,
            T::Error: ::std::fmt::Display,
        {
            self.security_requirements = value.try_into().map_err(|e| {
                format!("error converting supplied value for security_requirements: {e}")
            });
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
                security_requirements: value.security_requirements?,
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
                security_requirements: Ok(value.security_requirements),
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
        scheme: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for AuthenticationInfo {
        fn default() -> Self {
            Self {
                credentials: Ok(Default::default()),
                scheme: Err("no value supplied for scheme".to_string()),
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
    impl ::std::convert::TryFrom<AuthenticationInfo> for super::AuthenticationInfo {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AuthenticationInfo,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                credentials: value.credentials?,
                scheme: value.scheme?,
            })
        }
    }
    impl ::std::convert::From<super::AuthenticationInfo> for AuthenticationInfo {
        fn from(value: super::AuthenticationInfo) -> Self {
            Self {
                credentials: Ok(value.credentials),
                scheme: Ok(value.scheme),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AuthorizationCodeOAuthFlow {
        authorization_url: ::std::result::Result<::std::string::String, ::std::string::String>,
        pkce_required: ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
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
                pkce_required: Ok(Default::default()),
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
        pub fn pkce_required<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.pkce_required = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pkce_required: {e}"));
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
                pkce_required: value.pkce_required?,
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
                pkce_required: Ok(value.pkce_required),
                refresh_url: Ok(value.refresh_url),
                scopes: Ok(value.scopes),
                token_url: Ok(value.token_url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CancelTaskRequest {
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        metadata:
            ::std::result::Result<::std::option::Option<super::Struct>, ::std::string::String>,
        tenant: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CancelTaskRequest {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                metadata: Ok(Default::default()),
                tenant: Ok(Default::default()),
            }
        }
    }
    impl CancelTaskRequest {
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
    impl ::std::convert::TryFrom<CancelTaskRequest> for super::CancelTaskRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CancelTaskRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                metadata: value.metadata?,
                tenant: value.tenant?,
            })
        }
    }
    impl ::std::convert::From<super::CancelTaskRequest> for CancelTaskRequest {
        fn from(value: super::CancelTaskRequest) -> Self {
            Self {
                id: Ok(value.id),
                metadata: Ok(value.metadata),
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
    pub struct DeleteTaskPushNotificationConfigRequest {
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        task_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        tenant: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for DeleteTaskPushNotificationConfigRequest {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                task_id: Err("no value supplied for task_id".to_string()),
                tenant: Ok(Default::default()),
            }
        }
    }
    impl DeleteTaskPushNotificationConfigRequest {
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
    impl ::std::convert::TryFrom<DeleteTaskPushNotificationConfigRequest>
        for super::DeleteTaskPushNotificationConfigRequest
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeleteTaskPushNotificationConfigRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                task_id: value.task_id?,
                tenant: value.tenant?,
            })
        }
    }
    impl ::std::convert::From<super::DeleteTaskPushNotificationConfigRequest>
        for DeleteTaskPushNotificationConfigRequest
    {
        fn from(value: super::DeleteTaskPushNotificationConfigRequest) -> Self {
            Self {
                id: Ok(value.id),
                task_id: Ok(value.task_id),
                tenant: Ok(value.tenant),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DeviceCodeOAuthFlow {
        device_authorization_url:
            ::std::result::Result<::std::string::String, ::std::string::String>,
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
    impl ::std::default::Default for DeviceCodeOAuthFlow {
        fn default() -> Self {
            Self {
                device_authorization_url: Err(
                    "no value supplied for device_authorization_url".to_string()
                ),
                refresh_url: Ok(Default::default()),
                scopes: Err("no value supplied for scopes".to_string()),
                token_url: Err("no value supplied for token_url".to_string()),
            }
        }
    }
    impl DeviceCodeOAuthFlow {
        pub fn device_authorization_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.device_authorization_url = value.try_into().map_err(|e| {
                format!("error converting supplied value for device_authorization_url: {e}")
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
    impl ::std::convert::TryFrom<DeviceCodeOAuthFlow> for super::DeviceCodeOAuthFlow {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DeviceCodeOAuthFlow,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                device_authorization_url: value.device_authorization_url?,
                refresh_url: value.refresh_url?,
                scopes: value.scopes?,
                token_url: value.token_url?,
            })
        }
    }
    impl ::std::convert::From<super::DeviceCodeOAuthFlow> for DeviceCodeOAuthFlow {
        fn from(value: super::DeviceCodeOAuthFlow) -> Self {
            Self {
                device_authorization_url: Ok(value.device_authorization_url),
                refresh_url: Ok(value.refresh_url),
                scopes: Ok(value.scopes),
                token_url: Ok(value.token_url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GetExtendedAgentCardRequest {
        tenant: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for GetExtendedAgentCardRequest {
        fn default() -> Self {
            Self {
                tenant: Ok(Default::default()),
            }
        }
    }
    impl GetExtendedAgentCardRequest {
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
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        task_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        tenant: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for GetTaskPushNotificationConfigRequest {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                task_id: Err("no value supplied for task_id".to_string()),
                tenant: Ok(Default::default()),
            }
        }
    }
    impl GetTaskPushNotificationConfigRequest {
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
    impl ::std::convert::TryFrom<GetTaskPushNotificationConfigRequest>
        for super::GetTaskPushNotificationConfigRequest
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GetTaskPushNotificationConfigRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                task_id: value.task_id?,
                tenant: value.tenant?,
            })
        }
    }
    impl ::std::convert::From<super::GetTaskPushNotificationConfigRequest>
        for GetTaskPushNotificationConfigRequest
    {
        fn from(value: super::GetTaskPushNotificationConfigRequest) -> Self {
            Self {
                id: Ok(value.id),
                task_id: Ok(value.task_id),
                tenant: Ok(value.tenant),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GetTaskRequest {
        history_length: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        tenant: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for GetTaskRequest {
        fn default() -> Self {
            Self {
                history_length: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
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
                id: value.id?,
                tenant: value.tenant?,
            })
        }
    }
    impl ::std::convert::From<super::GetTaskRequest> for GetTaskRequest {
        fn from(value: super::GetTaskRequest) -> Self {
            Self {
                history_length: Ok(value.history_length),
                id: Ok(value.id),
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
        authorization_url: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
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
                authorization_url: Ok(Default::default()),
                refresh_url: Ok(Default::default()),
                scopes: Ok(Default::default()),
            }
        }
    }
    impl ImplicitOAuthFlow {
        pub fn authorization_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
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
    pub struct ListTaskPushNotificationConfigsRequest {
        page_size: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        page_token: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        task_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        tenant: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ListTaskPushNotificationConfigsRequest {
        fn default() -> Self {
            Self {
                page_size: Ok(Default::default()),
                page_token: Ok(Default::default()),
                task_id: Err("no value supplied for task_id".to_string()),
                tenant: Ok(Default::default()),
            }
        }
    }
    impl ListTaskPushNotificationConfigsRequest {
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
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.page_token = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for page_token: {e}"));
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
    impl ::std::convert::TryFrom<ListTaskPushNotificationConfigsRequest>
        for super::ListTaskPushNotificationConfigsRequest
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListTaskPushNotificationConfigsRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                page_size: value.page_size?,
                page_token: value.page_token?,
                task_id: value.task_id?,
                tenant: value.tenant?,
            })
        }
    }
    impl ::std::convert::From<super::ListTaskPushNotificationConfigsRequest>
        for ListTaskPushNotificationConfigsRequest
    {
        fn from(value: super::ListTaskPushNotificationConfigsRequest) -> Self {
            Self {
                page_size: Ok(value.page_size),
                page_token: Ok(value.page_token),
                task_id: Ok(value.task_id),
                tenant: Ok(value.tenant),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListTaskPushNotificationConfigsResponse {
        configs: ::std::result::Result<
            ::std::vec::Vec<super::TaskPushNotificationConfig>,
            ::std::string::String,
        >,
        next_page_token: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ListTaskPushNotificationConfigsResponse {
        fn default() -> Self {
            Self {
                configs: Ok(Default::default()),
                next_page_token: Ok(Default::default()),
            }
        }
    }
    impl ListTaskPushNotificationConfigsResponse {
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
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.next_page_token = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for next_page_token: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ListTaskPushNotificationConfigsResponse>
        for super::ListTaskPushNotificationConfigsResponse
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListTaskPushNotificationConfigsResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                configs: value.configs?,
                next_page_token: value.next_page_token?,
            })
        }
    }
    impl ::std::convert::From<super::ListTaskPushNotificationConfigsResponse>
        for ListTaskPushNotificationConfigsResponse
    {
        fn from(value: super::ListTaskPushNotificationConfigsResponse) -> Self {
            Self {
                configs: Ok(value.configs),
                next_page_token: Ok(value.next_page_token),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ListTasksRequest {
        context_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        history_length: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        include_artifacts:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        page_size: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        page_token: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        status:
            ::std::result::Result<::std::option::Option<super::TaskState>, ::std::string::String>,
        status_timestamp_after:
            ::std::result::Result<::std::option::Option<super::Timestamp>, ::std::string::String>,
        tenant: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ListTasksRequest {
        fn default() -> Self {
            Self {
                context_id: Ok(Default::default()),
                history_length: Ok(Default::default()),
                include_artifacts: Ok(Default::default()),
                page_size: Ok(Default::default()),
                page_token: Ok(Default::default()),
                status: Ok(Default::default()),
                status_timestamp_after: Ok(Default::default()),
                tenant: Ok(Default::default()),
            }
        }
    }
    impl ListTasksRequest {
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
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.page_token = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for page_token: {e}"));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TaskState>>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {e}"));
            self
        }
        pub fn status_timestamp_after<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Timestamp>>,
            T::Error: ::std::fmt::Display,
        {
            self.status_timestamp_after = value.try_into().map_err(|e| {
                format!("error converting supplied value for status_timestamp_after: {e}")
            });
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
    impl ::std::convert::TryFrom<ListTasksRequest> for super::ListTasksRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ListTasksRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                context_id: value.context_id?,
                history_length: value.history_length?,
                include_artifacts: value.include_artifacts?,
                page_size: value.page_size?,
                page_token: value.page_token?,
                status: value.status?,
                status_timestamp_after: value.status_timestamp_after?,
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
                page_size: Ok(value.page_size),
                page_token: Ok(value.page_token),
                status: Ok(value.status),
                status_timestamp_after: Ok(value.status_timestamp_after),
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
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for MutualTlsSecurityScheme {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
            }
        }
    }
    impl MutualTlsSecurityScheme {
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
        device_code: ::std::result::Result<
            ::std::option::Option<super::DeviceCodeOAuthFlow>,
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
                device_code: Ok(Default::default()),
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
        pub fn device_code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::DeviceCodeOAuthFlow>>,
            T::Error: ::std::fmt::Display,
        {
            self.device_code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for device_code: {e}"));
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
                device_code: value.device_code?,
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
                device_code: Ok(value.device_code),
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
        data: ::std::result::Result<::std::option::Option<super::Value>, ::std::string::String>,
        filename: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        media_type: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        metadata:
            ::std::result::Result<::std::option::Option<super::Struct>, ::std::string::String>,
        raw: ::std::result::Result<::std::option::Option<super::PartRaw>, ::std::string::String>,
        text: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        url: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Part {
        fn default() -> Self {
            Self {
                data: Ok(Default::default()),
                filename: Ok(Default::default()),
                media_type: Ok(Default::default()),
                metadata: Ok(Default::default()),
                raw: Ok(Default::default()),
                text: Ok(Default::default()),
                url: Ok(Default::default()),
            }
        }
    }
    impl Part {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {e}"));
            self
        }
        pub fn filename<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.filename = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for filename: {e}"));
            self
        }
        pub fn media_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.media_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for media_type: {e}"));
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
        pub fn raw<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::PartRaw>>,
            T::Error: ::std::fmt::Display,
        {
            self.raw = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for raw: {e}"));
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
    }
    impl ::std::convert::TryFrom<Part> for super::Part {
        type Error = super::error::ConversionError;
        fn try_from(value: Part) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                filename: value.filename?,
                media_type: value.media_type?,
                metadata: value.metadata?,
                raw: value.raw?,
                text: value.text?,
                url: value.url?,
            })
        }
    }
    impl ::std::convert::From<super::Part> for Part {
        fn from(value: super::Part) -> Self {
            Self {
                data: Ok(value.data),
                filename: Ok(value.filename),
                media_type: Ok(value.media_type),
                metadata: Ok(value.metadata),
                raw: Ok(value.raw),
                text: Ok(value.text),
                url: Ok(value.url),
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
        token_url: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PasswordOAuthFlow {
        fn default() -> Self {
            Self {
                refresh_url: Ok(Default::default()),
                scopes: Ok(Default::default()),
                token_url: Ok(Default::default()),
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
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
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
    pub struct SecurityRequirement {
        schemes: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, super::StringList>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for SecurityRequirement {
        fn default() -> Self {
            Self {
                schemes: Ok(Default::default()),
            }
        }
    }
    impl SecurityRequirement {
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
    impl ::std::convert::TryFrom<SecurityRequirement> for super::SecurityRequirement {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SecurityRequirement,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                schemes: value.schemes?,
            })
        }
    }
    impl ::std::convert::From<super::SecurityRequirement> for SecurityRequirement {
        fn from(value: super::SecurityRequirement) -> Self {
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
        history_length: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
        return_immediately:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        task_push_notification_config: ::std::result::Result<
            ::std::option::Option<super::TaskPushNotificationConfig>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for SendMessageConfiguration {
        fn default() -> Self {
            Self {
                accepted_output_modes: Ok(Default::default()),
                history_length: Ok(Default::default()),
                return_immediately: Ok(Default::default()),
                task_push_notification_config: Ok(Default::default()),
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
        pub fn return_immediately<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.return_immediately = value.try_into().map_err(|e| {
                format!("error converting supplied value for return_immediately: {e}")
            });
            self
        }
        pub fn task_push_notification_config<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TaskPushNotificationConfig>>,
            T::Error: ::std::fmt::Display,
        {
            self.task_push_notification_config = value.try_into().map_err(|e| {
                format!("error converting supplied value for task_push_notification_config: {e}")
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
                history_length: value.history_length?,
                return_immediately: value.return_immediately?,
                task_push_notification_config: value.task_push_notification_config?,
            })
        }
    }
    impl ::std::convert::From<super::SendMessageConfiguration> for SendMessageConfiguration {
        fn from(value: super::SendMessageConfiguration) -> Self {
            Self {
                accepted_output_modes: Ok(value.accepted_output_modes),
                history_length: Ok(value.history_length),
                return_immediately: Ok(value.return_immediately),
                task_push_notification_config: Ok(value.task_push_notification_config),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SendMessageRequest {
        configuration: ::std::result::Result<
            ::std::option::Option<super::SendMessageConfiguration>,
            ::std::string::String,
        >,
        message: ::std::result::Result<super::Message, ::std::string::String>,
        metadata:
            ::std::result::Result<::std::option::Option<super::Struct>, ::std::string::String>,
        tenant: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for SendMessageRequest {
        fn default() -> Self {
            Self {
                configuration: Ok(Default::default()),
                message: Err("no value supplied for message".to_string()),
                metadata: Ok(Default::default()),
                tenant: Ok(Default::default()),
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
            T: ::std::convert::TryInto<super::Message>,
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
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
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
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        tenant: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for SubscribeToTaskRequest {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                tenant: Ok(Default::default()),
            }
        }
    }
    impl SubscribeToTaskRequest {
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
    impl ::std::convert::TryFrom<SubscribeToTaskRequest> for super::SubscribeToTaskRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SubscribeToTaskRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                tenant: value.tenant?,
            })
        }
    }
    impl ::std::convert::From<super::SubscribeToTaskRequest> for SubscribeToTaskRequest {
        fn from(value: super::SubscribeToTaskRequest) -> Self {
            Self {
                id: Ok(value.id),
                tenant: Ok(value.tenant),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Task {
        artifacts: ::std::result::Result<::std::vec::Vec<super::Artifact>, ::std::string::String>,
        context_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
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
                context_id: Ok(Default::default()),
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
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
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
        authentication: ::std::result::Result<
            ::std::option::Option<super::AuthenticationInfo>,
            ::std::string::String,
        >,
        id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        task_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        tenant: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        token: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        url: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for TaskPushNotificationConfig {
        fn default() -> Self {
            Self {
                authentication: Ok(Default::default()),
                id: Ok(Default::default()),
                task_id: Ok(Default::default()),
                tenant: Ok(Default::default()),
                token: Ok(Default::default()),
                url: Err("no value supplied for url".to_string()),
            }
        }
    }
    impl TaskPushNotificationConfig {
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
    impl ::std::convert::TryFrom<TaskPushNotificationConfig> for super::TaskPushNotificationConfig {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TaskPushNotificationConfig,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                authentication: value.authentication?,
                id: value.id?,
                task_id: value.task_id?,
                tenant: value.tenant?,
                token: value.token?,
                url: value.url?,
            })
        }
    }
    impl ::std::convert::From<super::TaskPushNotificationConfig> for TaskPushNotificationConfig {
        fn from(value: super::TaskPushNotificationConfig) -> Self {
            Self {
                authentication: Ok(value.authentication),
                id: Ok(value.id),
                task_id: Ok(value.task_id),
                tenant: Ok(value.tenant),
                token: Ok(value.token),
                url: Ok(value.url),
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
        metadata:
            ::std::result::Result<::std::option::Option<super::Struct>, ::std::string::String>,
        status: ::std::result::Result<super::TaskStatus, ::std::string::String>,
        task_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for TaskStatusUpdateEvent {
        fn default() -> Self {
            Self {
                context_id: Err("no value supplied for context_id".to_string()),
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
                metadata: Ok(value.metadata),
                status: Ok(value.status),
                task_id: Ok(value.task_id),
            }
        }
    }
}
#[doc = " Error types."]
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

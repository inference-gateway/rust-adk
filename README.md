<h1 align="center">Agent Development Kit (ADK) - Rust</h1>

<p align="center">
  <strong>Build powerful, interoperable AI agents with the Agent-to-Agent (A2A) protocol</strong>
</p>

> ⚠️ **Early Stage Warning**: This project is in its early stages of development. Breaking changes are expected as we iterate and improve the API. Please use pinned versions in production environments and be prepared to update your code when upgrading versions.

<p align="center">
  <!-- CI Status Badge -->
  <a href="https://github.com/inference-gateway/rust-adk/actions/workflows/ci.yml?query=branch%3Amain">
    <img src="https://github.com/inference-gateway/rust-adk/actions/workflows/ci.yml/badge.svg?branch=main" alt="CI Status"/>
  </a>
  <!-- Version Badge -->
  <a href="https://github.com/inference-gateway/rust-adk/releases">
    <img src="https://img.shields.io/github/v/release/inference-gateway/rust-adk?color=blue&style=flat-square" alt="Version"/>
  </a>
  <!-- License Badge -->
  <a href="https://github.com/inference-gateway/rust-adk/blob/main/LICENSE">
    <img src="https://img.shields.io/github/license/inference-gateway/rust-adk?color=blue&style=flat-square" alt="License"/>
  </a>
  <!-- Rust Version -->
  <img src="https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fraw.githubusercontent.com%2Finference-gateway%2Frust-adk%2Fmain%2FCargo.toml&query=%24.package.rust-version&label=rust&suffix=%2B&color=blue&style=flat-square" alt="Rust Version"/>
</p>

---

## Table of Contents

- [Table of Contents](#table-of-contents)
- [Overview](#overview)
  - [What is A2A?](#what-is-a2a)
- [Quick Start](#quick-start)
  - [Installation](#installation)
  - [Basic Usage (Minimal Server)](#basic-usage-minimal-server)
  - [AI-Powered Server](#ai-powered-server)
  - [Health Check Example](#health-check-example)
  - [Examples](#examples)
- [Key Features](#key-features)
  - [Core Capabilities](#core-capabilities)
  - [Developer Experience](#developer-experience)
  - [Enterprise Ready](#enterprise-ready)
- [Development](#development)
  - [Prerequisites](#prerequisites)
  - [Development Workflow](#development-workflow)
  - [Available Tasks](#available-tasks)
  - [Build-Time Agent Metadata](#build-time-agent-metadata)
    - [Available Build-Time Variables](#available-build-time-variables)
    - [Usage Examples](#usage-examples)
- [API Reference](#api-reference)
  - [Core Components](#core-components)
    - [A2AServer](#a2aserver)
    - [A2AServerBuilder](#a2aserverbuilder)
    - [AgentBuilder](#agentbuilder)
    - [A2AClient](#a2aclient)
      - [A2A JSON-RPC methods](#a2a-json-rpc-methods)
    - [Agent Health Monitoring](#agent-health-monitoring)
    - [LLM Client](#llm-client)
  - [Configuration](#configuration)
- [Advanced Usage](#advanced-usage)
  - [Building Custom Agents with AgentBuilder](#building-custom-agents-with-agentbuilder)
    - [Basic Agent Creation](#basic-agent-creation)
    - [Agent with Custom Configuration](#agent-with-custom-configuration)
    - [Agent with Custom LLM Client](#agent-with-custom-llm-client)
    - [Fully Configured Agent](#fully-configured-agent)
  - [Custom Tools](#custom-tools)
  - [Custom Task Processing](#custom-task-processing)
  - [Push Notifications](#push-notifications)
    - [Webhook Payload](#webhook-payload)
  - [Agent Metadata](#agent-metadata)
    - [Build-Time Metadata (Recommended)](#build-time-metadata-recommended)
    - [Runtime Metadata Configuration](#runtime-metadata-configuration)
  - [TLS and mTLS](#tls-and-mtls)
  - [Environment Configuration](#environment-configuration)
- [A2A Ecosystem](#a2a-ecosystem)
  - [Related Projects](#related-projects)
  - [A2A Agents](#a2a-agents)
- [Requirements](#requirements)
- [OCI Compliant](#oci-compliant)
- [Testing](#testing)
- [License](#license)
- [Contributing](#contributing)
  - [Getting Started](#getting-started)
  - [Development Guidelines](#development-guidelines)
  - [Before Submitting](#before-submitting)
  - [Pull Request Process](#pull-request-process)
- [Support](#support)
  - [Issues \& Questions](#issues--questions)
- [Resources](#resources)
  - [Documentation](#documentation)

---

## Overview

The **A2A ADK (Agent Development Kit)** is a Rust library that simplifies building [Agent-to-Agent (A2A) protocol](https://github.com/inference-gateway/schemas/tree/main/a2a) compatible agents. A2A enables seamless communication between AI agents, allowing them to collaborate, delegate tasks, and share capabilities across different systems and providers.

### What is A2A?

Agent-to-Agent (A2A) is a standardized protocol that enables AI agents to:

- **Communicate** with each other using a unified JSON-RPC interface
- **Delegate tasks** to specialized agents with specific capabilities
- **Stream responses** in real-time for better user experience
- **Authenticate** securely using OIDC/OAuth2
- **Discover capabilities** through standardized agent cards

## Quick Start

### Installation

Add the ADK to your `Cargo.toml`:

```toml
[dependencies]
inference-gateway-adk = "0.4.0"
```

### Basic Usage (Minimal Server)

```rust
use inference_gateway_adk::A2AServerBuilder;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    // Smallest possible A2A server — no agent, no custom handlers.
    // Health, agent card, and JSON-RPC routes are all wired in by the builder.
    let server = A2AServerBuilder::new().build().await?;

    let addr = "0.0.0.0:8080".parse()?;
    info!("A2A server listening on {addr}");

    if let Err(e) = server.serve(addr).await {
        error!("server stopped: {e}");
    }
    Ok(())
}
```

### AI-Powered Server

`Config` is plain `serde`; pick whichever loader you like. The bundled
examples use [`envy`][envy] with the `A2A_` prefix — that's the convention
adopted by the sibling Go and TypeScript ADKs. With `A2A_AGENT_CLIENT_*`
env vars set, `AgentBuilder` produces a fully wired LLM agent:

```rust
use inference_gateway_adk::{A2AServerBuilder, AgentBuilder, Config};
use inference_gateway_sdk::{
    ChatCompletionTool, ChatCompletionToolType, FunctionObject, FunctionParameters,
};
use serde_json::{Value, json};
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    // Load `A2A_AGENT_CLIENT_PROVIDER`, `A2A_AGENT_CLIENT_MODEL`,
    // `A2A_AGENT_CLIENT_API_KEY`, `A2A_SERVER_PORT`, etc. AgentBuilder
    // fails fast at startup if provider/model are missing.
    let config: Config = envy::prefixed("A2A_").from_env()?;

    let tools = vec![ChatCompletionTool {
        type_: ChatCompletionToolType::Function,
        function: FunctionObject {
            name: "get_weather".to_string(),
            description: Some("Get weather information for a city".to_string()),
            parameters: Some(FunctionParameters(
                json!({
                    "type": "object",
                    "properties": {
                        "location": { "type": "string", "description": "City name" }
                    },
                    "required": ["location"]
                })
                .as_object()
                .unwrap()
                .clone(),
            )),
            strict: false,
        },
    }];

    let agent = AgentBuilder::new()
        .with_config(&config.agent_config)
        .with_system_prompt("You are a helpful weather assistant.")
        .with_toolbox(tools)
        .with_function_tool("get_weather".to_string(), |args: Value| {
            let location = args["location"].as_str().unwrap_or("Unknown");
            Ok(json!({ "location": location, "temperature": "22°C" }).to_string())
        })
        .build()
        .await?;

    let port = config.server_config.port;
    let server = A2AServerBuilder::new()
        .with_config(config)
        .with_agent(agent)
        .with_agent_card_from_file(".well-known/agent.json", None)
        .with_default_task_handlers()
        .build()
        .await?;

    let addr = format!("0.0.0.0:{port}").parse()?;
    info!("AI-powered A2A server running on {addr}");

    if let Err(e) = server.serve(addr).await {
        error!("Server failed to start: {e}");
    }
    Ok(())
}
```

[envy]: https://docs.rs/envy

### Health Check Example

Monitor the health status of A2A agents for service discovery and load balancing:

```rust
use inference_gateway_adk::client::A2AClient;
use tokio::time::{sleep, Duration};
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::init();

    // Create client
    let client = A2AClient::new("http://localhost:8080")?;

    // Single health check
    match client.get_health().await {
        Ok(health) => info!("Agent health: {}", health.status),
        Err(e) => {
            error!("Health check failed: {}", e);
            return Ok(());
        }
    }

    // Periodic health monitoring
    loop {
        sleep(Duration::from_secs(30)).await;

        match client.get_health().await {
            Ok(health) => match health.status.as_str() {
                "healthy" => info!("[{}] Agent is healthy", chrono::Utc::now().format("%H:%M:%S")),
                "degraded" => info!("[{}] Agent is degraded - some functionality may be limited", chrono::Utc::now().format("%H:%M:%S")),
                "unhealthy" => info!("[{}] Agent is unhealthy - may not be able to process requests", chrono::Utc::now().format("%H:%M:%S")),
                _ => info!("[{}] Unknown health status: {}", chrono::Utc::now().format("%H:%M:%S"), health.status),
            },
            Err(e) => error!("Health check failed: {}", e),
        }
    }
}
```

### Examples

For complete working examples, see the [examples](./examples/) directory.
The catalogue is grouped by whether a scenario needs an LLM provider; see
[`examples/README.md`](./examples/README.md) for the full table and a
suggested learning path.

**Without AI** (no Inference Gateway, no provider keys):

- **[Minimal](./examples/minimal/)** - Bare A2A server + client, no agent (built-in default echo reply)
- **[Static Agent Card](./examples/static-agent-card/)** - Load agent metadata from JSON with `AgentCardOverrides`
- **[Streaming](./examples/streaming/)** - Custom `StreamableTaskHandler` emits a sentence word-by-word over SSE
- **[Input Required](./examples/input-required/)** - Handler chooses `TaskStateInputRequired` when the user message is incomplete

**With AI** (Inference Gateway container + provider key):

- **[Default Handlers](./examples/default-handlers/)** - LLM agent + `with_default_task_handlers()`, no custom handler code
- **[AI Powered](./examples/ai-powered/)** - LLM agent with custom function tools (weather, math, search)
- **[AI Powered Streaming](./examples/ai-powered-streaming/)** - LLM agent streamed over `message/stream`

**Storage & protocol coverage:**

- **[Queue Storage](./examples/queue-storage/)** - Queue-driven `message/send` with in-memory or Redis storage (Compose profiles)
- **[A2A Methods](./examples/a2a-methods/)** - One client binary per JSON-RPC method exposed by the A2A spec
- **[Auth](./examples/auth/)** - Bearer-token authentication on `POST /a2a` with public `/health` and `/.well-known/agent.json`
- **[TLS / mTLS](./examples/tls/)** - TLS termination via `axum-server` + `rustls`, optional mTLS with client-cert subject as principal
- **[Health Check Example](#health-check-example)** - Monitor agent health status

## Key Features

### Core Capabilities

- 🤖 **A2A Protocol Compliance**: Full implementation of the Agent-to-Agent communication standard
- 🔌 **Multi-Provider Support**: Works with OpenAI, Ollama, Groq, Cohere, and other LLM providers
- 🌊 **Real-time Streaming**: Stream responses as they're generated from language models
- 🔧 **Custom Tools**: Easy integration of custom tools and capabilities
- 🔐 **Secure Authentication**: Built-in OIDC/OAuth2 authentication support
- 📨 **Push Notifications**: Webhook notifications for real-time task state updates

### Developer Experience

- ⚙️ **Environment Configuration**: Simple setup through environment variables
- 📊 **Task Management**: Built-in task queuing, polling, and lifecycle management
- 🏗️ **Extensible Architecture**: Pluggable components for custom business logic
- 📚 **Type-Safe**: Generated types from A2A schema for compile-time safety
- 🧪 **Well Tested**: Comprehensive test coverage with table-driven tests

### Enterprise Ready

- 🌿 **Lightweight**: Optimized binary size with Rust's zero-cost abstractions
- 🛡️ **Production Hardened**: Configurable timeouts, TLS support, and error handling
- 🐳 **Containerized**: OCI compliant and works with Docker and Docker Compose
- ☸️ **Kubernetes Native**: Ready for cloud-native deployments
- 📊 **Observability**: OpenTelemetry integration for monitoring and tracing

## Development

### Prerequisites

- Rust 1.94 or later
- [Task](https://taskfile.dev/) for build automation (optional, can use `cargo` directly)

### Development Workflow

1. **Download latest A2A schema**:

   ```bash
   task a2a:download-schema
   ```

2. **Generate types from schema**:

   ```bash
   task a2a:generate-types
   ```

3. **Run linting**:

   ```bash
   task lint
   ```

4. **Run tests**:
   ```bash
   task test
   ```

### Available Tasks

| Task                       | Description                                 |
| -------------------------- | ------------------------------------------- |
| `task a2a:download-schema` | Download the latest A2A schema              |
| `task a2a:generate-types`  | Generate Rust types from A2A schema         |
| `task lint`                | Run static analysis and linting with clippy |
| `task test`                | Run all tests                               |
| `task build`               | Build the project                           |
| `task clean`               | Clean up build artifacts                    |

### Build-Time Agent Metadata

The ADK supports injecting agent metadata at build time using Rust's build script and environment variables. This makes agent information immutable and embedded in the binary, which is useful for production deployments.

#### Available Build-Time Variables

The following build-time metadata variables can be set:

- **`AGENT_NAME`** - The agent's display name
- **`AGENT_DESCRIPTION`** - A description of the agent's capabilities
- **`AGENT_VERSION`** - The agent's version number

#### Usage Examples

**Direct Cargo Build:**

```bash
# Build your application with custom metadata
AGENT_NAME="MyAgent" \
AGENT_DESCRIPTION="My custom agent description" \
AGENT_VERSION="1.2.3" \
cargo build --release
```

**Docker Build:**

```dockerfile
# Build with custom metadata in Docker
FROM rust:1.94 AS builder

ARG AGENT_NAME="Production Agent"
ARG AGENT_DESCRIPTION="Production deployment agent with enhanced capabilities"
ARG AGENT_VERSION="1.0.0"

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch

COPY . .
RUN AGENT_NAME="${AGENT_NAME}" \
    AGENT_DESCRIPTION="${AGENT_DESCRIPTION}" \
    AGENT_VERSION="${AGENT_VERSION}" \
    cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/rust-adk .
CMD ["./rust-adk"]
```

## API Reference

### Core Components

#### A2AServer

The main server trait that handles A2A protocol communication.

```rust
use inference_gateway_adk::server::{A2AServer, A2AServerBuilder};

// Create a default A2A server
let server = A2AServerBuilder::new()
    .build()
    .await?;

// Create a server with agent integration
let server = A2AServerBuilder::new()
    .with_agent(agent)
    .with_agent_card_from_file(".well-known/agent.json")
    .build()
    .await?;

// Create a server with custom configuration
let server = A2AServerBuilder::new()
    .with_config(config)
    .with_task_handler(custom_task_handler)
    .with_task_processor(custom_processor)
    .build()
    .await?;
```

#### A2AServerBuilder

Build A2A servers with custom configurations using a fluent interface:

```rust
use inference_gateway_adk::server::{A2AServerBuilder, AgentBuilder};

// Basic server with agent
let server = A2AServerBuilder::new()
    .with_agent(agent)
    .with_agent_card_from_file(".well-known/agent.json")
    .build()
    .await?;

// Server with custom task handler
let server = A2AServerBuilder::new()
    .with_task_handler(custom_task_handler)
    .with_task_processor(custom_processor)
    .with_agent_card_from_file(".well-known/agent.json")
    .build()
    .await?;

// Server with custom configuration
let server = A2AServerBuilder::new()
    .with_config(config)
    .with_agent(agent)
    .with_agent_card_from_file(".well-known/agent.json")
    .build()
    .await?;
```

#### AgentBuilder

Build OpenAI-compatible agents that live inside the A2A server using a fluent interface:

```rust
use inference_gateway_adk::server::AgentBuilder;

// Basic agent with custom LLM
let agent = AgentBuilder::new()
    .with_config(&config)
    .with_toolbox(tools)
    .build()
    .await?;

// Agent with system prompt
let agent = AgentBuilder::new()
    .with_system_prompt("You are a helpful assistant")
    .with_max_chat_completion(10)
    .build()
    .await?;

// Use with A2A server builder
let server = A2AServerBuilder::new()
    .with_agent(agent)
    .with_agent_card_from_file(".well-known/agent.json")
    .build()
    .await?;
```

#### A2AClient

The client struct for communicating with A2A servers:

```rust
use inference_gateway_adk::A2AClient;

// Basic client creation
let client = A2AClient::new("http://localhost:8080")?;

// Client with custom configuration
let config = ClientConfig {
    base_url: "http://localhost:8080".to_string(),
    timeout: Duration::from_secs(45),
    max_retries: 5,
};
let client = A2AClient::with_config(config)?;

// Discovery endpoints
let agent_card = client.get_agent_card().await?;
let health = client.get_health().await?;

// Raw JSON-RPC envelope (escape hatch - most callers prefer the typed
// helpers documented in the section below)
let response = client.send_task(params).await?;
client.send_task_streaming(params, event_handler).await?;
```

##### A2A JSON-RPC methods

`A2AClient` exposes a typed helper for every method in the A2A specification.
Each helper takes a request struct and returns the matching response struct
from [`inference_gateway_adk::a2a_types`](src/a2a_types.rs). Runnable
end-to-end examples live in
[`examples/a2a-methods/`](examples/a2a-methods/README.md) - one client
binary per method.

| Method                                        | `A2AClient` helper                          | Request type                                  | Response type                            |
| --------------------------------------------- | ------------------------------------------- | --------------------------------------------- | ---------------------------------------- |
| `message/send`                                | `send_message`                              | `SendMessageRequest`                          | `SendMessageResponse`                    |
| `message/stream`                              | `send_streaming_message`                    | `SendMessageRequest`                          | `SendMessageResponse`                    |
| `tasks/get`                                   | `get_task`                                  | `GetTaskRequest`                              | `Task`                                   |
| `tasks/list`                                  | `list_tasks`                                | `ListTasksRequest`                            | `ListTasksResponse`                      |
| `tasks/cancel`                                | `cancel_task`                               | `CancelTaskRequest`                           | `Task`                                   |
| `tasks/resubscribe`                           | `resubscribe_task`                          | `SubscribeToTaskRequest`                      | `Stream<StreamResponse>` (SSE)           |
| `tasks/pushNotificationConfig/set`            | `set_task_push_notification_config`         | `SetTaskPushNotificationConfigRequest`        | `TaskPushNotificationConfig`             |
| `tasks/pushNotificationConfig/get`            | `get_task_push_notification_config`         | `GetTaskPushNotificationConfigRequest`        | `TaskPushNotificationConfig`             |
| `tasks/pushNotificationConfig/list`           | `list_task_push_notification_configs`       | `ListTaskPushNotificationConfigRequest`       | `ListTaskPushNotificationConfigResponse` |
| `tasks/pushNotificationConfig/delete`         | `delete_task_push_notification_config`      | `DeleteTaskPushNotificationConfigRequest`     | `serde_json::Value`                      |
| `agent/getAuthenticatedExtendedCard`          | `get_authenticated_extended_card`           | `GetExtendedAgentCardRequest`                 | `AgentCard`                              |

###### `message/send`

```rust
use inference_gateway_adk::a2a_types::{Message, Part, Role, SendMessageRequest};

let response = client
    .send_message(SendMessageRequest {
        configuration: None,
        message: Some(Message {
            context_id: None,
            extensions: vec![],
            message_id: uuid::Uuid::new_v4().to_string(),
            metadata: None,
            parts: vec![Part {
                data: None,
                file: None,
                metadata: None,
                text: Some("Hello via message/send".to_string()),
            }],
            reference_task_ids: vec![],
            role: Role::RoleUser,
            task_id: None,
        }),
        metadata: None,
        tenant: "example".to_string(),
    })
    .await?;

let task = response.task.expect("server returned a task");
```

###### `message/stream`

Same request shape as `message/send`; in the current client the response is
delivered as a single payload (true server-sent events arrive in a follow-up
ticket).

```rust
let response = client.send_streaming_message(request).await?;
```

###### `tasks/get`

```rust
use inference_gateway_adk::a2a_types::GetTaskRequest;

let task = client
    .get_task(GetTaskRequest {
        history_length: None,
        name: format!("tasks/{task_id}"),
        tenant: Some("example".to_string()),
    })
    .await?;
```

###### `tasks/list`

```rust
use inference_gateway_adk::a2a_types::{ListTasksRequest, TaskState};

let page = client
    .list_tasks(ListTasksRequest {
        context_id: String::new(),
        history_length: None,
        include_artifacts: None,
        last_updated_after: 0,
        page_size: Some(50),
        page_token: String::new(),
        status: TaskState::TaskStateUnspecified,
        tenant: "example".to_string(),
    })
    .await?;
```

###### `tasks/cancel`

```rust
use inference_gateway_adk::a2a_types::CancelTaskRequest;

let cancelled = client
    .cancel_task(CancelTaskRequest {
        name: format!("tasks/{task_id}"),
        tenant: "example".to_string(),
    })
    .await?;
```

###### `tasks/resubscribe`

Re-attach to an already-running task and stream subsequent state
transitions over SSE. The first event carries a snapshot of the task at
the current status; later events are `TaskStatusUpdateEvent` deltas. The
stream terminates after the server emits an event with `final: true`.

```rust
use futures::StreamExt;
use inference_gateway_adk::a2a_types::SubscribeToTaskRequest;

let mut stream = Box::pin(
    client
        .resubscribe_task(SubscribeToTaskRequest {
            name: format!("tasks/{task_id}"),
            tenant: "example".to_string(),
        })
        .await?,
);

while let Some(event) = stream.next().await {
    let event = event?;
    if let Some(update) = event.status_update.as_ref() {
        println!("task is now {:?}", update.status.state);
        if update.final_ {
            break;
        }
    }
}
```

###### `tasks/pushNotificationConfig/set`

```rust
use inference_gateway_adk::a2a_types::{
    PushNotificationConfig, SetTaskPushNotificationConfigRequest, TaskPushNotificationConfig,
};

let parent = format!("tasks/{task_id}");
let name = format!("{parent}/pushNotificationConfigs/primary");

client
    .set_task_push_notification_config(SetTaskPushNotificationConfigRequest {
        parent: parent.clone(),
        config_id: "primary".to_string(),
        tenant: Some("example".to_string()),
        config: TaskPushNotificationConfig {
            name: name.clone(),
            push_notification_config: PushNotificationConfig {
                authentication: None,
                id: None,
                token: Some("shared-secret".to_string()),
                url: "https://your-app.example/webhooks/a2a".to_string(),
            },
        },
    })
    .await?;
```

###### `tasks/pushNotificationConfig/get`

```rust
use inference_gateway_adk::a2a_types::GetTaskPushNotificationConfigRequest;

let cfg = client
    .get_task_push_notification_config(GetTaskPushNotificationConfigRequest {
        name: name.clone(),
        tenant: "example".to_string(),
    })
    .await?;
```

###### `tasks/pushNotificationConfig/list`

```rust
use inference_gateway_adk::a2a_types::ListTaskPushNotificationConfigRequest;

let listed = client
    .list_task_push_notification_configs(ListTaskPushNotificationConfigRequest {
        parent: parent.clone(),
        page_size: 10,
        page_token: String::new(),
        tenant: "example".to_string(),
    })
    .await?;
```

###### `tasks/pushNotificationConfig/delete`

```rust
use inference_gateway_adk::a2a_types::DeleteTaskPushNotificationConfigRequest;

client
    .delete_task_push_notification_config(DeleteTaskPushNotificationConfigRequest {
        name: name.clone(),
        tenant: "example".to_string(),
    })
    .await?;
```

###### `agent/getAuthenticatedExtendedCard`

Fetch the authenticated extended [`AgentCard`] for the calling tenant.
The server only honours the request when the agent card it serves at
`/.well-known/agent.json` advertises `supportsExtendedAgentCard: true`;
otherwise the call surfaces a JSON-RPC `METHOD_NOT_FOUND` error so the
client can fall back to the unauthenticated card.

```rust
use inference_gateway_adk::a2a_types::GetExtendedAgentCardRequest;

let card = client
    .get_authenticated_extended_card(GetExtendedAgentCardRequest {
        tenant: "example".to_string(),
    })
    .await?;
```

#### Agent Health Monitoring

Monitor the health status of A2A agents to ensure they are operational:

```rust
use inference_gateway_adk::client::A2AClient;

// Check agent health
let health = client.get_health().await?;

// Process health status
match health.status.as_str() {
    "healthy" => println!("Agent is healthy"),
    "degraded" => println!("Agent is degraded - some functionality may be limited"),
    "unhealthy" => println!("Agent is unhealthy - may not be able to process requests"),
    _ => println!("Unknown health status: {}", health.status),
}
```

**Health Status Values:**

- `healthy`: Agent is fully operational
- `degraded`: Agent is partially operational (some functionality may be limited)
- `unhealthy`: Agent is not operational or experiencing significant issues

**Use Cases:**

- Monitor agent availability in distributed systems
- Implement health checks for load balancers
- Detect and respond to agent failures
- Service discovery and routing decisions

#### LLM Client

Custom LLM transports are pluggable via the `LLMClient` trait. The bundled
`OpenAICompatibleLLMClient` wraps the Inference Gateway SDK and is what
`AgentBuilder` constructs by default when no client is supplied:

```rust
use inference_gateway_adk::{AgentBuilder, OpenAICompatibleLLMClient};

// Build the default OpenAI-compatible client from an AgentConfig
let llm_client = OpenAICompatibleLLMClient::new(&config.agent_config)?;

// Plug it into the agent (or implement `LLMClient` for a custom backend)
let agent = AgentBuilder::new()
    .with_llm_client(llm_client)
    .build()
    .await?;
```

The trait exposes two methods - `create_chat_completion` (non-streaming)
and `create_streaming_chat_completion` - mirroring the Go ADK's
`LLMClient` interface. Implement it manually to route requests through a
different backend (e.g. a mock for tests).

### Configuration

The configuration is managed through environment variables and the config module:

```rust
use inference_gateway_adk::config::{Config, AgentConfig};

#[derive(Debug, Clone)]
pub struct Config {
    pub agent_url: String,                           // AGENT_URL (default: http://helloworld-agent:8080)
    pub debug: bool,                                 // DEBUG (default: false)
    pub port: u16,                                   // PORT (default: 8080)
    pub streaming_status_update_interval: Duration,  // STREAMING_STATUS_UPDATE_INTERVAL (default: 1s)
    pub agent_config: AgentConfig,                   // AGENT_CLIENT_*
    pub capabilities_config: CapabilitiesConfig,     // CAPABILITIES_*
    pub tls_config: Option<TlsConfig>,               // TLS_*
    pub auth_config: Option<AuthConfig>,             // AUTH_*
    pub queue_config: QueueConfig,                   // QUEUE_*
    pub server_config: ServerConfig,                 // SERVER_*
    pub telemetry_config: TelemetryConfig,           // TELEMETRY_*
}

#[derive(Debug, Clone)]
pub struct AgentConfig {
    pub provider: String,                    // AGENT_CLIENT_PROVIDER
    pub model: String,                       // AGENT_CLIENT_MODEL
    pub base_url: Option<String>,            // AGENT_CLIENT_BASE_URL
    pub api_key: Option<String>,             // AGENT_CLIENT_API_KEY
    pub timeout: Duration,                   // AGENT_CLIENT_TIMEOUT (default: 30s)
    pub max_retries: u32,                    // AGENT_CLIENT_MAX_RETRIES (default: 3)
    pub max_chat_completion_iterations: u32, // AGENT_CLIENT_MAX_CHAT_COMPLETION_ITERATIONS (default: 10)
    pub max_tokens: u32,                     // AGENT_CLIENT_MAX_TOKENS (default: 4096)
    pub temperature: f32,                    // AGENT_CLIENT_TEMPERATURE (default: 0.7)
    pub system_prompt: Option<String>,       // AGENT_CLIENT_SYSTEM_PROMPT
}
```

## Advanced Usage

### Building Custom Agents with AgentBuilder

The `AgentBuilder` provides a fluent interface for creating highly customized agents with specific configurations, LLM clients, and toolboxes.

#### Basic Agent Creation

```rust
use inference_gateway_adk::server::AgentBuilder;
use tracing;

// Create a simple agent with defaults
let agent = AgentBuilder::new()
    .build()
    .await?;

// Or use the builder pattern for more control
let agent = AgentBuilder::new()
    .with_system_prompt("You are a helpful AI assistant specialized in customer support.")
    .with_max_chat_completion(15)
    .with_max_conversation_history(30)
    .build()
    .await?;
```

#### Agent with Custom Configuration

```rust
use inference_gateway_adk::config::AgentConfig;
use std::time::Duration;

let config = AgentConfig {
    provider: "deepseek".to_string(),
    model: "deepseek-v4-flash".to_string(),
    api_key: Some("your-api-key".to_string()),
    max_tokens: 4096,
    temperature: 0.7,
    max_chat_completion_iterations: 10,
    max_conversation_history: 20,
    system_prompt: Some("You are a travel planning assistant.".to_string()),
    ..Default::default()
};

let agent = AgentBuilder::new()
    .with_config(&config)
    .build()
    .await?;
```

#### Agent with Custom LLM Client

```rust
use inference_gateway_adk::{AgentBuilder, OpenAICompatibleLLMClient};

// Build the default OpenAI-compatible client (synchronous; no `await`)
let llm_client = OpenAICompatibleLLMClient::new(&config.agent_config)?;

// Build agent with the custom client
let agent = AgentBuilder::new()
    .with_llm_client(llm_client)
    .with_system_prompt("You are a coding assistant.")
    .build()
    .await?;
```

To plug in a non-OpenAI backend, implement the `LLMClient` trait directly
and pass your type to `.with_llm_client(...)`:

```rust
use inference_gateway_adk::LLMClient;

#[derive(Debug)]
struct MyCustomLLM;

#[async_trait::async_trait]
impl LLMClient for MyCustomLLM {
    async fn create_chat_completion(/* ... */) -> anyhow::Result<_> { /* ... */ }
    fn create_streaming_chat_completion(/* ... */) -> _ { /* ... */ }
}
```

#### Fully Configured Agent

```rust
use inference_gateway_adk::server::AgentBuilder;
use inference_gateway_sdk::{Tool, ToolType, FunctionObject};
use serde_json::json;

// Create tools for the agent's toolbox
let tools = vec![
    Tool {
        r#type: ToolType::Function,
        function: FunctionObject {
            name: "get_weather".to_string(),
            description: "Get current weather for a location".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "location": {
                        "type": "string",
                        "description": "The city and state, e.g. San Francisco, CA"
                    },
                    "unit": {
                        "type": "string",
                        "enum": ["celsius", "fahrenheit"],
                        "description": "Temperature unit"
                    }
                },
                "required": ["location"]
            }),
        },
    },
    Tool {
        r#type: ToolType::Function,
        function: FunctionObject {
            name: "calculate".to_string(),
            description: "Perform basic mathematical calculations".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "expression": {
                        "type": "string",
                        "description": "Mathematical expression to evaluate"
                    }
                },
                "required": ["expression"]
            }),
        },
    },
];

// Build a fully configured agent with toolbox
let agent = AgentBuilder::new()
    .with_config(&config)
    .with_system_prompt("You are a helpful assistant with weather and calculation capabilities.")
    .with_max_chat_completion(15)
    .with_max_conversation_history(30)
    .with_toolbox(tools)
    .build()
    .await?;
);

// Build a fully configured agent
let agent = AgentBuilder::new()
    .with_config(&config)
    .with_llm_client(llm_client)
    .with_toolbox(toolbox)
    .with_system_prompt("You are a comprehensive AI assistant with weather capabilities.")
    .with_max_chat_completion(20)
    .with_max_conversation_history(50)
    .build()
    .await?;

// Use the agent in your server
let server = A2AServerBuilder::new()
    .with_agent(agent)
    .with_agent_card_from_file(".well-known/agent.json")
    .build()
    .await?;
```

### Custom Tools

Create custom tools to extend your agent's capabilities using the Inference Gateway SDK's tool system:

```rust
use inference_gateway_adk::server::AgentBuilder;
use inference_gateway_sdk::{Tool, ToolType, FunctionObject};
use serde_json::json;

// Define tools for your agent's toolbox
let tools = vec![
    Tool {
        r#type: ToolType::Function,
        function: FunctionObject {
            name: "get_weather".to_string(),
            description: "Get current weather for a location".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "location": {
                        "type": "string",
                        "description": "The city and state, e.g. San Francisco, CA"
                    },
                    "unit": {
                        "type": "string",
                        "enum": ["celsius", "fahrenheit"],
                        "description": "Temperature unit (default: celsius)"
                    }
                },
                "required": ["location"]
            }),
        },
    },
    Tool {
        r#type: ToolType::Function,
        function: FunctionObject {
            name: "search_web".to_string(),
            description: "Search the web for information".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "Search query"
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Maximum number of results (default: 5)",
                        "default": 5
                    }
                },
                "required": ["query"]
            }),
        },
    },
];

// Create an agent with the toolbox
let agent = AgentBuilder::new()
    .with_config(&config)
    .with_system_prompt(
        "You are a helpful assistant with access to weather information and web search. \
        Use the provided tools when users ask for weather or need web search results."
    )
    .with_toolbox(tools)
    .build()
    .await?;
```

The toolbox integrates with the Inference Gateway SDK's function calling system. When the LLM decides to use a tool, the tool call information is automatically sent through the gateway to the configured LLM provider, which will return tool call requests that can be processed by your application logic.

### Custom Task Processing

Implement custom business logic for task completion:

```rust
use inference_gateway_adk::server::{TaskProcessor, TaskResult};
use inference_gateway_adk::types::Message;

struct CustomTaskProcessor;

impl TaskProcessor for CustomTaskProcessor {
    async fn process_tool_result(&self, tool_call_result: &str) -> Option<Message> {
        // Parse the tool result
        let result: serde_json::Value = serde_json::from_str(tool_call_result).ok()?;

        // Apply your business logic
        if should_complete_task(&result) {
            Some(Message {
                role: "assistant".to_string(),
                parts: vec![Part {
                    kind: "text".to_string(),
                    content: "Task completed successfully!".to_string(),
                }],
            })
        } else {
            // Return None to continue processing
            None
        }
    }
}

// Set the processor when building your server
let server = A2AServerBuilder::new()
    .with_task_processor(CustomTaskProcessor)
    .with_agent_card_from_file(".well-known/agent.json")
    .build()
    .await?;
```

### Push Notifications

A2A servers persist per-task webhook configurations through four JSON-RPC
methods on `A2AClient`:

- `tasks/pushNotificationConfig/set` - `client.set_task_push_notification_config(...)`
- `tasks/pushNotificationConfig/get` - `client.get_task_push_notification_config(...)`
- `tasks/pushNotificationConfig/list` - `client.list_task_push_notification_configs(...)`
- `tasks/pushNotificationConfig/delete` - `client.delete_task_push_notification_config(...)`

Each call uses the typed structs from
[`inference_gateway_adk::a2a_types`](src/a2a_types.rs) and is exercised by a
dedicated example under
[`examples/a2a-methods/`](examples/a2a-methods/README.md).

#### Storing a webhook configuration

```rust
use inference_gateway_adk::A2AClient;
use inference_gateway_adk::a2a_types::{
    PushNotificationConfig, SetTaskPushNotificationConfigRequest, TaskPushNotificationConfig,
};

let client = A2AClient::new("http://localhost:8080")?;

let parent = format!("tasks/{}", task_id);
let config_id = "primary";
let name = format!("{parent}/pushNotificationConfigs/{config_id}");

client
    .set_task_push_notification_config(SetTaskPushNotificationConfigRequest {
        parent: parent.clone(),
        config_id: config_id.to_string(),
        tenant: Some("example".to_string()),
        config: TaskPushNotificationConfig {
            name: name.clone(),
            push_notification_config: PushNotificationConfig {
                authentication: None,
                id: None,
                token: Some("shared-secret".to_string()),
                url: "https://your-app.example/webhooks/a2a".to_string(),
            },
        },
    })
    .await?;
```

#### Reading, listing, and removing configurations

```rust
use inference_gateway_adk::a2a_types::{
    DeleteTaskPushNotificationConfigRequest, GetTaskPushNotificationConfigRequest,
    ListTaskPushNotificationConfigRequest,
};

// get
let cfg = client
    .get_task_push_notification_config(GetTaskPushNotificationConfigRequest {
        name: name.clone(),
        tenant: "example".to_string(),
    })
    .await?;

// list (paged)
let page = client
    .list_task_push_notification_configs(ListTaskPushNotificationConfigRequest {
        parent: parent.clone(),
        page_size: 10,
        page_token: String::new(),
        tenant: "example".to_string(),
    })
    .await?;

// delete
client
    .delete_task_push_notification_config(DeleteTaskPushNotificationConfigRequest {
        name,
        tenant: "example".to_string(),
    })
    .await?;
```

> **Webhook delivery is still in development.** The four control-plane
> methods above (set/get/list/delete) are fully wired up and durably stored
> by the server, but the HTTP _sender_ that fans state changes out to the
> configured URLs is tracked in a follow-up ticket. Configurations attached
> today are picked up automatically once that sender lands.

#### Expected webhook payload

When the sender lands, each task state transition will POST a payload of
roughly this shape to the configured `url`:

```json
{
  "type": "task_update",
  "taskId": "task-123",
  "state": "TASK_STATE_COMPLETED",
  "timestamp": "2026-05-11T10:30:00Z",
  "task": {
    "id": "task-123",
    "contextId": "context-456",
    "status": {
      "state": "TASK_STATE_COMPLETED",
      "timestamp": "2026-05-11T10:30:00Z"
    },
    "history": [],
    "artifacts": []
  }
}
```

### Agent Metadata

Agent metadata can be configured in two ways: at build-time via environment variables (recommended for production) or at runtime via configuration.

#### Build-Time Metadata (Recommended)

Agent metadata is embedded directly into the binary during compilation using environment variables. This approach ensures immutable agent information and is ideal for production deployments:

```bash
# Build your application with custom metadata
AGENT_NAME="Weather Assistant" \
AGENT_DESCRIPTION="Specialized weather analysis agent" \
AGENT_VERSION="2.0.0" \
cargo build --release
```

#### Runtime Metadata Configuration

For development or when dynamic configuration is needed, override individual
agent card fields at runtime via `AgentCardOverrides`. The builder layers
your overrides on top of whatever was loaded from disk:

```rust
use inference_gateway_adk::{A2AServerBuilder, AgentCardOverrides, Config};

let config: Config = envy::prefixed("A2A_").from_env()?;

let server = A2AServerBuilder::new()
    .with_config(config)
    .with_agent_card_from_file(
        ".well-known/agent.json",
        Some(
            AgentCardOverrides::new()
                .with_name("Development Weather Assistant")
                .with_description("Development version with debug features")
                .with_version("dev-1.0.0"),
        ),
    )
    .build()
    .await?;
```

**Note:** The file on disk supplies the baseline; `AgentCardOverrides` wins
for any field you set explicitly. See [`examples/static-agent-card/`](./examples/static-agent-card/)
for a runnable end-to-end demo.

### Authentication

When `A2A_AUTH_ENABLE=true`, the server gates `POST /a2a` behind an
`Authorization: Bearer <token>` header validated against the OIDC issuer
configured by `A2A_AUTH_ISSUER_URL`. The bundled `OidcJwtVerifier`:

1. Performs OIDC discovery at `<A2A_AUTH_ISSUER_URL>/.well-known/openid-configuration`.
2. Fetches and caches the JWKS advertised by the discovery document.
3. Validates the JWT signature, `iss`, `exp`, and (when `A2A_AUTH_CLIENT_ID`
   is set) `aud` claims.

`GET /health` and `GET /.well-known/agent.json` are always public so
health probes and discovery clients keep working without a credential.
Tokens that fail any check produce **HTTP 401** with a
`WWW-Authenticate: Bearer realm="a2a"` header.

To plug in a custom backend (static keys, internal identity service,
mocks for tests) implement `AuthVerifier` and pass it to
`A2AServerBuilder::with_auth_verifier(...)` - this overrides whatever
`A2A_AUTH_ENABLE` selects and works the same way `with_storage(...)` does.

The authenticated principal (subject, tenant, all JWT claims) is
attached to the request via an Axum extension and forwarded to the
JSON-RPC dispatcher so per-tenant filtering of the extended agent card
is a future no-op behind a feature flag rather than a breaking change.

**Behaviour when `A2A_AUTH_ENABLE=false`** — the middleware is not attached
and `agent/getAuthenticatedExtendedCard` returns the configured card
whenever `supportsExtendedAgentCard == true` on the agent card. This
preserves backwards compatibility for callers who have not opted in to
authentication. Operators that want the method to hard-fail when auth
is globally off should set `supportsExtendedAgentCard: false` on the
agent card; the handler returns JSON-RPC `-32601 METHOD_NOT_FOUND` in
that case.

See [`examples/auth/`](./examples/auth/) for a runnable end-to-end demo.

### TLS and mTLS

When `A2A_SERVER_TLS_ENABLE=true`, `A2AServer::serve` swaps its plaintext
Axum listener for `axum-server` backed by `rustls` (with the `ring`
crypto provider) and serves the same Axum router over HTTPS. The
configuration lives on `Config.tls_config` and is populated by whatever
loader you used — `envy::prefixed("A2A_").from_env::<Config>()` in the
bundled examples:

| Variable | Purpose |
| --- | --- |
| `A2A_SERVER_TLS_ENABLE` | Set to `true` to flip `A2AServer::serve` onto the TLS listener. |
| `A2A_SERVER_TLS_CERT_PATH` | PEM file with the server certificate chain. |
| `A2A_SERVER_TLS_KEY_PATH` | PEM file with the server private key (PKCS#1, PKCS#8, or SEC1). |
| `A2A_SERVER_TLS_CLIENT_CA_PATH` | Optional. When set, the server requires every TLS client to present a certificate signed by one of the CAs in this PEM bundle — i.e. mutual TLS, the `MutualTlsSecurityScheme` the A2A spec describes. |

The rustls stack was chosen over native-tls because (1) it is pure Rust
and avoids the OpenSSL toolchain on container builds, and (2) it gives
us programmatic access to the negotiated `ServerConnection`, which is
what makes the mTLS subject extraction below tractable.

When mTLS is enabled, the server's TLS acceptor parses the peer's leaf
certificate and exposes it to handlers as an `axum::Extension<PeerCert>`
extension — the same plumbing pattern the bearer-token auth middleware
uses for `AuthenticatedPrincipal`. The wrapped `ClientCertPrincipal`
carries the subject DN, the Common Name (when present), the issuer DN,
and the raw DER bytes of the leaf:

```rust
use axum::Extension;
use inference_gateway_adk::PeerCert;

async fn my_handler(Extension(peer): Extension<PeerCert>) {
    if let Some(p) = peer.0 {
        tracing::info!("authenticated client: {} (issued by {})", p.subject, p.issuer);
    }
}
```

For plain HTTPS (no `A2A_SERVER_TLS_CLIENT_CA_PATH`) the `PeerCert` is still
injected, but its inner `Option` is `None` because the client did not
present a certificate.

See [`examples/tls/`](./examples/tls/) for a runnable end-to-end demo
with a `make-certs.sh` script that mints a self-signed CA, a server
cert, and a client cert under `examples/tls/certs/`. The example
exercises both modes via the `tls` and `mtls` Compose profiles.

### Environment Configuration

Runtime config flows in via the `A2A_*` env-var family. The library
doesn't read env itself — pick any loader; the bundled examples use
[`envy`][envy] (`envy::prefixed("A2A_").from_env::<Config>()`). The
`A2A_` prefix is a convention; clients are free to use a different
prefix as long as the leaf names match the `#[serde(rename = "...")]`
tags on `Config`.

```bash
# Server
A2A_SERVER_HOST="0.0.0.0"
A2A_SERVER_PORT="8080"
A2A_DEBUG="false"

# Build-time agent metadata (compile-time env vars, read by env! macros)
AGENT_NAME="My Agent"
AGENT_DESCRIPTION="My agent description"
AGENT_VERSION="1.0.0"
AGENT_CARD_FILE_PATH="./.well-known/agent.json"

# LLM client (the ADK fails fast at AgentBuilder::build if provider/model are unset)
A2A_AGENT_CLIENT_PROVIDER="deepseek"            # groq, google, openai, anthropic, cohere, cloudflare, deepseek, ollama
A2A_AGENT_CLIENT_MODEL="deepseek-v4-flash"
A2A_AGENT_CLIENT_API_KEY="your-api-key"
A2A_AGENT_CLIENT_BASE_URL="http://inference-gateway:8080/v1"
A2A_AGENT_CLIENT_MAX_TOKENS="4096"
A2A_AGENT_CLIENT_TEMPERATURE="0.7"
A2A_AGENT_CLIENT_SYSTEM_PROMPT="You are a helpful assistant"

# Capabilities (surfaced in the agent card)
A2A_CAPABILITIES_STREAMING="true"
A2A_CAPABILITIES_PUSH_NOTIFICATIONS="true"
A2A_CAPABILITIES_STATE_TRANSITION_HISTORY="false"

# Queue / storage
A2A_QUEUE_PROVIDER="memory"                     # `memory` (default) or `redis` (requires the `redis` Cargo feature)
A2A_QUEUE_URL="redis://localhost:6379"          # required when provider=redis
A2A_QUEUE_NAMESPACE="a2a"
A2A_QUEUE_WORKERS="1"

# Authentication (optional, OIDC bearer-token JWT)
A2A_AUTH_ENABLE="false"                                                  # when true, POST /a2a requires a valid bearer token
A2A_AUTH_ISSUER_URL="http://keycloak:8080/realms/inference-gateway-realm" # OIDC issuer; the server performs discovery + JWKS lookup
A2A_AUTH_CLIENT_ID="inference-gateway-client"                            # validated as the JWT audience when set
A2A_AUTH_CLIENT_SECRET="your-secret"                                     # currently unused server-side (reserved for client-side OAuth2)

# TLS (optional)
A2A_SERVER_TLS_ENABLE="false"                   # when true, A2AServer::serve binds an HTTPS listener via axum-server + rustls
A2A_SERVER_TLS_CERT_PATH="/path/to/cert.pem"    # PEM-encoded server certificate chain
A2A_SERVER_TLS_KEY_PATH="/path/to/key.pem"      # PEM-encoded private key (PKCS#1, PKCS#8, or SEC1)
A2A_SERVER_TLS_CLIENT_CA_PATH=""                # optional: when set, the server requires mTLS and trusts client certs signed by the CAs in this PEM bundle
```

## A2A Ecosystem

This ADK is part of the broader Inference Gateway ecosystem:

### Related Projects

- **[Inference Gateway](https://github.com/inference-gateway/inference-gateway)** - Unified API gateway for AI providers
- **[Go ADK](https://github.com/inference-gateway/adk)** - Go library for building A2A agents
- **[Go SDK](https://github.com/inference-gateway/go-sdk)** - Go client library for Inference Gateway
- **[TypeScript SDK](https://github.com/inference-gateway/typescript-sdk)** - TypeScript/JavaScript client library
- **[Python SDK](https://github.com/inference-gateway/python-sdk)** - Python client library

### A2A Agents

- **[Awesome A2A](https://github.com/inference-gateway/awesome-a2a)** - Curated list of A2A-compatible agents
- **[Google Calendar Agent](https://github.com/inference-gateway/google-calendar-agent)** - Google Calendar integration agent

## Requirements

- **Rust**: 1.94 or later
- **Dependencies**: See [Cargo.toml](./Cargo.toml) for full dependency list

## OCI Compliant

Build and run your A2A agent application in any OCI-compliant container runtime (Docker, Podman, containerd, etc.). Here's an example Containerfile for an application using the ADK:

```dockerfile
FROM rust:1.94 AS builder

# Build arguments for agent metadata
ARG AGENT_NAME="My A2A Agent"
ARG AGENT_DESCRIPTION="A custom A2A agent built with the Rust ADK"
ARG AGENT_VERSION="1.0.0"

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch

COPY . .

# Build with custom agent metadata
RUN AGENT_NAME="${AGENT_NAME}" \
    AGENT_DESCRIPTION="${AGENT_DESCRIPTION}" \
    AGENT_VERSION="${AGENT_VERSION}" \
    cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/rust-adk .
CMD ["./rust-adk"]
```

**Build with custom metadata:**

```bash
docker build \
  --build-arg AGENT_NAME="Weather Assistant" \
  --build-arg AGENT_DESCRIPTION="AI-powered weather forecasting agent" \
  --build-arg AGENT_VERSION="2.0.0" \
  -t my-a2a-agent .
```

## Testing

The ADK follows table-driven testing patterns and provides comprehensive test coverage:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test;

    #[derive(Debug)]
    struct TestCase {
        name: &'static str,
        endpoint: &'static str,
        method: &'static str,
        expected_status: u16,
    }

    #[tokio::test]
    async fn test_a2a_server_endpoints() {
        let test_cases = vec![
            TestCase {
                name: "health check",
                endpoint: "/health",
                method: "GET",
                expected_status: 200,
            },
            TestCase {
                name: "agent info",
                endpoint: "/.well-known/agent.json",
                method: "GET",
                expected_status: 200,
            },
            TestCase {
                name: "a2a endpoint",
                endpoint: "/a2a",
                method: "POST",
                expected_status: 200,
            },
        ];

        for test_case in test_cases {
            // Each test case has isolated mocks
            let server = setup_test_server().await;

            // Test implementation with table-driven approach
            let response = make_request(&server, test_case.method, test_case.endpoint).await;
            assert_eq!(test_case.expected_status, response.status().as_u16());
        }
    }
}
```

Run tests with:

```bash
task test
```

Or directly with cargo:

```bash
cargo test
```

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.

## Contributing

We welcome contributions! Here's how you can help:

### Getting Started

1. **Fork the repository**
2. **Clone your fork**:

   ```bash
   git clone https://github.com/your-username/rust-adk.git
   cd rust-adk
   ```

3. **Create a feature branch**:
   ```bash
   git checkout -b feature/amazing-feature
   ```

### Development Guidelines

- Follow the established code style and conventions (use `rustfmt`)
- Write table-driven tests for new functionality
- Use early returns to simplify logic and avoid deep nesting
- Prefer match statements over if-else chains
- Ensure type safety with proper error handling
- Use lowercase log messages for consistency

### Before Submitting

1. **Download latest schema**: `task a2a:download-schema`
2. **Generate types**: `task a2a:generate-types`
3. **Run linting**: `task lint`
4. **All tests pass**: `task test`

### Pull Request Process

1. Update documentation for any new features
2. Add tests for new functionality
3. Ensure all CI checks pass
4. Request review from maintainers

For more details, see [CONTRIBUTING.md](./CONTRIBUTING.md).

## Support

### Issues & Questions

- **Bug Reports**: [GitHub Issues](https://github.com/inference-gateway/rust-adk/issues)
- **Documentation**: [Official Docs](https://docs.inference-gateway.com)

## Resources

### Documentation

- [A2A Protocol Specification](https://github.com/inference-gateway/schemas/tree/main/a2a)
- [API Documentation](https://docs.inference-gateway.com/a2a)

---

<p align="center">
  <strong>Built with ❤️ by the Inference Gateway team</strong>
</p>

<p align="center">
  <a href="https://github.com/inference-gateway">GitHub</a> •
  <a href="https://docs.inference-gateway.com">Documentation</a>
</p>

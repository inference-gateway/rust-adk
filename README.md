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
  <img src="https://img.shields.io/badge/rust-1.70+-blue.svg?style=flat-square" alt="Rust Version"/>
</p>

---

## Table of Contents

- [Table of Contents](#table-of-contents)
- [Overview](#overview)
  - [What is A2A?](#what-is-a2a)
- [🚀 Quick Start](#-quick-start)
  - [Installation](#installation)
  - [Basic Usage (Minimal Server)](#basic-usage-minimal-server)
  - [AI-Powered Server](#ai-powered-server)
  - [Health Check Example](#health-check-example)
  - [Examples](#examples)
- [✨ Key Features](#-key-features)
  - [Core Capabilities](#core-capabilities)
  - [Developer Experience](#developer-experience)
  - [Production Ready](#production-ready)
- [🛠️ Development](#️-development)
  - [Prerequisites](#prerequisites)
  - [Development Workflow](#development-workflow)
  - [Available Tasks](#available-tasks)
  - [Build-Time Agent Metadata](#build-time-agent-metadata)
    - [Available Build-Time Variables](#available-build-time-variables)
    - [Usage Examples](#usage-examples)
- [📖 API Reference](#-api-reference)
  - [Core Components](#core-components)
    - [A2AServer](#a2aserver)
    - [A2AServerBuilder](#a2aserverbuilder)
    - [AgentBuilder](#agentbuilder)
    - [A2AClient](#a2aclient)
    - [Agent Health Monitoring](#agent-health-monitoring)
    - [LLM Client](#llm-client)
  - [Configuration](#configuration)
- [🔧 Advanced Usage](#-advanced-usage)
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
  - [Environment Configuration](#environment-configuration)
- [🌐 A2A Ecosystem](#-a2a-ecosystem)
  - [Related Projects](#related-projects)
  - [A2A Agents](#a2a-agents)
- [📋 Requirements](#-requirements)
- [🐳 Docker Support](#-docker-support)
- [🧪 Testing](#-testing)
- [📄 License](#-license)
- [🤝 Contributing](#-contributing)
  - [Getting Started](#getting-started)
  - [Development Guidelines](#development-guidelines)
  - [Before Submitting](#before-submitting)
  - [Pull Request Process](#pull-request-process)
- [📞 Support](#-support)
  - [Issues \& Questions](#issues--questions)
- [🔗 Resources](#-resources)
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

## 🚀 Quick Start

### Installation

Add the ADK to your `Cargo.toml`:

```toml
[dependencies]
inference-gateway-adk = "0.1.0"
```

### Basic Usage (Minimal Server)

```rust
use rust_adk::server::{A2AServer, A2AServerBuilder};
use tokio;
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::init();

    // Create the simplest A2A server
    let server = A2AServerBuilder::new()
        .build()
        .await?;

    // Start server
    let addr = "0.0.0.0:8080".parse()?;
    info!("Server running on port 8080");

    if let Err(e) = server.serve(addr).await {
        error!("Server failed to start: {}", e);
    }

    Ok(())
}
```

### AI-Powered Server

```rust
use rust_adk::{
    server::{A2AServer, A2AServerBuilder, AgentBuilder},
    config::Config,
    tools::ToolBox,
};
use serde_json::json;
use tokio;
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::init();

    // Load configuration from environment
    let config = Config::from_env()?;

    // Create toolbox with custom tools
    let mut toolbox = ToolBox::new();

    // Add a weather tool
    toolbox.add_tool(
        "get_weather",
        "Get weather information",
        json!({
            "type": "object",
            "properties": {
                "location": {
                    "type": "string",
                    "description": "City name"
                }
            },
            "required": ["location"]
        }),
        |args| async move {
            let location = args["location"].as_str().unwrap_or("Unknown");
            Ok(format!(r#"{{"location": "{}", "temperature": "22°C"}}"#, location))
        },
    );

    // Create LLM client (requires AGENT_CLIENT_API_KEY environment variable)
    let server = if let Some(api_key) = &config.agent_config.api_key {
        // AI-powered agent
        let agent = AgentBuilder::new()
            .with_config(&config.agent_config)
            .with_toolbox(toolbox)
            .build()
            .await?;

        A2AServerBuilder::new()
            .with_config(config)
            .with_agent(agent)
            .with_agent_card_from_file(".well-known/agent.json")
            .build()
            .await?
    } else {
        // Mock mode without actual LLM
        let agent = AgentBuilder::new()
            .with_toolbox(toolbox)
            .build()
            .await?;

        A2AServerBuilder::new()
            .with_config(config)
            .with_agent(agent)
            .with_agent_card_from_file(".well-known/agent.json")
            .build()
            .await?
    };

    // Start server
    let addr = "0.0.0.0:8080".parse()?;
    info!("AI-powered A2A server running on port 8080");

    if let Err(e) = server.serve(addr).await {
        error!("Server failed to start: {}", e);
    }

    Ok(())
}
```

### Health Check Example

Monitor the health status of A2A agents for service discovery and load balancing:

```rust
use rust_adk::client::A2AClient;
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

For complete working examples, see the [examples](./examples/) directory:

- **[Minimal Server](./examples/minimal-server/)** - Basic A2A server without AI capabilities
- **[AI-Powered Server](./examples/ai-powered-server/)** - Full A2A server with LLM integration
- **[JSON AgentCard Server](./examples/json-agentcard-server/)** - A2A server with agent metadata loaded from JSON file
- **[Client Example](./examples/client/)** - A2A client implementation
- **[Health Check Example](#health-check-example)** - Monitor agent health status

## ✨ Key Features

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
- 📋 **Task Listing**: Listing with filtering and pagination (`tasks/list`)
- 🏗️ **Extensible Architecture**: Pluggable components for custom business logic
- 📚 **Type-Safe**: Generated types from A2A schema for compile-time safety
- 🧪 **Well Tested**: Comprehensive test coverage with table-driven tests

### Production Ready

- 🌿 **Lightweight**: Optimized binary size with Rust's zero-cost abstractions
- 🛡️ **Production Hardened**: Configurable timeouts, TLS support, and error handling
- 🐳 **Containerized**: OCI compliant and works with Docker and Docker Compose
- ☸️ **Kubernetes Native**: Ready for cloud-native deployments
- 📊 **Observability**: OpenTelemetry integration for monitoring and tracing

## 🛠️ Development

### Prerequisites

- Rust 1.70 or later
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
FROM rust:1.88 AS builder

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

## 📖 API Reference

### Core Components

#### A2AServer

The main server trait that handles A2A protocol communication.

```rust
use rust_adk::server::{A2AServer, A2AServerBuilder};

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
use rust_adk::server::{A2AServerBuilder, AgentBuilder};

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
use rust_adk::server::AgentBuilder;

// Basic agent with custom LLM
let agent = AgentBuilder::new()
    .with_llm_client(custom_llm_client)
    .with_toolbox(toolbox)
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
use rust_adk::client::A2AClient;

// Basic client creation
let client = A2AClient::new("http://localhost:8080")?;

// Client with custom configuration
let config = ClientConfig {
    base_url: "http://localhost:8080".to_string(),
    timeout: Duration::from_secs(45),
    max_retries: 5,
};
let client = A2AClient::with_config(config)?;

// Using the client
let agent_card = client.get_agent_card().await?;
let health = client.get_health().await?;
let response = client.send_task(params).await?;
client.send_task_streaming(params, event_handler).await?;
```

#### Agent Health Monitoring

Monitor the health status of A2A agents to ensure they are operational:

```rust
use rust_adk::client::A2AClient;

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

Create OpenAI-compatible LLM clients for agents:

```rust
use rust_adk::llm::OpenAICompatibleClient;

// Create LLM client with configuration
let llm_client = OpenAICompatibleClient::new(agent_config).await?;

// Use with agent builder
let agent = AgentBuilder::new()
    .with_llm_client(llm_client)
    .build()
    .await?;
```

### Configuration

The configuration is managed through environment variables and the config module:

```rust
use rust_adk::config::{Config, AgentConfig};

#[derive(Debug, Clone)]
pub struct Config {
    pub agent_url: String,                    // AGENT_URL (default: http://helloworld-agent:8080)
    pub debug: bool,                          // DEBUG (default: false)
    pub port: u16,                           // PORT (default: 8080)
    pub streaming_status_update_interval: Duration, // STREAMING_STATUS_UPDATE_INTERVAL (default: 1s)
    pub agent_config: AgentConfig,           // AGENT_CLIENT_*
    pub capabilities_config: CapabilitiesConfig, // CAPABILITIES_*
    pub tls_config: Option<TlsConfig>,       // TLS_*
    pub auth_config: Option<AuthConfig>,     // AUTH_*
    pub queue_config: QueueConfig,           // QUEUE_*
    pub server_config: ServerConfig,         // SERVER_*
    pub telemetry_config: TelemetryConfig,   // TELEMETRY_*
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

## 🔧 Advanced Usage

### Building Custom Agents with AgentBuilder

The `AgentBuilder` provides a fluent interface for creating highly customized agents with specific configurations, LLM clients, and toolboxes.

#### Basic Agent Creation

```rust
use rust_adk::server::AgentBuilder;
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
use rust_adk::config::AgentConfig;
use std::time::Duration;

let config = AgentConfig {
    provider: "openai".to_string(),
    model: "gpt-4".to_string(),
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
use rust_adk::llm::OpenAICompatibleClient;

// Create a custom LLM client
let llm_client = OpenAICompatibleClient::new(&config).await?;

// Build agent with the custom client
let agent = AgentBuilder::new()
    .with_llm_client(llm_client)
    .with_system_prompt("You are a coding assistant.")
    .build()
    .await?;
```

#### Fully Configured Agent

```rust
use rust_adk::tools::ToolBox;
use serde_json::json;

// Create toolbox with custom tools
let mut toolbox = ToolBox::new();

// Add custom tools (see Custom Tools section below)
toolbox.add_tool(
    "get_weather",
    "Get weather information",
    json!({
        "type": "object",
        "properties": {
            "location": {
                "type": "string",
                "description": "City name"
            }
        },
        "required": ["location"]
    }),
    |args| async move {
        let location = args["location"].as_str().unwrap_or("Unknown");
        Ok(format!(r#"{{"location": "{}", "temperature": "22°C"}}"#, location))
    },
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

Create custom tools to extend your agent's capabilities:

```rust
use rust_adk::tools::ToolBox;
use serde_json::json;

// Create a toolbox
let mut toolbox = ToolBox::new();

// Create a custom tool
toolbox.add_tool(
    "get_weather",
    "Get current weather for a location",
    json!({
        "type": "object",
        "properties": {
            "location": {
                "type": "string",
                "description": "The city and state, e.g. San Francisco, CA"
            }
        },
        "required": ["location"]
    }),
    |args| async move {
        let location = args["location"].as_str().unwrap_or("Unknown");

        // Your weather API logic here
        let result = get_weather(location).await?;

        let response = serde_json::to_string(&result)?;
        Ok(response)
    },
);

// Set the toolbox on your agent
let agent = AgentBuilder::new()
    .with_toolbox(toolbox)
    .build()
    .await?;
```

### Custom Task Processing

Implement custom business logic for task completion:

```rust
use rust_adk::server::{TaskProcessor, TaskResult};
use rust_adk::types::Message;

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

Configure webhook notifications to receive real-time updates when task states change:

```rust
use rust_adk::notifications::{HttpPushNotificationSender, TaskPushNotificationConfig};
use rust_adk::server::TaskManager;

// Create an HTTP push notification sender
let notification_sender = HttpPushNotificationSender::new();

// Create a task manager with push notification support
let task_manager = TaskManager::with_notifications(
    100, // max conversation history
    notification_sender,
);

// Configure push notification webhooks for a task
let config = TaskPushNotificationConfig {
    task_id: "task-123".to_string(),
    push_notification_config: PushNotificationConfig {
        url: "https://your-app.com/webhooks/task-updates".to_string(),
        token: Some(token),
        authentication: Some(PushNotificationAuthenticationInfo {
            schemes: vec!["bearer".to_string()],
            credentials: bearer_token,
        }),
    },
};

// Set the configuration
task_manager.set_task_push_notification_config(config).await?;
```

#### Webhook Payload

When a task state changes, your webhook will receive a POST request with this payload:

```json
{
  "type": "task_update",
  "taskId": "task-123",
  "state": "completed",
  "timestamp": "2025-06-16T10:30:00Z",
  "task": {
    "id": "task-123",
    "kind": "task",
    "status": {
      "state": "completed",
      "message": {
        "role": "assistant",
        "parts": [{ "kind": "text", "text": "Task completed successfully" }]
      },
      "timestamp": "2025-06-16T10:30:00Z"
    },
    "contextId": "context-456",
    "history": []
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

For development or when dynamic configuration is needed, you can override the build-time metadata through the server's configuration:

```rust
use rust_adk::config::Config;

let mut config = Config::from_env()?;

// Override build-time metadata for development
config.agent_name = Some("Development Weather Assistant".to_string());
config.agent_description = Some("Development version with debug features".to_string());
config.agent_version = Some("dev-1.0.0".to_string());

let server = A2AServerBuilder::new()
    .with_config(config)
    .with_agent_card_from_file(".well-known/agent.json")
    .build()
    .await?;
```

**Note:** Build-time metadata takes precedence as defaults, but can be overridden at runtime using the configuration.

### Environment Configuration

Key environment variables for configuring your agent:

```bash
# Server configuration
PORT="8080"

# Agent metadata configuration (via build-time environment variables)
AGENT_NAME="My Agent"                       # Build-time only
AGENT_DESCRIPTION="My agent description"    # Build-time only
AGENT_VERSION="1.0.0"                      # Build-time only
AGENT_CARD_FILE_PATH="./.well-known/agent.json"    # Path to JSON AgentCard file (optional)

# LLM client configuration
AGENT_CLIENT_PROVIDER="openai"              # openai, anthropic, deepseek, ollama
AGENT_CLIENT_MODEL="gpt-4"                  # Model name
AGENT_CLIENT_API_KEY="your-api-key"         # Required for AI features
AGENT_CLIENT_BASE_URL="https://api.openai.com/v1"  # Custom endpoint
AGENT_CLIENT_MAX_TOKENS="4096"              # Max tokens for completion
AGENT_CLIENT_TEMPERATURE="0.7"              # Temperature for completion
AGENT_CLIENT_SYSTEM_PROMPT="You are a helpful assistant"

# Capabilities
CAPABILITIES_STREAMING="true"
CAPABILITIES_PUSH_NOTIFICATIONS="true"
CAPABILITIES_STATE_TRANSITION_HISTORY="false"

# Authentication (optional)
AUTH_ENABLE="false"
AUTH_ISSUER_URL="http://keycloak:8080/realms/inference-gateway-realm"
AUTH_CLIENT_ID="inference-gateway-client"
AUTH_CLIENT_SECRET="your-secret"

# TLS (optional)
SERVER_TLS_ENABLE="false"
SERVER_TLS_CERT_PATH="/path/to/cert.pem"
SERVER_TLS_KEY_PATH="/path/to/key.pem"
```

## 🌐 A2A Ecosystem

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

## 📋 Requirements

- **Rust**: 1.70 or later
- **Dependencies**: See [Cargo.toml](./Cargo.toml) for full dependency list

## 🐳 Docker Support

Build and run your A2A agent application in a container. Here's an example Dockerfile for an application using the ADK:

```dockerfile
FROM rust:1.70 AS builder

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

## 🧪 Testing

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

## 📄 License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.

## 🤝 Contributing

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

## 📞 Support

### Issues & Questions

- **Bug Reports**: [GitHub Issues](https://github.com/inference-gateway/rust-adk/issues)
- **Documentation**: [Official Docs](https://docs.inference-gateway.com)

## 🔗 Resources

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

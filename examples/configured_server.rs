use inference_gateway_adk::{A2AServerBuilder, AgentBuilder, Config};
use tokio;
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt().init();

    // Load configuration from environment
    let config = Config::from_env()?;

    info!("Starting A2A server with configuration...");
    info!("Agent provider: {}", config.agent_config.provider);
    info!("Agent model: {}", config.agent_config.model);
    info!("Has API key: {}", config.agent_config.api_key.is_some());

    // Create agent if API key is available
    let server = if config.agent_config.api_key.is_some() {
        // AI-powered agent
        let agent = AgentBuilder::new()
            .with_config(&config.agent_config)
            .with_system_prompt("You are a helpful A2A assistant built with Rust.")
            .with_max_chat_completion(15)
            .build()
            .await?;

        A2AServerBuilder::new()
            .with_config(config)
            .with_agent(agent)
            .with_agent_card_from_file(".well-known/agent.json")
            .build()
            .await?
    } else {
        // Basic server without AI agent
        info!("No API key provided, running in basic mode");
        A2AServerBuilder::new()
            .with_config(config)
            .with_agent_card_from_file(".well-known/agent.json")
            .build()
            .await?
    };

    // Start server
    let addr = "0.0.0.0:8080".parse()?;
    info!("Configured A2A server running on port 8080");

    if let Err(e) = server.serve(addr).await {
        error!("Server failed to start: {}", e);
    }

    Ok(())
}
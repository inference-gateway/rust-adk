use inference_gateway_adk::{A2AServerBuilder, AgentBuilder, Config};
use std::env;
use tokio;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    let config = Config::from_env()?;

    let gateway_url = env::var("INFERENCE_GATEWAY_URL")
        .unwrap_or_else(|_| "http://localhost:8080/v1".to_string());

    info!("Starting A2A server with Inference Gateway SDK...");
    info!("Gateway URL: {}", gateway_url);
    info!("Agent provider: {}", config.agent_config.provider);
    info!("Agent model: {}", config.agent_config.model);
    info!("Has API key: {}", config.agent_config.api_key.is_some());

    let agent = AgentBuilder::new()
        .with_config(&config.agent_config)
        .with_system_prompt("You are a helpful A2A assistant built with Rust and powered by the Inference Gateway SDK.")
        .with_max_chat_completion(15)
        .build()
        .await?;

    let server = A2AServerBuilder::new()
        .with_config(config)
        .with_agent(agent)
        .with_agent_card_from_file(".well-known/agent.json")
        .with_gateway_url(gateway_url)
        .build()
        .await?;

    let addr = "0.0.0.0:8081".parse()?;
    info!("Configured A2A server with SDK integration running on port 8081");

    if let Err(e) = server.serve(addr).await {
        error!("Server failed to start: {}", e);
    }

    Ok(())
}

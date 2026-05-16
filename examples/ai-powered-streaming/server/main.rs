use inference_gateway_adk::{A2AServerBuilder, AgentBuilder, Config};
use std::env;
use tracing::{error, info};

/// `ai-powered-streaming` server: LLM agent attached, streaming over
/// `message/stream` using the built-in `DefaultStreamingTaskHandler`,
/// which converts LLM delta chunks into `TaskArtifactUpdateEvent`s.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();
    dotenvy::dotenv().ok();

    let mut config: Config = envy::prefixed("A2A_").from_env()?;

    let gateway_url = env::var("INFERENCE_GATEWAY_URL")
        .unwrap_or_else(|_| "http://localhost:8080/v1".to_string());

    if config.agent_config.base_url.is_none() {
        config.agent_config.base_url = Some(gateway_url.clone());
    }

    info!("Starting ai-powered-streaming A2A server");
    info!(
        "agent provider={} model={}",
        config.agent_config.provider, config.agent_config.model
    );

    let agent = AgentBuilder::new()
        .with_config(&config.agent_config)
        .with_system_prompt(
            "You are a concise, helpful assistant. Keep replies under three sentences.",
        )
        .build()
        .await?;

    let server = A2AServerBuilder::new()
        .with_config(config)
        .with_agent(agent)
        .with_agent_card_from_file(".well-known/agent.json", None)
        .with_gateway_url(gateway_url)
        .with_default_task_handlers()
        .build()
        .await?;

    let addr = "0.0.0.0:8084".parse()?;
    info!("ai-powered-streaming A2A server listening on {addr}");

    if let Err(e) = server.serve(addr).await {
        error!("server stopped: {e}");
    }

    Ok(())
}

use inference_gateway_adk::{A2AServerBuilder, AgentBuilder, AgentCardOverrides, Config};
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    let config: Config = envy::prefixed("A2A_").from_env()?;

    info!(
        provider = %config.agent_config.provider,
        model = %config.agent_config.model,
        has_api_key = config.agent_config.api_key.is_some(),
        "starting static-agent-card A2A server",
    );

    let agent = AgentBuilder::new()
        .with_config(&config.agent_config)
        .with_system_prompt("You are a helpful A2A assistant built with Rust and powered by the Inference Gateway SDK.")
        .with_max_chat_completion(15)
        .build()
        .await?;

    let port = config.server_config.port;
    let server = A2AServerBuilder::new()
        .with_config(config)
        .with_agent(agent)
        .with_agent_card_from_file(
            ".well-known/agent.json",
            Some(
                AgentCardOverrides::new()
                    .with_name("My Custom Agent")
                    .with_version("2.0.0")
                    .with_description(
                        "A customized A2A assistant built with Rust and powered by the Inference Gateway SDK.",
                    ),
            ),
        )
        .with_default_task_handlers()
        .build()
        .await?;

    let addr = format!("0.0.0.0:{port}").parse()?;
    info!("Configured A2A server with SDK integration running on port {port}");

    if let Err(e) = server.serve(addr).await {
        error!("Server failed to start: {}", e);
    }

    Ok(())
}

use inference_gateway_adk::{A2AServerBuilder, AgentBuilder, Config};
use tracing::{error, info};

/// `default-handlers` server: LLM agent attached, no custom `TaskHandler`,
/// no tools. Demonstrates that `with_default_task_handlers()` alone is
/// enough to drive an LLM-backed A2A server end to end.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    let config: Config = envy::prefixed("A2A_").from_env()?;

    info!(
        provider = %config.agent_config.provider,
        model = %config.agent_config.model,
        "starting default-handlers A2A server",
    );

    let agent = AgentBuilder::new()
        .with_config(&config.agent_config)
        .with_system_prompt(
            "You are a concise, helpful assistant. Keep replies under three sentences.",
        )
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
    info!("default-handlers A2A server listening on {addr}");

    if let Err(e) = server.serve(addr).await {
        error!("server stopped: {e}");
    }

    Ok(())
}

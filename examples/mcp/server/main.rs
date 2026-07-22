use inference_gateway_adk::{A2AServerBuilder, AgentBuilder, Config, McpClient, McpConfig};
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    let config: Config = envy::prefixed("A2A_").from_env()?;
    let mcp_config: McpConfig = envy::prefixed("MCP_").from_env().unwrap_or_default();

    info!(
        provider = %config.agent_config.provider,
        model = %config.agent_config.model,
        mcp_enabled = mcp_config.enable,
        mcp_servers = %mcp_config.servers,
        "starting MCP-enabled A2A server",
    );

    let mut agent_builder = AgentBuilder::new()
        .with_config(&config.agent_config)
        .with_system_prompt(
            "You are an A2A assistant with access to Model Context Protocol (MCP) tools. \
             Call mcp_list_tools to discover what is available (optionally passing a `search` \
             term), then mcp_call_tool to invoke a tool by `name` with an `arguments` object.",
        )
        .with_max_chat_completion(15);

    match McpClient::from_config(&mcp_config) {
        Some(client) => {
            client.start();
            agent_builder = agent_builder.with_mcp_client(client);
            info!("MCP client started; mcp_list_tools / mcp_call_tool registered on the agent");
        }
        None => info!("MCP disabled - set MCP_ENABLE=true and MCP_SERVERS=<urls> to enable"),
    }

    let agent = agent_builder.build().await?;

    let port = config.server_config.port;
    let server = A2AServerBuilder::new()
        .with_config(config)
        .with_agent(agent)
        .with_agent_card_from_file(".well-known/agent.json", None)
        .with_default_task_handlers()
        .build()
        .await?;

    let addr = format!("0.0.0.0:{port}").parse()?;
    info!("MCP-enabled A2A server running on port {port}");

    if let Err(e) = server.serve(addr).await {
        error!("Server failed to start: {e}");
    }

    Ok(())
}

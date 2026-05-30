use inference_gateway_adk::{A2AServerBuilder, AgentBuilder, Config};
use inference_gateway_sdk::{
    ChatCompletionTool, ChatCompletionToolType, FunctionObject, FunctionParameters,
};
use serde_json::{Value, json};
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    let config: Config = envy::prefixed("A2A_").from_env()?;

    info!(
        provider = %config.agent_config.provider,
        model = %config.agent_config.model,
        enable_usage_metadata = config.agent_config.enable_usage_metadata,
        "starting usage-metadata A2A server",
    );

    // A single tool so the emitted `execution_stats` shows a non-zero
    // `tool_calls` count alongside the token `usage` block.
    let tools = vec![ChatCompletionTool {
        type_: ChatCompletionToolType::Function,
        function: FunctionObject {
            name: "calculate_sum".to_string(),
            description: Some("Add a list of numbers together".to_string()),
            parameters: Some(FunctionParameters(
                json!({
                    "type": "object",
                    "properties": {
                        "numbers": {
                            "type": "array",
                            "items": {"type": "number"},
                            "description": "The numbers to add up"
                        }
                    },
                    "required": ["numbers"]
                })
                .as_object()
                .expect("schema is an object")
                .clone(),
            )),
            strict: false,
        },
    }];

    let agent = AgentBuilder::new()
        .with_config(&config.agent_config)
        .with_system_prompt(
            "You are a helpful assistant. When the user asks for a sum, call the \
             calculate_sum tool with the numbers, then report the result.",
        )
        .with_max_chat_completion(10)
        .with_toolbox(tools)
        .with_function_tool("calculate_sum".to_string(), |args: Value| {
            let sum: f64 = args["numbers"]
                .as_array()
                .map(|nums| nums.iter().filter_map(|n| n.as_f64()).sum())
                .unwrap_or(0.0);
            Ok(json!({ "sum": sum }).to_string())
        })
        .build()
        .await?;

    info!(
        "Agent built; usage metadata is {}",
        if agent.usage_metadata_enabled() {
            "enabled - terminal tasks will carry `usage` + `execution_stats`"
        } else {
            "disabled - terminal tasks will NOT carry usage metadata"
        }
    );

    let port = config.server_config.port;
    let server = A2AServerBuilder::new()
        .with_config(config)
        .with_agent(agent)
        .with_agent_card_from_file(".well-known/agent.json", None)
        .with_default_task_handlers()
        .build()
        .await?;

    let addr = format!("0.0.0.0:{port}").parse()?;
    info!("usage-metadata A2A server running on port {port}");

    if let Err(e) = server.serve(addr).await {
        error!("Server failed to start: {}", e);
    }

    Ok(())
}

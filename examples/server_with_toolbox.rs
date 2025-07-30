use inference_gateway_adk::{A2AServerBuilder, AgentBuilder, Config};
use inference_gateway_sdk::{FunctionObject, Tool, ToolType};
use serde_json::{Value, json};
use std::env;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    let config = Config::from_env()?;

    let gateway_url = env::var("INFERENCE_GATEWAY_URL")
        .unwrap_or_else(|_| "http://localhost:8080/v1".to_string());

    info!("Starting A2A server with toolbox functionality...");
    info!("Gateway URL: {}", gateway_url);
    info!("Agent provider: {}", config.agent_config.provider);
    info!("Agent model: {}", config.agent_config.model);
    info!("Has API key: {}", config.agent_config.api_key.is_some());

    let tools = vec![
        Tool {
            r#type: ToolType::Function,
            function: FunctionObject {
                name: "get_current_weather".to_string(),
                description: "Get the current weather information for a specific location"
                    .to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "location": {
                            "type": "string",
                            "description": "The city name to get weather for"
                        },
                        "unit": {
                            "type": "string",
                            "enum": ["celsius", "fahrenheit"],
                            "description": "The temperature unit"
                        }
                    },
                    "required": ["location"]
                }),
            },
        },
        Tool {
            r#type: ToolType::Function,
            function: FunctionObject {
                name: "calculate_math".to_string(),
                description: "Perform basic mathematical calculations".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "expression": {
                            "type": "string",
                            "description": "Mathematical expression to evaluate (e.g., '2 + 2', '10 * 5')"
                        }
                    },
                    "required": ["expression"]
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
                            "description": "Maximum number of results to return",
                            "default": 5
                        }
                    },
                    "required": ["query"]
                }),
            },
        },
    ];

    let agent = AgentBuilder::new()
        .with_config(&config.agent_config)
        .with_system_prompt(
            "You are a helpful A2A assistant with access to tools. \
            You can get weather information, perform calculations, and search the web. \
            When users ask for these capabilities, use the appropriate tools to provide accurate information."
        )
        .with_max_chat_completion(15)
        .with_toolbox(tools)
        .with_function_tool("get_current_weather".to_string(), |args: Value| {
            let location = args["location"].as_str().unwrap_or("Unknown");
            let unit = args["unit"].as_str().unwrap_or("celsius");

            let temperature = if unit == "fahrenheit" { "72°F" } else { "22°C" };

            Ok(json!({
                "location": location,
                "temperature": temperature,
                "condition": "sunny",
                "humidity": "65%",
                "wind": "5 mph"
            }).to_string())
        })
        .with_function_tool("calculate_math".to_string(), |args: Value| {
            let expression = args["expression"].as_str().unwrap_or("");

            let result = match expression {
                expr if expr.contains(" + ") => {
                    let parts: Vec<&str> = expr.split(" + ").collect();
                    if parts.len() == 2 {
                        let a: f64 = parts[0].parse().unwrap_or(0.0);
                        let b: f64 = parts[1].parse().unwrap_or(0.0);
                        (a + b).to_string()
                    } else {
                        "Invalid expression".to_string()
                    }
                },
                expr if expr.contains(" * ") => {
                    let parts: Vec<&str> = expr.split(" * ").collect();
                    if parts.len() == 2 {
                        let a: f64 = parts[0].parse().unwrap_or(0.0);
                        let b: f64 = parts[1].parse().unwrap_or(0.0);
                        (a * b).to_string()
                    } else {
                        "Invalid expression".to_string()
                    }
                },
                expr if expr.contains(" - ") => {
                    let parts: Vec<&str> = expr.split(" - ").collect();
                    if parts.len() == 2 {
                        let a: f64 = parts[0].parse().unwrap_or(0.0);
                        let b: f64 = parts[1].parse().unwrap_or(0.0);
                        (a - b).to_string()
                    } else {
                        "Invalid expression".to_string()
                    }
                },
                expr if expr.contains(" / ") => {
                    let parts: Vec<&str> = expr.split(" / ").collect();
                    if parts.len() == 2 {
                        let a: f64 = parts[0].parse().unwrap_or(0.0);
                        let b: f64 = parts[1].parse().unwrap_or(1.0);
                        if b != 0.0 {
                            (a / b).to_string()
                        } else {
                            "Division by zero".to_string()
                        }
                    } else {
                        "Invalid expression".to_string()
                    }
                },
                _ => "Unsupported operation".to_string()
            };

            Ok(json!({
                "expression": expression,
                "result": result
            }).to_string())
        })
        .with_async_function_tool("search_web".to_string(), |args: Value| async move {
            let query = args["query"].as_str().unwrap_or("");
            let limit = args["limit"].as_i64().unwrap_or(5);

            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

            let results = [
                json!({
                    "title": format!("Search result 1 for '{query}'"),
                    "url": "https://example.com/1",
                    "snippet": "This is a mock search result..."
                }),
                json!({
                    "title": format!("Search result 2 for '{query}'"),
                    "url": "https://example.com/2", 
                    "snippet": "Another mock search result..."
                })
            ];

            Ok(json!({
                "query": query,
                "results": results[..std::cmp::min(limit as usize, results.len())].to_vec(),
                "total_results": results.len()
            }).to_string())
        })
        .build()
        .await?;

    info!("Agent built with toolbox and handlers");

    let server = A2AServerBuilder::new()
        .with_config(config)
        .with_agent(agent)
        .with_agent_card_from_file(".well-known/agent.json", None)
        .with_gateway_url(gateway_url)
        .build()
        .await?;

    let addr = "0.0.0.0:8082".parse()?;
    info!("A2A server with toolbox running on port 8082");

    if let Err(e) = server.serve(addr).await {
        error!("Server failed to start: {}", e);
    }

    Ok(())
}

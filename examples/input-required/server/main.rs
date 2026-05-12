use inference_gateway_adk::A2AServerBuilder;
use inference_gateway_adk::a2a_types::{
    Message, Part, Role, Task, TaskState, TaskStatus, Timestamp,
};
use inference_gateway_adk::{Config, TaskHandler};
use tracing::{error, info};

/// Non-LLM handler that pauses tasks in `TaskStateInputRequired` when
/// the user message doesn't mention a known city. Demonstrates the
/// state-machine flow without any AI noise.
///
/// Note: this rust-adk version does not yet wire a "resume" path for
/// `message/send` carrying an existing `task_id` — every send creates
/// a new task. The example therefore demonstrates the InputRequired
/// state on its own; the client dispatches two independent tasks (one
/// with a city, one without) to show both branches side by side.
#[derive(Debug)]
struct WeatherHandler;

const KNOWN_CITIES: &[&str] = &[
    "london",
    "paris",
    "tokyo",
    "berlin",
    "new york",
    "san francisco",
];

fn extract_city(text: &str) -> Option<String> {
    let lower = text.to_lowercase();
    KNOWN_CITIES
        .iter()
        .find(|city| lower.contains(*city))
        .map(|city| (*city).to_string())
}

fn agent_text(task: &Task, text: &str) -> Message {
    Message {
        context_id: Some(task.context_id.clone()),
        extensions: vec![],
        message_id: uuid::Uuid::new_v4().to_string(),
        metadata: None,
        parts: vec![Part {
            data: None,
            file: None,
            metadata: None,
            text: Some(text.to_string()),
        }],
        reference_task_ids: vec![],
        role: Role::RoleAgent,
        task_id: Some(task.id.clone()),
    }
}

#[async_trait::async_trait]
impl TaskHandler for WeatherHandler {
    async fn handle_task(&self, mut task: Task, message: Option<Message>) -> anyhow::Result<Task> {
        let user_text = message
            .as_ref()
            .and_then(|m| m.parts.first())
            .and_then(|p| p.text.clone())
            .unwrap_or_default();

        match extract_city(&user_text) {
            Some(city) => {
                info!(task_id = %task.id, city, "weather request — completing");
                let reply = agent_text(&task, &format!("Weather in {city}: 18°C, partly cloudy."));
                task.history.push(reply.clone());
                task.status = TaskStatus {
                    message: Some(reply),
                    state: TaskState::TaskStateCompleted,
                    timestamp: Some(Timestamp(chrono::Utc::now())),
                };
            }
            None => {
                info!(task_id = %task.id, "weather request — pausing for city input");
                let prompt = agent_text(
                    &task,
                    "Which city would you like the weather for? \
                     (Try one of: London, Paris, Tokyo, Berlin, New York, San Francisco)",
                );
                task.history.push(prompt.clone());
                task.status = TaskStatus {
                    message: Some(prompt),
                    state: TaskState::TaskStateInputRequired,
                    timestamp: Some(Timestamp(chrono::Utc::now())),
                };
            }
        }
        Ok(task)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    let config = Config::from_env()?;

    let server = A2AServerBuilder::new()
        .with_config(config)
        .with_agent_card_from_file(".well-known/agent.json", None)
        .with_background_task_handler(WeatherHandler)
        .with_default_streaming_task_handler()
        .build()
        .await?;

    let addr = "0.0.0.0:8087".parse()?;
    info!("input-required A2A server listening on {addr}");

    if let Err(e) = server.serve(addr).await {
        error!("server stopped: {e}");
    }

    Ok(())
}

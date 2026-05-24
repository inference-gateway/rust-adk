use inference_gateway_adk::a2a_types::{Message as A2AMessage, Task, TaskState};
use inference_gateway_adk::{A2AServerBuilder, StreamEmitter, StreamableTaskHandler};
use std::time::Duration;
use tracing::{error, info};

/// Streams a fixed sentence one word at a time so the SSE flow is
/// visible in the client log. No LLM dependency - the cadence and
/// content are hardcoded.
#[derive(Debug)]
struct WordByWordStreamHandler;

#[async_trait::async_trait]
impl StreamableTaskHandler for WordByWordStreamHandler {
    async fn handle_streaming_task(
        &self,
        task: Task,
        _message: Option<A2AMessage>,
        emitter: StreamEmitter,
    ) -> anyhow::Result<()> {
        let sentence = "The quick brown fox jumps over the lazy dog.";

        emitter
            .emit_status(
                &task.id,
                &task.context_id,
                TaskState::TaskStateWorking,
                None,
                false,
            )
            .await?;

        let words: Vec<&str> = sentence.split_whitespace().collect();
        for (i, word) in words.iter().enumerate() {
            tokio::time::sleep(Duration::from_millis(150)).await;
            let is_last = i == words.len() - 1;
            let chunk = if is_last {
                word.to_string()
            } else {
                format!("{word} ")
            };
            emitter
                .emit_text_artifact(&task.id, &task.context_id, chunk, is_last)
                .await?;
        }

        emitter
            .emit_status(
                &task.id,
                &task.context_id,
                TaskState::TaskStateCompleted,
                None,
                true,
            )
            .await?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();

    let server = A2AServerBuilder::new()
        .with_agent_card_from_file(".well-known/agent.json", None)
        .with_streaming_task_handler(WordByWordStreamHandler)
        .with_default_background_task_handler()
        .build()
        .await?;

    let addr = "0.0.0.0:8080".parse()?;
    info!("streaming A2A server listening on {addr}");

    if let Err(e) = server.serve(addr).await {
        error!("server stopped: {e}");
    }

    Ok(())
}

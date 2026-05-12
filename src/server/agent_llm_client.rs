use crate::config::AgentConfig;
use anyhow::{Result, anyhow};
use futures_util::stream::{Stream, StreamExt};
use inference_gateway_sdk::{
    ChatCompletionTool, CreateChatCompletionResponse, InferenceGatewayAPI, InferenceGatewayClient,
    Message, Provider, SSEvents,
};
use std::pin::Pin;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;
use tokio_stream::wrappers::ReceiverStream;
use tracing::debug;

/// Defines the interface for Language Model clients used by an [`Agent`].
///
/// Mirrors the Go ADK's `LLMClient` interface
/// (`adk/server/agent_llm_client.go`): a non-streaming chat completion
/// method plus a streaming one. Custom implementations let callers plug in
/// alternative LLM backends or test doubles while keeping the rest of the
/// ADK (`run_tool_loop`, default task handlers) unchanged.
///
/// [`Agent`]: super::agent::Agent
#[async_trait::async_trait]
pub trait LLMClient: Send + Sync + std::fmt::Debug {
    /// Send a non-streaming chat completion request.
    async fn create_chat_completion(
        &self,
        messages: Vec<Message>,
        tools: Option<Vec<ChatCompletionTool>>,
    ) -> Result<CreateChatCompletionResponse>;

    /// Send a streaming chat completion request.
    ///
    /// Returns a boxed `Stream` of SSE events from the gateway. The stream
    /// terminates when the gateway signals end-of-stream or an error occurs.
    fn create_streaming_chat_completion(
        &self,
        messages: Vec<Message>,
        tools: Option<Vec<ChatCompletionTool>>,
    ) -> Pin<Box<dyn Stream<Item = Result<SSEvents>> + Send>>;
}

/// `LLMClient` implementation that talks to an OpenAI-compatible HTTP API
/// via the [`InferenceGatewayClient`] SDK.
///
/// Construct one via [`OpenAICompatibleLLMClient::new`] (reads provider,
/// model, and base URL from [`AgentConfig`]). Each chat completion call
/// retries up to `config.max_retries` times on failure, with a linear
/// 1-second backoff per attempt — matching the Go ADK's behaviour in
/// `OpenAICompatibleLLMClient.CreateChatCompletion`.
pub struct OpenAICompatibleLLMClient {
    base_url: String,
    config: AgentConfig,
    provider: Provider,
    model: String,
}

impl std::fmt::Debug for OpenAICompatibleLLMClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpenAICompatibleLLMClient")
            .field("base_url", &self.base_url)
            .field("provider", &self.provider)
            .field("model", &self.model)
            .finish()
    }
}

impl OpenAICompatibleLLMClient {
    /// Build a client from an [`AgentConfig`]. Reads `provider`, `model`,
    /// and `base_url`. If `base_url` is `None`, defaults to
    /// `http://localhost:8080/v1` (mirrors the Inference Gateway's default).
    pub fn new(config: &AgentConfig) -> Result<Self> {
        if config.provider.is_empty() {
            return Err(anyhow!("provider is required"));
        }
        if config.model.is_empty() {
            return Err(anyhow!("model is required"));
        }

        let provider = parse_provider(&config.provider)?;
        let base_url = config
            .base_url
            .clone()
            .unwrap_or_else(|| "http://localhost:8080/v1".to_string());

        Ok(Self {
            base_url,
            config: config.clone(),
            provider,
            model: config.model.clone(),
        })
    }

    /// Override the base URL after construction. Useful when the URL only
    /// becomes known after `AgentConfig` has been built (e.g. tests using a
    /// random port for a mock gateway).
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// Access the base URL currently in use.
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    fn sdk_client(&self, tools: Option<Vec<ChatCompletionTool>>) -> InferenceGatewayClient {
        let client = InferenceGatewayClient::new(&self.base_url);
        match tools {
            Some(t) if !t.is_empty() => client.with_tools(Some(t)),
            _ => client,
        }
    }
}

#[async_trait::async_trait]
impl LLMClient for OpenAICompatibleLLMClient {
    async fn create_chat_completion(
        &self,
        messages: Vec<Message>,
        tools: Option<Vec<ChatCompletionTool>>,
    ) -> Result<CreateChatCompletionResponse> {
        let max_retries = self.config.max_retries;
        let mut last_err: Option<anyhow::Error> = None;

        for attempt in 0..=max_retries {
            if attempt > 0 {
                debug!("retrying llm request (attempt {}/{})", attempt, max_retries);
                sleep(Duration::from_secs(attempt as u64)).await;
            }

            let client = self.sdk_client(tools.clone());
            match client
                .generate_content(self.provider, &self.model, messages.clone())
                .await
            {
                Ok(response) => {
                    if response.choices.is_empty() {
                        return Err(anyhow!("no choices returned from llm"));
                    }
                    return Ok(response);
                }
                Err(e) => {
                    debug!("llm request failed (attempt {}): {e}", attempt + 1);
                    last_err = Some(anyhow!("{e}"));
                }
            }
        }

        Err(anyhow!(
            "llm request failed after {} retries: {}",
            max_retries,
            last_err
                .map(|e| e.to_string())
                .unwrap_or_else(|| "unknown error".to_string())
        ))
    }

    fn create_streaming_chat_completion(
        &self,
        messages: Vec<Message>,
        tools: Option<Vec<ChatCompletionTool>>,
    ) -> Pin<Box<dyn Stream<Item = Result<SSEvents>> + Send>> {
        let base_url = self.base_url.clone();
        let provider = self.provider;
        let model = self.model.clone();

        let (tx, rx) = mpsc::channel::<Result<SSEvents>>(32);

        tokio::spawn(async move {
            let client = InferenceGatewayClient::new(&base_url);
            let client = match tools {
                Some(t) if !t.is_empty() => client.with_tools(Some(t)),
                _ => client,
            };
            let mut sdk_stream =
                Box::pin(client.generate_content_stream(provider, &model, messages));
            while let Some(item) = sdk_stream.next().await {
                let mapped = item.map_err(|e| anyhow!("{e}"));
                if tx.send(mapped).await.is_err() {
                    break;
                }
            }
        });

        Box::pin(ReceiverStream::new(rx))
    }
}

pub(super) fn parse_provider(provider_str: &str) -> Result<Provider> {
    match provider_str.to_lowercase().as_str() {
        "groq" => Ok(Provider::Groq),
        "google" => Ok(Provider::Google),
        "openai" => Ok(Provider::Openai),
        "anthropic" => Ok(Provider::Anthropic),
        "cohere" => Ok(Provider::Cohere),
        "cloudflare" => Ok(Provider::Cloudflare),
        "deepseek" => Ok(Provider::Deepseek),
        "ollama" => Ok(Provider::Ollama),
        _ => Err(anyhow!(
            "Unsupported provider: {}. Supported providers: groq, google, openai, anthropic, cohere, cloudflare, deepseek, ollama",
            provider_str
        )),
    }
}

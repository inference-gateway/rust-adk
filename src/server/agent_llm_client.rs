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
/// A non-streaming chat completion
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
/// model, base URL, API key, `max_tokens` and `timeout_secs` from
/// [`AgentConfig`]). Each chat completion call retries up to
/// `config.max_retries` times on failure, with a linear 1-second backoff per
/// attempt, and is bounded by `config.timeout_secs` (`0` disables the bound).
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
    /// `base_url`, `api_key`, `max_tokens`, `timeout_secs` and `max_retries`.
    /// If `base_url` is `None`, defaults to
    /// `http://gateway:8080/v1` (matches the typical docker-compose service
    /// name for the Inference Gateway).
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
            .unwrap_or_else(|| "http://gateway:8080/v1".to_string());

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

    /// Build an SDK client carrying the configured API key (bearer token) and
    /// `max_tokens`. The SDK omits `max_tokens` from streaming requests, so it
    /// only takes effect on non-streaming completions.
    fn sdk_client(&self, tools: Option<Vec<ChatCompletionTool>>) -> InferenceGatewayClient {
        let mut client = InferenceGatewayClient::new(&self.base_url);
        if let Some(api_key) = self.config.api_key.as_deref().filter(|k| !k.is_empty()) {
            client = client.with_token(api_key);
        }
        if self.config.max_tokens > 0 {
            client = client.with_max_tokens(Some(self.config.max_tokens as i64));
        }
        match tools {
            Some(t) if !t.is_empty() => client.with_tools(Some(t)),
            _ => client,
        }
    }

    /// Per-request timeout, or `None` when `timeout_secs` is `0`.
    fn request_timeout(&self) -> Option<Duration> {
        (self.config.timeout_secs > 0).then(|| self.config.timeout())
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
            let request = client.generate_content(self.provider, &self.model, messages.clone());
            let result = match self.request_timeout() {
                Some(d) => match tokio::time::timeout(d, request).await {
                    Ok(r) => r.map_err(|e| anyhow!("{e}")),
                    Err(_) => Err(anyhow!("llm request timed out after {}s", d.as_secs())),
                },
                None => request.await.map_err(|e| anyhow!("{e}")),
            };

            match result {
                Ok(response) => {
                    if response.choices.is_empty() {
                        return Err(anyhow!("no choices returned from llm"));
                    }
                    return Ok(response);
                }
                Err(e) => {
                    debug!("llm request failed (attempt {}): {e}", attempt + 1);
                    last_err = Some(e);
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
        let provider = self.provider;
        let model = self.model.clone();
        let client = self.sdk_client(tools);
        let timeout = self.request_timeout();

        let (tx, rx) = mpsc::channel::<Result<SSEvents>>(32);

        tokio::spawn(async move {
            let mut sdk_stream =
                Box::pin(client.generate_content_stream(provider, &model, messages));
            loop {
                // `timeout` bounds the wait for each event, not the whole stream.
                let next = match timeout {
                    Some(d) => match tokio::time::timeout(d, sdk_stream.next()).await {
                        Ok(item) => item,
                        Err(_) => {
                            let _ = tx
                                .send(Err(anyhow!(
                                    "llm stream stalled for more than {}s",
                                    d.as_secs()
                                )))
                                .await;
                            break;
                        }
                    },
                    None => sdk_stream.next().await,
                };
                let Some(item) = next else { break };
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
        "nvidia" => Ok(Provider::Nvidia),
        "llamacpp" => Ok(Provider::Llamacpp),
        _ => Err(anyhow!(
            "Unsupported provider: {}. Supported providers: groq, google, openai, anthropic, cohere, cloudflare, deepseek, ollama, nvidia, llamacpp",
            provider_str
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{Json, Router, extract::State, http::HeaderMap, routing::post};
    use std::sync::{Arc, Mutex};
    use tokio::net::TcpListener;

    type Captured = Arc<Mutex<Option<(Option<String>, serde_json::Value)>>>;

    /// Mock OpenAI-compatible gateway recording the `Authorization` header and
    /// request body of the last `POST /chat/completions`, answering after
    /// `delay`.
    async fn spawn_gateway(delay: Duration) -> (String, Captured) {
        async fn chat(
            State((captured, delay)): State<(Captured, Duration)>,
            headers: HeaderMap,
            body: axum::body::Bytes,
        ) -> Json<serde_json::Value> {
            *captured.lock().expect("mutex poisoned") = Some((
                headers
                    .get("authorization")
                    .and_then(|v| v.to_str().ok())
                    .map(str::to_string),
                serde_json::from_slice(&body).expect("valid JSON"),
            ));
            sleep(delay).await;
            Json(serde_json::json!({
                "id": "chatcmpl-1",
                "object": "chat.completion",
                "created": 0,
                "model": "test-model",
                "choices": [{
                    "index": 0,
                    "finish_reason": "stop",
                    "message": {"role": "assistant", "content": "ok", "tool_calls": []},
                }],
            }))
        }

        let captured: Captured = Arc::new(Mutex::new(None));
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
        let addr = listener.local_addr().expect("addr");
        let app = Router::new()
            .route("/chat/completions", post(chat))
            .with_state((Arc::clone(&captured), delay));
        tokio::spawn(async move {
            axum::serve(listener, app).await.ok();
        });
        (format!("http://{addr}"), captured)
    }

    fn config(base_url: String) -> AgentConfig {
        AgentConfig {
            provider: "openai".to_string(),
            model: "test-model".to_string(),
            base_url: Some(base_url),
            max_retries: 0,
            ..AgentConfig::default()
        }
    }

    #[tokio::test]
    async fn forwards_api_key_and_max_tokens() {
        let (base_url, captured) = spawn_gateway(Duration::ZERO).await;
        let client = OpenAICompatibleLLMClient::new(&AgentConfig {
            api_key: Some("secret-key".to_string()),
            max_tokens: 16,
            ..config(base_url)
        })
        .expect("client builds");

        client
            .create_chat_completion(vec![], None)
            .await
            .expect("completion succeeds");

        let (auth, body) = captured
            .lock()
            .expect("mutex poisoned")
            .clone()
            .expect("gateway was called");
        assert_eq!(
            auth.as_deref(),
            Some("Bearer secret-key"),
            "api_key should be sent as a bearer token"
        );
        assert_eq!(
            body.get("max_tokens").and_then(|v| v.as_i64()),
            Some(16),
            "max_tokens should reach the gateway"
        );
    }

    #[tokio::test]
    async fn omits_credentials_when_unset() {
        let (base_url, captured) = spawn_gateway(Duration::ZERO).await;
        let client = OpenAICompatibleLLMClient::new(&AgentConfig {
            max_tokens: 0,
            ..config(base_url)
        })
        .expect("client builds");

        client
            .create_chat_completion(vec![], None)
            .await
            .expect("completion succeeds");

        let (auth, body) = captured
            .lock()
            .expect("mutex poisoned")
            .clone()
            .expect("gateway was called");
        assert!(auth.is_none(), "no api_key means no Authorization header");
        assert!(
            body.get("max_tokens").is_none_or(|v| v.is_null()),
            "max_tokens of 0 should leave the gateway default in place"
        );
    }

    #[tokio::test]
    async fn times_out_slow_requests() {
        let (base_url, _captured) = spawn_gateway(Duration::from_secs(30)).await;
        let client = OpenAICompatibleLLMClient::new(&AgentConfig {
            timeout_secs: 1,
            ..config(base_url)
        })
        .expect("client builds");

        let err = client
            .create_chat_completion(vec![], None)
            .await
            .expect_err("slow gateway should time out")
            .to_string();
        assert!(err.contains("timed out"), "unexpected error: {err}");
    }

    #[derive(Debug)]
    struct ProviderCase {
        input: &'static str,
        expected: Provider,
    }

    #[test]
    fn parse_provider_maps_known_providers() {
        let cases = vec![
            ProviderCase {
                input: "groq",
                expected: Provider::Groq,
            },
            ProviderCase {
                input: "google",
                expected: Provider::Google,
            },
            ProviderCase {
                input: "openai",
                expected: Provider::Openai,
            },
            ProviderCase {
                input: "anthropic",
                expected: Provider::Anthropic,
            },
            ProviderCase {
                input: "cohere",
                expected: Provider::Cohere,
            },
            ProviderCase {
                input: "cloudflare",
                expected: Provider::Cloudflare,
            },
            ProviderCase {
                input: "deepseek",
                expected: Provider::Deepseek,
            },
            ProviderCase {
                input: "ollama",
                expected: Provider::Ollama,
            },
            ProviderCase {
                input: "nvidia",
                expected: Provider::Nvidia,
            },
            ProviderCase {
                input: "llamacpp",
                expected: Provider::Llamacpp,
            },
        ];

        for case in cases {
            let parsed = parse_provider(case.input)
                .unwrap_or_else(|e| panic!("provider {} should parse: {e}", case.input));
            assert_eq!(
                parsed, case.expected,
                "provider {} mapped to the wrong variant",
                case.input
            );
        }
    }

    #[test]
    fn parse_provider_is_case_insensitive() {
        assert_eq!(parse_provider("NVIDIA").unwrap(), Provider::Nvidia);
        assert_eq!(parse_provider("Nvidia").unwrap(), Provider::Nvidia);
    }

    #[test]
    fn parse_provider_rejects_unknown() {
        let err = parse_provider("does-not-exist")
            .expect_err("unknown provider should be rejected")
            .to_string();
        assert!(err.contains("Unsupported provider"));
        assert!(
            err.contains("nvidia"),
            "error should advertise nvidia support"
        );
    }
}

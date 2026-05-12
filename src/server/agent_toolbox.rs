use anyhow::Result;
use serde_json::Value;

/// Trait for handling tool calls
#[async_trait::async_trait]
pub trait ToolHandler: Send + Sync {
    /// Handle a tool call with the given arguments and return the result
    async fn handle(&self, args: Value) -> Result<String>;
}

/// A simple function-based tool handler
pub struct FunctionToolHandler<F>
where
    F: Fn(Value) -> Result<String> + Send + Sync,
{
    handler: F,
}

impl<F> FunctionToolHandler<F>
where
    F: Fn(Value) -> Result<String> + Send + Sync,
{
    pub fn new(handler: F) -> Self {
        Self { handler }
    }
}

#[async_trait::async_trait]
impl<F> ToolHandler for FunctionToolHandler<F>
where
    F: Fn(Value) -> Result<String> + Send + Sync,
{
    async fn handle(&self, args: Value) -> Result<String> {
        (self.handler)(args)
    }
}

/// An async function-based tool handler
pub struct AsyncFunctionToolHandler<F, Fut>
where
    F: Fn(Value) -> Fut + Send + Sync,
    Fut: std::future::Future<Output = Result<String>> + Send,
{
    handler: F,
}

impl<F, Fut> AsyncFunctionToolHandler<F, Fut>
where
    F: Fn(Value) -> Fut + Send + Sync,
    Fut: std::future::Future<Output = Result<String>> + Send,
{
    pub fn new(handler: F) -> Self {
        Self { handler }
    }
}

#[async_trait::async_trait]
impl<F, Fut> ToolHandler for AsyncFunctionToolHandler<F, Fut>
where
    F: Fn(Value) -> Fut + Send + Sync,
    Fut: std::future::Future<Output = Result<String>> + Send,
{
    async fn handle(&self, args: Value) -> Result<String> {
        (self.handler)(args).await
    }
}

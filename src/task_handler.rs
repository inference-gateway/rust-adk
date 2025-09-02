use crate::a2a_types::{Message, Task, TaskState, TaskStatus, MessageRole, Part, TextPart};
use crate::config::{Config, QueueConfig};
use crate::server::Agent;
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use tokio::time::{Duration, Instant};
use tracing::{debug, error, info, warn};
use uuid::Uuid;
use inference_gateway_sdk::{Message as SdkMessage, MessageRole as SdkMessageRole};

/// Convert SDK message to A2A message format
fn convert_sdk_message_to_a2a(sdk_msg: &SdkMessage) -> Message {
    let role = match sdk_msg.role {
        SdkMessageRole::System => MessageRole::Agent, // Map system to agent
        SdkMessageRole::User => MessageRole::User,
        SdkMessageRole::Assistant => MessageRole::Agent,
        SdkMessageRole::Tool => MessageRole::Agent, // Map tool to agent
    };

    let parts = if let Some(ref content) = sdk_msg.content {
        vec![Part::TextPart(TextPart {
            kind: "text".to_string(),
            metadata: Default::default(),
            text: content.clone(),
        })]
    } else {
        Vec::new()
    };

    Message {
        context_id: None,
        extensions: Vec::new(),
        kind: "message".to_string(),
        message_id: Uuid::new_v4().to_string(),
        metadata: Default::default(),
        parts,
        reference_task_ids: Vec::new(),
        role,
        task_id: None,
    }
}

/// Convert A2A message to SDK message format
fn convert_a2a_message_to_sdk(a2a_msg: &Message) -> SdkMessage {
    let role = match a2a_msg.role {
        MessageRole::User => SdkMessageRole::User,
        MessageRole::Agent => SdkMessageRole::Assistant,
    };

    let content = a2a_msg.parts.iter()
        .find_map(|part| match part {
            Part::TextPart(text_part) => Some(text_part.text.clone()),
            _ => None,
        });

    SdkMessage {
        role,
        content,
        ..Default::default()
    }
}

/// Helper function to create A2A message with text content
fn create_a2a_message(role: MessageRole, text: &str) -> Message {
    Message {
        context_id: None,
        extensions: Vec::new(),
        kind: "message".to_string(),
        message_id: Uuid::new_v4().to_string(),
        metadata: Default::default(),
        parts: vec![Part::TextPart(TextPart {
            kind: "text".to_string(),
            metadata: Default::default(),
            text: text.to_string(),
        })],
        reference_task_ids: Vec::new(),
        role,
        task_id: None,
    }
}

/// Configuration for background task handler
#[derive(Debug, Clone)]
pub struct BackgroundTaskHandlerConfig {
    /// Maximum number of tasks that can be queued
    pub max_queue_size: usize,
    /// Interval between polling for new tasks
    pub poll_interval: Duration,
    /// Timeout for individual task processing
    pub task_timeout: Duration,
    /// Interval after which completed tasks are cleaned up
    pub cleanup_interval: Duration,
    /// Maximum number of retries for failed tasks
    pub max_retries: u32,
    /// Interval between retry attempts
    pub retry_interval: Duration,
    /// Maximum time tasks stay in dead letter queue before removal
    pub dead_letter_ttl: Duration,
    /// Maximum chat completion iterations per task
    pub max_chat_completion_iterations: u32,
}

impl Default for BackgroundTaskHandlerConfig {
    fn default() -> Self {
        Self {
            max_queue_size: 1000,
            poll_interval: Duration::from_millis(100),
            task_timeout: Duration::from_secs(300), // 5 minutes
            cleanup_interval: Duration::from_secs(60), // 1 minute
            max_retries: 3,
            retry_interval: Duration::from_secs(10),
            dead_letter_ttl: Duration::from_secs(3600), // 1 hour
            max_chat_completion_iterations: 10,
        }
    }
}

impl From<&Config> for BackgroundTaskHandlerConfig {
    fn from(config: &Config) -> Self {
        Self {
            max_queue_size: config.queue_config.max_size,
            task_timeout: config.queue_config.timeout,
            max_chat_completion_iterations: config.agent_config.max_chat_completion_iterations,
            ..Self::default()
        }
    }
}

/// Represents a queued task with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueuedTask {
    pub id: String,
    pub context_id: String,
    pub messages: Vec<SdkMessage>,
    pub submitted_at: Instant,
    pub retries: u32,
    pub last_retry_at: Option<Instant>,
    pub metadata: HashMap<String, serde_json::Value>,
}

impl QueuedTask {
    pub fn new(
        messages: Vec<SdkMessage>,
        context_id: Option<String>,
        metadata: HashMap<String, serde_json::Value>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            context_id: context_id.unwrap_or_else(|| Uuid::new_v4().to_string()),
            messages,
            submitted_at: Instant::now(),
            retries: 0,
            last_retry_at: None,
            metadata,
        }
    }

    pub fn increment_retry(&mut self) {
        self.retries += 1;
        self.last_retry_at = Some(Instant::now());
    }
}

/// A task in the system with full A2A Task structure
#[derive(Debug, Clone)]
pub struct ManagedTask {
    pub task: Task,
    pub queued_task: QueuedTask,
    pub status: TaskStatus,
    pub created_at: Instant,
    pub updated_at: Instant,
}

impl ManagedTask {
    pub fn from_queued_task(queued_task: QueuedTask) -> Self {
        let status = TaskStatus {
            state: TaskState::Submitted,
            message: None,
            timestamp: chrono::Utc::now(),
        };

        let task = Task {
            id: queued_task.id.clone(),
            kind: "task".to_string(),
            context_id: queued_task.context_id.clone(),
            status: status.clone(),
            history: queued_task.messages.iter().map(|m| convert_sdk_message_to_a2a(m)).collect(),
            artifacts: Vec::new(),
        };

        Self {
            task,
            queued_task,
            status,
            created_at: Instant::now(),
            updated_at: Instant::now(),
        }
    }

    pub fn update_status(&mut self, new_state: TaskState, message: Option<Message>) {
        self.status.state = new_state;
        self.status.message = message;
        self.status.timestamp = chrono::Utc::now();
        self.task.status = self.status.clone();
        self.updated_at = Instant::now();
    }

    pub fn is_terminal(&self) -> bool {
        matches!(
            self.status.state,
            TaskState::Completed | TaskState::Canceled
        )
    }

    pub fn should_retry(&self, config: &BackgroundTaskHandlerConfig) -> bool {
        self.queued_task.retries < config.max_retries
            && !self.is_terminal()
            && self
                .queued_task
                .last_retry_at
                .map_or(true, |last_retry| {
                    last_retry.elapsed() >= config.retry_interval
                })
    }
}

/// Trait for task handlers that can process tasks in different ways
#[async_trait::async_trait]
pub trait TaskHandler: Send + Sync {
    /// Process a task and return the result
    async fn process_task(&self, task: &mut ManagedTask) -> Result<()>;

    /// Get the handler name for logging/debugging
    fn name(&self) -> &'static str;
}

/// Default background task handler that processes tasks with agents
pub struct DefaultBackgroundTaskHandler {
    pub config: BackgroundTaskHandlerConfig,
    pub agent: Option<Arc<Agent>>,
    pub gateway_url: String,
}

impl DefaultBackgroundTaskHandler {
    pub fn new(
        config: BackgroundTaskHandlerConfig,
        agent: Option<Arc<Agent>>,
        gateway_url: String,
    ) -> Self {
        Self {
            config,
            agent,
            gateway_url,
        }
    }
}

#[async_trait::async_trait]
impl TaskHandler for DefaultBackgroundTaskHandler {
    async fn process_task(&self, task: &mut ManagedTask) -> Result<()> {
        debug!("Processing task {} with default handler", task.task.id);

        // Update task to working state
        task.update_status(TaskState::Working, None);

        // Get agent or return early if none configured
        let agent = match &self.agent {
            Some(agent) => agent.clone(),
            None => {
                let error_message = create_a2a_message(
                    MessageRole::Agent, 
                    "No agent configured for task processing"
                );
                task.update_status(TaskState::Completed, Some(error_message));
                return Ok(());
            }
        };

        // Process with agent using the existing logic from a2a_handler
        let gateway_client = inference_gateway_sdk::InferenceGatewayClient::new(&self.gateway_url);

        // Prepare messages with system prompt
        let mut final_messages = Vec::new();
        if let Some(ref system_prompt) = agent.system_prompt {
            final_messages.push(SdkMessage {
                role: SdkMessageRole::System,
                content: Some(system_prompt.clone()),
                ..Default::default()
            });
        }
        final_messages.extend(task.queued_task.messages.clone());

        // Add tools if available
        let client_with_tools = if let Some(tools) = agent.toolbox() {
            gateway_client.with_tools(Some(tools.clone()))
        } else {
            gateway_client
        };

        // Process task with configurable iterations
        let mut iteration = 0;
        let max_iterations = self.config.max_chat_completion_iterations;

        while iteration < max_iterations {
            iteration += 1;
            debug!("Task {} iteration {}/{}", task.task.id, iteration, max_iterations);

            match client_with_tools
                .generate_content(agent.provider, &agent.model, final_messages.clone())
                .await
            {
                Ok(response) => {
                    let choice = match response.choices.first() {
                        Some(choice) => choice,
                        None => {
                            let error_message = create_a2a_message(
                                MessageRole::Agent,
                                "No response generated from LLM"
                            );
                            task.update_status(TaskState::Completed, Some(error_message));
                            return Ok(());
                        }
                    };

                    // Check for tool calls
                    if let Some(tool_calls) = &choice.message.tool_calls {
                        debug!("Processing {} tool calls", tool_calls.len());

                        // Add assistant message with tool calls to conversation
                        final_messages.push(SdkMessage {
                            role: SdkMessageRole::Assistant,
                            content: choice.message.content.clone(),
                            tool_calls: choice.message.tool_calls.clone(),
                            ..Default::default()
                        });

                        let mut has_tool_results = false;

                        // Process each tool call
                        for tool_call in tool_calls {
                            debug!("Processing tool call: {}", tool_call.function.name);

                            if let Some(handler) = agent.tool_handlers.get(&tool_call.function.name) {
                                match tool_call.function.parse_arguments() {
                                    Ok(args) => match handler.handle(args).await {
                                        Ok(result) => {
                                            debug!("Tool call '{}' completed successfully", tool_call.function.name);
                                            final_messages.push(SdkMessage {
                                                role: SdkMessageRole::Tool,
                                                content: Some(result),
                                                tool_call_id: Some(tool_call.id.clone()),
                                                ..Default::default()
                                            });
                                            has_tool_results = true;
                                        }
                                        Err(e) => {
                                            error!("Tool call '{}' failed: {}", tool_call.function.name, e);
                                            final_messages.push(SdkMessage {
                                                role: SdkMessageRole::Tool,
                                                content: Some(format!("Error: {e}")),
                                                tool_call_id: Some(tool_call.id.clone()),
                                                ..Default::default()
                                            });
                                            has_tool_results = true;
                                        }
                                    },
                                    Err(e) => {
                                        error!("Failed to parse arguments for tool '{}': {}", tool_call.function.name, e);
                                        final_messages.push(SdkMessage {
                                            role: SdkMessageRole::Tool,
                                            content: Some(format!("Error parsing arguments: {e}")),
                                            tool_call_id: Some(tool_call.id.clone()),
                                            ..Default::default()
                                        });
                                        has_tool_results = true;
                                    }
                                }
                            } else {
                                error!("No handler found for tool: {}", tool_call.function.name);
                                final_messages.push(SdkMessage {
                                    role: SdkMessageRole::Tool,
                                    content: Some(format!("Error: No handler found for tool '{}'", tool_call.function.name)),
                                    tool_call_id: Some(tool_call.id.clone()),
                                    ..Default::default()
                                });
                                has_tool_results = true;
                            }
                        }

                        // Continue to next iteration if we have tool results to process
                        if has_tool_results {
                            continue;
                        }
                    }

                    // No tool calls or final response - complete the task
                    let final_message = create_a2a_message(
                        MessageRole::Agent,
                        &choice.message.content.unwrap_or_else(|| "Task completed".to_string())
                    );

                    task.update_status(TaskState::Completed, Some(final_message));
                    debug!("Task {} completed successfully", task.task.id);
                    return Ok(());
                }
                Err(e) => {
                    error!("Failed to generate content for task {}: {}", task.task.id, e);
                    return Err(anyhow!("Failed to generate content: {}", e));
                }
            }
        }

        // If we reach here, we've hit the max iterations
        warn!("Task {} reached maximum iterations ({})", task.task.id, max_iterations);
        let timeout_message = create_a2a_message(
            MessageRole::Agent,
            &format!("Task reached maximum iterations ({}) without completion", max_iterations)
        );
        task.update_status(TaskState::Completed, Some(timeout_message));
        Ok(())
    }

    fn name(&self) -> &'static str {
        "default_background"
    }
}

/// Background task queue manager
pub struct BackgroundTaskQueue {
    config: BackgroundTaskHandlerConfig,
    pending_tasks: Arc<Mutex<Vec<QueuedTask>>>,
    active_tasks: Arc<RwLock<HashMap<String, ManagedTask>>>,
    completed_tasks: Arc<RwLock<HashMap<String, ManagedTask>>>,
    dead_letter_queue: Arc<RwLock<HashMap<String, (ManagedTask, Instant)>>>,
    task_handler: Arc<dyn TaskHandler>,
    shutdown_signal: Arc<Mutex<bool>>,
}

impl BackgroundTaskQueue {
    pub fn new(
        config: BackgroundTaskHandlerConfig,
        task_handler: Arc<dyn TaskHandler>,
    ) -> Self {
        Self {
            config,
            pending_tasks: Arc::new(Mutex::new(Vec::new())),
            active_tasks: Arc::new(RwLock::new(HashMap::new())),
            completed_tasks: Arc::new(RwLock::new(HashMap::new())),
            dead_letter_queue: Arc::new(RwLock::new(HashMap::new())),
            task_handler,
            shutdown_signal: Arc::new(Mutex::new(false)),
        }
    }

    /// Submit a new task to the queue
    pub async fn submit_task(
        &self,
        messages: Vec<SdkMessage>,
        context_id: Option<String>,
        metadata: HashMap<String, serde_json::Value>,
    ) -> Result<String> {
        let queued_task = QueuedTask::new(messages, context_id, metadata);
        let task_id = queued_task.id.clone();

        let mut pending = self.pending_tasks.lock().await;
        if pending.len() >= self.config.max_queue_size {
            return Err(anyhow!("Task queue is full"));
        }

        pending.push(queued_task);
        debug!("Task {} submitted to queue", task_id);
        Ok(task_id)
    }

    /// Get task by ID from any queue
    pub async fn get_task(&self, task_id: &str) -> Option<ManagedTask> {
        // Check active tasks first
        if let Some(task) = self.active_tasks.read().await.get(task_id) {
            return Some(task.clone());
        }

        // Check completed tasks
        if let Some(task) = self.completed_tasks.read().await.get(task_id) {
            return Some(task.clone());
        }

        // Check dead letter queue
        if let Some((task, _)) = self.dead_letter_queue.read().await.get(task_id) {
            return Some(task.clone());
        }

        // Check pending tasks (convert to ManagedTask for consistency)
        let pending = self.pending_tasks.lock().await;
        if let Some(queued_task) = pending.iter().find(|t| t.id == task_id) {
            return Some(ManagedTask::from_queued_task(queued_task.clone()));
        }

        None
    }

    /// Start the background worker
    pub async fn start_worker(&self) -> Result<()> {
        info!("Starting background task worker with handler: {}", self.task_handler.name());

        let pending_tasks = self.pending_tasks.clone();
        let active_tasks = self.active_tasks.clone();
        let completed_tasks = self.completed_tasks.clone();
        let dead_letter_queue = self.dead_letter_queue.clone();
        let task_handler = self.task_handler.clone();
        let config = self.config.clone();
        let shutdown_signal = self.shutdown_signal.clone();

        // Main worker loop
        let worker_handle = tokio::spawn(async move {
            loop {
                // Check shutdown signal
                if *shutdown_signal.lock().await {
                    info!("Background worker received shutdown signal");
                    break;
                }

                // Process pending tasks
                let task_to_process = {
                    let mut pending = pending_tasks.lock().await;
                    pending.pop()
                };

                if let Some(queued_task) = task_to_process {
                    let task_id = queued_task.id.clone();
                    let mut managed_task = ManagedTask::from_queued_task(queued_task);

                    // Move to active tasks
                    {
                        let mut active = active_tasks.write().await;
                        active.insert(task_id.clone(), managed_task.clone());
                    }

                    debug!("Processing task {}", task_id);

                    // Process the task
                    let processing_result = tokio::time::timeout(
                        config.task_timeout,
                        task_handler.process_task(&mut managed_task),
                    )
                    .await;

                    match processing_result {
                        Ok(Ok(())) => {
                            debug!("Task {} processed successfully", task_id);
                            
                            // Move to completed tasks
                            {
                                let mut active = active_tasks.write().await;
                                active.remove(&task_id);
                            }
                            {
                                let mut completed = completed_tasks.write().await;
                                completed.insert(task_id, managed_task);
                            }
                        }
                        Ok(Err(e)) => {
                            error!("Task {} failed: {}", task_id, e);
                            
                            // Handle retry or move to dead letter queue
                            if managed_task.should_retry(&config) {
                                managed_task.queued_task.increment_retry();
                                
                                // Move back to pending for retry
                                {
                                    let mut active = active_tasks.write().await;
                                    active.remove(&task_id);
                                }
                                {
                                    let mut pending = pending_tasks.lock().await;
                                    pending.push(managed_task.queued_task);
                                }
                                
                                debug!("Task {} queued for retry {}/{}", task_id, managed_task.queued_task.retries, config.max_retries);
                            } else {
                                // Move to dead letter queue
                                let failure_message = create_a2a_message(
                                    MessageRole::Agent,
                                    &format!("Task failed after {} retries: {}", config.max_retries, e)
                                );
                                managed_task.update_status(TaskState::Canceled, Some(failure_message));
                                
                                {
                                    let mut active = active_tasks.write().await;
                                    active.remove(&task_id);
                                }
                                {
                                    let mut dead_letter = dead_letter_queue.write().await;
                                    dead_letter.insert(task_id.clone(), (managed_task, Instant::now()));
                                }
                                
                                warn!("Task {} moved to dead letter queue after {} failures", task_id, config.max_retries);
                            }
                        }
                        Err(_timeout) => {
                            error!("Task {} timed out after {:?}", task_id, config.task_timeout);
                            
                            // Handle timeout same as failure
                            if managed_task.should_retry(&config) {
                                managed_task.queued_task.increment_retry();
                                
                                {
                                    let mut active = active_tasks.write().await;
                                    active.remove(&task_id);
                                }
                                {
                                    let mut pending = pending_tasks.lock().await;
                                    pending.push(managed_task.queued_task);
                                }
                            } else {
                                let timeout_message = create_a2a_message(
                                    MessageRole::Agent,
                                    &format!("Task timed out after {:?}", config.task_timeout)
                                );
                                managed_task.update_status(TaskState::Canceled, Some(timeout_message));
                                
                                {
                                    let mut active = active_tasks.write().await;
                                    active.remove(&task_id);
                                }
                                {
                                    let mut dead_letter = dead_letter_queue.write().await;
                                    dead_letter.insert(task_id.clone(), (managed_task, Instant::now()));
                                }
                            }
                        }
                    }
                }

                // Sleep between polling
                tokio::time::sleep(config.poll_interval).await;
            }
        });

        // Cleanup task
        let cleanup_handle = {
            let completed_tasks = self.completed_tasks.clone();
            let dead_letter_queue = self.dead_letter_queue.clone();
            let config = self.config.clone();
            let shutdown_signal = self.shutdown_signal.clone();

            tokio::spawn(async move {
                loop {
                    if *shutdown_signal.lock().await {
                        break;
                    }

                    let now = Instant::now();

                    // Clean up old completed tasks
                    {
                        let mut completed = completed_tasks.write().await;
                        let initial_count = completed.len();
                        completed.retain(|_id, task| {
                            now.duration_since(task.updated_at) < config.cleanup_interval
                        });
                        let removed = initial_count - completed.len();
                        if removed > 0 {
                            debug!("Cleaned up {} completed tasks", removed);
                        }
                    }

                    // Clean up old dead letter tasks
                    {
                        let mut dead_letter = dead_letter_queue.write().await;
                        let initial_count = dead_letter.len();
                        dead_letter.retain(|_id, (_task, timestamp)| {
                            now.duration_since(*timestamp) < config.dead_letter_ttl
                        });
                        let removed = initial_count - dead_letter.len();
                        if removed > 0 {
                            debug!("Cleaned up {} dead letter tasks", removed);
                        }
                    }

                    tokio::time::sleep(config.cleanup_interval).await;
                }
            })
        };

        // Store handles for graceful shutdown (in a real implementation)
        tokio::try_join!(worker_handle, cleanup_handle)?;
        Ok(())
    }

    /// Shutdown the background worker
    pub async fn shutdown(&self) {
        info!("Shutting down background task queue");
        *self.shutdown_signal.lock().await = true;
    }

    /// Get queue statistics
    pub async fn get_stats(&self) -> TaskQueueStats {
        let pending_count = self.pending_tasks.lock().await.len();
        let active_count = self.active_tasks.read().await.len();
        let completed_count = self.completed_tasks.read().await.len();
        let dead_letter_count = self.dead_letter_queue.read().await.len();

        TaskQueueStats {
            pending: pending_count,
            active: active_count,
            completed: completed_count,
            dead_letter: dead_letter_count,
            total: pending_count + active_count + completed_count + dead_letter_count,
        }
    }
}

/// Queue statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskQueueStats {
    pub pending: usize,
    pub active: usize,
    pub completed: usize,
    pub dead_letter: usize,
    pub total: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockTaskHandler;

    #[async_trait::async_trait]
    impl TaskHandler for MockTaskHandler {
        async fn process_task(&self, task: &mut ManagedTask) -> Result<()> {
            task.update_status(TaskState::Working, None);
            tokio::time::sleep(Duration::from_millis(10)).await;
            let response_message = create_a2a_message(MessageRole::Agent, "Mock response");
            task.update_status(TaskState::Completed, Some(response_message));
            Ok(())
        }

        fn name(&self) -> &'static str {
            "mock"
        }
    }

    #[tokio::test]
    async fn test_queued_task_creation() {
        let messages = vec![SdkMessage {
            role: SdkMessageRole::User,
            content: Some("Test message".to_string()),
            ..Default::default()
        }];
        let metadata = HashMap::new();
        
        let task = QueuedTask::new(messages.clone(), None, metadata);
        
        assert!(!task.id.is_empty());
        assert!(!task.context_id.is_empty());
        assert_eq!(task.messages.len(), 1);
        assert_eq!(task.retries, 0);
        assert!(task.last_retry_at.is_none());
    }

    #[tokio::test]
    async fn test_managed_task_from_queued_task() {
        let queued_task = QueuedTask::new(
            vec![SdkMessage {
                role: SdkMessageRole::User,
                content: Some("Test".to_string()),
                ..Default::default()
            }],
            None,
            HashMap::new(),
        );
        
        let managed_task = ManagedTask::from_queued_task(queued_task.clone());
        
        assert_eq!(managed_task.task.id, queued_task.id);
        assert_eq!(managed_task.queued_task.id, queued_task.id);
        assert!(matches!(managed_task.status.state, TaskState::Submitted));
        assert!(!managed_task.is_terminal());
    }

    #[tokio::test]
    async fn test_task_queue_submit_and_get() {
        let config = BackgroundTaskHandlerConfig::default();
        let handler = Arc::new(MockTaskHandler);
        let queue = BackgroundTaskQueue::new(config, handler);

        let messages = vec![SdkMessage {
            role: SdkMessageRole::User,
            content: Some("Test message".to_string()),
            ..Default::default()
        }];

        let task_id = queue.submit_task(messages, None, HashMap::new()).await.unwrap();
        let task = queue.get_task(&task_id).await.unwrap();

        assert_eq!(task.task.id, task_id);
        assert!(matches!(task.status.state, TaskState::Submitted));
    }

    #[tokio::test]
    async fn test_queue_stats() {
        let config = BackgroundTaskHandlerConfig::default();
        let handler = Arc::new(MockTaskHandler);
        let queue = BackgroundTaskQueue::new(config, handler);

        let messages = vec![SdkMessage {
            role: SdkMessageRole::User,
            content: Some("Test".to_string()),
            ..Default::default()
        }];

        queue.submit_task(messages, None, HashMap::new()).await.unwrap();
        
        let stats = queue.get_stats().await;
        assert_eq!(stats.pending, 1);
        assert_eq!(stats.total, 1);
    }
}
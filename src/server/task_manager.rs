//! Background task manager that drains the [`Storage`] queue and
//! dispatches each dequeued task to the configured [`TaskHandler`].
//!
//! Mirrors the Go ADK's `task_manager.go` + `server.go` worker loop:
//! one or more workers call [`Storage::dequeue_task`] (blocking), move
//! the task into the active store, drive the handler to a terminal
//! state, then route the result to the active store (intermediate
//! state) or the dead-letter store (terminal state) based on the
//! handler's returned `status.state`.
//!
//! Construction is decoupled from spawning so the server builder can
//! own the manager configuration and call [`DefaultTaskManager::start`]
//! at serve time:
//!
//! ```text
//!     let manager = DefaultTaskManager::new(storage, handler, workers);
//!     let runner = manager.start();
//!     // ... serve until SIGINT ...
//!     runner.shutdown().await;
//! ```

use super::storage::Storage;
use super::task_handler::TaskHandler;
use crate::a2a_types::{TaskState, TaskStatus, Timestamp};
use std::sync::Arc;
use std::time::Duration;
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;
use tracing::{debug, warn};

/// Drains the storage queue and dispatches each dequeued task to the
/// configured background [`TaskHandler`]. Construct via
/// [`DefaultTaskManager::new`], then call [`start`](Self::start) when
/// the server is ready to begin processing.
pub struct DefaultTaskManager {
    storage: Arc<dyn Storage>,
    handler: Arc<dyn TaskHandler>,
    worker_count: usize,
}

impl std::fmt::Debug for DefaultTaskManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DefaultTaskManager")
            .field("worker_count", &self.worker_count)
            .finish_non_exhaustive()
    }
}

impl DefaultTaskManager {
    pub fn new(
        storage: Arc<dyn Storage>,
        handler: Arc<dyn TaskHandler>,
        worker_count: usize,
    ) -> Self {
        let worker_count = worker_count.max(1);
        Self {
            storage,
            handler,
            worker_count,
        }
    }

    /// Spawn `worker_count` workers. Returns a [`TaskManagerRunner`]
    /// that holds the join set and cancellation token; call
    /// [`TaskManagerRunner::shutdown`] to drain workers gracefully on
    /// server shutdown.
    pub fn start(&self) -> TaskManagerRunner {
        let token = CancellationToken::new();
        let mut join_set: JoinSet<()> = JoinSet::new();
        for worker_id in 0..self.worker_count {
            let storage = Arc::clone(&self.storage);
            let handler = Arc::clone(&self.handler);
            let token = token.clone();
            join_set.spawn(async move {
                run_worker(worker_id, storage, handler, token).await;
            });
        }
        debug!("task manager started with {} worker(s)", self.worker_count);
        TaskManagerRunner {
            shutdown: token,
            join_set,
        }
    }
}

/// Handle returned by [`DefaultTaskManager::start`]. Drop to detach the
/// workers (they keep running) or call [`shutdown`](Self::shutdown) to
/// cooperatively cancel + await them.
#[derive(Debug)]
pub struct TaskManagerRunner {
    shutdown: CancellationToken,
    join_set: JoinSet<()>,
}

impl TaskManagerRunner {
    /// Cancel the workers' cancellation token and wait for every worker
    /// to exit its loop. Each worker stops at the next `select!` point
    /// (between dequeues; an in-flight handler call is allowed to finish).
    pub async fn shutdown(mut self) {
        self.shutdown.cancel();
        while self.join_set.join_next().await.is_some() {}
        debug!("task manager shutdown complete");
    }

    /// Trigger cancellation without waiting. Useful when the caller
    /// needs to interleave shutdown with other concurrent work.
    pub fn cancel(&self) {
        self.shutdown.cancel();
    }
}

async fn run_worker(
    worker_id: usize,
    storage: Arc<dyn Storage>,
    handler: Arc<dyn TaskHandler>,
    shutdown: CancellationToken,
) {
    debug!(worker_id, "task manager worker started");
    loop {
        let queued = tokio::select! {
            biased;
            _ = shutdown.cancelled() => {
                debug!(worker_id, "task manager worker exiting on cancellation");
                return;
            }
            res = storage.dequeue_task() => match res {
                Ok(q) => q,
                Err(e) => {
                    warn!(worker_id, error = %e, "dequeue_task failed; backing off");
                    tokio::select! {
                        _ = shutdown.cancelled() => return,
                        _ = tokio::time::sleep(Duration::from_secs(1)) => continue,
                    }
                }
            }
        };

        let task = queued.task;
        let task_id = task.id.clone();

        if let Err(e) = storage.create_active_task(&task).await {
            debug!(worker_id, task_id = %task_id, error = %e, "create_active_task: continuing");
        }

        let last_message = task.history.last().cloned();
        match handler.handle_task(task.clone(), last_message).await {
            Ok(result) => route_terminal_or_active(&storage, worker_id, result).await,
            Err(e) => {
                warn!(worker_id, task_id = %task_id, error = %e, "task handler failed");
                let mut failed = task;
                failed.status = TaskStatus {
                    message: failed.status.message.clone(),
                    state: TaskState::TaskStateFailed,
                    timestamp: Some(Timestamp(chrono::Utc::now())),
                };
                if let Err(store_err) = storage.store_dead_letter_task(&failed).await {
                    warn!(worker_id, task_id = %task_id, error = %store_err,
                        "store_dead_letter_task failed after handler error");
                }
            }
        }
    }
}

async fn route_terminal_or_active(
    storage: &Arc<dyn Storage>,
    worker_id: usize,
    result: crate::a2a_types::Task,
) {
    let terminal = matches!(
        result.status.state,
        TaskState::TaskStateCompleted
            | TaskState::TaskStateFailed
            | TaskState::TaskStateCancelled
            | TaskState::TaskStateRejected
    );
    if terminal {
        if let Err(e) = storage.store_dead_letter_task(&result).await {
            warn!(worker_id, task_id = %result.id, error = %e, "store_dead_letter_task failed");
        }
    } else if let Err(e) = storage.update_active_task(&result).await {
        warn!(worker_id, task_id = %result.id, error = %e, "update_active_task failed");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::a2a_types::{
        Message as A2AMessage, Part, Role, Task, TaskState, TaskStatus, Timestamp,
    };
    use crate::server::storage::InMemoryStorage;
    use crate::server::task_handler::TaskHandler;
    use anyhow::Result;
    use async_trait::async_trait;
    use std::sync::Mutex;

    fn make_task(id: &str) -> Task {
        Task {
            artifacts: vec![],
            context_id: "ctx".to_string(),
            history: vec![A2AMessage {
                context_id: Some("ctx".to_string()),
                extensions: vec![],
                message_id: format!("msg-{id}"),
                metadata: None,
                parts: vec![Part {
                    data: None,
                    file: None,
                    metadata: None,
                    text: Some("hello".to_string()),
                }],
                reference_task_ids: vec![],
                role: Role::RoleUser,
                task_id: Some(id.to_string()),
            }],
            id: id.to_string(),
            metadata: None,
            status: TaskStatus {
                message: None,
                state: TaskState::TaskStateSubmitted,
                timestamp: Some(Timestamp(chrono::Utc::now())),
            },
        }
    }

    /// Records every task it sees, then returns it with a configurable
    /// terminal state so we can exercise the active/dead-letter routing.
    #[derive(Debug)]
    struct RecordingHandler {
        seen: Arc<Mutex<Vec<String>>>,
        terminal_state: TaskState,
    }

    #[async_trait]
    impl TaskHandler for RecordingHandler {
        async fn handle_task(&self, mut task: Task, _message: Option<A2AMessage>) -> Result<Task> {
            self.seen
                .lock()
                .expect("mutex poisoned")
                .push(task.id.clone());
            task.status = TaskStatus {
                message: None,
                state: self.terminal_state,
                timestamp: Some(Timestamp(chrono::Utc::now())),
            };
            Ok(task)
        }
    }

    /// Always errors, so we can verify failures route to dead-letter
    /// with state == Failed.
    #[derive(Debug)]
    struct FailingHandler;

    #[async_trait]
    impl TaskHandler for FailingHandler {
        async fn handle_task(&self, _task: Task, _message: Option<A2AMessage>) -> Result<Task> {
            Err(anyhow::anyhow!("handler always fails"))
        }
    }

    async fn wait_for_terminal(storage: &Arc<InMemoryStorage>, task_id: &str) -> Task {
        for _ in 0..50 {
            if let Some(task) = storage.get_task(task_id).await
                && matches!(
                    task.status.state,
                    TaskState::TaskStateCompleted
                        | TaskState::TaskStateFailed
                        | TaskState::TaskStateCancelled
                        | TaskState::TaskStateRejected
                )
            {
                return task;
            }
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
        panic!("task {task_id} never reached terminal state");
    }

    #[tokio::test]
    async fn worker_dequeues_and_routes_completed_to_dead_letter() {
        let storage: Arc<InMemoryStorage> = Arc::new(InMemoryStorage::new());
        let seen = Arc::new(Mutex::new(Vec::new()));
        let handler = Arc::new(RecordingHandler {
            seen: Arc::clone(&seen),
            terminal_state: TaskState::TaskStateCompleted,
        });

        let manager = DefaultTaskManager::new(
            storage.clone() as Arc<dyn Storage>,
            handler as Arc<dyn TaskHandler>,
            1,
        );
        let runner = manager.start();

        storage
            .enqueue_task(make_task("t1"), serde_json::Value::Null)
            .await
            .expect("enqueue");

        let terminal = wait_for_terminal(&storage, "t1").await;
        assert_eq!(terminal.status.state, TaskState::TaskStateCompleted);
        assert!(
            storage.get_active_task("t1").await.expect("ok").is_none(),
            "completed tasks must be evicted from active store",
        );
        let stats = storage.get_stats().await;
        assert_eq!(stats.dead_letter_tasks, 1);
        assert_eq!(stats.active_tasks, 0);
        assert_eq!(seen.lock().expect("mutex poisoned").as_slice(), &["t1"]);

        runner.shutdown().await;
    }

    #[tokio::test]
    async fn worker_routes_input_required_to_active_store() {
        let storage: Arc<InMemoryStorage> = Arc::new(InMemoryStorage::new());
        let handler = Arc::new(RecordingHandler {
            seen: Arc::new(Mutex::new(Vec::new())),
            terminal_state: TaskState::TaskStateInputRequired,
        });

        let manager = DefaultTaskManager::new(
            storage.clone() as Arc<dyn Storage>,
            handler as Arc<dyn TaskHandler>,
            1,
        );
        let runner = manager.start();

        storage
            .enqueue_task(make_task("t2"), serde_json::Value::Null)
            .await
            .expect("enqueue");

        for _ in 0..50 {
            let active = storage.get_active_task("t2").await.expect("ok");
            if matches!(
                active.as_ref().map(|t| t.status.state),
                Some(TaskState::TaskStateInputRequired)
            ) {
                break;
            }
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
        let active = storage
            .get_active_task("t2")
            .await
            .expect("ok")
            .expect("task should remain in active store");
        assert_eq!(active.status.state, TaskState::TaskStateInputRequired);
        assert_eq!(storage.get_stats().await.dead_letter_tasks, 0);

        runner.shutdown().await;
    }

    #[tokio::test]
    async fn handler_failure_routes_to_dead_letter_as_failed() {
        let storage: Arc<InMemoryStorage> = Arc::new(InMemoryStorage::new());
        let manager = DefaultTaskManager::new(
            storage.clone() as Arc<dyn Storage>,
            Arc::new(FailingHandler) as Arc<dyn TaskHandler>,
            1,
        );
        let runner = manager.start();

        storage
            .enqueue_task(make_task("t3"), serde_json::Value::Null)
            .await
            .expect("enqueue");

        let terminal = wait_for_terminal(&storage, "t3").await;
        assert_eq!(terminal.status.state, TaskState::TaskStateFailed);

        runner.shutdown().await;
    }

    #[tokio::test]
    async fn shutdown_exits_workers_even_with_empty_queue() {
        let storage: Arc<InMemoryStorage> = Arc::new(InMemoryStorage::new());
        let handler = Arc::new(RecordingHandler {
            seen: Arc::new(Mutex::new(Vec::new())),
            terminal_state: TaskState::TaskStateCompleted,
        });
        let manager = DefaultTaskManager::new(
            storage.clone() as Arc<dyn Storage>,
            handler as Arc<dyn TaskHandler>,
            2,
        );
        let runner = manager.start();

        runner.shutdown().await;
        assert_eq!(storage.get_stats().await.dead_letter_tasks, 0);
    }
}

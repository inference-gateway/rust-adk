//! Storage abstraction backing the A2A task manager.
//!
//! [`Storage`] is the trait the A2A server holds as `Arc<dyn Storage>` to
//! persist tasks, contexts, and push-notification configurations and to
//! drive the background-task queue. [`InMemoryStorage`] is the bundled
//! default — a `Mutex`+`Notify`-backed structure suitable for tests,
//! single-instance deployments, and bootstrap. Implement [`Storage`]
//! yourself to plug in Redis, Postgres, or any other backend without
//! forking the crate.
//!
//! The trait surface mirrors the Go ADK's `Storage` interface
//! (`adk/server/storage.go`): a queue (enqueue/dequeue/length/clear),
//! an active-task store (create/get/update), a dead-letter store
//! (store/list), context bookkeeping, cleanup helpers, and stats.

use crate::a2a_types::{Task, TaskPushNotificationConfig, TaskState};
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde_json::Value;
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Mutex;
use tokio::sync::Notify;

/// A task pulled off the queue, plus the JSON-RPC `request_id` that
/// originally enqueued it. The `request_id` is preserved for
/// correlation/tracing — it is not consumed by the worker today, mirroring
/// the Go ADK's behavior.
#[derive(Debug, Clone)]
pub struct QueuedTask {
    pub task: Task,
    pub request_id: Value,
    pub enqueued_at: DateTime<Utc>,
}

/// Filter / pagination applied to `list_tasks` / `list_tasks_by_context`.
/// `state == None` and `limit == None` means "no filtering / no cap".
#[derive(Debug, Clone, Default)]
pub struct TaskFilter {
    pub state: Option<TaskState>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

/// Counters returned by [`Storage::get_stats`]. Used by health endpoints
/// and operational dashboards.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StorageStats {
    pub queue_length: usize,
    pub active_tasks: usize,
    pub dead_letter_tasks: usize,
    pub contexts: usize,
}

/// Pluggable storage for the A2A task manager. Mirrors the Go ADK's
/// `Storage` interface so a Redis/Postgres/etc. backend can be swapped
/// in via [`A2AServerBuilder::with_storage`](super::server_builder::A2AServerBuilder::with_storage).
///
/// All methods are async to accommodate backends that need to issue
/// I/O. Implementations use interior mutability (mutex, connection
/// pool, etc.) so signatures stay `Arc<dyn Storage>`-friendly.
#[async_trait]
pub trait Storage: Send + Sync + std::fmt::Debug {
    // ----- Queue -----------------------------------------------------

    /// Push a task onto the back of the work queue.
    async fn enqueue_task(&self, task: Task, request_id: Value) -> Result<()>;

    /// Pop the next task off the front of the queue, **blocking** until
    /// one is available (Redis: `BRPOP`).
    async fn dequeue_task(&self) -> Result<QueuedTask>;

    async fn queue_length(&self) -> usize;

    async fn clear_queue(&self) -> Result<()>;

    // ----- Active-task store ----------------------------------------

    async fn create_active_task(&self, task: &Task) -> Result<()>;

    async fn get_active_task(&self, task_id: &str) -> Result<Option<Task>>;

    async fn update_active_task(&self, task: &Task) -> Result<()>;

    // ----- Dead-letter + general task read --------------------------

    /// Move a task to the dead-letter store (terminal-state archive).
    /// Implementations also remove the task from the active store if
    /// it is present there.
    async fn store_dead_letter_task(&self, task: &Task) -> Result<()>;

    /// Look up a task in any store (active first, then dead-letter).
    async fn get_task(&self, task_id: &str) -> Option<Task>;

    /// Upsert a task into the active store, replacing any existing entry
    /// with the same id. Convenience method retained for callers that
    /// don't want to distinguish create-vs-update.
    async fn put_task(&self, task: Task);

    async fn get_task_by_context_and_id(&self, context_id: &str, task_id: &str) -> Option<Task>;

    /// Remove a task from both active and dead-letter stores.
    async fn delete_task(&self, task_id: &str) -> Result<()>;

    /// List tasks across active + dead-letter, applying `filter`.
    async fn list_tasks(&self, filter: TaskFilter) -> Vec<Task>;

    async fn list_tasks_by_context(&self, context_id: &str, filter: TaskFilter) -> Vec<Task>;

    // ----- Contexts -------------------------------------------------

    async fn get_contexts(&self) -> Vec<String>;

    async fn get_contexts_with_tasks(&self) -> Vec<String>;

    async fn delete_context(&self, context_id: &str) -> Result<()>;

    async fn delete_context_and_tasks(&self, context_id: &str) -> Result<()>;

    // ----- Cleanup / stats ------------------------------------------

    /// Remove every task in dead-letter whose state is `Completed`.
    /// Returns the number of tasks deleted.
    async fn cleanup_completed_tasks(&self) -> usize;

    /// Trim the dead-letter store to at most `max_completed` `Completed`
    /// tasks and `max_failed` `Failed` tasks (oldest first). Returns the
    /// total number of tasks deleted.
    async fn cleanup_tasks_with_retention(&self, max_completed: usize, max_failed: usize) -> usize;

    async fn get_stats(&self) -> StorageStats;

    // ----- Push-notification configs (Rust-specific) ----------------

    async fn put_push_notification_config(&self, config: TaskPushNotificationConfig);

    async fn get_push_notification_config(&self, name: &str) -> Option<TaskPushNotificationConfig>;

    async fn list_push_notification_configs(&self, parent: &str)
    -> Vec<TaskPushNotificationConfig>;

    async fn delete_push_notification_config(&self, name: &str) -> bool;
}

/// Simple in-memory [`Storage`] implementation. Suitable for tests,
/// single-instance deployments, and as a baseline reference. Holds a
/// `std::sync::Mutex` plus a `tokio::sync::Notify` to park the dequeue
/// loop until an enqueue notifies it.
#[derive(Debug, Default)]
pub struct InMemoryStorage {
    inner: Mutex<StorageInner>,
    queue_notify: Notify,
}

#[derive(Debug, Default)]
struct StorageInner {
    queue: VecDeque<QueuedTask>,
    active_tasks: HashMap<String, Task>,
    dead_letter_tasks: HashMap<String, Task>,
    /// Set of `context_id` values seen via `enqueue_task` /
    /// `create_active_task` / `store_dead_letter_task`. Used by
    /// `get_contexts`.
    contexts: HashSet<String>,
    push_notification_configs: HashMap<String, TaskPushNotificationConfig>,
}

impl InMemoryStorage {
    pub fn new() -> Self {
        Self::default()
    }
}

fn apply_filter(mut tasks: Vec<Task>, filter: TaskFilter) -> Vec<Task> {
    if let Some(state) = filter.state {
        tasks.retain(|t| t.status.state == state);
    }
    if let Some(offset) = filter.offset {
        if offset >= tasks.len() {
            return Vec::new();
        }
        tasks.drain(..offset);
    }
    if let Some(limit) = filter.limit
        && tasks.len() > limit
    {
        tasks.truncate(limit);
    }
    tasks
}

#[async_trait]
impl Storage for InMemoryStorage {
    // ----- Queue -----------------------------------------------------

    async fn enqueue_task(&self, task: Task, request_id: Value) -> Result<()> {
        {
            let mut inner = self.inner.lock().expect("storage mutex poisoned");
            inner.contexts.insert(task.context_id.clone());
            inner.queue.push_back(QueuedTask {
                task,
                request_id,
                enqueued_at: Utc::now(),
            });
        }
        self.queue_notify.notify_one();
        Ok(())
    }

    async fn dequeue_task(&self) -> Result<QueuedTask> {
        loop {
            // Take the notified() future BEFORE checking the queue so we
            // don't miss a notify_one() that lands between our check and
            // our await (tokio::sync::Notify documents this pattern).
            let notified = self.queue_notify.notified();
            {
                let mut inner = self.inner.lock().expect("storage mutex poisoned");
                if let Some(queued) = inner.queue.pop_front() {
                    return Ok(queued);
                }
            }
            notified.await;
        }
    }

    async fn queue_length(&self) -> usize {
        let inner = self.inner.lock().expect("storage mutex poisoned");
        inner.queue.len()
    }

    async fn clear_queue(&self) -> Result<()> {
        let mut inner = self.inner.lock().expect("storage mutex poisoned");
        inner.queue.clear();
        Ok(())
    }

    // ----- Active-task store ----------------------------------------

    async fn create_active_task(&self, task: &Task) -> Result<()> {
        let mut inner = self.inner.lock().expect("storage mutex poisoned");
        if inner.active_tasks.contains_key(&task.id) {
            return Err(anyhow!("active task {:?} already exists", task.id));
        }
        inner.contexts.insert(task.context_id.clone());
        inner.active_tasks.insert(task.id.clone(), task.clone());
        Ok(())
    }

    async fn get_active_task(&self, task_id: &str) -> Result<Option<Task>> {
        let inner = self.inner.lock().expect("storage mutex poisoned");
        Ok(inner.active_tasks.get(task_id).cloned())
    }

    async fn update_active_task(&self, task: &Task) -> Result<()> {
        let mut inner = self.inner.lock().expect("storage mutex poisoned");
        if !inner.active_tasks.contains_key(&task.id) {
            return Err(anyhow!(
                "cannot update active task {:?}: not found",
                task.id
            ));
        }
        inner.active_tasks.insert(task.id.clone(), task.clone());
        Ok(())
    }

    // ----- Dead-letter + general task read --------------------------

    async fn store_dead_letter_task(&self, task: &Task) -> Result<()> {
        let mut inner = self.inner.lock().expect("storage mutex poisoned");
        inner.contexts.insert(task.context_id.clone());
        inner.active_tasks.remove(&task.id);
        inner
            .dead_letter_tasks
            .insert(task.id.clone(), task.clone());
        Ok(())
    }

    async fn get_task(&self, task_id: &str) -> Option<Task> {
        let inner = self.inner.lock().expect("storage mutex poisoned");
        inner
            .active_tasks
            .get(task_id)
            .or_else(|| inner.dead_letter_tasks.get(task_id))
            .cloned()
    }

    async fn put_task(&self, task: Task) {
        let mut inner = self.inner.lock().expect("storage mutex poisoned");
        inner.contexts.insert(task.context_id.clone());
        inner.active_tasks.insert(task.id.clone(), task);
    }

    async fn get_task_by_context_and_id(&self, context_id: &str, task_id: &str) -> Option<Task> {
        let inner = self.inner.lock().expect("storage mutex poisoned");
        inner
            .active_tasks
            .get(task_id)
            .or_else(|| inner.dead_letter_tasks.get(task_id))
            .filter(|t| t.context_id == context_id)
            .cloned()
    }

    async fn delete_task(&self, task_id: &str) -> Result<()> {
        let mut inner = self.inner.lock().expect("storage mutex poisoned");
        let active_removed = inner.active_tasks.remove(task_id).is_some();
        let dead_removed = inner.dead_letter_tasks.remove(task_id).is_some();
        if !active_removed && !dead_removed {
            return Err(anyhow!("task {task_id:?} not found in any store"));
        }
        Ok(())
    }

    async fn list_tasks(&self, filter: TaskFilter) -> Vec<Task> {
        let inner = self.inner.lock().expect("storage mutex poisoned");
        let tasks: Vec<Task> = inner
            .active_tasks
            .values()
            .chain(inner.dead_letter_tasks.values())
            .cloned()
            .collect();
        drop(inner);
        apply_filter(tasks, filter)
    }

    async fn list_tasks_by_context(&self, context_id: &str, filter: TaskFilter) -> Vec<Task> {
        let inner = self.inner.lock().expect("storage mutex poisoned");
        let tasks: Vec<Task> = inner
            .active_tasks
            .values()
            .chain(inner.dead_letter_tasks.values())
            .filter(|t| t.context_id == context_id)
            .cloned()
            .collect();
        drop(inner);
        apply_filter(tasks, filter)
    }

    // ----- Contexts -------------------------------------------------

    async fn get_contexts(&self) -> Vec<String> {
        let inner = self.inner.lock().expect("storage mutex poisoned");
        inner.contexts.iter().cloned().collect()
    }

    async fn get_contexts_with_tasks(&self) -> Vec<String> {
        let inner = self.inner.lock().expect("storage mutex poisoned");
        let mut out: HashSet<String> = HashSet::new();
        for t in inner.active_tasks.values() {
            out.insert(t.context_id.clone());
        }
        for t in inner.dead_letter_tasks.values() {
            out.insert(t.context_id.clone());
        }
        out.into_iter().collect()
    }

    async fn delete_context(&self, context_id: &str) -> Result<()> {
        let mut inner = self.inner.lock().expect("storage mutex poisoned");
        inner.contexts.remove(context_id);
        Ok(())
    }

    async fn delete_context_and_tasks(&self, context_id: &str) -> Result<()> {
        let mut inner = self.inner.lock().expect("storage mutex poisoned");
        inner.active_tasks.retain(|_, t| t.context_id != context_id);
        inner
            .dead_letter_tasks
            .retain(|_, t| t.context_id != context_id);
        inner.contexts.remove(context_id);
        Ok(())
    }

    // ----- Cleanup / stats ------------------------------------------

    async fn cleanup_completed_tasks(&self) -> usize {
        let mut inner = self.inner.lock().expect("storage mutex poisoned");
        let before = inner.dead_letter_tasks.len();
        inner
            .dead_letter_tasks
            .retain(|_, t| t.status.state != TaskState::TaskStateCompleted);
        before - inner.dead_letter_tasks.len()
    }

    async fn cleanup_tasks_with_retention(&self, max_completed: usize, max_failed: usize) -> usize {
        let mut inner = self.inner.lock().expect("storage mutex poisoned");

        // Tasks in the dead-letter store don't track an eviction timestamp
        // separate from their last status timestamp, so we sort by status
        // timestamp (oldest first) and drop the leading overage.
        fn evict(store: &mut HashMap<String, Task>, state: TaskState, keep: usize) -> usize {
            let mut matching: Vec<(String, Option<DateTime<Utc>>)> = store
                .iter()
                .filter(|(_, t)| t.status.state == state)
                .map(|(k, t)| (k.clone(), t.status.timestamp.as_ref().map(|ts| ts.0)))
                .collect();
            if matching.len() <= keep {
                return 0;
            }
            matching.sort_by_key(|(_, ts)| *ts);
            let evict_count = matching.len() - keep;
            for (id, _) in matching.into_iter().take(evict_count) {
                store.remove(&id);
            }
            evict_count
        }

        let completed_removed = evict(
            &mut inner.dead_letter_tasks,
            TaskState::TaskStateCompleted,
            max_completed,
        );
        let failed_removed = evict(
            &mut inner.dead_letter_tasks,
            TaskState::TaskStateFailed,
            max_failed,
        );
        completed_removed + failed_removed
    }

    async fn get_stats(&self) -> StorageStats {
        let inner = self.inner.lock().expect("storage mutex poisoned");
        StorageStats {
            queue_length: inner.queue.len(),
            active_tasks: inner.active_tasks.len(),
            dead_letter_tasks: inner.dead_letter_tasks.len(),
            contexts: inner.contexts.len(),
        }
    }

    // ----- Push-notification configs -------------------------------

    async fn put_push_notification_config(&self, config: TaskPushNotificationConfig) {
        let mut inner = self.inner.lock().expect("storage mutex poisoned");
        inner
            .push_notification_configs
            .insert(config.name.clone(), config);
    }

    async fn get_push_notification_config(&self, name: &str) -> Option<TaskPushNotificationConfig> {
        let inner = self.inner.lock().expect("storage mutex poisoned");
        inner.push_notification_configs.get(name).cloned()
    }

    async fn list_push_notification_configs(
        &self,
        parent: &str,
    ) -> Vec<TaskPushNotificationConfig> {
        let prefix = format!("{parent}/pushNotificationConfigs/");
        let inner = self.inner.lock().expect("storage mutex poisoned");
        inner
            .push_notification_configs
            .values()
            .filter(|c| c.name.starts_with(&prefix))
            .cloned()
            .collect()
    }

    async fn delete_push_notification_config(&self, name: &str) -> bool {
        let mut inner = self.inner.lock().expect("storage mutex poisoned");
        inner.push_notification_configs.remove(name).is_some()
    }
}

/// Extract the bare task id from a resource name of the form `tasks/{task_id}`.
/// Returns `None` if `name` does not start with the `tasks/` prefix.
pub fn parse_task_name(name: &str) -> Option<&str> {
    name.strip_prefix("tasks/")
        .filter(|rest| !rest.is_empty() && !rest.contains('/'))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::a2a_types::{
        PushNotificationConfig, TaskPushNotificationConfig, TaskState, TaskStatus, Timestamp,
    };

    fn make_task(id: &str) -> Task {
        make_task_in_context(id, "ctx")
    }

    fn make_task_in_context(id: &str, context_id: &str) -> Task {
        Task {
            artifacts: vec![],
            context_id: context_id.to_string(),
            history: vec![],
            id: id.to_string(),
            metadata: None,
            status: TaskStatus {
                message: None,
                state: TaskState::TaskStateSubmitted,
                timestamp: Some(Timestamp(Utc::now())),
            },
        }
    }

    fn make_config(name: &str, url: &str) -> TaskPushNotificationConfig {
        TaskPushNotificationConfig {
            name: name.to_string(),
            push_notification_config: PushNotificationConfig {
                authentication: None,
                id: None,
                token: None,
                url: url.to_string(),
            },
        }
    }

    // ----- Queue ----------------------------------------------------

    #[tokio::test]
    async fn queue_enqueue_dequeue_round_trip() {
        let storage = InMemoryStorage::new();
        let task = make_task("t1");
        storage
            .enqueue_task(task.clone(), Value::String("req-1".to_string()))
            .await
            .expect("enqueue");
        assert_eq!(storage.queue_length().await, 1);

        let dequeued = storage.dequeue_task().await.expect("dequeue");
        assert_eq!(dequeued.task.id, "t1");
        assert_eq!(dequeued.request_id, Value::String("req-1".to_string()));
        assert_eq!(storage.queue_length().await, 0);
    }

    #[tokio::test]
    async fn dequeue_parks_until_enqueue() {
        let storage = std::sync::Arc::new(InMemoryStorage::new());
        let storage_consumer = storage.clone();
        let consumer = tokio::spawn(async move { storage_consumer.dequeue_task().await });
        // Give the consumer a moment to park on the notify
        tokio::task::yield_now().await;
        storage
            .enqueue_task(make_task("t2"), Value::Null)
            .await
            .expect("enqueue");
        let result = consumer.await.expect("join").expect("dequeue");
        assert_eq!(result.task.id, "t2");
    }

    #[tokio::test]
    async fn clear_queue_drops_pending_tasks() {
        let storage = InMemoryStorage::new();
        for n in 0..3 {
            storage
                .enqueue_task(make_task(&format!("t{n}")), Value::Null)
                .await
                .expect("enqueue");
        }
        assert_eq!(storage.queue_length().await, 3);
        storage.clear_queue().await.expect("clear");
        assert_eq!(storage.queue_length().await, 0);
    }

    // ----- Active + dead-letter -------------------------------------

    #[tokio::test]
    async fn create_active_task_rejects_duplicates() {
        let storage = InMemoryStorage::new();
        storage
            .create_active_task(&make_task("t1"))
            .await
            .expect("first create");
        let err = storage
            .create_active_task(&make_task("t1"))
            .await
            .expect_err("duplicate must fail");
        assert!(err.to_string().contains("already exists"));
    }

    #[tokio::test]
    async fn update_active_task_requires_existing_entry() {
        let storage = InMemoryStorage::new();
        let err = storage
            .update_active_task(&make_task("missing"))
            .await
            .expect_err("update of missing task must fail");
        assert!(err.to_string().contains("not found"));
    }

    #[tokio::test]
    async fn store_dead_letter_moves_task_out_of_active() {
        let storage = InMemoryStorage::new();
        let mut task = make_task("t1");
        storage.create_active_task(&task).await.expect("create");
        task.status.state = TaskState::TaskStateCompleted;
        storage
            .store_dead_letter_task(&task)
            .await
            .expect("dead-letter");

        assert!(
            storage.get_active_task("t1").await.expect("ok").is_none(),
            "task should be removed from active"
        );
        let fetched = storage.get_task("t1").await.expect("task in dead-letter");
        assert_eq!(fetched.status.state, TaskState::TaskStateCompleted);
    }

    #[tokio::test]
    async fn list_tasks_includes_active_and_dead_letter() {
        let storage = InMemoryStorage::new();
        storage
            .create_active_task(&make_task("active-1"))
            .await
            .expect("active create");
        let mut dead = make_task("dead-1");
        dead.status.state = TaskState::TaskStateFailed;
        storage
            .store_dead_letter_task(&dead)
            .await
            .expect("dead-letter store");

        let all = storage.list_tasks(TaskFilter::default()).await;
        let ids: Vec<String> = all.iter().map(|t| t.id.clone()).collect();
        assert_eq!(all.len(), 2, "expected 2 tasks across stores, got {ids:?}");
        assert!(ids.contains(&"active-1".to_string()));
        assert!(ids.contains(&"dead-1".to_string()));
    }

    #[tokio::test]
    async fn list_tasks_applies_state_filter() {
        let storage = InMemoryStorage::new();
        let mut active = make_task("a");
        active.status.state = TaskState::TaskStateWorking;
        storage.create_active_task(&active).await.expect("create");
        let mut dead = make_task("d");
        dead.status.state = TaskState::TaskStateFailed;
        storage
            .store_dead_letter_task(&dead)
            .await
            .expect("dead-letter store");

        let filter = TaskFilter {
            state: Some(TaskState::TaskStateFailed),
            ..Default::default()
        };
        let filtered = storage.list_tasks(filter).await;
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].id, "d");
    }

    #[tokio::test]
    async fn list_tasks_by_context_scopes_results() {
        let storage = InMemoryStorage::new();
        storage
            .create_active_task(&make_task_in_context("a", "ctx-1"))
            .await
            .expect("create a");
        storage
            .create_active_task(&make_task_in_context("b", "ctx-2"))
            .await
            .expect("create b");

        let scoped = storage
            .list_tasks_by_context("ctx-1", TaskFilter::default())
            .await;
        assert_eq!(scoped.len(), 1);
        assert_eq!(scoped[0].id, "a");
    }

    #[tokio::test]
    async fn delete_task_removes_from_both_stores() {
        let storage = InMemoryStorage::new();
        storage
            .create_active_task(&make_task("active"))
            .await
            .expect("create");
        let mut dead = make_task("dead");
        dead.status.state = TaskState::TaskStateFailed;
        storage
            .store_dead_letter_task(&dead)
            .await
            .expect("dead-letter store");

        storage.delete_task("active").await.expect("delete active");
        storage.delete_task("dead").await.expect("delete dead");
        assert!(storage.get_task("active").await.is_none());
        assert!(storage.get_task("dead").await.is_none());
        assert!(
            storage.delete_task("nonexistent").await.is_err(),
            "deleting unknown task must error"
        );
    }

    // ----- Contexts -------------------------------------------------

    #[tokio::test]
    async fn contexts_track_seen_tasks() {
        let storage = InMemoryStorage::new();
        storage
            .enqueue_task(make_task_in_context("t1", "ctx-q"), Value::Null)
            .await
            .expect("enqueue");
        storage
            .create_active_task(&make_task_in_context("t2", "ctx-a"))
            .await
            .expect("create");

        let mut contexts = storage.get_contexts().await;
        contexts.sort();
        assert_eq!(contexts, vec!["ctx-a".to_string(), "ctx-q".to_string()]);
    }

    #[tokio::test]
    async fn delete_context_and_tasks_clears_both_stores() {
        let storage = InMemoryStorage::new();
        storage
            .create_active_task(&make_task_in_context("a", "ctx-x"))
            .await
            .expect("create active");
        let mut dead = make_task_in_context("b", "ctx-x");
        dead.status.state = TaskState::TaskStateFailed;
        storage
            .store_dead_letter_task(&dead)
            .await
            .expect("dead-letter");
        storage
            .create_active_task(&make_task_in_context("survivor", "ctx-y"))
            .await
            .expect("create survivor");

        storage
            .delete_context_and_tasks("ctx-x")
            .await
            .expect("delete context");
        let remaining = storage.list_tasks(TaskFilter::default()).await;
        assert_eq!(remaining.len(), 1);
        assert_eq!(remaining[0].id, "survivor");
    }

    // ----- Cleanup / stats ------------------------------------------

    #[tokio::test]
    async fn cleanup_completed_tasks_only_drops_completed() {
        let storage = InMemoryStorage::new();
        let mut done = make_task("done");
        done.status.state = TaskState::TaskStateCompleted;
        let mut failed = make_task("failed");
        failed.status.state = TaskState::TaskStateFailed;
        storage
            .store_dead_letter_task(&done)
            .await
            .expect("dead-letter completed");
        storage
            .store_dead_letter_task(&failed)
            .await
            .expect("dead-letter failed");

        let removed = storage.cleanup_completed_tasks().await;
        assert_eq!(removed, 1);
        let remaining = storage.list_tasks(TaskFilter::default()).await;
        assert_eq!(remaining.len(), 1);
        assert_eq!(remaining[0].id, "failed");
    }

    #[tokio::test]
    async fn cleanup_with_retention_keeps_newest() {
        let storage = InMemoryStorage::new();
        // Three completed tasks, each with a slightly newer timestamp.
        for (i, id) in ["old", "mid", "new"].iter().enumerate() {
            let mut t = make_task(id);
            t.status.state = TaskState::TaskStateCompleted;
            // bias the timestamps so cleanup picks oldest first
            t.status.timestamp = Some(Timestamp(Utc::now() + chrono::Duration::seconds(i as i64)));
            storage
                .store_dead_letter_task(&t)
                .await
                .expect("dead-letter store");
        }
        let removed = storage.cleanup_tasks_with_retention(1, 0).await;
        assert_eq!(removed, 2);
        let remaining = storage.list_tasks(TaskFilter::default()).await;
        assert_eq!(remaining.len(), 1);
        assert_eq!(remaining[0].id, "new");
    }

    #[tokio::test]
    async fn stats_count_everything() {
        let storage = InMemoryStorage::new();
        storage
            .enqueue_task(make_task("queued"), Value::Null)
            .await
            .expect("enqueue");
        storage
            .create_active_task(&make_task("active"))
            .await
            .expect("create");
        let mut dead = make_task("dead");
        dead.status.state = TaskState::TaskStateFailed;
        storage
            .store_dead_letter_task(&dead)
            .await
            .expect("dead-letter store");

        let stats = storage.get_stats().await;
        assert_eq!(stats.queue_length, 1);
        assert_eq!(stats.active_tasks, 1);
        assert_eq!(stats.dead_letter_tasks, 1);
        assert_eq!(stats.contexts, 1, "all three share 'ctx'");
    }

    // ----- Existing surface (smoke tests) ---------------------------

    #[tokio::test]
    async fn get_task_falls_back_to_dead_letter() {
        let storage = InMemoryStorage::new();
        let mut task = make_task("t1");
        task.status.state = TaskState::TaskStateCompleted;
        storage
            .store_dead_letter_task(&task)
            .await
            .expect("dead-letter store");
        let got = storage.get_task("t1").await.expect("dead-letter read");
        assert_eq!(got.status.state, TaskState::TaskStateCompleted);
    }

    #[tokio::test]
    async fn push_notification_configs_filter_by_parent() {
        let storage = InMemoryStorage::new();
        storage
            .put_push_notification_config(make_config(
                "tasks/abc/pushNotificationConfigs/c1",
                "https://a.example/webhook",
            ))
            .await;
        storage
            .put_push_notification_config(make_config(
                "tasks/abc/pushNotificationConfigs/c2",
                "https://b.example/webhook",
            ))
            .await;
        storage
            .put_push_notification_config(make_config(
                "tasks/other/pushNotificationConfigs/c3",
                "https://c.example/webhook",
            ))
            .await;

        let configs = storage.list_push_notification_configs("tasks/abc").await;
        assert_eq!(configs.len(), 2);

        assert!(
            storage
                .delete_push_notification_config("tasks/abc/pushNotificationConfigs/c1")
                .await
        );
        assert_eq!(
            storage
                .list_push_notification_configs("tasks/abc")
                .await
                .len(),
            1
        );
        assert!(
            !storage
                .delete_push_notification_config("tasks/abc/pushNotificationConfigs/c1")
                .await
        );
    }

    #[test]
    fn parse_task_name_strips_prefix() {
        assert_eq!(parse_task_name("tasks/abc"), Some("abc"));
        assert_eq!(
            parse_task_name("tasks/abc/pushNotificationConfigs/c1"),
            None
        );
        assert_eq!(parse_task_name("tasks/"), None);
        assert_eq!(parse_task_name("notasks/abc"), None);
    }

    #[tokio::test]
    async fn dyn_storage_dispatches_through_trait() {
        let storage: std::sync::Arc<dyn Storage> = std::sync::Arc::new(InMemoryStorage::new());
        storage
            .create_active_task(&make_task("abc"))
            .await
            .expect("create");
        let got = storage
            .get_task("abc")
            .await
            .expect("task should be present");
        assert_eq!(got.id, "abc");
    }
}

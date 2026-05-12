//! Redis-backed implementation of the [`Storage`] trait. Mirrors the Go
//! ADK's `server/storage_redis.go` key layout so a single Redis instance
//! can be shared between Rust and Go ADK servers.
//!
//! Behind the `redis` cargo feature: `cargo build --features redis`.
//!
//! Key layout (all keys prefixed with the configured namespace,
//! default `a2a`):
//!
//! | Purpose                | Type     | Key                          |
//! |------------------------|----------|------------------------------|
//! | Queue                  | LIST     | `{ns}:queue`                 |
//! | Active task            | STRING   | `{ns}:active:{task_id}`      |
//! | Dead-letter task       | STRING   | `{ns}:deadletter:{task_id}`  |
//! | Context membership     | SET      | `{ns}:context:{context_id}`  |
//! | Known contexts         | SET      | `{ns}:contexts`              |
//! | Push-notification cfg  | STRING   | `{ns}:pushconfig:{name}`     |
//! | Push-config index      | SET      | `{ns}:pushconfigs`           |
//!
//! Queue semantics: `enqueue_task` does `LPUSH`; `dequeue_task` does
//! `BRPOP {key} 0` for indefinite blocking (Redis's native parking
//! primitive). The worker uses a dedicated connection per call so the
//! pool stays usable.

use super::storage::{QueuedTask, Storage, StorageStats, TaskFilter};
use crate::a2a_types::{Task, TaskPushNotificationConfig, TaskState};
use anyhow::{Context, Result, anyhow};
use async_trait::async_trait;
use redis::AsyncCommands;
use redis::aio::ConnectionManager;
use serde_json::Value;

/// Redis-backed [`Storage`]. Construct via [`RedisStorage::connect`].
pub struct RedisStorage {
    client: redis::Client,
    manager: ConnectionManager,
    namespace: String,
}

impl std::fmt::Debug for RedisStorage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RedisStorage")
            .field("namespace", &self.namespace)
            .finish_non_exhaustive()
    }
}

impl RedisStorage {
    /// Connect to Redis and PING to verify reachability. `url` is a
    /// standard `redis://...` URL; `namespace` is the key prefix
    /// (e.g. `"a2a"`).
    pub async fn connect(url: &str, namespace: &str) -> Result<Self> {
        let client = redis::Client::open(url)
            .with_context(|| format!("failed to parse Redis URL {url:?}"))?;
        let mut manager = ConnectionManager::new(client.clone())
            .await
            .with_context(|| format!("failed to connect to Redis at {url:?}"))?;
        let _: String = redis::cmd("PING")
            .query_async(&mut manager)
            .await
            .context("Redis PING failed")?;
        Ok(Self {
            client,
            manager,
            namespace: namespace.to_string(),
        })
    }

    fn queue_key(&self) -> String {
        format!("{}:queue", self.namespace)
    }
    fn active_key(&self, task_id: &str) -> String {
        format!("{}:active:{}", self.namespace, task_id)
    }
    fn active_index_key(&self) -> String {
        format!("{}:active_index", self.namespace)
    }
    fn dead_letter_key(&self, task_id: &str) -> String {
        format!("{}:deadletter:{}", self.namespace, task_id)
    }
    fn dead_letter_index_key(&self) -> String {
        format!("{}:deadletter_index", self.namespace)
    }
    fn context_members_key(&self, context_id: &str) -> String {
        format!("{}:context:{}", self.namespace, context_id)
    }
    fn contexts_set_key(&self) -> String {
        format!("{}:contexts", self.namespace)
    }
    fn push_config_key(&self, name: &str) -> String {
        format!("{}:pushconfig:{}", self.namespace, name)
    }
    fn push_config_index_key(&self) -> String {
        format!("{}:pushconfigs", self.namespace)
    }

    fn conn(&self) -> ConnectionManager {
        self.manager.clone()
    }

    async fn record_context(&self, context_id: &str, task_id: &str) -> Result<()> {
        let mut conn = self.conn();
        let _: () = conn
            .sadd(self.contexts_set_key(), context_id)
            .await
            .map_err(redis_err)?;
        let _: () = conn
            .sadd(self.context_members_key(context_id), task_id)
            .await
            .map_err(redis_err)?;
        Ok(())
    }

    async fn read_tasks(&self, keys: &[String]) -> Result<Vec<Task>> {
        if keys.is_empty() {
            return Ok(Vec::new());
        }
        let mut conn = self.conn();
        let values: Vec<Option<String>> = conn.mget(keys).await.map_err(redis_err)?;
        let mut tasks = Vec::with_capacity(values.len());
        for v in values.into_iter().flatten() {
            if let Ok(t) = serde_json::from_str::<Task>(&v) {
                tasks.push(t);
            }
        }
        Ok(tasks)
    }
}

fn redis_err(e: redis::RedisError) -> anyhow::Error {
    anyhow!("redis error: {e}")
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
impl Storage for RedisStorage {
    // ----- Queue -----------------------------------------------------

    async fn enqueue_task(&self, task: Task, request_id: Value) -> Result<()> {
        let queued = QueuedTask {
            task: task.clone(),
            request_id,
            enqueued_at: chrono::Utc::now(),
        };
        let payload = serde_json::to_string(&queued).context("serialize QueuedTask")?;
        let mut conn = self.conn();
        let _: () = conn
            .lpush(self.queue_key(), payload)
            .await
            .map_err(redis_err)?;
        self.record_context(&task.context_id, &task.id).await?;
        Ok(())
    }

    async fn dequeue_task(&self) -> Result<QueuedTask> {
        // BRPOP needs a dedicated connection because it parks for the
        // full blocking duration. Use a fresh `aio` connection so we
        // don't tie up the shared manager.
        let mut conn = self
            .client
            .get_multiplexed_async_connection()
            .await
            .map_err(redis_err)?;
        let popped: Option<(String, String)> = redis::cmd("BRPOP")
            .arg(self.queue_key())
            .arg(0.0_f64) // 0 = block indefinitely
            .query_async(&mut conn)
            .await
            .map_err(redis_err)?;
        let (_, payload) = popped.ok_or_else(|| anyhow!("BRPOP returned nil unexpectedly"))?;
        let queued: QueuedTask =
            serde_json::from_str(&payload).context("deserialize QueuedTask")?;
        Ok(queued)
    }

    async fn queue_length(&self) -> usize {
        let mut conn = self.conn();
        conn.llen::<_, usize>(self.queue_key()).await.unwrap_or(0)
    }

    async fn clear_queue(&self) -> Result<()> {
        let mut conn = self.conn();
        let _: () = conn.del(self.queue_key()).await.map_err(redis_err)?;
        Ok(())
    }

    // ----- Active-task store ----------------------------------------

    async fn create_active_task(&self, task: &Task) -> Result<()> {
        let mut conn = self.conn();
        let key = self.active_key(&task.id);
        let exists: bool = conn.exists(&key).await.map_err(redis_err)?;
        if exists {
            return Err(anyhow!("active task {:?} already exists", task.id));
        }
        let payload = serde_json::to_string(task).context("serialize Task")?;
        let _: () = conn.set(&key, payload).await.map_err(redis_err)?;
        let _: () = conn
            .sadd(self.active_index_key(), &task.id)
            .await
            .map_err(redis_err)?;
        self.record_context(&task.context_id, &task.id).await?;
        Ok(())
    }

    async fn get_active_task(&self, task_id: &str) -> Result<Option<Task>> {
        let mut conn = self.conn();
        let payload: Option<String> = conn
            .get(self.active_key(task_id))
            .await
            .map_err(redis_err)?;
        match payload {
            Some(s) => Ok(Some(serde_json::from_str(&s).context("deserialize Task")?)),
            None => Ok(None),
        }
    }

    async fn update_active_task(&self, task: &Task) -> Result<()> {
        let mut conn = self.conn();
        let key = self.active_key(&task.id);
        let exists: bool = conn.exists(&key).await.map_err(redis_err)?;
        if !exists {
            return Err(anyhow!(
                "cannot update active task {:?}: not found",
                task.id
            ));
        }
        let payload = serde_json::to_string(task).context("serialize Task")?;
        let _: () = conn.set(&key, payload).await.map_err(redis_err)?;
        Ok(())
    }

    async fn put_task(&self, task: Task) {
        // Convenience upsert into the active store: matches
        // `InMemoryStorage::put_task` semantics.
        let mut conn = self.conn();
        let Ok(payload) = serde_json::to_string(&task) else {
            return;
        };
        let _: redis::RedisResult<()> = conn.set(self.active_key(&task.id), payload).await;
        let _: redis::RedisResult<()> = conn.sadd(self.active_index_key(), &task.id).await;
        let _: redis::RedisResult<()> = conn.sadd(self.contexts_set_key(), &task.context_id).await;
        let _: redis::RedisResult<()> = conn
            .sadd(self.context_members_key(&task.context_id), &task.id)
            .await;
    }

    // ----- Dead-letter + general task read --------------------------

    async fn store_dead_letter_task(&self, task: &Task) -> Result<()> {
        let mut conn = self.conn();
        let payload = serde_json::to_string(task).context("serialize Task")?;
        let _: () = conn
            .set(self.dead_letter_key(&task.id), payload)
            .await
            .map_err(redis_err)?;
        let _: () = conn
            .sadd(self.dead_letter_index_key(), &task.id)
            .await
            .map_err(redis_err)?;
        // Evict from active store if present (matches Go).
        let _: () = conn
            .del(self.active_key(&task.id))
            .await
            .map_err(redis_err)?;
        let _: () = conn
            .srem(self.active_index_key(), &task.id)
            .await
            .map_err(redis_err)?;
        self.record_context(&task.context_id, &task.id).await?;
        Ok(())
    }

    async fn get_task(&self, task_id: &str) -> Option<Task> {
        let mut conn = self.conn();
        let active: Option<String> = conn.get(self.active_key(task_id)).await.ok()?;
        if let Some(s) = active {
            return serde_json::from_str(&s).ok();
        }
        let dead: Option<String> = conn.get(self.dead_letter_key(task_id)).await.ok()?;
        dead.and_then(|s| serde_json::from_str(&s).ok())
    }

    async fn get_task_by_context_and_id(&self, context_id: &str, task_id: &str) -> Option<Task> {
        let task = self.get_task(task_id).await?;
        if task.context_id == context_id {
            Some(task)
        } else {
            None
        }
    }

    async fn delete_task(&self, task_id: &str) -> Result<()> {
        let mut conn = self.conn();
        let active_removed: i32 = conn
            .del(self.active_key(task_id))
            .await
            .map_err(redis_err)?;
        let dead_removed: i32 = conn
            .del(self.dead_letter_key(task_id))
            .await
            .map_err(redis_err)?;
        if active_removed == 0 && dead_removed == 0 {
            return Err(anyhow!("task {task_id:?} not found in any store"));
        }
        let _: () = conn
            .srem(self.active_index_key(), task_id)
            .await
            .map_err(redis_err)?;
        let _: () = conn
            .srem(self.dead_letter_index_key(), task_id)
            .await
            .map_err(redis_err)?;
        Ok(())
    }

    async fn list_tasks(&self, filter: TaskFilter) -> Vec<Task> {
        let mut conn = self.conn();
        let active_ids: Vec<String> = conn
            .smembers(self.active_index_key())
            .await
            .unwrap_or_default();
        let dead_ids: Vec<String> = conn
            .smembers(self.dead_letter_index_key())
            .await
            .unwrap_or_default();
        let active_keys: Vec<String> = active_ids.iter().map(|id| self.active_key(id)).collect();
        let dead_keys: Vec<String> = dead_ids.iter().map(|id| self.dead_letter_key(id)).collect();
        let mut tasks = self.read_tasks(&active_keys).await.unwrap_or_default();
        tasks.extend(self.read_tasks(&dead_keys).await.unwrap_or_default());
        apply_filter(tasks, filter)
    }

    async fn list_tasks_by_context(&self, context_id: &str, filter: TaskFilter) -> Vec<Task> {
        let mut conn = self.conn();
        let task_ids: Vec<String> = conn
            .smembers(self.context_members_key(context_id))
            .await
            .unwrap_or_default();
        let mut keys: Vec<String> = Vec::with_capacity(task_ids.len() * 2);
        for id in &task_ids {
            keys.push(self.active_key(id));
            keys.push(self.dead_letter_key(id));
        }
        let tasks = self.read_tasks(&keys).await.unwrap_or_default();
        apply_filter(tasks, filter)
    }

    // ----- Contexts -------------------------------------------------

    async fn get_contexts(&self) -> Vec<String> {
        let mut conn = self.conn();
        conn.smembers(self.contexts_set_key())
            .await
            .unwrap_or_default()
    }

    async fn get_contexts_with_tasks(&self) -> Vec<String> {
        let mut conn = self.conn();
        let candidates: Vec<String> = conn
            .smembers(self.contexts_set_key())
            .await
            .unwrap_or_default();
        let mut out = Vec::new();
        for ctx in candidates {
            let len: i32 = conn
                .scard(self.context_members_key(&ctx))
                .await
                .unwrap_or(0);
            if len > 0 {
                out.push(ctx);
            }
        }
        out
    }

    async fn delete_context(&self, context_id: &str) -> Result<()> {
        let mut conn = self.conn();
        let _: () = conn
            .srem(self.contexts_set_key(), context_id)
            .await
            .map_err(redis_err)?;
        Ok(())
    }

    async fn delete_context_and_tasks(&self, context_id: &str) -> Result<()> {
        let mut conn = self.conn();
        let task_ids: Vec<String> = conn
            .smembers(self.context_members_key(context_id))
            .await
            .unwrap_or_default();
        for id in &task_ids {
            let _: () = conn.del(self.active_key(id)).await.map_err(redis_err)?;
            let _: () = conn
                .del(self.dead_letter_key(id))
                .await
                .map_err(redis_err)?;
            let _: () = conn
                .srem(self.active_index_key(), id)
                .await
                .map_err(redis_err)?;
            let _: () = conn
                .srem(self.dead_letter_index_key(), id)
                .await
                .map_err(redis_err)?;
        }
        let _: () = conn
            .del(self.context_members_key(context_id))
            .await
            .map_err(redis_err)?;
        let _: () = conn
            .srem(self.contexts_set_key(), context_id)
            .await
            .map_err(redis_err)?;
        Ok(())
    }

    // ----- Cleanup / stats ------------------------------------------

    async fn cleanup_completed_tasks(&self) -> usize {
        let mut conn = self.conn();
        let ids: Vec<String> = conn
            .smembers(self.dead_letter_index_key())
            .await
            .unwrap_or_default();
        let mut removed = 0;
        for id in ids {
            let payload: Option<String> = conn.get(self.dead_letter_key(&id)).await.ok().flatten();
            let Some(payload) = payload else { continue };
            let Ok(task) = serde_json::from_str::<Task>(&payload) else {
                continue;
            };
            if task.status.state == TaskState::TaskStateCompleted {
                let _: redis::RedisResult<()> = conn.del(self.dead_letter_key(&id)).await;
                let _: redis::RedisResult<()> = conn.srem(self.dead_letter_index_key(), &id).await;
                removed += 1;
            }
        }
        removed
    }

    async fn cleanup_tasks_with_retention(&self, max_completed: usize, max_failed: usize) -> usize {
        let mut conn = self.conn();
        let ids: Vec<String> = conn
            .smembers(self.dead_letter_index_key())
            .await
            .unwrap_or_default();
        // Fetch all dead-letter tasks so we can sort by status timestamp.
        let keys: Vec<String> = ids.iter().map(|id| self.dead_letter_key(id)).collect();
        let payloads: Vec<Option<String>> = if keys.is_empty() {
            Vec::new()
        } else {
            conn.mget(keys).await.unwrap_or_default()
        };
        let mut completed: Vec<(String, Option<chrono::DateTime<chrono::Utc>>)> = Vec::new();
        let mut failed: Vec<(String, Option<chrono::DateTime<chrono::Utc>>)> = Vec::new();
        for (id, payload) in ids.into_iter().zip(payloads.into_iter()) {
            let Some(payload) = payload else { continue };
            let Ok(task) = serde_json::from_str::<Task>(&payload) else {
                continue;
            };
            let ts = task.status.timestamp.as_ref().map(|t| t.0);
            match task.status.state {
                TaskState::TaskStateCompleted => completed.push((id, ts)),
                TaskState::TaskStateFailed => failed.push((id, ts)),
                _ => {}
            }
        }
        fn evict(
            mut entries: Vec<(String, Option<chrono::DateTime<chrono::Utc>>)>,
            keep: usize,
        ) -> Vec<String> {
            if entries.len() <= keep {
                return Vec::new();
            }
            entries.sort_by_key(|(_, ts)| *ts);
            let evict_count = entries.len() - keep;
            entries
                .into_iter()
                .take(evict_count)
                .map(|(id, _)| id)
                .collect()
        }
        let mut to_remove = evict(completed, max_completed);
        to_remove.extend(evict(failed, max_failed));
        let removed = to_remove.len();
        for id in to_remove {
            let _: redis::RedisResult<()> = conn.del(self.dead_letter_key(&id)).await;
            let _: redis::RedisResult<()> = conn.srem(self.dead_letter_index_key(), &id).await;
        }
        removed
    }

    async fn get_stats(&self) -> StorageStats {
        let mut conn = self.conn();
        let queue_length: usize = conn.llen(self.queue_key()).await.unwrap_or(0);
        let active_tasks: usize = conn.scard(self.active_index_key()).await.unwrap_or(0);
        let dead_letter_tasks: usize = conn.scard(self.dead_letter_index_key()).await.unwrap_or(0);
        let contexts: usize = conn.scard(self.contexts_set_key()).await.unwrap_or(0);
        StorageStats {
            queue_length,
            active_tasks,
            dead_letter_tasks,
            contexts,
        }
    }

    // ----- Push-notification configs -------------------------------

    async fn put_push_notification_config(&self, config: TaskPushNotificationConfig) {
        let mut conn = self.conn();
        let Ok(payload) = serde_json::to_string(&config) else {
            return;
        };
        let _: redis::RedisResult<()> = conn.set(self.push_config_key(&config.name), payload).await;
        let _: redis::RedisResult<()> = conn.sadd(self.push_config_index_key(), &config.name).await;
    }

    async fn get_push_notification_config(&self, name: &str) -> Option<TaskPushNotificationConfig> {
        let mut conn = self.conn();
        let payload: Option<String> = conn.get(self.push_config_key(name)).await.ok()?;
        payload.and_then(|s| serde_json::from_str(&s).ok())
    }

    async fn list_push_notification_configs(
        &self,
        parent: &str,
    ) -> Vec<TaskPushNotificationConfig> {
        let mut conn = self.conn();
        let names: Vec<String> = conn
            .smembers(self.push_config_index_key())
            .await
            .unwrap_or_default();
        let prefix = format!("{parent}/pushNotificationConfigs/");
        let filtered: Vec<String> = names
            .into_iter()
            .filter(|n| n.starts_with(&prefix))
            .collect();
        let keys: Vec<String> = filtered.iter().map(|n| self.push_config_key(n)).collect();
        if keys.is_empty() {
            return Vec::new();
        }
        let values: Vec<Option<String>> = conn.mget(keys).await.unwrap_or_default();
        values
            .into_iter()
            .flatten()
            .filter_map(|s| serde_json::from_str(&s).ok())
            .collect()
    }

    async fn delete_push_notification_config(&self, name: &str) -> bool {
        let mut conn = self.conn();
        let removed: i32 = match conn.del(self.push_config_key(name)).await {
            Ok(n) => n,
            Err(_) => return false,
        };
        let _: redis::RedisResult<()> = conn.srem(self.push_config_index_key(), name).await;
        removed > 0
    }
}

//! Storage abstraction backing the A2A task manager.
//!
//! [`Storage`] is the trait the A2A server holds as `Arc<dyn Storage>` to
//! persist tasks and push-notification configurations. [`InMemoryStorage`]
//! is the bundled default — a `Mutex`-backed `HashMap` suitable for tests,
//! single-instance deployments, and bootstrap. Implement [`Storage`]
//! yourself to plug in Redis, Postgres, or any other backend without
//! forking the crate.

use crate::a2a_types::{Task, TaskPushNotificationConfig};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Mutex;

/// Pluggable storage for the A2A task manager. Implement this trait to
/// back the server with a non-default persistence layer (Redis,
/// Postgres, etc.) and wire it in via
/// [`A2AServerBuilder::with_storage`](super::server_builder::A2AServerBuilder::with_storage).
///
/// All methods are async to accommodate backends that need to issue
/// I/O (Redis, Postgres, etc.). Implementations use interior
/// mutability (mutex, connection pool, etc.) so the public signatures
/// stay `Arc<dyn Storage>`-friendly.
#[async_trait]
pub trait Storage: Send + Sync + std::fmt::Debug {
    /// Insert or replace a task, keyed by its bare `id` (the portion after
    /// `tasks/` in a resource name).
    async fn put_task(&self, task: Task);

    /// Fetch a task by bare id, returning a cloned copy.
    async fn get_task(&self, id: &str) -> Option<Task>;

    /// Return every stored task. Pagination/filtering is applied by the
    /// caller.
    async fn list_tasks(&self) -> Vec<Task>;

    /// Insert or replace a push notification configuration. Keyed by the
    /// canonical resource name on the config.
    async fn put_push_notification_config(&self, config: TaskPushNotificationConfig);

    async fn get_push_notification_config(&self, name: &str) -> Option<TaskPushNotificationConfig>;

    /// Return every config whose resource name lives under `parent`
    /// (`parent` should be of the form `tasks/{task_id}`).
    async fn list_push_notification_configs(&self, parent: &str)
    -> Vec<TaskPushNotificationConfig>;

    async fn delete_push_notification_config(&self, name: &str) -> bool;
}

/// Simple in-memory [`Storage`] implementation for tasks and push
/// notification configurations. Suitable for tests, single-instance
/// deployments, and as a baseline reference implementation.
#[derive(Debug, Default)]
pub struct InMemoryStorage {
    inner: Mutex<StorageInner>,
}

#[derive(Debug, Default)]
struct StorageInner {
    tasks: HashMap<String, Task>,
    push_notification_configs: HashMap<String, TaskPushNotificationConfig>,
}

impl InMemoryStorage {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl Storage for InMemoryStorage {
    async fn put_task(&self, task: Task) {
        let mut inner = self.inner.lock().expect("storage mutex poisoned");
        inner.tasks.insert(task.id.clone(), task);
    }

    async fn get_task(&self, id: &str) -> Option<Task> {
        let inner = self.inner.lock().expect("storage mutex poisoned");
        inner.tasks.get(id).cloned()
    }

    async fn list_tasks(&self) -> Vec<Task> {
        let inner = self.inner.lock().expect("storage mutex poisoned");
        inner.tasks.values().cloned().collect()
    }

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
        PushNotificationConfig, TaskPushNotificationConfig, TaskState, TaskStatus,
    };

    fn make_task(id: &str) -> Task {
        Task {
            artifacts: vec![],
            context_id: "ctx".to_string(),
            history: vec![],
            id: id.to_string(),
            metadata: None,
            status: TaskStatus {
                message: None,
                state: TaskState::TaskStateSubmitted,
                timestamp: None,
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

    #[tokio::test]
    async fn task_round_trip() {
        let storage = InMemoryStorage::new();
        let task = make_task("abc");
        storage.put_task(task.clone()).await;
        let got = storage
            .get_task("abc")
            .await
            .expect("task should be present");
        assert_eq!(got.id, task.id);
        assert_eq!(storage.list_tasks().await.len(), 1);
    }

    #[tokio::test]
    async fn put_replaces_existing_task() {
        let storage = InMemoryStorage::new();
        storage.put_task(make_task("abc")).await;
        let mut task = storage
            .get_task("abc")
            .await
            .expect("task should be present");
        task.status.state = TaskState::TaskStateCancelled;
        storage.put_task(task).await;
        let updated = storage
            .get_task("abc")
            .await
            .expect("task should be present");
        assert_eq!(updated.status.state, TaskState::TaskStateCancelled);
        assert!(storage.get_task("missing").await.is_none());
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
        storage.put_task(make_task("abc")).await;
        let got = storage
            .get_task("abc")
            .await
            .expect("task should be present");
        assert_eq!(got.id, "abc");
    }
}

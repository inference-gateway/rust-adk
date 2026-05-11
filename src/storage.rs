//! Lightweight in-memory storage backing the A2A task manager.
//!
//! The full task manager / persistence layer is tracked separately; this
//! module provides just enough to dispatch the JSON-RPC methods listed in
//! the A2A specification and to keep them at parity with the Go ADK.

use crate::a2a_types::{Task, TaskPushNotificationConfig};
use std::collections::HashMap;
use std::sync::Mutex;

/// Simple in-memory storage for tasks and push notification configurations.
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

    /// Insert or replace a task (keyed by its bare `id`, not the resource name).
    pub fn put_task(&self, task: Task) {
        let mut inner = self.inner.lock().expect("storage mutex poisoned");
        inner.tasks.insert(task.id.clone(), task);
    }

    /// Fetch a task by bare id (the portion after `tasks/`).
    pub fn get_task(&self, id: &str) -> Option<Task> {
        let inner = self.inner.lock().expect("storage mutex poisoned");
        inner.tasks.get(id).cloned()
    }

    /// Return every stored task. Pagination/filtering is applied by the caller.
    pub fn list_tasks(&self) -> Vec<Task> {
        let inner = self.inner.lock().expect("storage mutex poisoned");
        inner.tasks.values().cloned().collect()
    }

    /// Apply `f` to the task identified by `id` if it exists. Returns the
    /// updated task (cloned) on success.
    pub fn update_task<F>(&self, id: &str, f: F) -> Option<Task>
    where
        F: FnOnce(&mut Task),
    {
        let mut inner = self.inner.lock().expect("storage mutex poisoned");
        let task = inner.tasks.get_mut(id)?;
        f(task);
        Some(task.clone())
    }

    /// Insert or replace a push notification configuration. Keyed by the
    /// canonical resource name in the config.
    pub fn put_push_notification_config(&self, config: TaskPushNotificationConfig) {
        let mut inner = self.inner.lock().expect("storage mutex poisoned");
        inner
            .push_notification_configs
            .insert(config.name.clone(), config);
    }

    pub fn get_push_notification_config(&self, name: &str) -> Option<TaskPushNotificationConfig> {
        let inner = self.inner.lock().expect("storage mutex poisoned");
        inner.push_notification_configs.get(name).cloned()
    }

    /// Return every config whose resource name lives under `parent`
    /// (`parent` should be of the form `tasks/{task_id}`).
    pub fn list_push_notification_configs(&self, parent: &str) -> Vec<TaskPushNotificationConfig> {
        let prefix = format!("{parent}/pushNotificationConfigs/");
        let inner = self.inner.lock().expect("storage mutex poisoned");
        inner
            .push_notification_configs
            .values()
            .filter(|c| c.name.starts_with(&prefix))
            .cloned()
            .collect()
    }

    pub fn delete_push_notification_config(&self, name: &str) -> bool {
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

    #[test]
    fn task_round_trip() {
        let storage = InMemoryStorage::new();
        let task = make_task("abc");
        storage.put_task(task.clone());
        let got = storage.get_task("abc").expect("task should be present");
        assert_eq!(got.id, task.id);
        assert_eq!(storage.list_tasks().len(), 1);
    }

    #[test]
    fn update_task_mutates_in_place() {
        let storage = InMemoryStorage::new();
        storage.put_task(make_task("abc"));
        let updated = storage
            .update_task("abc", |t| {
                t.status.state = TaskState::TaskStateCancelled;
            })
            .expect("task should be present");
        assert_eq!(updated.status.state, TaskState::TaskStateCancelled);
        assert!(storage.update_task("missing", |_| {}).is_none());
    }

    #[test]
    fn push_notification_configs_filter_by_parent() {
        let storage = InMemoryStorage::new();
        storage.put_push_notification_config(make_config(
            "tasks/abc/pushNotificationConfigs/c1",
            "https://a.example/webhook",
        ));
        storage.put_push_notification_config(make_config(
            "tasks/abc/pushNotificationConfigs/c2",
            "https://b.example/webhook",
        ));
        storage.put_push_notification_config(make_config(
            "tasks/other/pushNotificationConfigs/c3",
            "https://c.example/webhook",
        ));

        let configs = storage.list_push_notification_configs("tasks/abc");
        assert_eq!(configs.len(), 2);

        assert!(storage.delete_push_notification_config("tasks/abc/pushNotificationConfigs/c1"));
        assert_eq!(storage.list_push_notification_configs("tasks/abc").len(), 1);
        assert!(!storage.delete_push_notification_config("tasks/abc/pushNotificationConfigs/c1"));
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
}

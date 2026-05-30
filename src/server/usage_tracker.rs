use inference_gateway_sdk::CompletionUsage;
use serde_json::{Map, Value, json};
use std::sync::Mutex;

/// Accumulates token usage and execution statistics over a single task run.
///
/// The default task handlers create one [`UsageTracker`] per task, thread it
/// through [`run_tool_loop`](super::task_handler), and - when usage metadata is
/// enabled - merge [`metadata`](Self::metadata) into the task's
/// `metadata` field once the task reaches a terminal state.
///
/// Counters are split into two groups:
/// - **Token usage** (`prompt_tokens` / `completion_tokens` / `total_tokens`),
///   summed from each [`CompletionUsage`] the gateway returns. Every
///   [`add_token_usage`](Self::add_token_usage) call also bumps an internal
///   `llm_calls` counter so [`metadata`](Self::metadata) can decide whether a
///   `usage` block is meaningful.
/// - **Execution stats** (`iterations` / `messages` / `tool_calls` /
///   `failed_tools`), counted as the agent loop runs.
///
/// All mutators take `&self` (interior mutability via a [`Mutex`]) so the
/// tracker can be shared behind a shared reference across `.await` points.
#[derive(Debug, Default)]
pub struct UsageTracker {
    inner: Mutex<Stats>,
}

#[derive(Debug, Default, Clone)]
struct Stats {
    prompt_tokens: i64,
    completion_tokens: i64,
    total_tokens: i64,
    iterations: u64,
    messages: u64,
    tool_calls: u64,
    failed_tools: u64,
    llm_calls: u64,
}

impl UsageTracker {
    pub fn new() -> Self {
        Self::default()
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, Stats> {
        self.inner.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// Add the token counts from one chat-completion response and record that
    /// an LLM call was made.
    pub fn add_token_usage(&self, usage: &CompletionUsage) {
        let mut s = self.lock();
        s.prompt_tokens += usage.prompt_tokens;
        s.completion_tokens += usage.completion_tokens;
        s.total_tokens += usage.total_tokens;
        s.llm_calls += 1;
    }

    /// Count one pass through the agent's model ↔ tool loop.
    pub fn increment_iteration(&self) {
        self.lock().iterations += 1;
    }

    /// Count `count` conversation messages produced during the loop (the
    /// default handlers use this for tool-result messages fed back to the
    /// model).
    pub fn add_messages(&self, count: usize) {
        self.lock().messages += count as u64;
    }

    /// Count one dispatched tool call.
    pub fn increment_tool_calls(&self) {
        self.lock().tool_calls += 1;
    }

    /// Count one tool call that failed (missing handler or handler error).
    pub fn increment_failed_tools(&self) {
        self.lock().failed_tools += 1;
    }

    /// Whether anything worth reporting was recorded. Mirrors the Go ADK:
    /// true when at least one LLM call, iteration, message, or tool call
    /// happened.
    pub fn has_usage(&self) -> bool {
        let s = self.lock();
        s.llm_calls > 0 || s.iterations > 0 || s.messages > 0 || s.tool_calls > 0
    }

    /// Build the metadata object merged into a terminal task. Always emits an
    /// `execution_stats` block; only emits a `usage` block when at least one
    /// LLM call contributed token counts (otherwise the zeros would be
    /// misleading).
    pub fn metadata(&self) -> Map<String, Value> {
        let s = self.lock();
        let mut map = Map::new();
        if s.llm_calls > 0 {
            map.insert(
                "usage".to_string(),
                json!({
                    "prompt_tokens": s.prompt_tokens,
                    "completion_tokens": s.completion_tokens,
                    "total_tokens": s.total_tokens,
                }),
            );
        }
        map.insert(
            "execution_stats".to_string(),
            json!({
                "iterations": s.iterations,
                "messages": s.messages,
                "tool_calls": s.tool_calls,
                "failed_tools": s.failed_tools,
            }),
        );
        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn usage(prompt: i64, completion: i64, total: i64) -> CompletionUsage {
        CompletionUsage {
            prompt_tokens: prompt,
            completion_tokens: completion,
            total_tokens: total,
        }
    }

    #[test]
    fn fresh_tracker_reports_no_usage() {
        let tracker = UsageTracker::new();
        assert!(!tracker.has_usage());
        // Only execution_stats is present, with all-zero counters.
        let meta = tracker.metadata();
        assert!(!meta.contains_key("usage"));
        let stats = &meta["execution_stats"];
        assert_eq!(stats["iterations"], 0);
        assert_eq!(stats["messages"], 0);
        assert_eq!(stats["tool_calls"], 0);
        assert_eq!(stats["failed_tools"], 0);
    }

    #[test]
    fn token_usage_accumulates_across_calls() {
        let tracker = UsageTracker::new();
        tracker.add_token_usage(&usage(10, 5, 15));
        tracker.add_token_usage(&usage(20, 8, 28));

        assert!(tracker.has_usage());
        let meta = tracker.metadata();
        let usage_block = &meta["usage"];
        assert_eq!(usage_block["prompt_tokens"], 30);
        assert_eq!(usage_block["completion_tokens"], 13);
        assert_eq!(usage_block["total_tokens"], 43);
    }

    #[test]
    fn execution_stats_count_loop_activity() {
        let tracker = UsageTracker::new();
        tracker.increment_iteration();
        tracker.increment_iteration();
        tracker.add_messages(3);
        tracker.increment_tool_calls();
        tracker.increment_tool_calls();
        tracker.increment_tool_calls();
        tracker.increment_failed_tools();

        assert!(tracker.has_usage());
        let stats = &tracker.metadata()["execution_stats"];
        assert_eq!(stats["iterations"], 2);
        assert_eq!(stats["messages"], 3);
        assert_eq!(stats["tool_calls"], 3);
        assert_eq!(stats["failed_tools"], 1);
    }

    #[test]
    fn iterations_alone_count_as_usage_without_a_usage_block() {
        // A streaming turn that produced no token counts still reports
        // execution stats but omits the usage block.
        let tracker = UsageTracker::new();
        tracker.increment_iteration();

        assert!(tracker.has_usage());
        let meta = tracker.metadata();
        assert!(!meta.contains_key("usage"));
        assert_eq!(meta["execution_stats"]["iterations"], 1);
    }

    #[test]
    fn metadata_matches_expected_shape() {
        let tracker = UsageTracker::new();
        tracker.add_token_usage(&usage(123, 45, 168));
        tracker.increment_iteration();
        tracker.increment_iteration();
        tracker.add_messages(7);
        tracker.increment_tool_calls();
        tracker.increment_tool_calls();
        tracker.increment_tool_calls();
        tracker.increment_failed_tools();

        let meta = Value::Object(tracker.metadata());
        let expected = json!({
            "usage": {
                "prompt_tokens": 123,
                "completion_tokens": 45,
                "total_tokens": 168,
            },
            "execution_stats": {
                "iterations": 2,
                "messages": 7,
                "tool_calls": 3,
                "failed_tools": 1,
            },
        });
        assert_eq!(meta, expected);
    }
}

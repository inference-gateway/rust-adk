use super::agent_llm_client::LLMClient;
use super::agent_toolbox::ToolHandler;
use inference_gateway_sdk::ChatCompletionTool;
use std::collections::HashMap;
use std::sync::Arc;

pub struct Agent {
    pub(super) system_prompt: Option<String>,
    pub(super) llm_client: Arc<dyn LLMClient>,
    #[allow(dead_code)]
    pub(super) max_chat_completion: u32,
    #[allow(dead_code)]
    pub(super) max_conversation_history: u32,
    #[allow(dead_code)]
    pub(super) toolbox: Option<Vec<ChatCompletionTool>>,
    pub(super) tool_handlers: HashMap<String, Box<dyn ToolHandler>>,
}

impl std::fmt::Debug for Agent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Agent")
            .field("system_prompt", &self.system_prompt)
            .field("llm_client", &self.llm_client)
            .field("max_chat_completion", &self.max_chat_completion)
            .field("max_conversation_history", &self.max_conversation_history)
            .field("toolbox", &self.toolbox)
            .field(
                "tool_handlers",
                &format!("{} handlers", self.tool_handlers.len()),
            )
            .finish()
    }
}

impl Agent {
    pub fn toolbox(&self) -> Option<&Vec<ChatCompletionTool>> {
        self.toolbox.as_ref()
    }

    /// Look up a registered tool handler by name.
    pub fn tool_handler(&self, name: &str) -> Option<&dyn ToolHandler> {
        self.tool_handlers.get(name).map(|b| &**b)
    }

    /// Maximum number of model ↔ tool round-trips before the default tool
    /// loop gives up. Mirrors the existing builder knob.
    pub fn max_chat_completion(&self) -> u32 {
        self.max_chat_completion
    }

    /// The LLM client this agent dispatches chat completions through.
    pub fn llm_client(&self) -> &Arc<dyn LLMClient> {
        &self.llm_client
    }
}

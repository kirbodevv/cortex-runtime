use std::sync::Arc;

use crate::{
    app::session::ChatSession,
    config::core::CortexConfig,
    domain::{MemoryItem, Message},
};

pub struct PromptBuilder {
    config: Arc<CortexConfig>,
    system: String,
}

impl PromptBuilder {
    pub fn new(config: Arc<CortexConfig>, system: impl Into<String>) -> Self {
        Self {
            config,
            system: system.into(),
        }
    }

    pub fn build(&self, session: &ChatSession, memory: &[MemoryItem]) -> Vec<Message> {
        let mut messages = vec![Message::system(self.system.clone())];

        for m in memory {
            messages.push(Message::system(format!("Memory: {}", m.content())));
        }

        messages.extend(session.recent(self.config.context_window_size).to_vec());

        messages
    }
}

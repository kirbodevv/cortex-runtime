use crate::{
    app::session::ChatSession,
    domain::{MemoryItem, Message},
};

const RECENT_MESSAGE_COUNT: usize = 10;

pub struct PromptBuilder {
    system: String,
}

impl PromptBuilder {
    pub fn new(system: impl Into<String>) -> Self {
        Self {
            system: system.into(),
        }
    }

    pub fn build(&self, session: &ChatSession, memory: &[MemoryItem]) -> Vec<Message> {
        let mut messages = vec![Message::system(self.system.clone())];

        for m in memory {
            messages.push(Message::system(format!("Memory: {}", m.content())));
        }

        messages.extend(session.recent(RECENT_MESSAGE_COUNT).to_vec());

        messages
    }
}

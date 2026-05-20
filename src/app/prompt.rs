use crate::{
    app::session::ChatSession,
    domain::{MemoryItem, Message},
};

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

        messages.extend(session.messages().to_vec());

        messages
    }
}

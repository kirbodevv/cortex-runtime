use std::time::Instant;

use crate::domain::Message;

pub struct ChatSession {
    messages: Vec<Message>,
    last_activity: Instant,
}

impl ChatSession {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            last_activity: Instant::now(),
        }
    }

    pub fn append(&mut self, message: Message) {
        self.messages.push(message);
        self.last_activity = Instant::now();
    }

    #[allow(dead_code)]
    pub fn messages(&self) -> &[Message] {
        &self.messages
    }

    pub fn recent(&self, n: usize) -> &[Message] {
        let len = self.messages.len();
        let start = len.saturating_sub(n);
        &self.messages[start..]
    }

    pub fn clear(&mut self) {
        self.messages.clear();
        self.last_activity = Instant::now();
    }
}

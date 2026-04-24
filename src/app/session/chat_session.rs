use std::time::{Duration, Instant};

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

    pub fn messages(&self) -> &[Message] {
        &self.messages
    }

    pub fn timeout(&self, timeout: Duration) -> bool {
        self.last_activity.elapsed() > timeout
    }

    pub fn clear(&mut self) {
        self.messages.clear();
        self.last_activity = Instant::now();
    }
}

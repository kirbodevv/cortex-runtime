use serde_json::Value;

use crate::domain::Message;

pub struct LLMRequest {
    pub messages: Vec<Message>,
    pub json_schema: Value,
}

pub struct LLMRawResponse {
    pub text: String,
}

impl LLMRequest {
    pub fn new(messages: &[Message], json_schema: Value) -> Self {
        Self {
            messages: messages.to_vec(),
            json_schema,
        }
    }
}

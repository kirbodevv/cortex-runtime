use crate::domain::{Context, DomainError, Message};

pub struct LLMRequest {
    messages: Vec<Message>,
    context: Context,
}

pub struct LLMRawResponse {
    pub text: String,
}

impl LLMRequest {
    pub fn new(messages: Vec<Message>, context: Context) -> Result<Self, DomainError> {
        if messages.is_empty() {
            return Err(DomainError::EmptyRequest);
        }
        Ok(Self { messages, context })
    }

    pub fn messages(&self) -> &[Message] {
        &self.messages
    }

    pub fn context(&self) -> &Context {
        &self.context
    }
}

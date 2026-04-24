use crate::domain::{Context, DomainError, Message};

pub struct LLMRequest {
    messages: Vec<Message>,
    context: Context,
}

pub struct LLMRawResponse {
    pub text: String,
}

impl LLMRequest {
    pub fn new(messages: &[Message], context: Context) -> Result<Self, DomainError> {
        Ok(Self {
            messages: messages.to_vec(),
            context,
        })
    }

    pub fn messages(&self) -> &[Message] {
        &self.messages
    }

    pub fn context(&self) -> &Context {
        &self.context
    }
}

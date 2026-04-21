use crate::domain::{Context, Message};

pub struct LLMRequest {
    pub messages: Vec<Message>,
    pub context: Context,
}

pub struct LLMRawResponse {
    pub text: String,
}

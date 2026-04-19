use crate::{app::dto::LLMResponse, domain::error::AppError};

pub trait LLMService: Send + Sync {
    async fn process(&mut self, input: &str, context: Vec<String>)
    -> Result<LLMResponse, AppError>;
}

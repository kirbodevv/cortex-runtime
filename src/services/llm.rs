use crate::domain::error::AppError;

pub trait LLMService {
    fn process(&mut self, input: &str, context: Vec<String>) -> Result<String, AppError>;
}

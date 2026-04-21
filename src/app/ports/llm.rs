use crate::domain::{AppError, LLMRawResponse, LLMRequest};

#[async_trait::async_trait]
pub trait LLMClient: Send + Sync {
    async fn generate(&self, req: LLMRequest) -> Result<LLMRawResponse, AppError>;
}

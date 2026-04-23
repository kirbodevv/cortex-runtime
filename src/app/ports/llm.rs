use crate::domain::{LLMRawResponse, LLMRequest};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum LLMError {
    #[error("API call failed: {0}")]
    ApiError(#[source] Box<dyn std::error::Error + Send + Sync>),

    #[error("API returned empty response")]
    EmptyResponse,
}

#[async_trait::async_trait]
pub trait LLMClient: Send + Sync {
    async fn generate(&self, req: LLMRequest) -> Result<LLMRawResponse, LLMError>;
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Failed to parse LLM response: {0}")]
    ParseResponse(#[from] serde_json::Error),

    #[error("Request has no messages")]
    EmptyRequest,
}

use crate::{
    app::ports::{EmbedderError, LLMError, MemoryStoreError},
    domain::DomainError,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Domain error: {0}")]
    Domain(#[from] DomainError),

    #[error("LLM API error: {0}")]
    LLM(#[from] LLMError),

    #[error("Embedder error: {0}")]
    Embedder(#[from] EmbedderError),

    #[error("Memory storage error: {0}")]
    Memory(MemoryStoreError),
}

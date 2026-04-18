use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("LLM error")]
    LLMError,
}

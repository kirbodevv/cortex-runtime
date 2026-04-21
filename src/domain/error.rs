use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("LLM error")]
    LLMError(String),

    #[error("Runtime error")]
    RuntimeError(String),
}

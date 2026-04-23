#[derive(Debug, thiserror::Error)]
pub enum EmbedderError {
    #[error("Embedder error: {0}")]
    ApiError(#[source] Box<dyn std::error::Error + Send + Sync>),
}

#[async_trait::async_trait]
pub trait Embedder: Send + Sync {
    async fn embed(&self, text: &str) -> Result<Vec<f32>, EmbedderError>;
}

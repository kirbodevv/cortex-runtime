use genai::Client;

use crate::{app::ports::embedder::Embedder, domain::AppError};

pub struct OpenAiEmbedder {
    client: Client,
}

impl OpenAiEmbedder {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[async_trait::async_trait]
impl Embedder for OpenAiEmbedder {
    async fn embed(&self, text: &str) -> Result<Vec<f32>, AppError> {
        let embedding = self
            .client
            .embed("text-embedding-3-large", text, None)
            .await
            .map_err(|e| AppError::LLMError(e.to_string()))?;
        Ok(embedding.embeddings[0].clone().vector)
    }
}

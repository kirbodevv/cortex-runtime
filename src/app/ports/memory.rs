use crate::domain::{MemoryItem, StoredMemory};

#[allow(dead_code)]
#[derive(Debug, thiserror::Error)]
pub enum MemoryStoreError {
    #[error("Failed to insert memory")]
    FailedToInsert,

    #[error("Failed to compute similarity: vectors may have different dimensions")]
    SimilarityError,
}

#[async_trait::async_trait]
pub trait MemoryStore: Send + Sync {
    async fn insert(&mut self, memory: StoredMemory) -> Result<(), MemoryStoreError>;
    async fn search(
        &self,
        query: &[f32],
        threshold: f64,
        top_k: usize,
    ) -> Result<Vec<(&MemoryItem, f64)>, MemoryStoreError>;
    async fn max_similarity(&self, query: &[f32]) -> Result<(&MemoryItem, f64), MemoryStoreError>;
}

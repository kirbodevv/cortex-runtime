use crate::domain::{MemoryItem, StoredMemory};

#[async_trait::async_trait]
pub trait MemoryStore: Send + Sync {
    async fn insert(&mut self, memory: StoredMemory);
    async fn search(&self, query: &[f32], threshold: f64, top_k: usize) -> Vec<(&MemoryItem, f64)>;
}

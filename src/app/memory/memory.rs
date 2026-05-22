use std::sync::Arc;

use crate::{
    app::{
        dto::MemoryCandidate,
        ports::{Embedder, MemoryStore},
    },
    config::core::CortexConfig,
    domain::{MemoryItem, StoredMemory},
    error::AppError,
};

pub struct MemoryService<E, S>
where
    E: Embedder,
    S: MemoryStore,
{
    config: Arc<CortexConfig>,
    embedder: E,
    store: S,
}

impl<E, S> MemoryService<E, S>
where
    E: Embedder,
    S: MemoryStore,
{
    pub fn new(config: Arc<CortexConfig>, embedder: E, store: S) -> Self {
        Self {
            config,
            embedder,
            store,
        }
    }

    pub async fn search(&self, query: &str) -> Result<Vec<&MemoryItem>, AppError> {
        let query_vec = self.embedder.embed(query).await?;

        let result = self
            .store
            .search(
                &query_vec,
                self.config.memory_threshold,
                self.config.memory_top_k,
            )
            .await
            .map_err(|e| AppError::Memory(e))?;

        let items = result.iter().map(|(item, _)| *item).collect::<Vec<_>>();

        Ok(items)
    }

    pub async fn save(&mut self, memory: MemoryCandidate) -> Result<(), AppError> {
        if memory.importance < self.config.memory_importance_threshold {
            return Ok(());
        }

        let embedding = self
            .embedder
            .embed(&memory.summary)
            .await
            .map_err(|e| AppError::Embedder(e))?;

        if let Ok((_, similarity)) = self.store.max_similarity(&embedding).await {
            if similarity > 0.9 {
                return Ok(());
            }
        }

        let item = MemoryItem::new(&memory.summary);

        self.store
            .insert(StoredMemory { item, embedding })
            .await
            .map_err(|e| AppError::Memory(e))?;

        Ok(())
    }
}

use crate::{
    app::{
        dto::MemoryCandidate,
        ports::{Embedder, MemoryStore},
    },
    domain::{MemoryItem, StoredMemory},
    error::AppError,
};

pub struct MemoryService<E, S>
where
    E: Embedder,
    S: MemoryStore,
{
    embedder: E,
    store: S,
}

impl<E, S> MemoryService<E, S>
where
    E: Embedder,
    S: MemoryStore,
{
    pub fn new(embedder: E, store: S) -> Self {
        Self { embedder, store }
    }

    pub async fn search(&self, query: &str) -> Result<Vec<&MemoryItem>, AppError> {
        let query_vec = self.embedder.embed(query).await?;

        let result = self
            .store
            .search(&query_vec, 0.3, 5)
            .await
            .map_err(|e| AppError::Memory(e))?;

        let items = result.iter().map(|(item, _)| *item).collect::<Vec<_>>();

        println!(
            "[INFO MEM] added to context: {}",
            items
                .iter()
                .map(|i| i.content())
                .collect::<Vec<_>>()
                .join(", ")
        );

        Ok(items)
    }

    pub async fn save(&mut self, memory: MemoryCandidate) -> Result<(), AppError> {
        if memory.importance > 0.6 {
            let item = MemoryItem::new(&memory.summary);
            let embedding = self
                .embedder
                .embed(&memory.summary)
                .await
                .map_err(|e| AppError::Embedder(e))?;

            self.store
                .insert(StoredMemory { item, embedding })
                .await
                .map_err(|e| AppError::Memory(e))?;

            println!("[INFO MEM] saved memory: {}", memory.summary);
        }
        Ok(())
    }
}

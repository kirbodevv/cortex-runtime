use std::cmp::Ordering;

use similarity::{Similarity, similarity_traits::CosineSimilarity};

use crate::{
    app::ports::{MemoryStore, MemoryStoreError},
    domain::{MemoryItem, StoredMemory},
};

pub struct InMemoryStore {
    memories: Vec<StoredMemory>,
}

impl InMemoryStore {
    pub fn new() -> Self {
        Self {
            memories: Vec::new(),
        }
    }
}

#[async_trait::async_trait]
impl MemoryStore for InMemoryStore {
    async fn insert(&mut self, memory: StoredMemory) -> Result<(), MemoryStoreError> {
        self.memories.push(memory);
        Ok(())
    }

    async fn search(
        &self,
        query: &[f32],
        threshold: f64,
        top_k: usize,
    ) -> Result<Vec<(&MemoryItem, f64)>, MemoryStoreError> {
        let mut items = self
            .memories
            .iter()
            .map(|m| {
                let sim = CosineSimilarity::similarity((&query, &m.embedding))
                    .ok_or(MemoryStoreError::SimilarityError)?;
                Ok((&m.item, sim))
            })
            .collect::<Result<Vec<_>, _>>()?;

        items.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));

        let top_k: Vec<(&MemoryItem, f64)> = items
            .into_iter()
            .take(top_k)
            .filter_map(|(item, score)| {
                if score > threshold {
                    Some((item, score))
                } else {
                    None
                }
            })
            .collect();

        Ok(top_k)
    }

    async fn max_similarity(&self, query: &[f32]) -> Result<(&MemoryItem, f64), MemoryStoreError> {
        let max_sim = {
            let mut max_sim: Option<(&MemoryItem, f64)> = None;
            for memory in &self.memories {
                let sim = CosineSimilarity::similarity((&query, &memory.embedding))
                    .ok_or(MemoryStoreError::SimilarityError)?;

                if max_sim.is_none() || sim > max_sim.unwrap().1 {
                    max_sim = Some((&memory.item, sim));
                }
            }
            max_sim
        };

        max_sim.ok_or(MemoryStoreError::SimilarityError)
    }
}

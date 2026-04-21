use similarity::{Similarity, similarity_traits::CosineSimilarity};

use crate::{
    app::ports::MemoryStore,
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
    async fn insert(&mut self, memory: StoredMemory) {
        self.memories.push(memory);
    }

    async fn search(&self, query: &[f32], threshold: f64, top_k: usize) -> Vec<(&MemoryItem, f64)> {
        let mut items = self
            .memories
            .iter()
            .map(|m| {
                let sim = CosineSimilarity::similarity((&query, &m.embedding)).unwrap_or(0.0);
                (&m.item, sim)
            })
            .collect::<Vec<(&MemoryItem, f64)>>();

        items.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

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

        top_k
    }
}

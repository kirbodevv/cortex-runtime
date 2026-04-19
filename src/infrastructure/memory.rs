use chrono::Utc;
use genai::Client;
use similarity::{Similarity, similarity_traits::CosineSimilarity};

use crate::{
    app::dto::MemoryCandidate,
    domain::error::AppError,
    services::memory::{MemoryItem, MemoryService},
};

pub struct Memory {
    client: Client,
    memories: Vec<(MemoryItem, Vec<f32>)>,
}

impl Memory {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            memories: Vec::new(),
        }
    }

    async fn embed(&self, input: &str) -> Result<Vec<f32>, AppError> {
        let embedding = self
            .client
            .embed("text-embedding-3-large", input, None)
            .await
            .map_err(|e| AppError::LLMError(e.to_string()))?;
        Ok(embedding.embeddings[0].clone().vector)
    }
}

impl MemoryService for Memory {
    async fn search(&self, query: &str) -> Result<Vec<MemoryItem>, AppError> {
        let query_vec = self.embed(query).await?;

        let mut items = self
            .memories
            .iter()
            .map(|(m, e)| {
                let sim = CosineSimilarity::similarity((&query_vec, &e)).unwrap_or(0.0);
                (m.clone(), sim)
            })
            .collect::<Vec<(MemoryItem, f64)>>();

        items.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        let top5: Vec<MemoryItem> = items
            .into_iter()
            .take(5)
            .filter_map(|(item, score)| if score > 0.3 { Some(item) } else { None })
            .collect();

        if top5.len() > 0 {
            println!(
                "[INFO] ДОБАВЛЕНО В КОНТЕКСТ: {}",
                top5.iter()
                    .map(|m| m.content.clone())
                    .collect::<Vec<String>>()
                    .join(", ")
            );
        }

        Ok(top5)
    }

    async fn save(&mut self, item: MemoryCandidate) -> Result<(), AppError> {
        if item.importance > 0.6 {
            self.memories.push((
                MemoryItem {
                    content: item.summary.clone(),
                    created_at: Utc::now(),
                },
                self.embed(&item.summary).await?,
            ));
            println!("[INFO] ЗАПИСАНО В ПАМЯТЬ: {}", item.summary)
        }
        Ok(())
    }
}

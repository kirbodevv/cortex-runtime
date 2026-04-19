use chrono::Utc;
use openai_api_rust::{
    OpenAI,
    embeddings::{EmbeddingsApi, EmbeddingsBody},
};
use similarity::{Similarity, similarity_traits::CosineSimilarity};

use crate::{
    app::dto::MemoryCandidate,
    domain::error::AppError,
    services::memory::{MemoryItem, MemoryService},
};

pub struct Memory {
    openai: OpenAI,
    memories: Vec<(MemoryItem, Vec<f64>)>,
}

impl Memory {
    pub fn new(openai: OpenAI) -> Self {
        Self {
            openai,
            memories: Vec::new(),
        }
    }

    fn embed(&self, input: &str) -> Result<Vec<f64>, AppError> {
        let body = EmbeddingsBody {
            model: "text-embedding-3-large".to_string(),
            input: vec![input.to_string()],
            user: None,
        };

        let embeddings = self
            .openai
            .embeddings_create(&body)
            .map_err(|_| AppError::MemoryError)?
            .data
            .as_deref()
            .ok_or(AppError::MemoryError)?[0]
            .embedding
            .clone()
            .ok_or(AppError::MemoryError)?;
        Ok(embeddings)
    }
}

impl MemoryService for Memory {
    fn search(&self, query: &str) -> Result<Vec<MemoryItem>, AppError> {
        let query_vec = self.embed(query)?;

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

    fn save(&mut self, item: MemoryCandidate) -> Result<(), AppError> {
        if item.importance > 0.6 {
            self.memories.push((
                MemoryItem {
                    content: item.summary.clone(),
                    created_at: Utc::now(),
                },
                self.embed(item.summary.as_str())?,
            ));
            println!("[INFO] ЗАПИСАНО В ПАМЯТЬ: {}", item.summary)
        }
        Ok(())
    }
}

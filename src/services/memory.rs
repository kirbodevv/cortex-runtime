use chrono::{DateTime, Utc};

use crate::{app::dto::MemoryCandidate, domain::error::AppError};

#[derive(Clone)]
pub struct MemoryItem {
    pub content: String,
    pub created_at: DateTime<Utc>,
}

pub trait MemoryService {
    fn search(&self, query: &str) -> Result<Vec<MemoryItem>, AppError>;
    fn save(&mut self, item: MemoryCandidate) -> Result<(), AppError>;
}

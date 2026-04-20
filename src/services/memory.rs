use crate::{
    app::dto::MemoryCandidate,
    domain::{AppError, MemoryItem},
};

pub trait MemoryService: Send + Sync {
    async fn search(&self, query: &str) -> Result<Vec<MemoryItem>, AppError>;
    async fn save(&mut self, item: MemoryCandidate) -> Result<(), AppError>;
}

use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct MemoryItem {
    pub content: String,
    pub created_at: DateTime<Utc>,
}

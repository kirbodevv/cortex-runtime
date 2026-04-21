#[derive(Clone)]
pub struct MemoryItem(String);

pub struct StoredMemory {
    pub item: MemoryItem,
    pub embedding: Vec<f32>,
}

impl MemoryItem {
    pub fn new(content: impl Into<String>) -> Self {
        Self(content.into())
    }

    pub fn content(&self) -> &str {
        &self.0
    }
}

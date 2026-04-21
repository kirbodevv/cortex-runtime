use crate::domain::MemoryItem;

pub struct Context(Vec<MemoryItem>);

impl Context {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn push(&mut self, item: MemoryItem) {
        self.0.push(item);
    }

    pub fn get(&self) -> &[MemoryItem] {
        &self.0
    }
}

impl From<Vec<MemoryItem>> for Context {
    fn from(value: Vec<MemoryItem>) -> Self {
        Self(value)
    }
}

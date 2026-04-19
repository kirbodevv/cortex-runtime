use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct LLMResponse {
    pub response: String,
    pub actions: Option<Vec<Action>>,
    pub memory_candidates: Vec<MemoryCandidate>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Action {
    #[serde(rename = "type")]
    pub action_type: String,
    pub args: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct MemoryCandidate {
    pub summary: String,
    pub importance: f32,
}

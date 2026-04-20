use serde::{Deserialize, Serialize};

use crate::services::module::ModuleResult;

#[derive(Deserialize, Serialize, Clone)]
pub struct LLMResponse {
    pub response: String,
    pub actions: Option<Vec<Action>>,
    pub memory_candidates: Vec<MemoryCandidate>,
}

pub struct RuntimeResponse {
    pub response: String,
    pub action_results: Vec<ModuleResult>,
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

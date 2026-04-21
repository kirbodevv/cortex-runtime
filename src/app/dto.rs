use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::services::module::ModuleResult;

#[derive(Deserialize, Serialize, Clone)]
pub struct LLMResponse {
    pub response: String,
    pub actions: Option<Vec<Action>>,
    #[serde(rename = "mem")]
    pub memory_candidates: Option<Vec<MemoryCandidate>>,
}

pub struct RuntimeResponse {
    pub response: String,
    pub action_results: Vec<ModuleResult>,
}

impl RuntimeResponse {
    pub fn new(response: String, action_results: Vec<ModuleResult>) -> Self {
        Self {
            response,
            action_results,
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Action {
    #[serde(rename = "type")]
    pub action_type: String,
    pub args: Value,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct MemoryCandidate {
    pub summary: String,
    pub importance: f32,
}

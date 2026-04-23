use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    app::tools::{ToolResponse, ToolResult},
    domain::{DomainError, LLMRawResponse},
};

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

#[derive(Deserialize, Serialize, Clone)]
pub struct LLMResponse {
    pub response: String,
    pub tool_call: Option<Vec<Action>>,
    #[serde(rename = "mem")]
    pub memory_candidates: Option<Vec<MemoryCandidate>>,
}

pub struct CoreResponse {
    pub response: String,
    pub tool_call_result: Vec<ToolResponse>,
}

pub struct ExecutorResponse {
    pub action_results: Vec<ToolResult>,
}

impl TryFrom<LLMRawResponse> for LLMResponse {
    type Error = DomainError;

    fn try_from(value: LLMRawResponse) -> Result<Self, Self::Error> {
        serde_json::from_str::<Self>(&value.text).map_err(|e| DomainError::ParseResponse(e))
    }
}

impl ExecutorResponse {
    pub fn new(action_results: Vec<ToolResult>) -> Self {
        Self { action_results }
    }
}

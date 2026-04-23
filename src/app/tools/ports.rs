use serde_json::Value;
use thiserror::Error;

use crate::app::{dto::Action, tools::ToolResult};

#[derive(Debug, Error)]
pub enum ToolError {
    #[error("Tool not found: {name}")]
    NotFound { name: String },

    #[error("Bad JSON: {0}")]
    BadJSON(String),
}

pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> Value;
    fn keywords(&self) -> &[&str];
    fn execute(&self, action: Action) -> ToolResult;
}

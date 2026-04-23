use serde_json::Value;
use thiserror::Error;

use crate::app::{dto::Action, tools::ToolResult};

#[derive(Debug, Error)]
pub enum ToolError {
    #[error("Failed to execute tool")]
    Failed,

    #[error("Tool not found: {name}")]
    NotFound { name: String },

    #[error("Bad args: {0}")]
    BadArgs(String),

    #[error("Bad JSON: {0}")]
    BadJSON(#[from] serde_json::Error),
}

#[async_trait::async_trait]
pub trait ToolProvider {
    async fn load_tools(&self) -> Vec<Box<dyn Tool>>;
}

#[async_trait::async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn args_schema(&self) -> Value;
    fn keywords(&self) -> Vec<&str>;
    async fn execute(&self, action: Action) -> ToolResult;
}

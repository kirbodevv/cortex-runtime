use serde_json::Value;

use crate::app::{dto::Action, tools::ToolResult};

pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> Value;
    fn keywords(&self) -> &[&str];
    fn execute(&self, action: Action) -> ToolResult;
}

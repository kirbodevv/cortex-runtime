use serde_json::Value;

use crate::app::{
    dto::Action,
    tools::{ToolError, ports::Tool},
};

pub type ToolResult = Result<ToolResponse, ToolError>;

pub struct ToolResponse {
    pub message: String,
}

pub struct ToolRegistry {
    tools: Vec<Box<dyn Tool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self { tools: Vec::new() }
    }

    pub fn register(&mut self, module: Box<dyn Tool>) {
        self.tools.push(module);
    }

    pub fn execute(&self, action: Action) -> ToolResult {
        let tool = self
            .tools
            .iter()
            .find(|t| t.name() == action.action_type)
            .ok_or(ToolError::NotFound)?;

        tool.execute(action)
    }

    pub fn get_modules_json_schema(&self, query: &str) -> Vec<Value> {
        let words = query.split_whitespace().collect::<Vec<_>>();
        let mut used_modules = Vec::new();
        let modules = self
            .tools
            .iter()
            .filter_map(|m| {
                if m.keywords().iter().any(|w| words.contains(w)) {
                    used_modules.push(m.name());
                    Some(m.description())
                } else {
                    None
                }
            })
            .collect();
        println!("[INFO] Использованы модули: {:?}", used_modules);
        modules
    }
}

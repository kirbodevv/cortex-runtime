use std::sync::Arc;

use crate::app::{
    dto::{Action, ExecutorResponse},
    tools::ToolRegistry,
};

pub struct Executor {
    tool_registry: Arc<ToolRegistry>,
}

impl Executor {
    pub fn new(tool_registry: Arc<ToolRegistry>) -> Self {
        Self { tool_registry }
    }

    pub async fn execute(&mut self, actions: Vec<Action>) -> ExecutorResponse {
        let action_results = actions
            .into_iter()
            .map(|action| self.tool_registry.execute(action))
            .collect();

        ExecutorResponse::new(action_results)
    }
}

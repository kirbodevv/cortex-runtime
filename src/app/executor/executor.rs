use std::sync::Arc;

use crate::app::{
    dto::{Action, ExecutorResponse},
    tools::ToolRegistry,
};

pub type RuntimeResult<T> = Result<T, RuntimeError>;

#[derive(thiserror::Error, Debug)]
pub enum RuntimeError {}

pub struct Executor {
    tool_registry: Arc<ToolRegistry>,
}

impl Executor {
    pub fn new(tool_registry: Arc<ToolRegistry>) -> Self {
        Self { tool_registry }
    }

    pub async fn execute(&mut self, actions: Vec<Action>) -> RuntimeResult<ExecutorResponse> {
        let mut action_results = vec![];

        for action in actions {
            let res = self.tool_registry.execute(action);
            action_results.push(res);
        }

        Ok(ExecutorResponse::new(action_results))
    }
}

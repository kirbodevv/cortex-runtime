use std::sync::Arc;

use crate::{
    app::dto::{LLMResponse, RuntimeResponse},
    services::module::ModuleService,
};

pub type RuntimeResult<T> = Result<T, RuntimeError>;

#[derive(thiserror::Error, Debug)]
pub enum RuntimeError {}

pub struct Runtime {
    modules: Arc<dyn ModuleService + Send + Sync>,
}

impl Runtime {
    pub fn new(modules: Arc<dyn ModuleService + Send + Sync>) -> Self {
        Self { modules }
    }

    pub async fn execute(&mut self, llm: LLMResponse) -> RuntimeResult<RuntimeResponse> {
        let mut action_results = vec![];

        if let Some(actions) = llm.actions {
            for action in actions {
                let res = self.modules.execute(action);
                action_results.push(res);
            }
        }

        Ok(RuntimeResponse::new(llm.response, action_results))
    }
}

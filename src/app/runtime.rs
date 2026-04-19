use crate::{
    app::dto::LLMResponse,
    services::module_registry::{ModuleResult, ModuleService},
};

pub type RuntimeResult<T> = Result<T, RuntimeError>;

#[derive(thiserror::Error, Debug)]
pub enum RuntimeError {}

pub struct RuntimeResponse {
    pub response: String,
    pub action_results: Vec<ModuleResult>,
}

pub struct Runtime<M>
where
    M: ModuleService,
{
    pub modules: M,
}

impl<M> Runtime<M>
where
    M: ModuleService,
{
    pub fn new(modules: M) -> Self {
        Self { modules }
    }

    pub async fn execute(&mut self, llm: LLMResponse) -> RuntimeResult<RuntimeResponse> {
        let mut action_results = vec![];

        for action in llm.actions {
            let res = self.modules.execute(action);
            action_results.push(res);
        }

        Ok(RuntimeResponse {
            response: llm.response,
            action_results,
        })
    }
}

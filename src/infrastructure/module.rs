use crate::{
    app::dto::Action,
    services::module::{ModuleResponse, ModuleResult, ModuleService},
};

pub struct Modules {}

impl Modules {
    pub fn new() -> Self {
        Self {}
    }
}

impl ModuleService for Modules {
    fn execute(&self, _: Action) -> ModuleResult {
        Ok(ModuleResponse {
            status: 0,
            message: "".to_string(),
        })
    }
}

use crate::app::dto::Action;

pub type ModuleResult = Result<ModuleResponse, ModuleError>;

#[derive(thiserror::Error, Debug)]
pub enum ModuleError {}

pub struct ModuleResponse {
    pub status: u8,
    pub message: String,
}

pub trait ModuleService {
    fn execute(&self, action: Action) -> ModuleResult;
}

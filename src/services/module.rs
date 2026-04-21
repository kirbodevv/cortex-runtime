use serde_json::Value;

use crate::app::dto::Action;

pub type ModuleResult = Result<ModuleResponse, ModuleError>;

#[derive(thiserror::Error, Debug)]
pub enum ModuleError {
    #[error("Module not found")]
    NotFound,
    #[error("JSON error: {0}")]
    JSON(String),
}

pub struct ModuleResponse {
    pub message: String,
}

pub trait Module: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> Value;
    fn keywords(&self) -> &[&str];
    fn execute(&self, action: Action) -> ModuleResult;
}

pub trait ModuleService: Send + Sync {
    fn execute(&self, action: Action) -> ModuleResult;
    fn get_modules_schema(&self, query: &str) -> Vec<Value>;
}

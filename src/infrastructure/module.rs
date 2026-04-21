use serde_json::Value;

use crate::{
    app::dto::Action,
    services::module::{Module, ModuleError, ModuleResult, ModuleService},
};

pub struct Modules {
    modules: Vec<Box<dyn Module>>,
}

impl Modules {
    pub fn new() -> Self {
        Self {
            modules: Vec::new(),
        }
    }

    pub fn register(&mut self, module: Box<dyn Module>) {
        self.modules.push(module);
    }
}

impl ModuleService for Modules {
    fn execute(&self, action: Action) -> ModuleResult {
        match self.modules.iter().find(|m| m.name() == action.action_type) {
            Some(module) => module.execute(action),
            None => Err(ModuleError::NotFound),
        }
    }

    fn get_modules_schema(&self, query: &str) -> Vec<Value> {
        let words = query.split_whitespace().collect::<Vec<_>>();
        let mut used_modules = Vec::new();
        let modules = self
            .modules
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

use crate::{domain::error::AppError, services::llm::LLMService};

pub struct Core {
    llm: Box<dyn LLMService>,
}

impl Core {
    pub fn new(llm: Box<dyn LLMService>) -> Self {
        Self { llm }
    }

    pub fn process(&mut self, input: &str) -> Result<String, AppError> {
        self.llm.process(input, vec![])
    }
}

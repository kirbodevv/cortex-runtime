use crate::{
    app::{dto::RuntimeResponse, runtime::Runtime},
    domain::error::AppError,
    services::{llm::LLMService, memory::MemoryService},
};

pub struct Core<L, ME>
where
    L: LLMService,
    ME: MemoryService,
{
    llm: L,
    memory: ME,
    runtime: Runtime,
}

impl<L, ME> Core<L, ME>
where
    L: LLMService,
    ME: MemoryService,
{
    pub fn new(llm: L, memory: ME, runtime: Runtime) -> Self {
        Self {
            llm,
            memory,
            runtime,
        }
    }

    pub async fn process(&mut self, input: &str) -> Result<RuntimeResponse, AppError> {
        let memory_ctx = self
            .memory
            .search(input)
            .await?
            .into_iter()
            .map(|m| m.content)
            .collect();

        let llm_response = self.llm.process(input, memory_ctx).await?;

        let response = self.runtime.execute(llm_response.clone()).await;

        if let Some(memory_candidates) = llm_response.memory_candidates {
            for mem in memory_candidates {
                self.memory.save(mem).await?;
            }
        }

        response.map_err(|e| AppError::RuntimeError(e.to_string()))
    }
}

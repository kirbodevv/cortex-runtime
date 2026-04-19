use crate::{
    app::runtime::Runtime,
    domain::error::AppError,
    services::{llm::LLMService, memory::MemoryService, module_registry::ModuleService},
};

pub struct Core<L, ME, MO>
where
    L: LLMService,
    ME: MemoryService,
    MO: ModuleService,
{
    llm: L,
    memory: ME,
    runtime: Runtime<MO>,
}

impl<L, ME, MO> Core<L, ME, MO>
where
    L: LLMService,
    ME: MemoryService,
    MO: ModuleService,
{
    pub fn new(llm: L, memory: ME, runtime: Runtime<MO>) -> Self {
        Self {
            llm,
            memory,
            runtime,
        }
    }

    pub async fn process(&mut self, input: &str) -> Result<String, AppError> {
        let memory_ctx = self
            .memory
            .search(input)
            .await?
            .into_iter()
            .map(|m| m.content)
            .collect();

        let llm_response = self.llm.process(input, memory_ctx).await?;

        let _ = self.runtime.execute(llm_response.clone()).await;

        for mem in llm_response.memory_candidates {
            self.memory.save(mem).await?;
        }

        Ok(llm_response.response)
    }
}

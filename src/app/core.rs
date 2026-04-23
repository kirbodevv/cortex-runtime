use std::sync::Arc;

use crate::{
    app::{
        dto::{CoreResponse, ExecutorResponse, LLMResponse},
        executor::Executor,
        memory::memory::MemoryService,
        ports::{Embedder, LLMClient, MemoryStore},
        tools::ToolRegistry,
    },
    domain::{Context, LLMRequest, Message},
    error::AppError,
};

pub struct Core<L, E, S>
where
    L: LLMClient,
    E: Embedder,
    S: MemoryStore,
{
    llm: L,
    memory: MemoryService<E, S>,
    executor: Executor,
}

impl<L, E, S> Core<L, E, S>
where
    L: LLMClient,
    E: Embedder,
    S: MemoryStore,
{
    pub fn new(llm: L, embedder: E, store: S, tool_registry: Arc<ToolRegistry>) -> Self {
        Self {
            llm,
            memory: MemoryService::new(embedder, store),
            executor: Executor::new(tool_registry),
        }
    }

    pub async fn process(&mut self, input: &str) -> Result<CoreResponse, AppError> {
        let memories = self
            .memory
            .search(input)
            .await?
            .into_iter()
            .map(|m| m.clone())
            .collect::<Vec<_>>();
        let context = Context::from(memories);
        let request = LLMRequest::new(vec![Message::user(input)], context)?;

        let raw_response = self.llm.generate(request).await?;
        let llm_response = LLMResponse::try_from(raw_response)?;

        let executor_response = if let Some(tool_calls) = llm_response.tool_call {
            self.executor.execute(tool_calls).await
        } else {
            ExecutorResponse::new(vec![])
        };

        if let Some(memory_candidates) = llm_response.memory_candidates {
            for mem in memory_candidates {
                self.memory.save(mem).await?;
            }
        }

        let tool_call_result = executor_response
            .action_results
            .into_iter()
            .filter_map(|r| r.ok())
            .collect::<Vec<_>>();

        Ok(CoreResponse {
            response: llm_response.response,
            tool_call_result,
        })
    }
}

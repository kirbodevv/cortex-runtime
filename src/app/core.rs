use std::sync::Arc;

use crate::{
    app::{
        dto::{CoreResponse, ExecutorResponse, LLMResponse},
        executor::Executor,
        memory::memory::MemoryService,
        ports::{Embedder, LLMClient, MemoryStore},
        tools::ToolRegistry,
    },
    domain::{Context, LLMRequest, Message, error::AppError},
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
            .await
            .map_err(|e| AppError::LLMError(e.to_string()))?
            .into_iter()
            .map(|m| m.clone())
            .collect::<Vec<_>>();

        let context = Context::from(memories);

        let raw_response = self
            .llm
            .generate(LLMRequest {
                messages: vec![Message::user(input)],
                context,
            })
            .await?;

        let llm_response =
            LLMResponse::try_from(raw_response).map_err(|e| AppError::LLMError(e.to_string()))?;

        let executor_response = if let Some(tool_calls) = llm_response.tool_call {
            self.executor.execute(tool_calls).await
        } else {
            Ok(ExecutorResponse::new(vec![]))
        };

        if let Some(memory_candidates) = llm_response.memory_candidates {
            for mem in memory_candidates {
                self.memory.save(mem).await?;
            }
        }

        let tool_call_result = executor_response
            .ok()
            .map(|r| r.action_results)
            .unwrap_or_default()
            .into_iter()
            .filter_map(|r| {
                if let Some(res) = r.ok() {
                    Some(res)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        Ok(CoreResponse {
            response: llm_response.response,
            tool_call_result,
        })
    }
}

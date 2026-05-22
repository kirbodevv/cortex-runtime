use std::sync::Arc;

use crate::{
    app::{
        dto::{CoreResponse, ExecutorResponse, LLMResponse},
        executor::Executor,
        llm::request_builder::LLMRequestBuilder,
        memory::memory::MemoryService,
        ports::{Embedder, LLMClient, MemoryStore},
        session::ChatSession,
        tools::ToolRegistry,
    },
    config::core::CortexConfig,
    domain::Message,
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
    session: ChatSession,
    llm_request_builder: LLMRequestBuilder,
}

impl<L, E, S> Core<L, E, S>
where
    L: LLMClient,
    E: Embedder,
    S: MemoryStore,
{
    pub fn new(
        config: Arc<CortexConfig>,
        llm: L,
        embedder: E,
        store: S,
        tool_registry: Arc<ToolRegistry>,
    ) -> Self {
        let user_prompt = match config.custom_system_prompt.clone() {
            Some(prompt) => format!("USER PROMPT: {}", prompt),
            None => "".to_string(),
        };
        Self {
            llm,
            memory: MemoryService::new(config.clone(), embedder, store),
            executor: Executor::new(tool_registry.clone()),
            session: ChatSession::new(),
            llm_request_builder: LLMRequestBuilder::new(
                format!(
                    r#"
                You MUST:
                - always give a non-empty response
                - if you call tools — always give an explanation

                Store ONLY:
                - stable user facts
                - preferences
                - long-term state

                DO NOT store:
                - greetings
                - corrections
                - meta discussion about memory
                - system behavior

                {user_prompt}
                "#,
                ),
                tool_registry.clone(),
                config.clone(),
            ),
        }
    }

    pub async fn process(&mut self, input: &str) -> Result<CoreResponse, AppError> {
        let memories = self.memory.search(input).await?;

        self.session.append(Message::user(input));

        let request = self
            .llm_request_builder
            .build(&self.session, memories, input);

        let raw_response = self.llm.generate(request).await?;
        let llm_response = LLMResponse::try_from(raw_response)?;

        self.session
            .append(Message::assistant(llm_response.response.clone()));

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

    pub async fn clear_session(&mut self) {
        self.session.clear();
    }
}

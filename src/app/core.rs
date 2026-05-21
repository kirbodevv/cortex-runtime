use std::sync::Arc;

use crate::{
    app::{
        dto::{CoreResponse, ExecutorResponse, LLMResponse},
        executor::Executor,
        json_schema::JsonSchemaGenerator,
        memory::memory::MemoryService,
        ports::{Embedder, LLMClient, MemoryStore},
        prompt::PromptBuilder,
        session::ChatSession,
        tools::ToolRegistry,
    },
    config::core::CortexConfig,
    domain::{LLMRequest, Message},
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
    prompt_builder: PromptBuilder,
    json_schema: JsonSchemaGenerator,
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
        Self {
            llm,
            memory: MemoryService::new(config.clone(), embedder, store),
            executor: Executor::new(tool_registry.clone()),
            session: ChatSession::new(),
            prompt_builder: PromptBuilder::new(
                config,
                r#"
                Store ONLY:
                - stable user facts
                - preferences
                - long-term state

                DO NOT store:
                - greetings
                - corrections
                - meta discussion about memory
                - system behavior
                "#,
            ),
            json_schema: JsonSchemaGenerator::new(tool_registry),
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

        self.session.append(Message::user(input));

        let messages = &self.prompt_builder.build(&self.session, &memories);
        let json_schema = self.json_schema.generate(input);

        let request = LLMRequest::new(messages, json_schema);

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
        println!("[INFO] Clearing session");
        self.session.clear();
    }
}

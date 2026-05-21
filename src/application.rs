use std::sync::Arc;

use genai::{
    Client,
    resolver::{AuthData, AuthResolver},
};

use crate::{
    app::{
        core::Core,
        tools::{ToolProvider, ToolRegistry},
    },
    config::core::CortexConfig,
    infrastructure::{embedder::OpenAiEmbedder, llm::OpenAIClient, memory::InMemoryStore},
};

pub async fn build(
    tool_providers: Vec<Box<dyn ToolProvider>>,
) -> Core<OpenAIClient, OpenAiEmbedder, InMemoryStore> {
    let config = Arc::new(CortexConfig::from_env());

    let key = config.openai_api_key.clone();
    let auth_resolver =
        AuthResolver::from_resolver_fn(|_| -> Result<Option<AuthData>, genai::resolver::Error> {
            Ok(Some(AuthData::from_single(key)))
        });

    let client = Client::builder().with_auth_resolver(auth_resolver).build();

    let mut tool_registry = ToolRegistry::new();

    let futures = tool_providers.iter().map(|p| p.load_tools());
    let results = futures::future::join_all(futures).await;

    for tools in results {
        for tool in tools {
            tool_registry.register(tool);
        }
    }

    let tools = Arc::new(tool_registry);

    let llm_client = OpenAIClient::new(client.clone());
    let embedder = OpenAiEmbedder::new(client.clone());
    let memory = InMemoryStore::new();

    Core::new(config, llm_client, embedder, memory, tools)
}

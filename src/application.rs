use std::sync::Arc;

use genai::{
    Client,
    resolver::{AuthData, AuthResolver},
};

use crate::{
    app::{core::Core, tools::ToolRegistry},
    infrastructure::{embedder::OpenAiEmbedder, llm::OpenAIClient, memory::InMemoryStore},
    modules::echo::EchoModule,
};

pub fn build() -> Core<OpenAIClient, OpenAiEmbedder, InMemoryStore> {
    dotenvy::dotenv().ok();

    let auth_resolver =
        AuthResolver::from_resolver_fn(|_| -> Result<Option<AuthData>, genai::resolver::Error> {
            let key = dotenvy::var("OPENAI_API_KEY").map_err(|_| {
                genai::resolver::Error::ApiKeyEnvNotFound {
                    env_name: "OPENAI_API_KEY".to_string(),
                }
            })?;
            Ok(Some(AuthData::from_single(key)))
        });

    let client = Client::builder().with_auth_resolver(auth_resolver).build();

    let mut tools = ToolRegistry::new();
    tools.register(Box::new(EchoModule));
    let tools = Arc::new(tools);

    let llm_client = OpenAIClient::new(client.clone(), tools.clone());
    let embedder = OpenAiEmbedder::new(client.clone());
    let memory = InMemoryStore::new();

    Core::new(llm_client, embedder, memory, tools)
}

use std::sync::Arc;

use genai::{
    Client,
    resolver::{AuthData, AuthResolver},
};

use crate::{
    app::{core::Core, runtime::Runtime},
    infrastructure::{memory::Memory, module::Modules, openai::OpenAi},
    modules::echo::EchoModule,
};

pub fn build() -> Core<OpenAi, Memory> {
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

    let mut modules = Modules::new();
    modules.register(Box::new(EchoModule));
    let modules = Arc::new(modules);

    Core::new(
        OpenAi::new(client.clone(), modules.clone()),
        Memory::new(client),
        Runtime::new(modules),
    )
}

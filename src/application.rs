use genai::{
    Client,
    resolver::{AuthData, AuthResolver},
};

use crate::{
    app::{core::Core, runtime::Runtime},
    infrastructure::{memory::Memory, module::Modules, openai::OpenAi},
};

pub fn build() -> Core<OpenAi, Memory, Modules> {
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

    Core::new(
        OpenAi::new(client.clone()),
        Memory::new(client),
        Runtime::new(Modules::new()),
    )
}

use crate::{
    app::{core::Core, runtime::Runtime},
    infrastructure::{memory::Memory, modules::Modules, openai::OpenAi},
};
use genai::{
    Client, ModelIden,
    resolver::{AuthData, AuthResolver},
};

mod app;
mod domain;
mod infrastructure;
mod services;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let auth_resolver = AuthResolver::from_resolver_fn(
        |model_iden: ModelIden| -> Result<Option<AuthData>, genai::resolver::Error> {
            let key = dotenvy::var("OPENAI_API_KEY").map_err(|_| {
                genai::resolver::Error::ApiKeyEnvNotFound {
                    env_name: "OPENAI_API_KEY".to_string(),
                }
            })?;
            Ok(Some(AuthData::from_single(key)))
        },
    );

    let client = Client::builder().with_auth_resolver(auth_resolver).build();

    let mut core = Core::new(
        OpenAi::new(client.clone()),
        Memory::new(client),
        Runtime::new(Modules::new()),
    );

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        match core.process(input.as_str()).await {
            Ok(res) => println!("> {res}"),
            Err(e) => println!("Error: {e:?}"),
        }
    }
}

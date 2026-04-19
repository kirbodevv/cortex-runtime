use crate::{
    app::{core::Core, runtime::Runtime},
    infrastructure::{memory::Memory, modules::Modules, openai::OpenAi},
};
use openai_api_rust::{Auth, OpenAI};

mod app;
mod domain;
mod infrastructure;
mod services;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let auth = Auth::new(dotenvy::var("OPENAI_API_KEY").unwrap().as_str());
    let openai = OpenAI::new(auth, "https://api.openai.com/v1/");
    let mut core = Core::new(
        OpenAi::new(openai.clone()),
        Memory::new(openai),
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

use crate::{app::core::Core, infrastructure::openai::OpenAi};
use openai_api_rust::Auth;

mod app;
mod domain;
mod infrastructure;
mod services;

fn main() {
    dotenvy::dotenv().ok();

    let auth = Auth::new(dotenvy::var("OPENAI_API_KEY").unwrap().as_str());
    let mut core = Core::new(Box::new(OpenAi::new(auth)));

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        match core.process(input.as_str()) {
            Ok(res) => println!("{res}"),
            Err(e) => println!("Error: {e:?}"),
        }
    }
}

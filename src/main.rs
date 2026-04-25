use crate::{
    app::tools::ToolProvider,
    infrastructure::tools::{external::ExternalToolProvider, internal::InternalToolProvider},
};
use clap::Parser;
use std::path::PathBuf;

mod app;
mod application;
mod domain;
mod error;
mod infrastructure;
mod shared;
mod tools;

#[derive(Parser)]
struct Args {
    #[arg(long = "tools")]
    tools: Vec<PathBuf>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let tool_providers = {
        let mut providers: Vec<Box<dyn ToolProvider>> = vec![Box::new(InternalToolProvider)];
        for path in args.tools {
            providers.push(Box::new(ExternalToolProvider { dir: path }));
        }
        providers
    };

    let mut core = application::build(tool_providers).await;

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "clear" {
            core.clear_session().await;
            continue;
        }

        match core.process(input.as_str()).await {
            Ok(res) => {
                println!(
                    "> {}\n\n Modules result: {}",
                    res.response,
                    res.tool_call_result
                        .into_iter()
                        .map(|r| r.message)
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            Err(e) => println!("Error: {e:?}"),
        }
    }
}

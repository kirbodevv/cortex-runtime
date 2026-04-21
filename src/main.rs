use crate::services::module::ModuleResponse;

mod app;
mod application;
mod domain;
mod infrastructure;
mod modules;
mod services;

#[tokio::main]
async fn main() {
    let mut core = application::build();

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        match core.process(input.as_str()).await {
            Ok(res) => {
                println!(
                    "> {} Modules res: {}",
                    res.response,
                    res.action_results
                        .into_iter()
                        .map(|r| r
                            .unwrap_or(ModuleResponse {
                                message: "module error".to_string()
                            })
                            .message)
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            Err(e) => println!("Error: {e:?}"),
        }
    }
}

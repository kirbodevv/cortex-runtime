mod app;
mod application;
mod domain;
mod infrastructure;
mod tools;

#[tokio::main]
async fn main() {
    let mut core = application::build();

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

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

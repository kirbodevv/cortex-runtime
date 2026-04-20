mod app;
mod application;
mod domain;
mod infrastructure;
mod services;

#[tokio::main]
async fn main() {
    let mut core = application::build();

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        match core.process(input.as_str()).await {
            Ok(res) => println!("> {res}"),
            Err(e) => println!("Error: {e:?}"),
        }
    }
}

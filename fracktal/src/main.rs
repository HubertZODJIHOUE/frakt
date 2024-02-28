mod client;
mod server;


#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        let result = match args[1].as_str() {
            "server" => server::run().await,
            "client" => client::run().await,
            _ => {
                println!("Usage: cargo run -- [server|client]");
                Ok(()) // Changez cette ligne pour retourner `Ok(())`
            }
        };

        if let Err(e) = result {
            eprintln!("Error: {}", e);
        }
    } else {
        println!("Usage: cargo run -- [server|client]");
    }
}

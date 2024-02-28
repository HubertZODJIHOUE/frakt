// src/client.rs

use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use serde_json::json;
use std::error::Error;

pub async fn run() -> Result<(), Box<dyn Error>> {
    match TcpStream::connect("127.0.0.1:8080").await {
        Ok(mut stream) => {
            let task = json!({
                "id": 1,
                "description": "Task description"
            });

            let message = serde_json::to_vec(&task).unwrap();
            stream.write_all(&message).await?;
            println!("Task sent");
        },
        Err(e) => println!("Failed to connect: {}", e),
    }
    Ok(())
}

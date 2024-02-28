// src/server.rs

use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde::{Serialize, Deserialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    description: String,
}

pub async fn run() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    println!("Server running on port 8080");

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = vec![0; 1024]; // Initialiser le tampon pour la lecture

            match socket.read(&mut buf).await {
                Ok(nbytes) => {
                    // Réduire la taille du tampon pour correspondre au nombre exact d'octets lus
                    buf.truncate(nbytes);

                    // Tenter de désérialiser les données JSON à partir du tampon ajusté
                    let task: Task = match serde_json::from_slice(&buf) {
                        Ok(task) => task,
                        Err(e) => {
                            println!("Failed to parse task; err = {:?}", e);
                            return;
                        }
                    };
                    println!("Received: {:?}", task);

                    if let Err(e) = socket.write_all(b"Task processed").await {
                        println!("Failed to write to socket; err = {:?}", e);
                    }
                },
                Err(e) => println!("Failed to read from socket; err = {:?}", e),
            }
        });
    }
}

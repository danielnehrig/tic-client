use serde::{Deserialize, Serialize};
use std::io::{self, BufRead};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

#[derive(Serialize, Deserialize, Debug, Default)]
struct Credentials {
    username: String,
    password: String,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // get input handler
    // save stdin to buffer
    let mut username = String::new();
    let mut password = String::new();
    let stdin = io::stdin();
    println!("Enter username: ");
    stdin.lock().read_line(&mut username).unwrap();
    println!("Enter password: ");
    stdin.lock().read_line(&mut password).unwrap();

    let creds = Credentials { username, password };
    let connection = TcpStream::connect("127.0.0.1:3333").await;
    match connection {
        Ok(mut socket) => {
            let cred = bincode::serialize(&creds).unwrap();
            socket.write(&cred).await.unwrap();
        }
        Err(err) => {
            eprintln!("{:?}", err);
        }
    }

    Ok(())
}

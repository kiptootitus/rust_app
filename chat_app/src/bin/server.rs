#![allow(warnings)]

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    sync::broadcast,
};

use chrono::Local;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChatMessage {
    username: String,
    content: String,
    timestamp: String,
    message_type: MessageType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum MessageType {
    UserMessage,
    SystemNotification,
}

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    println!();
    println!("┌──────────────────────────────────────────────┐");
    println!("│    SUDO CHAT SERVER ACTIVE                   │");
    println!("│    Port: 8080   Host: 127.0.0.1              │");
    println!("│    Press Ctrl+C to shutdown                  │");
    println!("└──────────────────────────────────────────────┘");

    let (tx, _) = broadcast::channel::<String>(100);

    loop {
        let (socket, addr) = listener.accept().await?;

        println!("├── [{}] New connection", Local::now().format("%H:%M:%S"));
        println!("└── Address: {}", addr);

        let tx = tx.clone();
        let rx = tx.subscribe();

        tokio::spawn(async move {
            handle_connection(socket, tx, rx).await;
        });
    }
}

async fn handle_connection(
    mut socket: TcpStream,
    tx: broadcast::Sender<String>,
    mut rx: broadcast::Receiver<String>,
) {
    let (reader, mut writer) = socket.split();
    let mut reader = BufReader::new(reader);
    let mut username = String::new();

    // Read username
    if reader.read_line(&mut username).await.unwrap() == 0 {
        return;
    }

    let username = username.trim().to_string();

    let join_not = ChatMessage {
        username: username.clone(),
        content: "User has joined the chat".to_string(),
        timestamp: Local::now().format("%d/%m/%Y %H:%M").to_string(),
        message_type: MessageType::SystemNotification,
    };

    let join_json = serde_json::to_string(&join_not).unwrap();
    tx.send(join_json).unwrap();

    let mut line = String::new();
    loop {
        tokio::select! {
            result = reader.read_line(&mut line) => {
                if result.unwrap() == 0 {
                    break;
                }

                let msg = ChatMessage {
                    username: username.clone(),
                    content: line.trim().to_string(),
                    timestamp: Local::now().format("%d/%m/%Y %H:%M").to_string(),
                    message_type: MessageType::UserMessage,
                };

                let json = serde_json::to_string(&msg).unwrap();
                tx.send(json).unwrap();
                line.clear();
            }

            result = rx.recv() => {
                let msg = result.unwrap();
                writer.write_all(msg.as_bytes()).await.unwrap();
                writer.write_all(b"\n").await.unwrap();
            }
        }
    }

    // Optionally: Send a leave message
    let leave_msg = ChatMessage {
        username: username.clone(),
        content: "left the chat.".to_string(),
        timestamp: Local::now().format("%H:%M:%S").to_string(),
        message_type: MessageType::SystemNotification,
    };

    let leave_json = serde_json::to_string(&leave_msg).unwrap();
    tx.send(leave_json).unwrap();

    // log disconnection info to the terminal
    println!("├── [{}] {} Logging out...", Local::now().format("%H:%M:%S"), username);
    println!("└── Goodbye 👋");

}

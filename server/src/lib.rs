use crate::handlers::handle_request;
use crate::protocol::{Message, MessageType};
use crate::state::ServerInfo;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tracing::{Instrument, error, info};

pub mod error;
mod handlers;
pub mod protocol;
pub mod state;

fn parse_command(line: &str) -> Message {
    let mut parts = line.split_whitespace();
    let command_name = parts.next().unwrap_or("").to_string();
    let args: Vec<String> = parts.map(String::from).collect();

    Message {
        message: MessageType::COMMAND,
        command_line: line.to_string(),
        command_name,
        args,
        ..Message::default()
    }
}

pub async fn run(addr: String, port: String) -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let server_info: Arc<Mutex<ServerInfo>> = Arc::new(Mutex::new(ServerInfo::new()));
    let listener = TcpListener::bind(format!("{}:{}", addr, port)).await?;

    loop {
        let (mut socket, peer_addr) = listener.accept().await?;
        let span = tracing::info_span!("connection", %peer_addr);

        let server_info_copy = Arc::clone(&server_info);

        tokio::spawn(
            async move {
                if let Err(e) = socket
                    .write_all("Server → Client: OK hello proto=1\n".as_bytes())
                    .await
                {
                    error!(error = %e, "TCP connection failed");
                    return Err(e);
                }

                info!("TCP connection established");
                let (read_half, mut write_half) = socket.into_split();
                let mut reader = BufReader::new(read_half);
                let mut line = String::new();

                loop {
                    line.clear();
                    let n = reader.read_line(&mut line).await?;
                    if n == 0 {
                        break;
                    }
                    let request = parse_command(line.as_str()); // DEV TEST
                    // let request = Message::parse(line); // PROD
                    let response = handle_request(request, &server_info_copy);
                    let _ = write_half.write_all(response.to_str().as_bytes()).await;
                }

                Ok::<(), std::io::Error>(())
            }
            .instrument(span),
        );
    }
}

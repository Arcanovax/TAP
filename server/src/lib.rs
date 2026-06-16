use crate::config::load;
use crate::handlers::handle_request::handle_request;
use crate::protocol::Message;
use crate::state::{ServerInfo, SharedServer};
use std::net::SocketAddr;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tracing::{Instrument, error, info};

mod command;
mod config;
mod error;
mod game;
mod group;
mod handlers;
pub mod protocol;
mod state;
mod structures;
#[cfg(test)]
mod test_utils;

fn parse_command(line: &str) -> Message {
    let mut parts = line.split_whitespace();
    let command_name = parts.next().unwrap_or("").to_string();
    let args: Vec<String> = parts.map(String::from).collect();

    Message::Command {
        name: command_name,
        args,
    }
}

fn cleanup_tcp_connection(server_info: &SharedServer, peer_addr: SocketAddr) {
    let _ = server_info.lock().unwrap().try_leave_group(peer_addr);
    match server_info.lock().unwrap().try_remove_player(peer_addr) {
        Ok(name) => info!("{} disconnected", name),
        Err(_) => {}
    }
    server_info.lock().unwrap().send_players_event(peer_addr);
    info!("TCP connection closed");
}

pub async fn run(addr: String, port: String) -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let world = load(Path::new("config.yaml"))?;
    let server_info: SharedServer = Arc::new(Mutex::new(ServerInfo::new(world)));
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

                let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<Message>();

                loop {
                    tokio::select! {
                        result = reader.read_line(&mut line) => {
                            match result {
                                Ok(0) => break,
                                Ok(_) => {}
                                Err(_) => break,
                            }
                            let request = parse_command(line.as_str()); // DEV TEST
                            // let request = Message::parse(line)?; // PROD
                            let response = handle_request(&request, &server_info_copy, peer_addr, &tx);
                            let _ = write_half.write_all(response.to_str().as_bytes()).await;
                            line.clear();
                            if let Message::Command { name, .. } = &request {
                                if name.to_uppercase() == "QUIT" {
                                    break;
                                }
                            }
                        }
                        Some(event) = rx.recv() => {
                            let _ = write_half.write_all(event.to_str().as_bytes()).await;
                        }
                    }
                }

                cleanup_tcp_connection(&server_info_copy, peer_addr);
                Ok::<(), std::io::Error>(())
            }
            .instrument(span),
        );
    }
}

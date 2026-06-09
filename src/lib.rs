mod structures;
mod command;
pub mod error;
mod group;
mod handlers;
pub mod protocol;
pub mod state;

use crate::handlers::handle_request::handle_request;
use crate::protocol::{EventType, Message};
use crate::state::{ServerInfo, SharedServer};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use std::fs::File;
use tracing::{Instrument, error, info};


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
    info!("TCP connection closed");
}

pub async fn run(addr: String, port: String) -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

	let mut prev_server_info: ServerInfo = ServerInfo::new();

	for file_path in ["rooms.yaml", "npc.yaml", "items.yaml"] {
		let f = File::open(file_path)?;
		match file_path {
			"rooms.yaml" => prev_server_info.rooms = serde_yaml::from_reader(f)?,
			"npc.yaml" => prev_server_info.npcs = serde_yaml::from_reader(f)?,
			"items.yaml" => prev_server_info.items = serde_yaml::from_reader(f)?,
			_ => {}
		}	
	};

    let server_info: SharedServer = Arc::new(Mutex::new(prev_server_info));
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
                            // let request = Message::parse(line); // PROD
                            if let Message::Command { name, .. } = &request {
                                if name.to_uppercase() == "QUIT" {
                                    break;
                                }
                            }
                            let response = handle_request(request, &server_info_copy, peer_addr, &tx);
                            let _ = write_half.write_all(response.to_str().as_bytes()).await;
                            line.clear();
                        }
                        Some(event) = rx.recv() => {
                            if let Message::Event { kind, data } = event {
                                if kind == EventType::CHAT {
                                    let _ = write_half.write_all(data.as_bytes()).await;
                                }
                                else if kind == EventType::INVITE {
                                    let _ = write_half.write_all(data.as_bytes()).await;
                                }
                            }
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

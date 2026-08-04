use crate::config::load;
use crate::handlers::handle_request::handle_request;
use crate::persistence::world::{load_world, save_world};
use crate::protocol::{Message, Payload};
use crate::state::{ServerInfo, SharedServer};
use crate::structures::enums::error::ErrorCode;
use crate::structures::room::Owner;
use redb::Database;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::net::tcp::OwnedWriteHalf;
use tokio::time::interval;
use tracing::{Instrument, debug, error, info, warn};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[cfg(unix)]
use tokio::signal::unix::{signal, SignalKind};

mod config;
mod dungeon;
mod handlers;
mod persistence;
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

async fn shutdown_signal() {
    #[cfg(unix)]
    {
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {}
            _ = signal(SignalKind::terminate()).expect("failed to install SIGTERM handler") => {}
        }
    }

    #[cfg(not(unix))]
    {
        let _ = tokio::signal::ctrl_c().await;
    }
}

async fn cleanup_tcp_connection(
    server_info: &SharedServer,
    peer_addr: SocketAddr,
    mut write_half: OwnedWriteHalf,
) {
    let result = {
        let mut binding = server_info.lock().unwrap();
        binding.disconnect_player(peer_addr)
    };

    if let Err(code) = result {
        let _ = write_half
            .write_all(
                Message::Response {
                    error: code,
                    payload: Payload::Empty,
                }
                .to_str()
                .as_bytes(),
            )
            .await;
    }

    debug!("TCP connection closed");
}

pub async fn run(
    addr: String,
    port: String,
    config: PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let (file_writer, _guard) =
        tracing_appender::non_blocking(tracing_appender::rolling::daily("logs", "tap.log"));

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .with(tracing_subscriber::fmt::layer())
        .with(
            tracing_subscriber::fmt::layer()
                .json()
                .with_writer(file_writer),
        )
        .init();

    let db = Arc::new(Database::create("game.redb")?);
    let mut world = load(&config)?;
    let base_world = world.clone();

    if let Ok(Some(saved)) = load_world(&db) {
        for (id, room) in &mut world.rooms {
            if let Some(saved_room) = saved.rooms.get(id) {
                room.items.extend(
                    saved_room
                        .items
                        .iter()
                        .filter(|item| {
                            item.owner == Owner::Player && world.items.contains_key(&item.item)
                        })
                        .cloned(),
                );
            }
        }
    }

    let server_info: SharedServer =
        Arc::new(Mutex::new(ServerInfo::new(world.clone(), db.clone())));
    let listener = TcpListener::bind(format!("{}:{}", addr, port)).await?;

    let server_info_copy = Arc::clone(&server_info);
    let db_copy = Arc::clone(&db);
    let mut ticker = interval(Duration::from_secs(600));
    tokio::spawn(async move {
        loop {
            ticker.tick().await;
            let mut binding = server_info_copy.lock().unwrap();
            debug!("Server reset started");
            binding.reset(&base_world);
            debug!("Server reset done");

            debug!("Saving world...");
            let _ = save_world(&db_copy, &binding.world);
            debug!("World saved");
        }
    });

    loop {
        tokio::select! {
            res = listener.accept() => {
            let (mut socket, peer_addr) = res?;
            let span = tracing::info_span!("connection", %peer_addr, player = tracing::field::Empty);

            let server_info_copy = Arc::clone(&server_info);

            tokio::spawn(
                async move {
                    if let Err(e) = socket
                        .write_all("OK hello proto=1\n".as_bytes())
                        .await
                    {
                        error!(error = %e, "TCP connection failed");
                        return Err(e);
                    }

                    debug!("TCP connection established");
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
                                let request = parse_command(line.as_str());
                                if let Message::Command { name, args } = &request {
                                     info!(command = %name, params = ?args, "command received");
                                }
                                let response = handle_request(&request, &server_info_copy, peer_addr, &tx);
                                match &response {
                                    Message::Response { error: ErrorCode::SUCCESS, .. } => {
                                        info!(code = 0, "response sent");
                                    }
                                    Message::Response { error, .. } if error.code() >= 900 => {
                                        warn!(code = error.code(), error = %error.name(), "response sent");
                                    }
                                    Message::Response { error, .. } => {
                                        info!(code = error.code(), error = %error.name(), "response sent");
                                    }
                                    _ => {}
                                };
                                let _ = write_half.write_all(response.to_str().as_bytes()).await;
                                line.clear();
                                if let Message::Command { name, .. } = &request
                                    && name.to_uppercase() == "QUIT" {
                                        break;
                                    }
                            }
                            Some(event) = rx.recv() => {
                                let _ = write_half.write_all(event.to_str().as_bytes()).await;
                            }
                        }
                    }

                    cleanup_tcp_connection(&server_info_copy, peer_addr, write_half).await;
                    Ok::<(), std::io::Error>(())
                }
                .instrument(span),
            );
        }
        _ = shutdown_signal() => {
            server_shutdown(&db, &server_info);
            break Ok(());
        }
        }
    }
}

fn server_shutdown(db: &Database, server_info: &SharedServer) {
    debug!("Server closing...");
    let mut binding = server_info.lock().unwrap();

    debug!("Saving world...");
    let _ = save_world(db, &binding.world);
    debug!("World saved");

    debug!("Saving players...");
    let addrs: Vec<SocketAddr> = binding.connections.keys().copied().collect();
    for peer_addr in addrs {
        let _ = binding.disconnect_player(peer_addr);
    }
    debug!("Players saved");
    debug!("Server is shuting down now");
}

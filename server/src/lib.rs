use crate::config::load;
use crate::handlers::handle_request::handle_request;
use crate::persistence::players::save_player;
use crate::persistence::world::{load_world, save_world};
use crate::protocol::{Message, Payload};
use crate::state::{ServerInfo, SharedServer};
use crate::structures::enums::state::State;
use crate::structures::room::Owner;
use redb::Database;
use std::net::SocketAddr;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::net::tcp::OwnedWriteHalf;
use tokio::signal::unix::{SignalKind, signal};
use tokio::time::interval;
use tracing::{Instrument, debug, error, info};

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

fn cleanup_tcp_connection(
    server_info: &SharedServer,
    peer_addr: SocketAddr,
    mut write_half: OwnedWriteHalf,
) {
    let mut binding = server_info.lock().unwrap();
    let _ = binding.try_leave_group(peer_addr);

    let player_res = binding.get_player(peer_addr).cloned();

    match player_res {
        Ok(player) => match player.status {
            State::InFight { target_id } => {
                let _ = binding.try_leave_fight(peer_addr, target_id.clone());
            }
            _ => {}
        },
        Err(_code) => {}
    }

    match binding.try_save_player(peer_addr) {
        Ok(()) => {
            info!("Player info saved");
        }
        Err(code) => {
            let _ = write_half.write_all(
                Message::Response {
                    error: code,
                    payload: Payload::Empty,
                }
                .to_str()
                .as_bytes(),
            );
            return;
        }
    }

    match binding.try_remove_player(peer_addr) {
        Ok(name) => info!("{} disconnected", name),
        Err(_) => {}
    }
    binding.send_players_event(peer_addr);
    info!("TCP connection closed");
}

pub async fn run(addr: String, port: String) -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("debug")),
        )
        .init();

    let mut sigint = signal(SignalKind::interrupt())?;
    let mut sigterm = signal(SignalKind::terminate())?;

    let db = Arc::new(Database::create("game.redb")?);
    let mut world = load(Path::new("config.yaml"))?;
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

            info!("Saving world...");
            let _ = save_world(&db_copy, &binding.world);
            info!("World saved");
        }
    });

    loop {
        tokio::select! {
            res = listener.accept() => {
            let (mut socket, peer_addr) = res?;
            let span = tracing::info_span!("connection", %peer_addr);

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
                                let request = parse_command(line.as_str());
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

                    cleanup_tcp_connection(&server_info_copy, peer_addr, write_half);
                    Ok::<(), std::io::Error>(())
                }
                .instrument(span),
            );
        }
        _ = sigint.recv() => {
            server_shutdown(&db, &server_info);
            break Ok(());
        }
        _ = sigterm.recv() => {
            server_shutdown(&db, &server_info);
            break Ok(());
        }
        }
    }
}

fn server_shutdown(db: &Database, server_info: &SharedServer) {
    info!("Server closing...");
    let binding = server_info.lock().unwrap();

    info!("Saving world...");
    let _ = save_world(db, &binding.world);
    info!("World saved");

    info!("Saving players...");
    for (_, con) in &binding.connections {
        let _ = save_player(db, &con.player);
    }
    info!("Players saved");
    info!("Server is shuting down now");
}

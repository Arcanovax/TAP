use tracing::info;

use crate::protocol::{EventType, Message, Payload};
use crate::state::{SharedServer, Tx};
use crate::structures::enums::error::ErrorCode;
use std::net::SocketAddr;

#[cfg(test)]
mod tests;

pub(super) fn connect_request(
    args: &Vec<String>,
    server_info: &SharedServer,
    peer_addr: SocketAddr,
    tx: &Tx,
) -> Message {
    if args.len() != 1 {
        return Message::Response {
            error: ErrorCode::INVALID_ARGS,
            payload: Payload::Empty,
        };
    }
    let name = args[0].trim();
    let res = server_info
        .lock()
        .unwrap()
        .try_add_player(name.to_string(), peer_addr, tx);

    if res.is_ok() {
        tracing::Span::current().record("player", name);
        if let Ok(receivers) = server_info.lock().unwrap().get_room_receivers(peer_addr) {
            for con in receivers {
                let _ = con.tx.send(Message::Event(EventType::ROOM_JOIN {
                    player_name: name.to_string(),
                }));
            }
        }
        server_info.lock().unwrap().send_players_event(peer_addr);
        info!("{} connected", name);
    }
    res.into()
}

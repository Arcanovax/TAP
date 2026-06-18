use tracing::info;

use crate::protocol::{EventType, Message};
use crate::state::{SharedServer, Tx};
use crate::structures::enums::error::ErrorCode;
use std::net::SocketAddr;

#[cfg(test)]
mod tests;

// #[derive(Serialize)]
// struct LoginResponse<'a> {
// 	player: &'a Player,
// 	room: &'a Room
// }

pub(super) fn connect_request(
    args: &Vec<String>,
    server_info: &SharedServer,
    peer_addr: SocketAddr,
    tx: &Tx,
) -> Message {
    if args.len() != 1 {
        return Message::Response {
            error: ErrorCode::INVALID_ARGS,
            data: None,
        };
    }
    let name = args[0].to_string();
    let res = server_info
        .lock()
        .unwrap()
        .try_add_player(name.clone(), peer_addr, tx);

    if res.is_ok() {
        if let Ok(receivers) = server_info.lock().unwrap().get_room_receivers(peer_addr) {
            for con in receivers {
                let _ = con.tx.send(Message::Event(EventType::ROOM_JOIN {
                    player_name: name.clone(),
                }));
            }
        }
        server_info.lock().unwrap().send_players_event(peer_addr);
    }
    info!("{} connected", name);
    res.into()
}

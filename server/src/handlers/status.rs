use serde::Serialize;

use crate::error::ErrorCode;
use crate::protocol::Message;
use crate::state::SharedServer;
use crate::structures::enums::state::State;
use std::net::SocketAddr;

#[cfg(test)]
mod tests;

#[derive(Serialize)]
struct StatusView<'a> {
    hp: &'a u32,
    max_hp: &'a u32,
    status: &'a State,
}

pub(super) fn status_request(server_info: &SharedServer, peer_addr: SocketAddr) -> Message {
    match server_info.lock().unwrap().get_player(peer_addr) {
        Ok(player) => Message::Response {
            error: ErrorCode::SUCCESS,
            data: Some(
                serde_json::to_value(StatusView {
                    hp: &player.hp,
                    max_hp: &player.max_hp,
                    status: &player.status,
                })
                .unwrap(),
            ),
        },
        Err(code) => Message::Response {
            error: code,
            data: None,
        },
    }
}

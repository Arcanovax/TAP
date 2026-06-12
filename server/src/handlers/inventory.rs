use crate::{error::ErrorCode, protocol::Message, state::SharedServer};
use std::net::SocketAddr;

pub fn inventory_request(server_info: &SharedServer, peer_addr: SocketAddr) -> Message {
    match server_info.lock().unwrap().get_player(peer_addr) {
        Ok(player) => {
            return Message::Response {
                error: ErrorCode::SUCCESS,
                data: Some(serde_json::to_string(&player.inventory).unwrap()),
            };
        }
        Err(code) => {
            return Message::Response {
                error: code,
                data: None,
            };
        }
    }
}

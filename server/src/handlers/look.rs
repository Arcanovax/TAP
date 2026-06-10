use crate::{error::ErrorCode::SUCCESS, protocol::Message, state::SharedServer};
use std::net::SocketAddr;

pub fn look_request(server_info: &SharedServer, peer_addr: SocketAddr) -> Message {
    let binding = server_info.lock().unwrap();
    let room = match binding.get_player_room(peer_addr) {
        Ok(room) => room,
        Err(code) => {
            return Message::Response {
                error: code,
                data: None,
            };
        }
    };
    Message::Response {
        error: SUCCESS,
        data: Some(serde_json::to_string(room).unwrap()),
    }
}

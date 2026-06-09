use crate::error::ErrorCode;
use crate::protocol::Message;
use crate::state::SharedServer;
use std::net::SocketAddr;

pub(super) fn status_request(server_info: &SharedServer, peer_addr: SocketAddr) -> Message {
    match server_info.lock().unwrap().get_player(peer_addr) {
        Ok(player) => Message::Response {
            error: ErrorCode::SUCCESS,
            data: Some(serde_json::to_string(player).unwrap()),
        },
        Err(code) => Message::Response {
            error: code,
            data: None,
        },
    }
}

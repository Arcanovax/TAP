use crate::protocol::{Message, Payload};
use crate::state::SharedServer;
use crate::structures::enums::error::ErrorCode;
use std::collections::HashMap;
use std::net::SocketAddr;

#[cfg(test)]
mod tests;

pub(super) fn gold_request(server_info: &SharedServer, peer_addr: SocketAddr) -> Message {
    let gold = match server_info.lock().unwrap().get_player(peer_addr) {
        Ok(player) => player.gold,
        Err(code) => {
            return Message::Response {
                error: code,
                payload: Payload::Empty,
            };
        }
    };
    Message::Response {
        error: ErrorCode::SUCCESS,
        payload: Payload::Pair(HashMap::from([("gold".to_string(), gold.to_string())])),
    }
}

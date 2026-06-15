use crate::{error::ErrorCode, protocol::Message, state::SharedServer};
use std::net::SocketAddr;

#[cfg(test)]
mod tests;

pub fn drop_request(
    server_info: &SharedServer,
    peer_addr: SocketAddr,
    args: &Vec<String>,
) -> Message {
    let mut binding = server_info.lock().unwrap();
    if !binding.is_connected(peer_addr) {
        return Message::Response {
            error: ErrorCode::INVALID_COMMAND,
            data: None,
        };
    }
    if args.len() == 0 {
        return Message::Response {
            error: ErrorCode::INVALID_ARGS,
            data: None,
        };
    }
    let mut dropped: Vec<String> = Vec::new();
    for item in args {
        if let Ok(item) = binding.try_drop_item(peer_addr, item) {
            dropped.push(item);
        }
    }
    Message::Response {
        error: ErrorCode::SUCCESS,
        data: Some(serde_json::to_string(&dropped).unwrap()),
    }
}

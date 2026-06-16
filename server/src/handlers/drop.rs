use serde_json::json;

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
    match binding.try_drop_item(peer_addr, &args.join(" ")) {
        Ok(item) => Message::Response {
            error: ErrorCode::SUCCESS,
            data: Some(json!({ "dropped": item })),
        },
        Err(code) => Message::Response {
            error: code,
            data: None,
        },
    }
}

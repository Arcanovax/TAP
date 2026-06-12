use crate::{error::ErrorCode, protocol::Message, state::SharedServer};
use std::net::SocketAddr;

pub(super) fn quest_request(
    args: Vec<String>,
    server_info: &SharedServer,
    peer_addr: SocketAddr,
) -> Message {
    if !server_info.lock().unwrap().is_connected(peer_addr) {
        return Message::Response {
            error: ErrorCode::INVALID_COMMAND,
            data: None,
        };
    }
    if args.len() != 1 {
        return Message::Response {
            error: ErrorCode::INVALID_ARGS,
            data: None,
        };
    }

    // Il faut accepter la quete ici
    match server_info
        .lock()
        .unwrap()
        .try_accept_quest(peer_addr, &args[0])
    {
        Ok(quest) => {
            return Message::Response {
                error: ErrorCode::SUCCESS,
                data: Some(serde_json::to_string(quest).unwrap()),
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

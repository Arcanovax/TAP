use crate::{
    protocol::{Message, Payload},
    state::SharedServer,
    structures::enums::error::ErrorCode,
};
use std::net::SocketAddr;

#[cfg(test)]
mod tests;

pub(super) fn dungeon_request(
    args: &Vec<String>,
    server_info: &SharedServer,
    peer_addr: SocketAddr,
) -> Message {
    if args.len() != 1 {
        return Message::Response {
            error: ErrorCode::INVALID_COMMAND,
            payload: Payload::Empty,
        };
    }

    match args[0].to_uppercase().as_str() {
        "CREATE" => dungeon_create_request(server_info, peer_addr),
        "JOIN" => dungeon_join_request(server_info, peer_addr),
        _ => {
            return Message::Response {
                error: ErrorCode::INVALID_ARGS,
                payload: Payload::Empty,
            };
        }
    }
}

fn dungeon_create_request(server_info: &SharedServer, peer_addr: SocketAddr) -> Message {
    server_info
        .lock()
        .unwrap()
        .try_create_dungeon(peer_addr)
        .into()
}

fn dungeon_join_request(server_info: &SharedServer, peer_addr: SocketAddr) -> Message {
    Message::Response {
        error: ErrorCode::SUCCESS,
        payload: Payload::Empty,
    }
}

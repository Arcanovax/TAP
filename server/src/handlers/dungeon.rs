use std::net::SocketAddr;

use uuid::Uuid;

use crate::{
    dungeon::generation::generate_dungeon,
    protocol::{Message, Payload},
    state::SharedServer,
    structures::enums::error::ErrorCode,
};

#[cfg(test)]
mod tests;

pub(super) fn dungeon_request(
    args: &Vec<String>,
    server_info: &SharedServer,
    peer_addr: SocketAddr,
) -> Message {
    let dungeon = generate_dungeon(&server_info.lock().unwrap().world, Uuid::new_v4());

    println!("{:#?}", dungeon.rooms);

    Message::Response {
        error: ErrorCode::SUCCESS,
        payload: Payload::Empty,
    }
}

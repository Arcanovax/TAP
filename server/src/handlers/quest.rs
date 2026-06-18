use tracing::info;

use crate::{protocol::Message, state::SharedServer, structures::enums::error::ErrorCode};
use std::net::SocketAddr;

#[cfg(test)]
mod tests;

pub(super) fn quest_request(
    args: &Vec<String>,
    server_info: &SharedServer,
    peer_addr: SocketAddr,
) -> Message {
    if !server_info.lock().unwrap().is_connected(peer_addr) {
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

    let mut binding = server_info.lock().unwrap();
    let mut npc_ref = args.join(" ");
    if let Some(reference) = binding.world.name_to_ref.get(&npc_ref.to_lowercase()) {
        npc_ref = reference.clone();
    }
    let player_room = binding.get_player_room(peer_addr).unwrap();
    if !player_room.npc.iter().any(|npc| *npc == npc_ref) {
        return Message::Response {
            error: ErrorCode::NPC_NOT_FOUND,
            data: None,
        }
        .into();
    }
    let res = binding.try_accept_quest(peer_addr, &npc_ref);
    if let Ok(_) = &res {
        info!("Accepted {} quest", npc_ref);
    };
    res.into()
}

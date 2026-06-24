use serde_json::json;
use tracing::info;

use crate::{
    protocol::{Message, Payload},
    state::SharedServer,
    structures::{enums::error::ErrorCode, quest::Goal},
};
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
            payload: Payload::Empty,
        };
    }
    if args.len() == 0 {
        return Message::Response {
            error: ErrorCode::INVALID_ARGS,
            payload: Payload::Empty,
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
            payload: Payload::Empty,
        }
        .into();
    }
    match binding.try_accept_quest(peer_addr, &npc_ref) {
        Ok(quest) => {
            info!("Accepted {} quest", npc_ref);
            Message::Response {
                error: ErrorCode::SUCCESS,
                payload: Payload::Json(json!({
                    "quest_id": &quest.name,
                    "description": &<Goal as Into<String>>::into(quest.goals[0].clone()),
                    "reward": &quest.reward,
                    "status": "accepted"
                })),
            }
        }
        Err(code) => Message::Response {
            error: code,
            payload: Payload::Empty,
        },
    }
}

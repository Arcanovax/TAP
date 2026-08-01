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
    args: &[String],
    server_info: &SharedServer,
    peer_addr: SocketAddr,
) -> Message {
    if !server_info.lock().unwrap().is_connected(peer_addr) {
        return Message::Response {
            error: ErrorCode::INVALID_COMMAND,
            payload: Payload::Empty,
        };
    }
    if args.len() != 1 {
        return Message::Response {
            error: ErrorCode::INVALID_ARGS,
            payload: Payload::Empty,
        };
    }

    let mut binding = server_info.lock().unwrap();
    let npc_ref = &args[0];

    let player_room = binding.get_player_room(peer_addr).unwrap();
    if !player_room.npc.iter().any(|npc| npc == npc_ref) {
        return Message::Response {
            error: ErrorCode::NPC_NOT_FOUND,
            payload: Payload::Empty,
        };
    }
    match binding.try_accept_quest(peer_addr, npc_ref) {
        Ok(quest) => {
            info!(npc = npc_ref, "quest accepted");
            Message::Response {
                error: ErrorCode::SUCCESS,
                payload: Payload::Json(json!({
                    "quest_id": &quest.0,
                    "description": &<Goal as Into<String>>::into(quest.1.goals[0].clone()),
                    "reward": &quest.1.reward,
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

use std::net::SocketAddr;

use crate::{
    protocol::Message,
    state::SharedServer,
    structures::{
        enums::{error::ErrorCode, game_event::GameEvent},
        handler_outcome::HandlerOutcome,
    },
};

pub fn talk_request(
    peer_addr: SocketAddr,
    args: &Vec<String>,
    server_info: &SharedServer,
) -> HandlerOutcome {
    if args.len() == 0 {
        return Message::Response {
            error: ErrorCode::INVALID_ARGS,
            data: None,
        }
        .into();
    }
    let binding = server_info.lock().unwrap();
    let player = match binding.get_player(peer_addr) {
        Ok(player) => player,
        Err(code) => {
            return Message::Response {
                error: code,
                data: None,
            }
            .into();
        }
    };

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
    let npc = match binding.world.npcs.get(&npc_ref) {
        Some(npc) => npc,
        None => {
            return Message::Response {
                error: ErrorCode::NPC_NOT_FOUND,
                data: None,
            }
            .into();
        }
    };

    let step_entry = match &npc.quest {
        Some(quest_ref) => player
            .quests_in_progress
            .get(quest_ref)
            .map(|step| step.to_string())
            .and_then(|key| npc.dialog.get(&key).map(|d| (key, d))),
        None => None,
    };

    let (dialog_id, dialogs): (String, Vec<String>) = match step_entry {
        Some((key, d)) => (key, d.to_vec()),
        None => match npc.dialog.get("default") {
            Some(d) => ("default".to_string(), d.to_vec()),
            None => {
                return Message::Response {
                    error: ErrorCode::NO_DIALOG,
                    data: None,
                }
                .into();
            }
        },
    };

    HandlerOutcome {
        message: Message::Response {
            error: ErrorCode::SUCCESS,
            data: Some(serde_json::to_value(dialogs).unwrap()),
        },
        event: Some(GameEvent::Talked {
            dialog: format!("{npc_ref}.dialog.{dialog_id}"),
        }),
    }
}

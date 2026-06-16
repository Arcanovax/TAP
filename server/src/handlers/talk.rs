use std::net::SocketAddr;

use crate::{
    error::ErrorCode,
    // handlers::global_func::{get_player::get_player_mut, is_he_there::is_he_there},
    protocol::Message,
    state::SharedServer,
};

pub fn talk_request(
    peer_addr: SocketAddr,
    args: &Vec<String>,
    server_info: &SharedServer,
) -> Message {
    if args.len() == 0 {
        return Message::Response {
            error: ErrorCode::INVALID_ARGS,
            data: None,
        };
    }
    let binding = server_info.lock().unwrap();
    let player = match binding.get_player(peer_addr) {
        Ok(player) => player,
        Err(code) => {
            return Message::Response {
                error: code,
                data: None,
            };
        }
    };

    let mut npc_ref = args.join(" ");
    if let Some(reference) = binding.world.name_to_ref.get(&npc_ref.to_lowercase()) {
        npc_ref = reference.clone();
    }
    let npc = match binding.world.npcs.get(&npc_ref) {
        Some(npc) => npc,
        None => {
            return Message::Response {
                error: ErrorCode::NPC_NOT_FOUND,
                data: None,
            };
        }
    };

    let dialogs: Vec<String> = {
        let step_dialog = match &npc.quest {
            Some(quest_ref) => player
                .quests_in_progress
                .get(quest_ref)
                .and_then(|step| npc.dialog.get(&step.to_string())),
            None => None,
        };

        match step_dialog.or_else(|| npc.dialog.get("default")) {
            Some(dialogs) => dialogs.to_vec(),
            None => {
                return Message::Response {
                    error: ErrorCode::NO_DIALOG,
                    data: None,
                };
            }
        }
    };

    Message::Response {
        error: ErrorCode::SUCCESS,
        data: Some(serde_json::to_value(dialogs).unwrap()),
    }
}

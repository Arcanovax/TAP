use std::net::SocketAddr;

use tracing::info;

use crate::{
    protocol::{Message, Payload},
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
    if args.len() != 1 {
        return Message::Response {
            error: ErrorCode::INVALID_ARGS,
            payload: Payload::Empty,
        }
        .into();
    }

    let binding = server_info.lock().unwrap();
    let player = match binding.get_player(peer_addr) {
        Ok(player) => player,
        Err(code) => {
            return Message::Response {
                error: code,
                payload: Payload::Empty,
            }
            .into();
        }
    };

    let npc_ref = &args[0];

    let player_room = binding.get_player_room(peer_addr).unwrap();
    if !player_room.npc.iter().any(|npc| npc == npc_ref) {
        return Message::Response {
            error: ErrorCode::NPC_NOT_FOUND,
            payload: Payload::Empty,
        }
        .into();
    }
    let npc = match binding.resolve_npc(&npc_ref) {
        Some(npc) => npc,
        None => {
            return Message::Response {
                error: ErrorCode::NPC_NOT_FOUND,
                payload: Payload::Empty,
            }
            .into();
        }
    };

    let mut dialogs = None;
    for (quest_id, step) in &player.quests_in_progress {
        let key = format!("{quest_id}.{step}");
        let quest = binding.world.quests.get(quest_id).unwrap();
        if !quest.goals[*step].is_satisfied(
            player,
            Some(&GameEvent::Talked {
                dialog: format!("{npc_ref}.dialog.{key}"),
            }),
        ) {
            continue;
        }
        if let Some(dialog) = npc.dialog.get(&key) {
            dialogs = Some((dialog.to_vec(), key));
            break;
        }
    }

    if dialogs.is_none() {
        if let Some(dialog) = npc.dialog.get("default") {
            dialogs = Some((dialog.to_vec(), "default".to_string()));
        }
    }

    if dialogs.is_none() {
        return HandlerOutcome {
            message: Message::Response {
                error: ErrorCode::NO_DIALOG,
                payload: Payload::Empty,
            },
            event: None,
        };
    }

    info!("{} talked", npc_ref);

    let (lines, dialog_id) = dialogs.unwrap();
    HandlerOutcome {
        message: Message::Response {
            error: ErrorCode::SUCCESS,
            payload: Payload::Text(lines.join("\\")),
        },
        event: Some(GameEvent::Talked {
            dialog: format!("{npc_ref}.dialog.{}", dialog_id),
        }),
    }
}

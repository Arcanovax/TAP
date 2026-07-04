use std::{collections::HashMap, net::SocketAddr};

use crate::{
    protocol::{Message, Payload},
    state::SharedServer,
    structures::{
        dungeon::parse_dungeon_id,
        enums::{error::ErrorCode, npc_kind::NPCKind},
        npc::NPC,
    },
};
use serde::Serialize;
use tracing::info;

#[cfg(test)]
mod tests;

#[derive(Serialize)]
enum NPCKindView {
    Merchant {
        inventory: Vec<String>,
    },
    Enemy {
        hp: u32,
        max_hp: u32,
        defeated: bool,
    },
    Citizen,
}

#[derive(Serialize)]
struct NPCView<'a> {
    name: &'a str,
    kind: NPCKindView,
    has_quest: bool,
}

impl<'a> From<&'a NPC> for NPCView<'a> {
    fn from(npc: &'a NPC) -> Self {
        let kind = match &npc.kind {
            NPCKind::Citizen => NPCKindView::Citizen,
            NPCKind::Merchant { inventory, .. } => NPCKindView::Merchant {
                inventory: inventory.clone(),
            },
            NPCKind::Enemy {
                hp,
                max_hp,
                defeated,
                ..
            } => NPCKindView::Enemy {
                hp: *hp,
                max_hp: *max_hp,
                defeated: *defeated,
            },
        };

        NPCView {
            name: &npc.name,
            kind: kind,
            has_quest: npc.quest.is_some(),
        }
    }
}

pub(super) fn npc_request(server_info: &SharedServer, args: &Vec<String>) -> Message {
    if args.len() != 1 {
        return Message::Response {
            error: ErrorCode::INVALID_ARGS,
            payload: Payload::Empty,
        };
    }

    let binding = server_info.lock().unwrap();

    let npc_ref = &args[0];
    let npc = match binding.resolve_npc(npc_ref) {
        Some(npc) => npc,
        None => {
            return Message::Response {
                error: ErrorCode::NPC_NOT_FOUND,
                payload: Payload::Empty,
            };
        }
    };

    info!("Get {} info", npc_ref);

    Message::Response {
        error: ErrorCode::SUCCESS,
        payload: Payload::Json(serde_json::to_value::<NPCView>(npc.into()).unwrap()),
    }
}

pub(super) fn npcs_request(server_info: &SharedServer, peer_addr: SocketAddr) -> Message {
    info!("Get all npcs info");
    let binding = server_info.lock().unwrap();

    let mut npcs: HashMap<String, NPCView> = HashMap::new();

    for (id, npc) in &binding.world.npcs {
        npcs.insert(id.into(), npc.into());
    }

    match binding.get_player(peer_addr) {
        Ok(player) if parse_dungeon_id(&player.location).is_some() && player.group_id.is_some() => {
            match binding.dungeons.get(&player.group_id.unwrap()) {
                Some(dungeon) => {
                    let mut npcs: HashMap<String, NPCView> = HashMap::new();

                    for (id, npc) in &dungeon.npcs {
                        npcs.insert(id.into(), npc.into());
                    }
                    Message::Response {
                        error: ErrorCode::SUCCESS,
                        payload: Payload::Json(serde_json::to_value(&npcs).unwrap()),
                    }
                }
                _ => Message::Response {
                    error: ErrorCode::SUCCESS,
                    payload: Payload::Json(serde_json::to_value(&npcs).unwrap()),
                },
            }
        }
        _ => Message::Response {
            error: ErrorCode::SUCCESS,
            payload: Payload::Json(serde_json::to_value(&npcs).unwrap()),
        },
    }
}

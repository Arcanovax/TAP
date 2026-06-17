use serde::Serialize;

use crate::{
    protocol::Message,
    state::SharedServer,
    structures::{
        enums::{error::ErrorCode, npc_kind::NPCKind},
        npc::NPC,
    },
};

#[cfg(test)]
mod tests;

#[derive(Serialize)]
enum NPCKindView {
    Merchant,
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
        let kind = match npc.kind {
            NPCKind::Citizen => NPCKindView::Citizen,
            NPCKind::Merchant { .. } => NPCKindView::Merchant,
            NPCKind::Enemy {
                hp,
                max_hp,
                defeated,
                ..
            } => NPCKindView::Enemy {
                hp,
                max_hp,
                defeated,
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
    let binding = server_info.lock().unwrap();

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

    Message::Response {
        error: ErrorCode::SUCCESS,
        data: Some(serde_json::to_value::<NPCView>(npc.into()).unwrap()),
    }
}

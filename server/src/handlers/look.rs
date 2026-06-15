use crate::{
    error::ErrorCode::SUCCESS,
    protocol::Message,
    state::SharedServer,
    structures::{
        enums::{exits::Exit, npc_kind::NPCKind},
        item::Item,
        npc::NPC,
        room::Room,
    },
};
use serde::Serialize;
use std::net::SocketAddr;

#[cfg(test)]
mod tests;

#[derive(Serialize)]
enum NPCKindView {
    Citizen,
    Merchant,
    Enemy {
        hp: u32,
        max_hp: u32,
        defeated: bool,
    },
}

#[derive(Serialize)]
struct NpcView<'a> {
    name: &'a str,
    kind: NPCKindView,
    has_quest: bool,
}

impl<'a> From<&'a NPC> for NpcView<'a> {
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
        NpcView {
            name: &npc.name,
            kind,
            has_quest: npc.quest.is_some(),
        }
    }
}

#[derive(Serialize)]
struct LookView<'a> {
    name: &'a String,
    description: &'a String,
    exits: &'a [Exit],
    items: Vec<&'a Item>,
    npcs: Vec<NpcView<'a>>,
    players: Vec<String>,
}

pub fn look_request(server_info: &SharedServer, peer_addr: SocketAddr) -> Message {
    let binding = server_info.lock().unwrap();
    let player_name = match binding.get_player(peer_addr) {
        Ok(player) => player.name.clone(),
        Err(code) => {
            return Message::Response {
                error: code,
                data: None,
            };
        }
    };
    let room = match binding.get_player_room(peer_addr) {
        Ok(room) => room,
        Err(code) => {
            return Message::Response {
                error: code,
                data: None,
            };
        }
    };
    let room_players = match binding.get_room_receivers(peer_addr) {
        Ok(receivers) => receivers,
        Err(code) => {
            return Message::Response {
                error: code,
                data: None,
            };
        }
    };
    let mut players: Vec<String> = room_players
        .iter()
        .map(|con| con.player.name.clone())
        .collect();
    players.push(player_name);
    let view = LookView {
        name: &room.name,
        description: &room.description,
        exits: &room.exits,
        items: room
            .items
            .iter()
            .filter_map(|id| binding.world.items.get(id))
            .collect(),
        npcs: room
            .npc
            .iter()
            .filter_map(|id| binding.world.npcs.get(id))
            .map(NpcView::from)
            .collect(),
        players,
    };
    Message::Response {
        error: SUCCESS,
        data: Some(serde_json::to_string(&view).unwrap()),
    }
}

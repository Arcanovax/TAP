use crate::{
    protocol::{Message, Payload},
    state::SharedServer,
    structures::enums::{error::ErrorCode, exits::Direction},
};
use serde::Serialize;
use std::{collections::HashMap, net::SocketAddr};

#[cfg(test)]
mod tests;

#[derive(Serialize, Debug)]
pub struct RoomView<'a> {
    pub id: &'a String,
    pub name: &'a String,
    pub description: &'a String,
    pub exits: &'a HashMap<Direction, String>,
}

#[derive(Serialize, Debug)]
struct LookView<'a> {
    room: RoomView<'a>,
    players: Vec<String>,
    npcs: &'a Vec<String>,
    items: &'a Vec<String>,
}

pub fn look_request(server_info: &SharedServer, peer_addr: SocketAddr) -> Message {
    let binding = server_info.lock().unwrap();
    let player = match binding.get_player(peer_addr) {
        Ok(player) => player,
        Err(code) => {
            return Message::Response {
                error: code,
                payload: Payload::Empty,
            };
        }
    };
    let player_name = player.name.clone();
    let room_id = &player.location;
    let room = match binding.get_player_room(peer_addr) {
        Ok(room) => room,
        Err(code) => {
            return Message::Response {
                error: code,
                payload: Payload::Empty,
            };
        }
    };
    let room_players = match binding.get_room_receivers(peer_addr) {
        Ok(receivers) => receivers,
        Err(code) => {
            return Message::Response {
                error: code,
                payload: Payload::Empty,
            };
        }
    };
    let mut players: Vec<String> = room_players
        .iter()
        .map(|con| con.player.name.clone())
        .collect();
    players.push(player_name);
    let view = LookView {
        room: RoomView {
            id: room_id,
            name: &room.name,
            description: &room.description,
            exits: &room.exits,
        },
        players,
        npcs: &room.npc,
        items: &room.items.iter().map(|item| item.item.clone()).collect(),
    };
    Message::Response {
        error: ErrorCode::SUCCESS,
        payload: Payload::Json(serde_json::to_value(&view).unwrap()),
    }
}

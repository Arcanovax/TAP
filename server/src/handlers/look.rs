use crate::{
    protocol::Message,
    state::SharedServer,
    structures::enums::{error::ErrorCode, exits::Exit},
};
use serde::Serialize;
use std::net::SocketAddr;
use tracing::info;

#[cfg(test)]
mod tests;

#[derive(Serialize, Debug)]
struct RoomView<'a> {
    id: &'a String,
    name: &'a String,
    description: &'a String,
    exits: &'a [Exit],
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
                data: None,
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
        room: RoomView {
            id: room_id,
            name: &room.name,
            description: &room.description,
            exits: &room.exits,
        },
        players,
        npcs: &room.npc,
        items: &room.items,
    };
    info!("Get room info");
    Message::Response {
        error: ErrorCode::SUCCESS,
        data: Some(serde_json::to_value(&view).unwrap()),
    }
}

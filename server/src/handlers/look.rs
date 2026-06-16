use crate::{
    error::ErrorCode::SUCCESS, protocol::Message, state::SharedServer, structures::room::Room,
};
use serde::Serialize;
use std::net::SocketAddr;

#[cfg(test)]
mod tests;

#[derive(Serialize)]
struct LookView<'a> {
    room_id: &'a String,
    #[serde(flatten)]
    room: &'a Room,
    players: Vec<String>,
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
        room_id,
        room,
        players,
    };
    Message::Response {
        error: SUCCESS,
        data: Some(serde_json::to_string(&view).unwrap()),
    }
}

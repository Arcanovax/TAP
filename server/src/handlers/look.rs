use serde::Serialize;

use crate::{
    error::ErrorCode::SUCCESS, protocol::Message, state::SharedServer, structures::room::Room,
};
use std::net::SocketAddr;

#[derive(Serialize)]
struct LookView<'a> {
    #[serde(flatten)]
    room: &'a Room,
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
    let view = LookView { room, players };
    Message::Response {
        error: SUCCESS,
        data: Some(serde_json::to_string(&view).unwrap()),
    }
}

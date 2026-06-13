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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::ErrorCode;
    use crate::test_utils::{
        addr, assert_success_contains, connect, err, populated_server, test_server,
    };

    #[test]
    fn look_without_connection_returns_invalid_command() {
        let server = populated_server();
        assert_eq!(
            look_request(&server, addr(1)),
            err(ErrorCode::INVALID_COMMAND)
        );
    }

    #[test]
    fn look_in_existing_room_returns_room_and_players() {
        let server = populated_server();
        connect(&server, addr(1), "alice");
        let result = look_request(&server, addr(1));
        assert_success_contains(&result, "room.city_square");
        assert_success_contains(&result, "alice");
    }

    #[test]
    fn look_when_room_missing_returns_player_not_found() {
        let server = test_server(); // monde vide -> "room.city_square" n'existe pas
        connect(&server, addr(1), "alice");
        assert_eq!(
            look_request(&server, addr(1)),
            err(ErrorCode::ROOM_NOT_FOUND)
        );
    }
}

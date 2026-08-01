use crate::{
    protocol::{EventType, Message, Payload},
    state::SharedServer,
    structures::enums::{error::ErrorCode, exits::Direction},
};
use std::{collections::HashMap, net::SocketAddr};
use tracing::info;

#[cfg(test)]
mod tests;

pub fn move_request(server: &SharedServer, peer_addr: SocketAddr, dest: &[String]) -> Message {
    if dest.len() != 1 {
        return Message::Response {
            error: ErrorCode::INVALID_ARGS,
            payload: Payload::Empty,
        };
    }
    let destination = dest[0].clone();
    let mut server = server.lock().unwrap();

    let target = match server.get_player_room(peer_addr) {
        Ok(room) => {
            let wanted: Direction = match destination.parse() {
                Ok(exit) => exit,
                Err(code) => {
                    return Message::Response {
                        error: code,
                        payload: Payload::Empty,
                    };
                }
            };
            match room.exits.get(&wanted) {
                Some(toward) => toward.clone(),
                None => {
                    return Message::Response {
                        error: ErrorCode::NO_EXIT,
                        payload: Payload::Empty,
                    };
                }
            }
        }
        Err(code) => {
            return Message::Response {
                error: code,
                payload: Payload::Empty,
            };
        }
    };

    let player = server.get_player(peer_addr).unwrap();
    let old_room = player.location.clone();
    let exit_receivers = server.get_room_receivers(peer_addr).unwrap();
    for con in exit_receivers {
        let _ = con.tx.send(Message::Event(EventType::ROOM_LEAVE {
            player_name: player.name.clone(),
        }));
    }

    let player = server.get_player_mut(peer_addr).unwrap();
    player.location = target.clone();

    let player = server.get_player(peer_addr).unwrap();
    let enter_receivers = server.get_room_receivers(peer_addr).unwrap();
    for con in enter_receivers {
        let _ = con.tx.send(Message::Event(EventType::ROOM_JOIN {
            player_name: player.name.clone(),
        }));
    }

    info!(form = %old_room, to = %target, "player moved");
    Message::Response {
        error: ErrorCode::SUCCESS,
        payload: Payload::Pair(HashMap::from([("room".to_string(), target)])),
    }
}

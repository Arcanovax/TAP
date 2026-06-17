use serde_json::json;

use crate::{
    error::ErrorCode,
    protocol::{EventType, Message},
    state::SharedServer,
    structures::enums::exits::Exit,
};
use std::{mem::discriminant, net::SocketAddr};

#[cfg(test)]
mod tests;

pub fn move_request(server: &SharedServer, peer_addr: SocketAddr, dest: &Vec<String>) -> Message {
    if dest.len() != 1 {
        return Message::Response {
            error: ErrorCode::INVALID_ARGS,
            data: None,
        };
    }
    let destination = dest[0].clone();
    let mut server = server.lock().unwrap();

    let target = match server.get_player_room(peer_addr) {
        Ok(room) => {
            let wanted: Exit = match destination.parse() {
                Ok(exit) => exit,
                Err(code) => {
                    return Message::Response {
                        error: code,
                        data: None,
                    };
                }
            };
            match room
                .exits
                .iter()
                .find(|exit| discriminant(*exit) == discriminant(&wanted))
            {
                Some(
                    Exit::North { toward }
                    | Exit::South { toward }
                    | Exit::East { toward }
                    | Exit::West { toward },
                ) => toward.clone(),
                None => {
                    return Message::Response {
                        error: ErrorCode::NO_EXIT,
                        data: None,
                    };
                }
            }
        }
        Err(code) => {
            return Message::Response {
                error: code,
                data: None,
            };
        }
    };

    let player = server.get_player(peer_addr).unwrap();
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

    Message::Response {
        error: ErrorCode::SUCCESS,
        data: Some(json!({ "room": target })),
    }
}

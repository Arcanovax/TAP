use std::{mem::discriminant, net::SocketAddr};

use crate::{
    error::ErrorCode, protocol::Message, state::SharedServer, structures::enums::exits::Exit,
};

pub fn move_request(server: &SharedServer, peer_addr: SocketAddr, dest: &Vec<String>) -> Message {
    if dest.len() != 1 {
        return Message::Response {
            error: ErrorCode::INVALID_ARGS,
            data: None,
        };
    }
    let destination = dest[0].clone();
    let mut server = server.lock().unwrap();
    let loc = match server.get_player(peer_addr) {
        Ok(player) => player.location.clone(),
        Err(code) => {
            return Message::Response {
                error: code,
                data: None,
            };
        }
    };

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
    let player = server.get_player_mut(peer_addr).unwrap();
    player.location = target.clone();
    // Send les events de sortie et d'entrée ici
    Message::Response {
        error: ErrorCode::SUCCESS,
        data: Some(target),
    }
}

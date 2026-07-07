use crate::{
    handlers::look::RoomView,
    protocol::{Message, Payload},
    state::SharedServer,
    structures::{dungeon::parse_dungeon_id, enums::error::ErrorCode},
};
use std::{collections::HashMap, net::SocketAddr};

#[cfg(test)]
mod tests;

pub(super) fn room_request(server_info: &SharedServer, args: &Vec<String>) -> Message {
    if args.len() != 1 {
        return Message::Response {
            error: ErrorCode::INVALID_ARGS,
            payload: Payload::Empty,
        };
    }

    let binding = server_info.lock().unwrap();

    let room_ref = &args[0];
    let room = match binding.resolve_room(room_ref) {
        Some(room) => room,
        None => {
            return Message::Response {
                error: ErrorCode::ROOM_NOT_FOUND,
                payload: Payload::Empty,
            };
        }
    };

    Message::Response {
        error: ErrorCode::SUCCESS,
        payload: Payload::Json(
            serde_json::to_value(RoomView {
                id: room_ref,
                name: &room.name,
                description: &room.description,
                exits: &room.exits,
            })
            .unwrap(),
        ),
    }
}

pub(super) fn rooms_request(server_info: &SharedServer, peer_addr: SocketAddr) -> Message {
    let binding = server_info.lock().unwrap();

    let mut rooms = HashMap::new();
    for (id, room) in &binding.world.rooms {
        rooms.insert(
            id,
            RoomView {
                id: id,
                name: &room.name,
                description: &room.description,
                exits: &room.exits,
            },
        );
    }

    match binding.get_player(peer_addr) {
        Ok(player) if parse_dungeon_id(&player.location).is_some() && player.group_id.is_some() => {
            match binding.dungeons.get(&player.group_id.unwrap()) {
                Some(dungeon) => {
                    let mut rooms = HashMap::new();
                    for (id, room) in &dungeon.rooms {
                        rooms.insert(
                            id,
                            RoomView {
                                id: id,
                                name: &room.name,
                                description: &room.description,
                                exits: &room.exits,
                            },
                        );
                    }
                    Message::Response {
                        error: ErrorCode::SUCCESS,
                        payload: Payload::Json(serde_json::to_value(&rooms).unwrap()),
                    }
                }
                _ => Message::Response {
                    error: ErrorCode::SUCCESS,
                    payload: Payload::Json(serde_json::to_value(&rooms).unwrap()),
                },
            }
        }
        _ => Message::Response {
            error: ErrorCode::SUCCESS,
            payload: Payload::Json(serde_json::to_value(&rooms).unwrap()),
        },
    }
}

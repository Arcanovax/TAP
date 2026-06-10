use std::net::SocketAddr;

use crate::{
    error::ErrorCode,
    handlers::global_func::{get_player::get_player_mut, is_he_there::is_he_there},
    protocol::Message,
    state::SharedServer,
};

pub fn talk_to(player_name: SocketAddr, target: Vec<String>, world: &SharedServer) -> Message {
    if target.len() != 1 {
        return Message::Response {
            error: ErrorCode::INVALID_ARGS,
            data: None,
        };
    }
    let mut pre_world_mut = world.lock().unwrap();
    let world_mut = &mut *pre_world_mut;
    match get_player_mut(&mut world_mut.connections, &player_name) {
        Ok(player) => {
            if let Some(loc) = world_mut.world.rooms.get(&player.location) {
                if is_he_there(&target[0], loc) {
                    return Message::Response {
                        error: ErrorCode::SUCCESS,
                        data: Some(world_mut.world.npcs[&target[0]].dialogue[0].clone()),
                    };
                } else {
                    return Message::Response {
                        error: ErrorCode::NPC_NOT_FOUND,
                        data: Some(serde_json::to_string("No character by that name").unwrap()),
                    };
                };
            }
            Message::Response {
                error: ErrorCode::PLAYER_NOT_FOUND,
                data: Some(serde_json::to_string("There is no one by that name here").unwrap()),
            }
        }
        Err(msg) => Message::Response {
            error: ErrorCode::PLAYER_NOT_FOUND,
            data: Some(serde_json::to_string(msg).unwrap()),
        },
    }
}


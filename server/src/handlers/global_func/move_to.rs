use std::net::SocketAddr;

use crate::{
    error::ErrorCode, handlers::global_func::get_player::get_player_mut, protocol::Message,
    state::SharedServer, structures::enums::exits::Exit,
};

pub fn move_to(world: &SharedServer, player_addr: SocketAddr, dest: &Vec<String>) -> Message {
    if dest.len() != 1 {
        return Message::Response {
            error: ErrorCode::INVALID_ARGS,
            data: None,
        };
    }
    let mut pre_world_mut = world.lock().unwrap();
    let world_mut = &mut *pre_world_mut;
    match get_player_mut(&mut world_mut.connections, &player_addr) {
        Ok(player) => {
            if let Some(loc) = world_mut.world.rooms.get(&player.location) {
                for exit in &loc.exits {
                    let (dir_name, target) = match exit {
                        Exit::North { toward } => ("North", toward),
                        Exit::South { toward } => ("South", toward),
                        Exit::East { toward } => ("East", toward),
                        Exit::West { toward } => ("West", toward),
                    };
                    // println!("{} et {} et {}", dir_name, dest, &loc.name);
                    if dir_name == dest[0] {
                        // println!("{} et {}", dir_name, target);
                        player.location = target.clone();
                        return Message::Response {
                            error: ErrorCode::SUCCESS,
                            data: Some(
                                serde_json::to_string(&format!(
                                    "{} move to {}",
                                    player.name, target
                                ))
                                .unwrap(),
                            ),
                        };
                    }
                }
                return Message::Response {
                    error: ErrorCode::NO_EXIT,
                    data: Some(serde_json::to_string("No gateway on that direction.").unwrap()),
                };
            } else {
                Message::Response {
                    error: ErrorCode::ROOM_NOT_FOUND,
                    data: Some(serde_json::to_string("You're nowhere. I can't find you.").unwrap()),
                }
            }
        }
        Err(msg) => Message::Response {
            error: ErrorCode::PLAYER_NOT_FOUND,
            data: Some(serde_json::to_string(msg).unwrap()),
        },
    }
}

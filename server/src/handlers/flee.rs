use std::net::SocketAddr;

use crate::{protocol::{Message, Payload}, state::SharedServer, structures::enums::{error::ErrorCode, state::State}};

pub fn flee(
    peer_addr: SocketAddr,
	args: &Vec<String>,
    server_info: &SharedServer
) -> Message {
	if args.len() != 1 {
        return Message::Response {
            error: ErrorCode::INVALID_ARGS,
            payload: Payload::Empty,
        };
    }
    let mut pre_world = server_info.lock().unwrap();
    let world_mut = &mut *pre_world;
    let p_status = {
        let player = match world_mut.get_player_mut(peer_addr) {
            Ok(p) => p,
            Err(msg) => {
                return Message::Response {
                    error: ErrorCode::INVALID_COMMAND,
                    payload: Payload::Json(serde_json::to_value(msg).unwrap()),
                }
            }
        };
		if player.inventory.contains_key("item.gold") {
			let gold = player.inventory.get("item.gold").unwrap();
			if *gold >= 10 {
				player.inventory.insert("item.gold".to_string(), gold - 10);
			}
		}
        player.status.clone()
    };

	match p_status {
        State::InFight { .. } => {
            // let fight = world_mut.fights.get_mut(&target)
            //         .expect("There is no fight.");
            // fight.fighters.retain(|f| *f != p_name);
			match world_mut.try_leave_fight(peer_addr, args[0].clone()) {
				Ok(()) => {
					Message::Response { 
						error: ErrorCode::SUCCESS,
						payload: Payload::Text("You escape the fight. Unfortunatly, in your precipitation, you lose 10 golds on the floor. Thanks for the pool, man!".to_string())
					}
				}
				Err(code) => {
					Message::Response { 
						error: code,
						payload: Payload::Empty,
					}
				}
			}
        },
        _ => {
			return Message::Response { error: ErrorCode::INVALID_COMMAND, payload: Payload::Empty };
		}
}
}
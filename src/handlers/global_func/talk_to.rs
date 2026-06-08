use crate::{
	error::ErrorCode,
	handlers::global_func::{
		get_player::get_player_mut,
		is_he_there::is_he_there
	}, protocol::Message, state::{
		ServerInfo,
		SharedServer
	}
};

pub fn talk_to(player_name: &str, target: &str, world: &SharedServer) -> Message {
	if args.len() != 1 {
        return Message::Response {
            error: ErrorCode::INVALID_ARGS,
            data: None,
        };
    }
	let mut world_mut: &mut ServerInfo = world.lock().unwrap();
	match get_player_mut(&mut world_mut.connections, player_name) {
		Ok(player) => {
			if let Some(loc) = world_mut.rooms.get(&player.location) {
	
				if is_he_there(target, loc) {
					return Message::Response {
						error: ErrorCode::SUCCESS,
						data: world_mut.npcs[target].dialogue[0].clone(),
					};
				} else {
					return Message::Response {
						error: ErrorCode::NPC_NOT_FOUND,
						data: Some(serde_json::to_string("No character by that name").unwrap()),
					};
				};
			}
			Err("There is no one by that name here")
		}
		Err(msg) => Message::Response {
					error: ErrorCode::PLAYER_NOT_FOUND,
					data: Some(serde_json::to_string(msg).unwrap()),
				}
	}
	}
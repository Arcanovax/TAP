use crate::{global_func::{get_player::get_player_mut, is_he_there::is_he_there}, structures::world::World};

pub fn talk_to(player_name: &str, target: &str, world: &mut World) -> Result<String, &'static str> {
	match get_player_mut(&mut world.players, player_name) {
		Ok(player) => {
			if let Some(loc) = world.rooms.get(&player.location) {
	
				if is_he_there(target, loc) {
					return Ok(world.npcs[target].dialogue[0].clone());
				} else {
					return Err("No character by that name");
				};
			}
			Err("There is no one by that name here")
		}
		Err(msg) => Err(msg)
	}
	}
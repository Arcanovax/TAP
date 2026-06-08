use crate::{global_func::get_player::get_player_mut, structures::{enums::exits::Exits, world::World}};

pub fn move_to(world: &mut World, player_name: &str, dest: &str) -> Result<String, &'static str>{
	match get_player_mut(&mut world.players, player_name) {
		Ok(player) => {
			if let Some(loc) = world.rooms.get(&player.location) {
	
				for exit in &loc.exits {
					let (dir_name, target) = match exit {
						Exits::North { toward } => ("North", toward),
						Exits::South { toward } => ("South", toward),
						Exits::East { toward } => ("East", toward),
						Exits::West { toward } => ("West", toward),
					};
					// println!("{} et {} et {}", dir_name, dest, &loc.name);
					if dir_name == dest {
						// println!("{} et {}", dir_name, target);
						player.location = target.clone();
						return Ok(format!("{} move to {}", player.name, target));
					}
				}
				return Err("No gateway on that direction.");
			}
			Err("You're nowhere. I can't find you.")
		}
		Err(msg) => Err(msg)
	}
	}
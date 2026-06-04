mod structures;
mod global_func;

use std::collections::HashMap;
use std::fs::File;
use crate::global_func::{
	create_player::create_player,
	is_he_there::is_he_there,
};
use crate::structures::{
	// list_players::ListPlayers,
	location::Location,
	player::Player,
	items::Items,
	npc::NPC
};

fn main() -> Result<(), Box<dyn std::error::Error>> {

	let mut rooms_list: HashMap<String, Location> = HashMap::new();
	let mut items_list: HashMap<String, Items> = HashMap::new();
	let mut npc_list: HashMap<String, NPC> = HashMap::new();
	let mut list_players: HashMap<String, Player> = HashMap::new();

	for file_path in ["rooms.yaml", "npc.yaml", "items.yaml"] {
		let f = File::open(file_path)?;
		match file_path {
			"rooms.yaml" => rooms_list = serde_yaml::from_reader(f)?,
			"npc.yaml" => npc_list = serde_yaml::from_reader(f)?,
			"items.yaml" => items_list = serde_yaml::from_reader(f)?,
			_ => {}
		};
		
	};
	
	match create_player("Bruno", &mut list_players) {
		Ok(()) => println!("New Player!!! {:#?}", list_players),
		Err(e) => eprintln!("{}", e),
	}
	// match list_players.create_player(String::from("Bruno")) {
	// 	Ok(()) => println!("New Player!!! {:#?}", list_players.list),
	// 	Err(e) => eprintln!("{}", e),
	// }

	if let Some(pl) = list_players.get_mut("Bruno"){
		let current_loc: Option<&Location> = rooms_list.get(&pl.location);
	
		if let Some(loc) = current_loc {
			if is_he_there("npc.city_gard", loc) {
				println!("{}", npc_list["npc.city_gard"].dialogue[0]);
			} else {
				eprintln!("No character by that name");
			}
		} else {
			eprintln!("Oups!!");
		}
	
		if let Some(loc) = current_loc {
			match pl.move_to(loc, "North") {
			Ok(()) => println!("{:#?}", rooms_list.get(&pl.location)),
			Err(e) => eprintln!("{}", e),
			};
		} else {
			eprintln!("Oups!!");
		}
		
	}



	Ok(())
}

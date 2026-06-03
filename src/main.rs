mod structures;
mod global_func;

use crate::structures::{
	global::Global, list_players::ListPlayers, location::Location, player::Player
};

fn main() -> Result<(), Box<dyn std::error::Error>> {

	let f = std::fs::File::open("config.yaml")?;
	let mut d: Global = serde_yaml::from_reader(f)?;
	// println!("Read YAML string: {:#?}", d);
	let mut list_players: ListPlayers = ListPlayers{ list: Vec::new() };
	
	match list_players.create_player(String::from("Bruno")) {
		Ok(()) => println!("New Player!!! {:#?}", list_players.list),
		Err(e) => eprintln!("{}", e),
	}

	let pl: &mut Player = &mut list_players.list[0];

	let current_loc: Option<&Location> = d.locations.get(&pl.location);

	if let Some(loc) = current_loc {
		match pl.talk_to("gard", loc) {
		Ok(sentence) => println!("{}", sentence),
		Err(e) => eprintln!("{}", e),
		};
	}

	if let Some(loc) = current_loc {
		match pl.move_to(loc, "North") {
		Ok(()) => println!("{:#?}", d.locations.get(&pl.location)),
		Err(e) => eprintln!("{}", e),
		};
	}

	Ok(())
}

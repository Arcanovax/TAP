mod structures;
mod global_func;

use crate::structures::{
	global::Global,
	player::Player,
	list_players::ListPlayers
};

fn main() -> Result<(), Box<dyn std::error::Error>> {

	let f = std::fs::File::open("config.yaml")?;
	let mut d: Global = serde_yaml::from_reader(f)?;
	// println!("Read YAML string: {:#?}", d);
	let mut list_players: ListPlayers = ListPlayers{ list: Vec::new() };
	
	match list_players.create_player(String::from("Bruno"), String::from("start")) {
		Ok(()) => println!("New Player!!! {:#?}", list_players.list),
		Err(e) => eprintln!("{}", e),
	}
	Ok(())
}

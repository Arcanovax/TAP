#[warn(unused_variables)]

mod structures;
mod global_func;
mod fight_func;

use std::io;
use std::collections::HashMap;
use std::fs::File;
use crate::global_func::{
	create_player::create_player,
};

use crate::structures::enums::attack_res::AttackRes;
use crate::structures::enums::fight_outcome::FightOutput;
use crate::structures::{
	location::Location,
	player::Player,
	items::Items,
	npc::NPC,
	fight::Fight
};

fn main() -> Result<(), Box<dyn std::error::Error>> {

	let mut rooms_list: HashMap<String, Location> = HashMap::new();
	let mut fights_list: HashMap<String, Fight> = HashMap::new();
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

	// match create_player("Bruno", &mut list_players) {
	// 	Ok(()) => println!("New Player!!! {:#?}", list_players),
	// 	Err(e) => eprintln!("{}", e),
	// }

	let mut input = String::new();
	if let Some(pl) = list_players.get_mut("Bruno"){
		while io::stdin().read_line(&mut input).is_ok(){
			let splitted: Vec<&str> = input.split_whitespace().collect();
			let first: String = splitted[0].to_uppercase();
			match first.as_str() {
				"MOVE" => match pl.move_to(&rooms_list, splitted[1]) {
					Ok(()) => println!("{} move to {}", pl.name, pl.location),
					Err(e) => eprintln!("{}", e),
					},
				"TALK" => match pl.talk_to(splitted[1], &rooms_list, &npc_list) {
					Ok(s) => println!("{}", s),
					Err(e) => eprintln!("{}", e),
					},
				"ATTACK" => match pl.fight(splitted[1], &rooms_list, &mut fights_list){
					Ok(FightOutput::Enter(s)) => println!("{}", s),
					Ok(FightOutput::ReadyToAttack) => {
						match pl.attack(splitted[1], &mut npc_list, &items_list) {
							AttackRes::Hit(msg) => {
								println!("{}", msg);
								fights_list[splitted[1]].turn += 1;
							},
							AttackRes::KillTarget(msg) |
							AttackRes::KillPlayer(msg) |
							AttackRes::Peace(msg) |
							AttackRes::NotFound(msg) => println!("{}", msg),
					}
				},
					Ok(FightOutput::WaitingToAttack) => println!("It's not your turn!"),
					Err(error) => println!("{}", error)
				}
				"STATUS" => println!("{:#?}", pl),
				_ => println!("OUps!")
			}

			// println!("{:?}", splitted);
			input.clear();
		}
	}
	Ok(())
}

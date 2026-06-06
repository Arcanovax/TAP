#[warn(unused_variables)]

mod structures;
mod global_func;
mod fight_func;

use std::io;
use std::collections::HashMap;
use std::fs::File;
use crate::fight_func::enemy_attack::enemy_attack;
use crate::global_func::{
	create_player::create_player,
};

use crate::fight_func::add_loot::add_loot;
use crate::fight_func::attack::execute_attack;
use crate::structures::enums::attack_res::AttackRes::{self, Hit};
use crate::structures::enums::enn_att_res::EnnAttRes;
use crate::structures::enums::fight_outcome::FightOutput;
use crate::structures::world::World;


fn main() -> Result<(), Box<dyn std::error::Error>> {

	let mut world: World = World {
		rooms: HashMap::new(),
		items: HashMap::new(),
		players: HashMap::new(),
		npcs: HashMap::new(),
		fights: HashMap::new(),
	};
	// let mut rooms_list: HashMap<String, Location> = HashMap::new();
	// let mut fights_list: HashMap<String, Fight> = HashMap::new();
	// let mut items_list: HashMap<String, Items> = HashMap::new();
	// let mut npc_list: HashMap<String, NPC> = HashMap::new();
	// let mut list_players: HashMap<String, Player> = HashMap::new();

	for file_path in ["rooms.yaml", "npc.yaml", "items.yaml"] {
		let f = File::open(file_path)?;
		match file_path {
			"rooms.yaml" => world.rooms = serde_yaml::from_reader(f)?,
			"npc.yaml" => world.npcs = serde_yaml::from_reader(f)?,
			"items.yaml" => world.items = serde_yaml::from_reader(f)?,
			_ => {}
		};
		
	};
	
	match create_player("Bruno", &mut world.players) {
		Ok(()) => println!("New Player!!! {:#?}", world.players),
		Err(e) => eprintln!("{}", e),
	}

	// match create_player("Bruno", &mut world.players) {
	// 	Ok(()) => println!("New Player!!! {:#?}", world.players),
	// 	Err(e) => eprintln!("{}", e),
	// }

	let mut input = String::new();
	if let Some(pl) = world.players.get_mut("Bruno"){
		while io::stdin().read_line(&mut input).is_ok(){
			let splitted: Vec<&str> = input.split_whitespace().collect();
			let first: String = splitted[0].to_uppercase();
			match first.as_str() {
				"MOVE" => match pl.move_to(&world.rooms, splitted[1]) {
					Ok(()) => println!("{} move to {}", pl.name, pl.location),
					Err(e) => eprintln!("{}", e),
					},
				"TALK" => match pl.talk_to(splitted[1], &world.rooms, &world.npcs) {
					Ok(s) => println!("{}", s),
					Err(e) => eprintln!("{}", e),
					},
				"ATTACK" => match pl.fight(splitted[1], &world.rooms, &mut world.fights){
					Ok(FightOutput::Enter(s)) => println!("{}", s),
					Ok(FightOutput::ReadyToAttack) => {

						match execute_attack("Bruno", splitted[1], &mut world) {
							AttackRes::Hit(msg) => {
								println!("{}", msg);
								if world.fights.get(splitted[1]).unwrap().enemy_turn {
									match enemy_attack(splitted[1], &mut world) {
										EnnAttRes::Hit(msg) |
										EnnAttRes::Kill(msg) |
										EnnAttRes::KillAndWin(msg) |
										EnnAttRes::Error(msg) => println!("{}", msg)
									}
								}
							},
							AttackRes::KillTarget(msg) => {
								println!("{}", msg);
							},
							AttackRes::KillPlayer(msg) => println!("{}", msg),
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

			input.clear();
		}
	}
	Ok(())
}

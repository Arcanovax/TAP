#[warn(unused_variables)]

mod structures;
mod global_func;
mod fight_func;

use std::io;
use std::collections::HashMap;
use std::fs::File;
use crate::global_func::get_player::get_player_mut;
use crate::global_func::{
	create_player::create_player,
	move_to::move_to,
	talk_to::talk_to,
};

use crate::fight_func::{
	attack::execute_attack,
	fight::fight,
	enemy_attack::enemy_attack
};

use crate::structures::enums::attack_res::AttackRes;
use crate::structures::enums::{
	enn_att_res::EnnAttRes,
	fight_outcome::FightOutput
};
use crate::structures::world::World;


fn main() -> Result<(), Box<dyn std::error::Error>> {

	let mut world: World = World {
		rooms: HashMap::new(),
		items: HashMap::new(),
		players: HashMap::new(),
		npcs: HashMap::new(),
		fights: HashMap::new(),
	};

	for file_path in ["rooms.yaml", "npc.yaml", "items.yaml"] {
		let f = File::open(file_path)?;
		match file_path {
			"rooms.yaml" => world.rooms = serde_yaml::from_reader(f)?,
			"npc.yaml" => world.npcs = serde_yaml::from_reader(f)?,
			"items.yaml" => world.items = serde_yaml::from_reader(f)?,
			_ => {}
		};
		
	};

	let mut input = String::new();
	while io::stdin().read_line(&mut input).is_ok(){
		let splitted: Vec<&str> = input.split_whitespace().collect();
		let first: String = splitted[1].to_uppercase();
		match first.as_str() {
			"CONNECT" => match create_player(splitted[2], &mut world.players) {
					Ok(()) => println!("New Player!!! {:#?}", world.players),
					Err(e) => eprintln!("{}", e)
				},
			"MOVE" => match move_to(&mut world, splitted[0], splitted[2]) {
				Ok(msg) => println!("{}", msg),
				Err(e) => eprintln!("{}", e),
				},
			"TALK" => match talk_to(splitted[0], splitted[2], &mut world) {
				Ok(s) => println!("{}", s),
				Err(e) => eprintln!("{}", e),
				},
			"ATTACK" => match fight(splitted[0], splitted[2], &mut world){
				Ok(FightOutput::Enter(s)) => println!("{}", s),
				Ok(FightOutput::ReadyToAttack) => {

					match execute_attack(splitted[0], splitted[2], &mut world) {
						AttackRes::Hit(msg) => {
							println!("{}", msg);
							if world.fights.get(splitted[2]).unwrap().enemy_turn {
								match enemy_attack(splitted[2], &mut world) {
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
						AttackRes::Peace(msg) => println!("{}", msg),
				}
			},
				Ok(FightOutput::WaitingToAttack) => println!("It's not your turn!"),
				Err(error) => println!("{}", error)
			}
			"STATUS" => {
				match get_player_mut(&mut world.players, splitted[0]) {
					Ok(player) => println!("{:#?}", player),
					Err(msg) => println!("{}", msg)
				}
			}
			_ => println!("OUps!")
		}

		input.clear();
	}
	Ok(())
}

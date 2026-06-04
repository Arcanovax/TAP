use std::collections::HashMap;

use crate::structures::enums::{
	exits::Exits,
	state::State
};
use crate::structures::location::Location;

#[derive(Debug)]
pub struct Player {
	pub name: String,
	pub hp: u32,
	pub max_hp: u32,
	pub location: String,
	pub status: State,
	pub inventory: HashMap<String, u32>,
	pub available_quests: Vec<String>
}

impl Player {
	// pub fn talk_to(&self, npc: &NPC) -> String {
	// 	npc.dialogue.0.clone()
	// }

	pub fn move_to(&mut self, curr_loc: &Location, dest: &str) -> Result<(), &'static str>{
		for exit in &curr_loc.exits {
			let (dir_name, target) = match exit {
				Exits::North { toward } => ("North", toward),
				Exits::South { toward } => ("South", toward),
				Exits::East { toward } => ("East", toward),
				Exits::West { toward } => ("West", toward),
			};
			if dir_name == dest {
				self.location = target.clone();
			}
			return Ok(());
		}
		return Err("No gateway on that direction.");
	}

	pub fn fight(&mut self, target: &str,) {
		
	}
}
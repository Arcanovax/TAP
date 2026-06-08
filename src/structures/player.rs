use std::collections::HashMap;

use crate::structures::enums::state::State;

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

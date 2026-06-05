use std::collections::HashMap;
use crate::structures::{
	player::Player,
	enums::state::State
};

pub fn create_player(name: &str, players_list: &mut HashMap<String, Player>) -> Result<(), &'static str> {
	match players_list.get(name) {
        Some(_player) => return Err("This name is already use."),
        _ => players_list.insert(name.to_string(), Player {
		name: name.to_string(),
		hp: 100,
		max_hp: 100,
		status: State::Idle,
		location: String::from("loc.city_square"),
		inventory: HashMap::new(),
		available_quests: Vec::new()
	})
    };
	return Ok(());
}
use crate::structures::player::Player;
use std::collections::HashMap;

pub fn get_player_mut<'a>(players: &'a mut HashMap<String, Player>, name: &str) -> Result<&'a mut Player, &'static str> {
	if players.contains_key(name) {
		return  Ok(players.get_mut(name).unwrap());
	} else {
		Err("Player not found")
	}
}
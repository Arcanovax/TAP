use crate::structures::player::Player;
use std::collections::HashMap;

pub fn get_player_mut<'a>(players: &'a mut HashMap<String, Player>, name: &str) -> Result<&'a mut Player, &'static str> {
	if players.contains_key(name) {
		return  Ok(players.get_mut(name).unwrap());
	} else {
		for pl in players.values_mut() {
			if pl.name == name {
				return Ok(pl);
			}
		}
	}
	Err("Player not found")
}
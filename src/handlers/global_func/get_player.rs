
use std::{collections::HashMap, net::SocketAddr};
use crate::player::Player;

pub fn get_player_mut<'a>(players: &'a mut HashMap<SocketAddr, Player>, name: &SocketAddr) -> Result<&'a mut Player, &'static str> {
	if players.contains_key(name) {
		return  Ok(players.get_mut(name).unwrap());
	} else {
		Err("Player not found")
	}
}

use std::{collections::HashMap, net::SocketAddr};

use crate::{state::Connection, structures::player::Player};

pub fn get_player_mut<'a>(players: &'a mut HashMap<SocketAddr, Connection>, name: &SocketAddr) -> Result<&'a mut Player, &'static str> {
	if players.contains_key(name) {
		return  Ok(&mut players.get_mut(name).unwrap().player);
	} else {
		Err("Player not found")
	}
}
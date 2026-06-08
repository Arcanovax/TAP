use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Fight {
	pub fighters: Vec<SocketAddr>,
	pub turn: u32,
	pub enemy_turn: bool
}
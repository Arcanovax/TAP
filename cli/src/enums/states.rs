use std::fmt::Display;

use serde::{Deserialize, Serialize};


#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum States {
	Login,
	ServerWait,
	ServerError(String),
	Idle,
	InFight {target_id: String},
	InDiscuss(String, String),
	Respawn
}

impl Display for States {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let status = match self {
			States::Login => "Login",
			States::ServerWait => "ServerWait",
			States::ServerError(..) => "ServerError",
			States::Idle => "Idle",
			States::InFight { .. } => "InFight",
			States::InDiscuss(..) => "InDiscuss",
			States::Respawn => "Respawn",
		};
		write!(f, "{status}")
	}
}
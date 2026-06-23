use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum States {
	Login,
	ServerWait,
	ServerError(String),
	InGame,
	InFight,
	InDiscuss(String, String),
	Respawn
}
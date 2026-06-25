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
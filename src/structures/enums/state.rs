use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum State {
	InFight {target_id: String},
    Idle,
    Respawn,
	Discuss
}
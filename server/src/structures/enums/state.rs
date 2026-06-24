use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize
, Clone)]
pub enum State {
	InFight {target_id: String},
    Idle,
	Discuss
}
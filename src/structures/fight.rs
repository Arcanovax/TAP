use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Fight {
	fighters: Vec<String>,
	turn: u32
}
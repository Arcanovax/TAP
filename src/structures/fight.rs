use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Fight {
	pub fighters: Vec<String>,
	pub turn: u32
}
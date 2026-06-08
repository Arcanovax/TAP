use crate::structures::enums::exits::Exits;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Location {
	pub id: String,
	pub name: String,
	pub exits: Vec<Exits>,
	pub description: String,
	pub npc: Option<Vec<String>>,
	pub items: Option<Vec<String>>
}
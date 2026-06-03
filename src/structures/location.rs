use crate::structures::{items::Items, npc::NPC, enums::exits::Exits};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Location {
	pub name: String,
	pub exits: Vec<Exits>,
	pub description: String,
	pub npc: HashMap<String, NPC>,
	pub items: Option<Vec<Items>>
}
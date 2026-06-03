use crate::structures::{items::Items, npc::NPC, enums::exits::Exits};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Location {
	name: String,
	exits: Vec<Exits>,
	description: String,
	npc: HashMap<String, NPC>,
	items: Option<Vec<Items>>
}
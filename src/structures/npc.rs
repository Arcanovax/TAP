use crate::structures::enums::npc_kind::NPCKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NPC {
	pub name: String,
	pub dialogue: Vec<String>,
	pub kind: NPCKind
}
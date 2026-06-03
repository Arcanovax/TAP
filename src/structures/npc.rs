use crate::structures::enums::npc_kind::NPCKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NPC {
	name: String,
	dialogue: Vec<String>,
	description: String,
	kind: NPCKind
}
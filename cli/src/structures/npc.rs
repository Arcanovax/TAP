use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::enums::npc_kind::NPCKind;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NPC {
    pub name: String,
    pub kind: NPCKind,
    pub has_quest: bool,
}

impl Display for NPC {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let quest = {
			if self.has_quest {
				"He has a quest for you!"
			} else {
				"He doesn't have a quest for you!"
			}
		};
		write!(f, "The NPC in front of you is named {}.\n He defined himself of type {}\n{}", self.name, self.kind, quest)
	}
}
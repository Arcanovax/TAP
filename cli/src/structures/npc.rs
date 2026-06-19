use serde::{Deserialize, Serialize};

use crate::enums::npc_kind::NPCKind;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NPC {
    pub name: String,
    pub kind: NPCKind,
    pub has_quest: bool,
}
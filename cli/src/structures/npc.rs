use serde::{Deserialize, Serialize};

use crate::enums::npc_kind::NPCKind;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NPC {
    pub has_quest: bool,
    pub kind: NPCKind,
    pub name: String,
}
use crate::structures::enums::npc_kind::NPCKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NPC {
    pub name: String,
    pub dialogue: Vec<String>,
    pub kind: NPCKind,
}


impl NPC {
    pub fn references(&self) -> Vec<&str> {
        match &self.kind {
            NPCKind::Enemy { loot, .. } => loot.iter().map(String::as_str).collect(),
            NPCKind::Merchant { inventory, .. } => inventory.iter().map(String::as_str).collect(),
            _ => Vec::new(),
        }
    }
}

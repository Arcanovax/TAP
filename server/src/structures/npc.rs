use std::collections::HashMap;

use crate::structures::enums::npc_kind::NPCKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Npc {
    pub name: String,
    #[serde(default)]
    pub dialog: HashMap<String, Vec<String>>,
    pub kind: NPCKind,
    pub quest: Option<String>,
}

impl Npc {
    pub fn references(&self) -> Vec<&str> {
        let mut refs = match &self.kind {
            NPCKind::Enemy { loot, .. } => loot.iter().map(String::as_str).collect(),
            NPCKind::Merchant { inventory, .. } => inventory.iter().map(String::as_str).collect(),
            _ => Vec::new(),
        };
        if let Some(quest) = &self.quest {
            refs.push(quest.as_str());
        }
        refs
    }
}

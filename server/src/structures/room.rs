use crate::structures::enums::exits::Direction;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Room {
    pub name: String,
    pub exits: HashMap<Direction, String>,
    pub description: String,
    #[serde(default)]
    pub npc: Vec<String>,
    #[serde(default)]
    pub items: Vec<String>,
}

impl Room {
    pub fn references(&self) -> Vec<&str> {
        self.npc
            .iter()
            .chain(self.items.iter())
            .map(|s| String::as_str(s))
            .chain(self.exits.values().map(|str| str.as_str()))
            .collect()
    }
}

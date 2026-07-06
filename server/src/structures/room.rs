use crate::structures::enums::exits::Direction;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
pub enum Owner {
    #[default]
    Room,
    Player,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct OwnedItem {
    pub item: String,
    #[serde(default)]
    pub owner: Owner,
}

impl From<String> for OwnedItem {
    fn from(str: String) -> Self {
        OwnedItem {
            item: str,
            owner: Default::default(),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Room {
    pub name: String,
    pub exits: HashMap<Direction, String>,
    pub description: String,
    #[serde(default)]
    pub npc: Vec<String>,
    #[serde(default)]
    pub items: Vec<OwnedItem>,
}

impl Room {
    pub fn new(name: &str) -> Self {
        Room {
            name: name.to_string(),
            exits: HashMap::new(),
            description: String::new(),
            npc: Vec::new(),
            items: Vec::new(),
        }
    }

    pub fn references(&self) -> Vec<&str> {
        self.npc
            .iter()
            .chain(self.items.iter().map(|item| &item.item))
            .map(|s| String::as_str(s))
            .chain(self.exits.values().map(|str| str.as_str()))
            .collect()
    }
}

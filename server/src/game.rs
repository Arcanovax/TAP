use std::collections::HashMap;

use serde::Deserialize;

#[derive(PartialEq, Deserialize, Debug, Clone)]
pub enum Exit {
    North { toward: String },
    South { toward: String },
    East { toward: String },
    West { toward: String },
}

#[derive(PartialEq, Deserialize, Debug)]
pub struct Room {
    pub name: String,
    pub exits: Vec<Exit>,
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
            .chain(self.exits.iter().map(|exit| match exit {
                Exit::North { toward }
                | Exit::South { toward }
                | Exit::East { toward }
                | Exit::West { toward } => toward.as_str(),
            }))
            .collect()
    }
}

#[derive(PartialEq, Deserialize, Debug)]
pub enum NPCKind {
    Merchant {
        inventory: Vec<String>,
        gold: u32,
    },
    Enemy {
        hp: u32,
        max_hp: u32,
        damages: u32,
        loot: Vec<String>,
        beaten: bool,
    },
    Citizen,
}

#[derive(PartialEq, Deserialize, Debug)]
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

#[derive(PartialEq, Deserialize, Debug)]
pub enum ItemKind {
    Weapon { damages: u32 },
    Armor { protection: u32 },
    Potion { healing: u32 },
    Miscellaneous,
}

#[derive(Deserialize, Debug)]
pub struct Item {
    pub name: String,
    pub price: u32,
    pub kind: ItemKind,
}

impl Item {
    pub fn references(&self) -> Vec<&str> {
        Vec::new()
    }
}

#[derive(Debug)]
pub struct World {
    pub rooms: HashMap<String, Room>,
    pub npcs: HashMap<String, NPC>,
    pub items: HashMap<String, Item>,
}

impl World {
    pub fn new() -> Self {
        World {
            rooms: HashMap::new(),
            npcs: HashMap::new(),
            items: HashMap::new(),
        }
    }
}

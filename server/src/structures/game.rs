use crate::structures::{item::Item, npc::NPC, quest::Quest, room::Room};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct World {
    pub rooms: HashMap<String, Room>,
    pub npcs: HashMap<String, NPC>,
    pub items: HashMap<String, Item>,
    pub quests: HashMap<String, Quest>,
    pub spawn_room: String,
}

impl World {
    pub fn new() -> Self {
        World {
            rooms: HashMap::new(),
            npcs: HashMap::new(),
            items: HashMap::new(),
            quests: HashMap::new(),
            spawn_room: String::new(),
        }
    }
}

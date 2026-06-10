use crate::structures::{item::Item, npc::NPC, quest::Quest, room::Room};
use std::collections::HashMap;

#[derive(Debug)]
pub struct World {
    pub rooms: HashMap<String, Room>,
    pub npcs: HashMap<String, NPC>,
    pub items: HashMap<String, Item>,
    pub quests: HashMap<String, Quest>,
}

impl World {
    pub fn new() -> Self {
        World {
            rooms: HashMap::new(),
            npcs: HashMap::new(),
            items: HashMap::new(),
            quests: HashMap::new(),
        }
    }
}

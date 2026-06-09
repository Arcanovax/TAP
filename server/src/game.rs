use crate::structures::{item::Item, npc::NPC, room::Room};
use std::collections::HashMap;

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

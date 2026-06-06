use std::collections::HashMap;

use crate::structures::{fight::Fight, items::Items, location::Location, npc::NPC, player::Player};

pub struct World {
    pub rooms: HashMap<String, Location>,
    pub items: HashMap<String, Items>,
    pub fights: HashMap<String, Fight>,
    pub npcs: HashMap<String, NPC>,
    pub players: HashMap<String, Player>,
}
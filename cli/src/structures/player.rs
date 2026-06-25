use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Player {
    pub name: String,
    pub hp: u32,
    pub max_hp: u32,
    pub inventory: HashMap<String, u32>,
    pub available_quests: Vec<String>
}

impl Player {
    pub fn new() -> Self {
        let mut p = Player {
            name: "".to_string(),
            hp: 100,
            max_hp: 100,
            inventory: HashMap::new(),
            available_quests: Vec::new()
        };
		p.inventory.insert("item.gold".to_string(), 50);
		p
    }
}
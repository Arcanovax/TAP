use std::collections::HashMap;

pub struct Player {
    pub name: String,
    pub hp: u32,
    pub max_hp: u32,
    pub inventory: HashMap<String, u32>,
    pub available_quests: Vec<String>
}

impl Player {
    pub fn new() -> Self {
        Player {
            name: "".to_string(),
            hp: 100,
            max_hp: 100,
            inventory: HashMap::new(),
            available_quests: Vec::new()
        }
    }
}
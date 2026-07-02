use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Player {
    pub name: String,
    pub hp: u32,
    pub max_hp: u32,
    pub inventory: HashMap<String, u32>,
	pub gold: u32,
}

impl Player {
    pub fn new() -> Self {
        Player {
            name: "".to_string(),
            hp: 100,
            max_hp: 100,
            inventory: HashMap::new(),
			gold: 0,
        }
    }
}
use serde::Deserialize;

use crate::enums::exits::Exits;

#[derive(Deserialize, Debug)]
pub struct Room {
    pub name: String,
    pub exits: Vec<Exits>,
    pub description: String,
    pub npc: Vec<String>,
    pub items: Vec<String>,
}

impl Room {
	pub fn new() -> Self {
		Room {
			name: String::from(""),
			exits: Vec::new(),
			description: String::from(""),
			npc: Vec::new(),
			items: Vec::new()
		}
	}
}
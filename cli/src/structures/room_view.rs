use std::fmt::Display;

use serde::Deserialize;

use crate::{enums::exits::Exits};

#[derive(Deserialize, Debug)]
pub struct RoomView {
    pub id: String,
    pub name: String,
    pub description: String,
    pub exits: Vec<Exits>,
}

impl RoomView {
    pub fn new() -> Self {
        RoomView {
            id: "".to_string(),
            name: "".to_string(),
            description: "".to_string(),
            exits: Vec::new(),
        }
    }
}

impl Display for RoomView {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut final_exits: Vec<String> = Vec::new();
		for exit in &self.exits {
			match exit {
				Exits::East { toward } => final_exits.push(format!("East => {toward}")), 
				Exits::North { toward } => final_exits.push(format!("North => {toward}")), 
				Exits::South { toward } => final_exits.push(format!("South => {toward}")), 
				Exits::West { toward } => final_exits.push(format!("West => {toward}")), 
			}
		}
		write!(f, "Id: {}\nName: {}\nDescription: {}\n Exits: {}", self.id, self.name, self.description, final_exits.join("\n"))
	}
}
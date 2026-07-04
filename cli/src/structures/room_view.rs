use std::{collections::HashMap, fmt::Display};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RoomView {
    pub description: String,
    pub exits: HashMap<String, String>,
    pub id: String,
    pub name: String,
}

impl RoomView {
    pub fn new() -> Self {
        RoomView {
            description: "".to_string(),
            exits: HashMap::new(),
            id: "".to_string(),
            name: "".to_string(),
        }
    }
}

impl Display for RoomView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut final_exits: Vec<String> = Vec::new();
        for (dir, dest) in &self.exits {
            final_exits.push(format!("{dir} => {dest}"));
        }
        write!(
            f,
            "Id: {}\nName: {}\nDescription: {}\nExits:\n{}",
            self.id,
            self.name,
            self.description,
            final_exits.join("\n")
        )
    }
}

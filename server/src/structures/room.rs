use crate::structures::enums::exits::Exit;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Room {
    pub name: String,
    pub exits: Vec<Exit>,
    pub description: String,
    #[serde(default)]
    pub npc: Vec<String>,
    #[serde(default)]
    pub items: Vec<String>,
}

impl Room {
    pub fn references(&self) -> Vec<&str> {
        self.npc
            .iter()
            .chain(self.items.iter())
            .map(|s| String::as_str(s))
            .chain(self.exits.iter().map(|exit| match exit {
                Exit::North { toward }
                | Exit::South { toward }
                | Exit::East { toward }
                | Exit::West { toward } => toward.as_str(),
            }))
            .collect()
    }
}

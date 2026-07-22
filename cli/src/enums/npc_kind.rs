use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum NPCKind {
    Merchant {
        inventory: Vec<String>,
    },
    Enemy {
        hp: u32,
        max_hp: u32,
		kind: String,
        defeated: bool,
    },
    Citizen,
}

impl Display for NPCKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NPCKind::Merchant { inventory } => {
                let mut list_items: Vec<String> = Vec::new();
                for item in inventory {
                    list_items.push(format!("- {item}"));
                }
                write!(f, "Merchant\nItems you can buy:\n{}", list_items.join("\n"))
            }
            NPCKind::Enemy { hp, max_hp, kind, .. } => write!(f, "Enemy\nHP: {}/{}\nType: {}", hp, max_hp, kind),
            NPCKind::Citizen => write!(f, "Citizen"),
        }
    }
}

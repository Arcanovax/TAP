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
        damages: u32,
        loot: Vec<String>,
        defeated: bool,
    },
    Citizen,
}

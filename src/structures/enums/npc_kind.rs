use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum NPCKind {
	Merchant {
		inventory: Vec<String>,
		gold: u32
	},
	Enemy {
		hp: u32,
		max_hp: u32,
		damages: u32,
		loot: Vec<String>,
		beaten_by: Option<Vec<String>>
	},
	Citizen
}
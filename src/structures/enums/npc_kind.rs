use crate::structures::items::Items;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum NPCKind {
	Merchant {
		inventory: Vec<Items>,
		gold: u32
	},
	Ennemy {
		hp: u32,
		max_hp: u32,
		damages: u32,
		loot: Vec<Items>
	},
	Citizen,
	Gard
}
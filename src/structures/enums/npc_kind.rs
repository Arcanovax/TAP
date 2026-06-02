use crate::structures::items::Items;

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
	Citizen {}
}
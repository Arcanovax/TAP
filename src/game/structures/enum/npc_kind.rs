mod items;

pub enum NPCKind {
	merchant: {
		inventory: Vec<Items>,
		gold: u32
	},
	ennemy: {
		hp: u32,
		max_hp: u32,
		damages: u32,
		loot: Vec<Items>
	},
	citizen: {}
}
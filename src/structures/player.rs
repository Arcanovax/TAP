use crate::structures::items::Items;
use crate::structures::quests::Quest;

struct Player {
	name: String,
	hp: u32,
	max_hp: u32,
	inventory: Vec<Items>,
	available_quests: Vec<Quest>
}
use crate::structures::items::Items;
use crate::structures::quests::Quest;

#[derive(Debug)]
pub struct Player {
	pub name: String,
	pub hp: u32,
	pub max_hp: u32,
	pub inventory: Vec<Items>,
	pub available_quests: Vec<Quest>
}
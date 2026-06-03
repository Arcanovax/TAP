use crate::structures::enums::exits::Exits;
use crate::structures::items::Items;
use crate::structures::location::Location;
use crate::structures::quests::Quest;

#[derive(Debug)]
pub struct Player {
	pub name: String,
	pub hp: u32,
	pub max_hp: u32,
	pub location: String,
	pub inventory: Vec<Items>,
	pub available_quests: Vec<Quest>
}

impl Player {
	pub fn talk_to(&self, name: &str, pl_loc: &Location) -> Result<String, &'static str> {
		match pl_loc.npc.get(name) {
			Some(npc) => Ok(npc.dialogue[0].clone()),
			_ => Err("This npc is not here."),
		}
	}

	pub fn move_to(&mut self, curr_loc: &Location, dest: &str) -> Result<(), &'static str>{
		for exit in &curr_loc.exits {
			let (dir_name, target) = match exit {
				Exits::North { toward } => ("North", toward),
				Exits::South { toward } => ("South", toward),
				Exits::East { toward } => ("East", toward),
				Exits::West { toward } => ("West", toward),
			};
			if dir_name == dest {
				self.location = target.clone();
			}
			return Ok(());
		}
		return Err("No gateway on that direction.");
	}
}
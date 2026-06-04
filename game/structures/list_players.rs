use crate::structures::{location::Location, player::Player};

#[derive(Debug)]
pub struct ListPlayers {
	pub list: Vec<Player>
}

impl ListPlayers {
	pub fn create_player(&mut self, name: String) -> Result<(), &'static str> {
	for player in &self.list {
		if *player.name == name {
			return Err("This name is already use.");
		}
	}
	self.list.push(Player {
		name,
		hp: 100,
		max_hp: 100,
		location: String::from("start"),
		inventory: Vec::new(),
		available_quests: Vec::new()
	});
	return Ok(());
}
}
// use crate::structures::player::Player;

// pub fn create_player(name: String, list_pl: &Vec<Player>) -> Result<Player, &'static str> {
// 	for player in list_pl {
// 		if *player.name == name {
// 			return Err("This name is already use.");
// 		}
// 	}
// 	Player {
// 		name: name,
// 		hp: 100,
// 		max_hp: 100,
// 		inventory: Vec::new(),
// 		available_quests: Vec::new()
// 	}
// }
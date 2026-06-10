use crate::enums::exits::Exits;

pub struct Room {
	name: String,
	exits: Vec<Exits>,
	npcs: Vec<String>,
	items: Vec<String>
}

// impl Room {
// 	pub fn new() -> Self {
// 		name: String::from("Login"),
// 		exits: 
// 	}
// }
use crate::structures::fight::Fight;

pub fn is_it_my_turn(name: &str, fight: &Fight) -> bool {
	for part in &fight.fighters {
		if part == name {
			return true;
		}
	}
	false
}

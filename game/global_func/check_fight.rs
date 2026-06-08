use crate::structures::fight::Fight;
use std::collections::HashMap;

pub fn check_fight<'a>(target: &str, list_fights: &'a mut HashMap<String, Fight>) -> Option<&'a mut Fight> {
	list_fights.get_mut(target)
}
use crate::structures::{enums::turn_res::TurnRes, fight::Fight};

pub fn is_it_my_turn(name: &str, fight: &Fight) -> TurnRes {
	if fight.enemy_turn {
		TurnRes::EnemyTurn
	} else {
		if fight.fighters[fight.turn as usize] == name {
			TurnRes::MyTurn
		} else {
			TurnRes::NotMyTurn
		}
	}
}

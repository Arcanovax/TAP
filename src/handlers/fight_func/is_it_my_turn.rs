use std::net::SocketAddr;

use crate::structures::{enums::turn_res::TurnRes, fight::Fight};

pub fn is_it_my_turn(name: SocketAddr, fight: &Fight) -> TurnRes {
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

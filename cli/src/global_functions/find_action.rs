use crate::{enums::actions::PendingAction, structures::world::World};

pub fn find_action(command: &str, world: &mut World) {
	match command {
		"move" => world.action = PendingAction::Move,
		"talk" => world.action = PendingAction::Talk,
		"drop" => world.action = PendingAction::Drop,
		"take" => world.action = PendingAction::Take,
		"look" => world.action = PendingAction::Look,
		_ => {}
	}
}
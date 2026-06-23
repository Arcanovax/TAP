use crate::{enums::actions::PendingAction, structures::world::World};

pub fn find_action(command: Vec<&str>, world: &mut World) {

	match command[0].to_lowercase().as_str() {
		"move" => world.action = PendingAction::Move,
		"talk" => world.action = PendingAction::Talk(command[1..].join(" ")),
		"drop" => world.action = PendingAction::Drop,
		"take" => world.action = PendingAction::Take,
		"look" => world.action = PendingAction::Look,
		"who" => world.action = PendingAction::Who,
		"chat" => world.action = PendingAction::SendChat(command[..2].join(" ").clone().to_lowercase(), command[2..].join(" ")),
		"group" => {
			match command[1].to_lowercase().as_str() {
				"join" => world.action = PendingAction::GroupJoin(command[2..].join(" ")),
				"invite" => world.action = PendingAction::GroupInvite(command[2..].join(" ")),
				"create" => world.action = PendingAction::GroupCreate(command[2..].join(" ")),
				"leave" => world.action = PendingAction::GroupLeave(command[2..].join(" ")),
				_ => {}
			}
		}
		"status" => world.action = PendingAction::Status,
		"attack" => world.action = PendingAction::Attack(command[1..].join(" ")),
		"inventory" => world.action = PendingAction::Inventory,
		"quest" => world.action = PendingAction::Quest,
		_ => {}
	}
}
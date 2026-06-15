use crate::{enums::actions::PendingAction, structures::world::World};

pub fn find_action(command: &str, world: &mut World, args: Option<String>) {
	match command.to_lowercase().as_str() {
		"move" => world.action = PendingAction::Move,
		"talk" => world.action = PendingAction::Talk,
		"drop" => world.action = PendingAction::Drop,
		"take" => world.action = PendingAction::Take,
		"look" => world.action = PendingAction::Look,
		"who" => world.action = PendingAction::Who,
		"chat" => world.action = PendingAction::SendChat(world.chat.channel.clone(), args.unwrap_or("".to_string())),
		"group join" => world.action = PendingAction::GroupJoin(args.unwrap_or("".to_string())),
		"group invite" => world.action = PendingAction::GroupInvite(args.unwrap_or("".to_string())),
		"group create" => world.action = PendingAction::GroupCreate(args.unwrap_or("".to_string())),
		"group leave" => world.action = PendingAction::GroupLeave(args.unwrap_or("".to_string())),
		"status" => world.action = PendingAction::Status,
		"attack" => world.action = PendingAction::Attack,
		"inventory" => world.action = PendingAction::Inventory,
		"quest" => world.action = PendingAction::Quest,
		_ => {}
	}
}
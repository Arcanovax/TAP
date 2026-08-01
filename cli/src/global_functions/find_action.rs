use crate::{enums::actions::PendingAction, structures::world::World};

pub fn find_action(command: Vec<&str>, world: &mut World) {
    match command[0].to_lowercase().as_str() {
        "move" => world.action = PendingAction::Move,
        "talk" => world.action = PendingAction::Talk(command[1..].join(" ")),
        "drop" => world.action = PendingAction::Drop(command[1..].join(" ")),
        "take" => world.action = PendingAction::Take(command[1..].join(" ")),
        "look" => world.action = PendingAction::Look,
        "who" => world.action = PendingAction::Who,
        "chat" => {
            world.action = PendingAction::SendChat(
                command[..2].join(" ").clone().to_lowercase(),
                command[2..].join(" "),
            )
        }
        "group" => match command[1].to_lowercase().as_str() {
            "join" => world.action = PendingAction::GroupJoin(command[2..].join(" ")),
            "invite" => world.action = PendingAction::GroupInvite(command[2..].join(" ")),
            "create" => world.action = PendingAction::GroupCreate(command[2..].join(" ")),
            "leave" => world.action = PendingAction::GroupLeave,
            _ => {}
        },
        "status" => world.action = PendingAction::Status,
        "attack" => world.action = PendingAction::Attack(command[1..].join(" ")),
        "inventory" => world.action = PendingAction::Inventory,
        "flee" => world.action = PendingAction::Flee,
        "quest" => world.action = PendingAction::Quest,
        "gold" => world.action = PendingAction::Gold,
        "room" => world.action = PendingAction::Room,
        "rooms" => world.action = PendingAction::Rooms,
        "item" => world.action = PendingAction::Item,
        "items" => world.action = PendingAction::Items,
        "npc" => world.action = PendingAction::Npc,
        "npcs" => world.action = PendingAction::Npcs,
        "quests" => world.action = PendingAction::Quests,
        "dices" => world.action = PendingAction::Dices(command[1..].join(" ")),
        "slot_machine" => world.action = PendingAction::Slot,
        "dungeon" => match command[1].to_lowercase().as_str() {
            "create" => world.action = PendingAction::DungeonCreate,
            "join" => world.action = PendingAction::DungeonJoin,
            _ => {}
        },
        "answer" => world.action = PendingAction::Answer,
        "help" => world.action = PendingAction::Help,
        _ => {}
    }
}

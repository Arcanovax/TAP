use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum::{EnumIter, EnumString};

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, PartialEq, EnumIter, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Command {
    CONNECT,
    QUIT,
    WHO,
    CHAT,
    GROUP,
    STATUS,
    MOVE,
    TALK,
    ATTACK,
    FLEE,
    CONSUME,
    LOOK,
    DROP,
    TAKE,
    INVENTORY,
    QUEST,
    ITEM,
    ITEMS,
    NPC,
    QUESTS,
    NPCS,
    BUY,
    SELL,
    GOLD,
    DUNGEON,
    SLOT_MACHINE,
    QUEST_INFO,
    HELP,
    ROOMS,
}

impl Command {
    pub fn description(&self) -> &'static str {
        match self {
            Command::CONNECT => "Connect to the server with a player name.",
            Command::QUIT => "Disconnect from the server.",
            Command::WHO => "Show how many players are currently online.",
            Command::CHAT => "Send a chat message (global, group or room scope).",
            Command::GROUP => "Manage your group: create, invite, join, leave or kick.",
            Command::STATUS => "Display your character's status.",
            Command::MOVE => "Move to an adjacent room in a given direction.",
            Command::TALK => "Talk to an NPC in your current room.",
            Command::ATTACK => "Attack an enemy to start or continue a fight.",
            Command::FLEE => "Flee from the current fight.",
            Command::CONSUME => "Consume an item from your inventory.",
            Command::LOOK => "Look around the room you are in.",
            Command::DROP => "Drop an item from your inventory into the room.",
            Command::TAKE => "Pick up an item from the current room.",
            Command::INVENTORY => "List the items you are carrying.",
            Command::QUEST => "Accept or hand in a quest from an NPC.",
            Command::ITEM => "Show the details of a specific item.",
            Command::ITEMS => "List the items lying in your current room.",
            Command::NPC => "Show the details of a specific NPC.",
            Command::NPCS => "List the NPCs in your current room.",
            Command::QUESTS => "List your active and completed quests.",
            Command::BUY => "Buy an item from a merchant.",
            Command::SELL => "Sell an item to a merchant.",
            Command::GOLD => "Show how much gold you are carrying.",
            Command::DUNGEON => "Create or join a dungeon from the dungeon entrance.",
            Command::SLOT_MACHINE => "Play the slot machine in the gambling room.",
            Command::QUEST_INFO => "Show detailed information about a quest.",
            Command::HELP => "List all available commands and what they do.",
            Command::ROOMS => "List the rooms",
        }
    }

    pub fn parse(name: &str) -> Option<Self> {
        Command::from_str(name).ok()
    }
}

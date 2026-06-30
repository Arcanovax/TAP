use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[allow(non_camel_case_types)]
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
    BAG,
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
    SLOT_MACHINE,
}

impl Command {
    pub fn parse(name: &str) -> Option<Self> {
        match name.to_uppercase().as_str() {
            "CONNECT" => Some(Command::CONNECT),
            "QUIT" => Some(Command::QUIT),
            "WHO" => Some(Command::WHO),
            "CHAT" => Some(Command::CHAT),
            "GROUP" => Some(Command::GROUP),
            "STATUS" => Some(Command::STATUS),
            "MOVE" => Some(Command::MOVE),
            "TALK" => Some(Command::TALK),
            "ATTACK" => Some(Command::ATTACK),
            "FLEE" => Some(Command::FLEE),
            "BAG" => Some(Command::BAG),
            "LOOK" => Some(Command::LOOK),
            "DROP" => Some(Command::DROP),
            "TAKE" => Some(Command::TAKE),
            "INVENTORY" => Some(Command::INVENTORY),
            "QUEST" => Some(Command::QUEST),
            "ITEM" => Some(Command::ITEM),
            "ITEMS" => Some(Command::ITEMS),
            "NPC" => Some(Command::NPC),
            "NPCS" => Some(Command::NPCS),
            "QUESTS" => Some(Command::QUESTS),
            "BUY" => Some(Command::BUY),
            "SELL" => Some(Command::SELL),
            "GOLD" => Some(Command::GOLD),
            "SLOT_MACHINE" => Some(Command::SLOT_MACHINE),
            _ => None,
        }
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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
    SELL
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
            _ => None,
        }
    }
}

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
            _ => None,
        }
    }
}

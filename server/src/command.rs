pub enum Command {
    CONNECT,
    QUIT,
    WHO,
    CHAT,
    GROUP,
}

impl Command {
    pub fn parse(name: &str) -> Option<Self> {
        match name.to_uppercase().as_str() {
            "CONNECT" => Some(Command::CONNECT),
            "QUIT" => Some(Command::QUIT),
            "WHO" => Some(Command::WHO),
            "CHAT" => Some(Command::CHAT),
            "GROUP" => Some(Command::GROUP),
            _ => None,
        }
    }
}

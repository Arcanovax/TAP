pub enum Command {
    CONNECT,
}

impl Command {
    pub fn parse(name: &str) -> Option<Self> {
        match name.to_uppercase().as_str() {
            "CONNECT" => Some(Command::CONNECT),
            _ => None,
        }
    }
}

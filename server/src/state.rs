use std::collections::HashSet;

pub struct ServerInfo {
    players: HashSet<String>,
}

impl ServerInfo {
    pub fn new() -> Self {
        ServerInfo {
            players: HashSet::new(),
        }
    }

    pub fn try_add_player(&mut self, name: String) -> bool {
        self.players.insert(name)
    }
}

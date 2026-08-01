use std::net::SocketAddr;

use uuid::Uuid;

pub struct Group {
    pub id: Uuid,
    pub name: String,
    pub players: Vec<SocketAddr>,
    pub group_leader: SocketAddr,
}

impl Group {
    pub fn new(name: &str, group_leader: SocketAddr) -> Self {
        Group {
            id: Uuid::new_v4(),
            name: String::from(name),
            players: Vec::new(),
            group_leader,
        }
    }

    pub fn get_group_size(&self) -> usize {
        self.players.len()
    }
}

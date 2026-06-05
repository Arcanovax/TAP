use std::{collections::HashMap, net::SocketAddr};

use crate::error::ErrorCode;

struct Connection {
    player_name: String,
}

pub struct ServerInfo {
    connections: HashMap<SocketAddr, Connection>,
}

impl ServerInfo {
    pub fn new() -> Self {
        ServerInfo {
            connections: HashMap::new(),
        }
    }

    pub fn try_add_player(&mut self, name: String, peer_addr: SocketAddr) -> Result<(), ErrorCode> {
        if self.connections.contains_key(&peer_addr) {
            return Err(ErrorCode::ALREADY_CONNECTED);
        }
        for (_, con) in self.connections.iter() {
            if con.player_name == name {
                return Err(ErrorCode::NAME_IN_USE);
            }
        }
        self.connections
            .insert(peer_addr, Connection { player_name: name });
        Ok(())
    }

    pub fn try_remove_player(&mut self, peer_addr: SocketAddr) -> Result<String, ErrorCode> {
        let con = self.connections.get(&peer_addr);
        if con.is_none() {
            return Err(ErrorCode::INVALID_COMMAND);
        }
        let name = con.unwrap().player_name.clone();
        self.connections.remove(&peer_addr);
        Ok(name)
    }
}

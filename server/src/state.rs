use crate::{error::ErrorCode, protocol::Message};
use std::{collections::HashMap, net::SocketAddr};
use tokio::sync::mpsc::UnboundedSender;

pub type Tx = UnboundedSender<Message>;

pub struct Connection {
    pub addr: SocketAddr,
    pub tx: Tx,
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

    pub fn try_add_player(
        &mut self,
        name: String,
        peer_addr: SocketAddr,
        tx: &Tx,
    ) -> Result<(), ErrorCode> {
        if self.connections.contains_key(&peer_addr) {
            return Err(ErrorCode::ALREADY_CONNECTED);
        }
        for (_, con) in self.connections.iter() {
            if con.player_name == name {
                return Err(ErrorCode::NAME_IN_USE);
            }
        }
        self.connections.insert(
            peer_addr,
            Connection {
                player_name: name,
                addr: peer_addr,
                tx: tx.clone(),
            },
        );
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

    pub fn get_number_of_players(&mut self) -> usize {
        self.connections.len()
    }

    pub fn get_global_receivers(&mut self, peer_addr: SocketAddr) -> Vec<&Connection> {
        let mut receivers = Vec::new();

        for (_, con) in &self.connections {
            if con.addr != peer_addr {
                receivers.push(con);
            }
        }
        receivers
    }

    pub fn is_connected(&mut self, peer_addr: SocketAddr) -> bool {
        self.connections.contains_key(&peer_addr)
    }
}

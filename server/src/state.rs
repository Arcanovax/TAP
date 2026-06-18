use crate::{
    protocol::{EventType, Message},
    structures::{fight::Fight, game::World, group::Group, player::Player, room::Room},
};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::sync::mpsc::UnboundedSender;
use tracing::info;
use uuid::Uuid;

mod broadcast;
mod group;
mod player;
mod quest;
mod world;

pub type Tx = UnboundedSender<Message>;
pub type SharedServer = Arc<Mutex<ServerInfo>>;

#[derive(Debug)]
pub struct Connection {
    pub addr: SocketAddr,
    pub tx: Tx,
    pub player: Player,
}

pub struct ServerInfo {
    pub connections: HashMap<SocketAddr, Connection>,
    name_to_addr: HashMap<String, SocketAddr>,
    groups: HashMap<Uuid, Group>,
    invitations: HashMap<SocketAddr, Uuid>,
    pub fights: HashMap<String, Fight>,
    pub world: World,
}

impl ServerInfo {
    pub fn new(world: World) -> Self {
        ServerInfo {
            connections: HashMap::new(),
            name_to_addr: HashMap::new(),
            groups: HashMap::new(),
            invitations: HashMap::new(),
            fights: HashMap::new(),
            world: world,
        }
    }
}

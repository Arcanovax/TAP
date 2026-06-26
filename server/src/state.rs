use crate::{
    protocol::{EventType, Message},
    structures::{
        enums::npc_kind::NPCKind, fight::Fight, game::World, group::Group, player::Player,
        room::Room,
    },
};
use redb::Database;
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
    invitations: HashMap<SocketAddr, HashMap<String, Uuid>>,
    pub fights: HashMap<String, Fight>,
    pub world: World,
    pub db: Arc<Database>,
}

impl ServerInfo {
    pub fn new(world: World, db: Arc<Database>) -> Self {
        ServerInfo {
            connections: HashMap::new(),
            name_to_addr: HashMap::new(),
            groups: HashMap::new(),
            invitations: HashMap::new(),
            fights: HashMap::new(),
            world: world,
            db: db,
        }
    }

    pub fn reset(&mut self, base_world: &World) {
        self.world.rooms = base_world.rooms.clone();

        for (id, npc) in &mut self.world.npcs {
            if let Some(base_npc) = base_world.npcs.get(id) {
                match &npc.kind {
                    NPCKind::Merchant { .. } => *npc = base_npc.clone(),
                    NPCKind::Enemy { defeated, .. } => {
                        if !*defeated {
                            continue;
                        }
                        *npc = base_npc.clone();
                    }
                    _ => {}
                }
            }
        }

        for con in self.connections.values() {
            let _ = con.tx.send(Message::Event(EventType::SERVER_RESET));
        }
    }
}

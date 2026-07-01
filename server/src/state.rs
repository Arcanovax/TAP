use crate::{
    protocol::{EventType, Message},
    structures::{
        dungeon::{Dungeon, parse_dungeon_id},
        enums::npc_kind::NPCKind,
        fight::Fight,
        game::World,
        group::Group,
        item::Item,
        npc::NPC,
        player::Player,
        room::{Owner, Room},
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
pub mod group;
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
    pub dungeons: HashMap<Uuid, Dungeon>,
}

impl ServerInfo {
    pub fn new(world: World, db: Arc<Database>) -> Self {
        ServerInfo {
            connections: HashMap::new(),
            name_to_addr: HashMap::new(),
            groups: HashMap::new(),
            invitations: HashMap::new(),
            fights: HashMap::new(),
            dungeons: HashMap::new(),
            world: world,
            db: db,
        }
    }

    pub fn reset(&mut self, base_world: &World) {
        for (id, room) in &mut self.world.rooms {
            if let Some(base_room) = base_world.rooms.get(id) {
                room.items.retain(|item| item.owner == Owner::Player);
                room.items.extend(base_room.items.clone());
            }
        }

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

    pub fn resolve_room(&self, id: &str) -> Option<&Room> {
        match parse_dungeon_id(id) {
            Some(gid) => match self.dungeons.get(&gid) {
                Some(dungeon) => dungeon.rooms.get(id),
                None => None,
            },
            None => self.world.rooms.get(id),
        }
    }

    pub fn resolve_item(&self, id: &str) -> Option<&Item> {
        match parse_dungeon_id(id) {
            Some(gid) => match self.dungeons.get(&gid) {
                Some(dungeon) => dungeon.items.get(id),
                None => None,
            },
            None => self.world.items.get(id),
        }
    }

    pub fn resolve_npc(&self, id: &str) -> Option<&NPC> {
        match parse_dungeon_id(id) {
            Some(gid) => match self.dungeons.get(&gid) {
                Some(dungeon) => dungeon.npcs.get(id),
                None => None,
            },
            None => self.world.npcs.get(id),
        }
    }

    pub fn resolve_room_mut(&mut self, id: &str) -> Option<&mut Room> {
        match parse_dungeon_id(id) {
            Some(gid) => match self.dungeons.get_mut(&gid) {
                Some(dungeon) => dungeon.rooms.get_mut(id),
                None => None,
            },
            None => self.world.rooms.get_mut(id),
        }
    }

    pub fn resolve_item_mut(&mut self, id: &str) -> Option<&mut Item> {
        match parse_dungeon_id(id) {
            Some(gid) => match self.dungeons.get_mut(&gid) {
                Some(dungeon) => dungeon.items.get_mut(id),
                None => None,
            },
            None => self.world.items.get_mut(id),
        }
    }

    pub fn resolve_npc_mut(&mut self, id: &str) -> Option<&mut NPC> {
        match parse_dungeon_id(id) {
            Some(gid) => match self.dungeons.get_mut(&gid) {
                Some(dungeon) => dungeon.npcs.get_mut(id),
                None => None,
            },
            None => self.world.npcs.get_mut(id),
        }
    }
}

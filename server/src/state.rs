use crate::{
    error::ErrorCode,
    game::World,
    group::Group,
    protocol::{EventType, Message},
    structures::{fight::Fight, player::Player, room::Room},
};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::sync::mpsc::UnboundedSender;
use tracing::info;
use uuid::Uuid;

pub type Tx = UnboundedSender<Message>;
pub type SharedServer = Arc<Mutex<ServerInfo>>;

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
            if con.player.name == name {
                return Err(ErrorCode::NAME_IN_USE);
            }
        }
        self.connections.insert(
            peer_addr,
            Connection {
                player: Player::new(name.clone()),
                addr: peer_addr,
                tx: tx.clone(),
            },
        );
        self.name_to_addr.insert(name, peer_addr);
        Ok(())
    }

    pub fn try_remove_player(&mut self, peer_addr: SocketAddr) -> Result<String, ErrorCode> {
        let con = self.connections.get(&peer_addr);
        if con.is_none() {
            return Err(ErrorCode::INVALID_COMMAND);
        }
        let name = con.unwrap().player.name.clone();
        self.connections.remove(&peer_addr);
        self.name_to_addr.remove(&name);
        self.cleanup_player_invitation(peer_addr);
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

    pub fn get_group_receivers(
        &mut self,
        peer_addr: SocketAddr,
    ) -> Result<Vec<&Connection>, ErrorCode> {
        let sender_con = self.get_connection(peer_addr)?;
        let group_id = sender_con.player.group_id.ok_or(ErrorCode::NOT_IN_GROUP)?;
        let receivers = self
            .groups
            .get(&group_id)
            .ok_or(ErrorCode::INVALID_COMMAND)?
            .players
            .iter()
            .filter_map(|addr| self.connections.get(addr))
            .filter(|con| con.addr != peer_addr)
            .collect();
        Ok(receivers)
    }

    pub fn get_room_receivers(&self, peer_addr: SocketAddr) -> Result<Vec<&Connection>, ErrorCode> {
        let room = &self.get_player(peer_addr)?.location;
        let mut receivers: Vec<&Connection> = Vec::new();
        for (_, con) in &self.connections {
            if con.addr != peer_addr && con.player.location == room.to_string() {
                receivers.push(con);
            }
        }
        Ok(receivers)
    }

    pub fn is_connected(&mut self, peer_addr: SocketAddr) -> bool {
        self.connections.contains_key(&peer_addr)
    }

    fn create_new_group(&mut self, name: &str) -> Uuid {
        let group = Group::new(name);
        let group_id = group.id.clone();
        self.groups.insert(group.id, group);
        group_id
    }

    pub fn try_add_player_to_group(
        &mut self,
        peer_addr: SocketAddr,
        group_id: Uuid,
    ) -> Result<(), ErrorCode> {
        let con = self
            .connections
            .get(&peer_addr)
            .ok_or(ErrorCode::INVALID_COMMAND)?;
        if con.player.group_id.is_some() {
            return Err(ErrorCode::ALREADY_IN_GROUP);
        }

        let con = self.connections.get_mut(&peer_addr).unwrap();
        con.player.group_id = Some(group_id);
        self.groups
            .get_mut(&group_id)
            .unwrap()
            .players
            .push(con.addr);
        info!("{} added to group({})", con.player.name, group_id);
        let player_name = con.player.name.clone();
        let receivers = self.get_group_receivers(peer_addr).unwrap();
        for c in receivers {
            let _ = c.tx.send(Message::Event(EventType::GROUP_JOIN {
                player_name: player_name.clone(),
            }));
        }
        Ok(())
    }

    pub fn try_create_group(
        &mut self,
        peer_addr: SocketAddr,
        group_name: &str,
    ) -> Result<(), ErrorCode> {
        let con = self
            .connections
            .get(&peer_addr)
            .ok_or(ErrorCode::INVALID_COMMAND)?;
        if con.player.group_id.is_some() {
            return Err(ErrorCode::ALREADY_IN_GROUP);
        }

        let name = con.player.name.clone();
        let group_id = self.create_new_group(group_name);
        info!("{} created group({}:{})", name, group_name, group_id);
        self.try_add_player_to_group(peer_addr, group_id)
    }

    pub fn cleanup_player_invitation(&mut self, peer_addr: SocketAddr) {
        self.invitations.remove_entry(&peer_addr);
    }

    fn cleanup_group_invitation(&mut self, group_id: Uuid) {
        self.invitations.retain(|_, gid| *gid != group_id);
    }

    fn try_delete_group(&mut self, group_id: Uuid) {
        let group = self.groups.get(&group_id).ok_or(()).unwrap();
        if group.get_group_size() == 0 {
            self.groups.remove(&group_id);
            self.cleanup_group_invitation(group_id);
            info!("group({}) deleted", group_id);
        }
    }

    pub fn try_leave_group(&mut self, peer_addr: SocketAddr) -> Result<(), ErrorCode> {
        let con = self.get_connection(peer_addr)?;
        if con.player.group_id.is_none() {
            return Err(ErrorCode::NOT_IN_GROUP);
        }

        let player_name = con.player.name.clone();
        let receivers = self.get_group_receivers(peer_addr).unwrap();
        for c in receivers {
            let _ = c.tx.send(Message::Event(EventType::GROUP_LEAVE {
                player_name: player_name.clone(),
            }));
        }

        let con = self.connections.get_mut(&peer_addr).unwrap();
        let group_id = con.player.group_id.unwrap();
        self.groups
            .get_mut(&group_id)
            .unwrap()
            .players
            .retain(|&addr| addr != con.addr);
        info!("{} leaved group({})", con.player.name, group_id);
        con.player.group_id = None;
        self.try_delete_group(group_id);
        Ok(())
    }

    pub fn get_player(&self, peer_addr: SocketAddr) -> Result<&Player, ErrorCode> {
        Ok(&self.get_connection(peer_addr)?.player)
    }

    pub fn get_connection(&self, peer_addr: SocketAddr) -> Result<&Connection, ErrorCode> {
        let con = self
            .connections
            .get(&peer_addr)
            .ok_or(ErrorCode::INVALID_COMMAND)?;
        Ok(con)
    }

    pub fn get_player_mut(&mut self, peer_addr: SocketAddr) -> Result<&mut Player, ErrorCode> {
        Ok(&mut self.get_connection_mut(peer_addr)?.player)
    }

    pub fn get_connection_mut(
        &mut self,
        peer_addr: SocketAddr,
    ) -> Result<&mut Connection, ErrorCode> {
        let con = self
            .connections
            .get_mut(&peer_addr)
            .ok_or(ErrorCode::INVALID_COMMAND)?;
        Ok(con)
    }

    fn get_name_addr(&self, name: String) -> Result<&SocketAddr, ErrorCode> {
        let addr = self
            .name_to_addr
            .get(&name)
            .ok_or(ErrorCode::INVALID_ARGS)?;
        Ok(addr)
    }

    pub fn try_invite_group(
        &mut self,
        receiver_name: String,
        peer_addr: SocketAddr,
    ) -> Result<(), ErrorCode> {
        let con = self.get_connection(peer_addr)?;
        if con.player.group_id.is_none() {
            return Err(ErrorCode::NOT_IN_GROUP);
        }
        let inviter_name = con.player.name.clone();
        let group_id = con.player.group_id.unwrap();
        let group_name = &self.groups.get(&group_id).unwrap().name;

        let receiver_con = self.get_connection(*self.get_name_addr(receiver_name)?)?;
        let receiver_addr = receiver_con.addr;
        let receiver_tx = receiver_con.tx.clone();
        if peer_addr == receiver_addr {
            return Err(ErrorCode::INVALID_ARGS);
        }
        self.invitations.insert(receiver_addr, group_id);
        let _ = receiver_tx.send(Message::Event(EventType::INVITE {
            sender: inviter_name,
            group_name: String::from(group_name),
        }));
        Ok(())
    }

    pub fn try_join_group(&mut self, peer_addr: SocketAddr) -> Result<(), ErrorCode> {
        let group_id = *self
            .invitations
            .get(&peer_addr)
            .ok_or(ErrorCode::INVALID_COMMAND)?;
        match self.try_add_player_to_group(peer_addr, group_id) {
            Ok(()) => {
                self.invitations.remove_entry(&peer_addr);
                Ok(())
            }
            Err(code) => Err(code),
        }
    }

    pub fn try_get_group_list(&self, peer_addr: SocketAddr) -> Result<Vec<String>, ErrorCode> {
        let con = self.get_connection(peer_addr)?;
        if con.player.group_id.is_none() {
            return Err(ErrorCode::NOT_IN_GROUP);
        }
        let mut group_members = Vec::new();
        for addr in &self
            .groups
            .get(&con.player.group_id.unwrap())
            .unwrap()
            .players
        {
            group_members.push(self.get_connection(*addr)?.player.name.clone());
        }
        Ok(group_members)
    }

    pub fn get_player_room(&self, peer_addr: SocketAddr) -> Result<&Room, ErrorCode> {
        let room_name = &self.get_player(peer_addr)?.location;

        let room = match self.world.rooms.get(room_name) {
            Some(room) => room,
            None => return Err(ErrorCode::ROOM_NOT_FOUND),
        };
        Ok(room)
    }

    pub fn try_drop_item(
        &mut self,
        peer_addr: SocketAddr,
        item: &str,
    ) -> Result<String, ErrorCode> {
        if !self.connections.contains_key(&peer_addr) {
            return Err(ErrorCode::PLAYER_NOT_FOUND);
        }
        let con = self.connections.get_mut(&peer_addr).unwrap();
        match con.player.inventory.get_mut(item) {
            Some(count) => {
                *count -= 1;
                if *count == 0 {
                    con.player.inventory.remove(item);
                }
            }
            None => return Err(ErrorCode::ITEM_NOT_IN_INVENTORY),
        }
        self.world
            .rooms
            .get_mut(&con.player.location)
            .unwrap()
            .items
            .push(String::from(item));
        Ok(String::from(item))
    }

    pub fn try_take_item(
        &mut self,
        peer_addr: SocketAddr,
        item: &str,
    ) -> Result<String, ErrorCode> {
        if !self.connections.contains_key(&peer_addr) {
            return Err(ErrorCode::PLAYER_NOT_FOUND);
        }
        let con = self.connections.get_mut(&peer_addr).unwrap();
        let room = match self.world.rooms.get_mut(&con.player.location) {
            Some(room) => room,
            None => return Err(ErrorCode::ROOM_NOT_FOUND),
        };
        for (i, value) in room.items.iter().enumerate() {
            if item == value {
                room.items.remove(i);
                *con.player.inventory.entry(item.to_string()).or_insert(0) += 1;
                return Ok(String::from(item));
            }
        }
        Err(ErrorCode::ITEM_NOT_FOUND)
    }

    pub fn try_accept_quest(
        &mut self,
        peer_addr: SocketAddr,
        npc_name: &str,
    ) -> Result<String, ErrorCode> {
        let npc = match self.world.npcs.get(npc_name) {
            Some(npc) => npc,
            None => return Err(ErrorCode::NPC_NOT_FOUND),
        };
        if let None = npc.quest {
            return Err(ErrorCode::NO_QUEST_AVAILABLE);
        }
        let quest_ref = npc.quest.clone().unwrap();
        let player = self.get_player_mut(peer_addr)?;
        if player.finished_quest.contains(&quest_ref)
            || player.quests_in_progress.contains_key(&quest_ref)
        {
            return Err(ErrorCode::NO_QUEST_AVAILABLE);
        }
        player.quests_in_progress.insert(quest_ref.clone(), 0);
        let quest = self.world.quests.get(&quest_ref).unwrap();
        Ok(serde_json::to_string(quest).unwrap())
    }
}

use super::*;
use crate::{
    persistence::players::{load_player, save_player},
    structures::{enums::error::ErrorCode, room::OwnedItem},
};

impl ServerInfo {
    pub fn try_add_player(
        &mut self,
        name: String,
        peer_addr: SocketAddr,
        tx: &Tx,
    ) -> Result<String, ErrorCode> {
        if self.connections.contains_key(&peer_addr) {
            return Err(ErrorCode::ALREADY_CONNECTED);
        }
        for (_, con) in self.connections.iter() {
            if con.player.name == name {
                return Err(ErrorCode::NAME_IN_USE);
            }
        }
        let db = self.db.clone();
        let player = match load_player(&db, name.as_str()) {
            Ok(Some(mut player)) => {
                info!("{} player data loaded", name);
                if !self.world.rooms.contains_key(&player.location) {
                    player.location = self.world.spawn_room.clone();
                }
                player
                    .inventory
                    .retain(|item, _| self.world.items.contains_key(item));
                player
            }
            Ok(None) => Player::new(name.clone()),
            Err(_) => return Err(ErrorCode::CONNECTION_FAILED),
        };
        self.connections.insert(
            peer_addr,
            Connection {
                player,
                addr: peer_addr,
                tx: tx.clone(),
            },
        );
        self.name_to_addr.insert(name, peer_addr);
        Ok("connected".to_string())
    }

    pub fn try_remove_player(&mut self, peer_addr: SocketAddr) -> Result<String, ErrorCode> {
        let con = self.connections.get(&peer_addr);
        if con.is_none() {
            return Err(ErrorCode::INVALID_COMMAND);
        }
        let name = con.unwrap().player.name.clone();
        if let Ok(receivers) = self.get_room_receivers(peer_addr) {
            for con in receivers {
                let _ = con.tx.send(Message::Event(EventType::ROOM_LEAVE {
                    player_name: name.clone(),
                }));
            }
        }
        self.cleanup_player_invitation(peer_addr, &name);
        self.connections.remove(&peer_addr);
        self.name_to_addr.remove(&name);
        Ok(name)
    }

    pub fn get_number_of_players(&self) -> usize {
        self.connections.len()
    }

    pub fn send_players_event(&mut self, peer_addr: SocketAddr) {
        let players = self.get_number_of_players();
        let receivers = self.get_global_receivers(peer_addr);

        for con in receivers {
            let _ = con
                .tx
                .send(Message::Event(EventType::STATS_PLAYERS { players }));
        }
    }

    pub fn get_player(&self, peer_addr: SocketAddr) -> Result<&Player, ErrorCode> {
        Ok(&self.get_connection(peer_addr)?.player)
    }

    pub fn get_player_mut(&mut self, peer_addr: SocketAddr) -> Result<&mut Player, ErrorCode> {
        Ok(&mut self.get_connection_mut(peer_addr)?.player)
    }

    pub fn try_drop_item(
        &mut self,
        peer_addr: SocketAddr,
        item: &str,
    ) -> Result<String, ErrorCode> {
        if !self.connections.contains_key(&peer_addr) {
            return Err(ErrorCode::INVALID_COMMAND);
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
            .push(OwnedItem {
                item: item.to_string(),
                owner: Owner::Player,
            });
        Ok(String::from(item))
    }

    pub fn try_take_item(
        &mut self,
        peer_addr: SocketAddr,
        item: &str,
    ) -> Result<String, ErrorCode> {
        if !self.connections.contains_key(&peer_addr) {
            return Err(ErrorCode::INVALID_COMMAND);
        }
        let con = self.connections.get_mut(&peer_addr).unwrap();
        let room = match self.world.rooms.get_mut(&con.player.location) {
            Some(room) => room,
            None => return Err(ErrorCode::ROOM_NOT_FOUND),
        };
        for (i, value) in room.items.iter().enumerate() {
            if item == value.item {
                room.items.remove(i);
                *con.player.inventory.entry(item.to_string()).or_insert(0) += 1;
                return Ok(String::from(item));
            }
        }
        Err(ErrorCode::ITEM_NOT_FOUND)
    }

    pub fn try_save_player(&self, peer_addr: SocketAddr) -> Result<(), ErrorCode> {
        let player = self.get_player(peer_addr)?;
        let db = self.db.clone();
        match save_player(&db, &player) {
            Ok(()) => {}
            Err(_) => return Err(ErrorCode::DISCONNECTION_FAIL),
        };
        Ok(())
    }
}

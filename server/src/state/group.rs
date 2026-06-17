use serde_json::{Value, json};

use super::*;

impl ServerInfo {
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
    ) -> Result<Value, ErrorCode> {
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
        Ok(json!({ "group": group_id }))
    }

    pub fn try_create_group(
        &mut self,
        peer_addr: SocketAddr,
        group_name: &str,
    ) -> Result<Value, ErrorCode> {
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
        self.try_add_player_to_group(peer_addr, group_id)?;
        Ok(json!({ "group": group_id }))
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

    pub fn try_join_group(&mut self, peer_addr: SocketAddr) -> Result<Value, ErrorCode> {
        let group_id = *self
            .invitations
            .get(&peer_addr)
            .ok_or(ErrorCode::INVALID_COMMAND)?;
        match self.try_add_player_to_group(peer_addr, group_id) {
            Ok(value) => {
                self.invitations.remove_entry(&peer_addr);
                Ok(value)
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
}

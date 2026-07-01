
use super::*;

use crate::handlers::fight::enemy_attack::enemy_attack;
use crate::structures::enums::error::ErrorCode;
use crate::structures::enums::state::State;

impl ServerInfo {
    fn create_new_group(&mut self, name: &str, group_leader: SocketAddr) -> Uuid {
        let group = Group::new(name, group_leader);
        let group_id = group.id.clone();
        self.groups.insert(group.id, group);
        group_id
    }

    pub fn try_add_player_to_group(
        &mut self,
        peer_addr: SocketAddr,
        group_id: Uuid,
    ) -> Result<String, ErrorCode> {
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
        Ok(group_id.to_string())
    }

    pub fn try_create_group(
        &mut self,
        peer_addr: SocketAddr,
        group_name: &str,
    ) -> Result<String, ErrorCode> {
        let con = self
            .connections
            .get(&peer_addr)
            .ok_or(ErrorCode::INVALID_COMMAND)?;
        if con.player.group_id.is_some() {
            return Err(ErrorCode::ALREADY_IN_GROUP);
        }

        let name = con.player.name.clone();
        let group_id = self.create_new_group(group_name, peer_addr);
        info!("{} created group({}:{})", name, group_name, group_id);
        match self.try_add_player_to_group(peer_addr, group_id) {
            Ok(_) => {}
            Err(code) => {
                self.try_delete_group(group_id);
                return Err(code);
            }
        };
        Ok(group_id.to_string())
    }

    pub fn cleanup_player_invitation(&mut self, peer_addr: SocketAddr, player_name: &str) {
        self.invitations.remove_entry(&peer_addr);
        self.invitations.retain(|_, invites| {
            invites.retain(|name, _| *name != player_name);
            !invites.is_empty()
        });
    }

    fn cleanup_group_invitation(&mut self, group_id: Uuid) {
        self.invitations.retain(|_, invites| {
            invites.retain(|_, gid| *gid != group_id);
            !invites.is_empty()
        });
    }

    fn try_delete_group(&mut self, group_id: Uuid) -> bool {
        let group = self.groups.get(&group_id).unwrap();
        if group.get_group_size() == 0 {
            self.groups.remove(&group_id);
            self.cleanup_group_invitation(group_id);
            info!("group({}) deleted", group_id);
            return true;
        }
        return false;
    }

	pub fn try_leave_fight(&mut self, peer_addr: SocketAddr, target: String) -> Result<(), ErrorCode> {
        let player_name = {
			let connection = self.get_connection_mut(peer_addr)?;
			connection.player.status = State::Idle;
			connection.player.name.clone()
		};

        let receivers = {
			let fighters = &mut self.fights.get_mut(&target).unwrap().fighters;
			fighters.retain(|f| f != &player_name);
			fighters.clone()
		};
		let nb_receivers = receivers.len();

		if nb_receivers > 0 {
			for name in receivers {
				if let Some(con) = self.connections.values().find(|pl_conn| pl_conn.player.name == name) {
					let _ = con.tx.send(Message::Event(EventType::FIGHT_LEAVE { player_name: player_name.clone() }));
				}
			}
	
			let fight_turn = {
				let turn = self.fights.get(&target).unwrap().turn;
				turn.clone()
			};
	
			if nb_receivers == fight_turn  as usize {
				enemy_attack(&target, self);
			}
		} else {
			let enemy = self.world.npcs.get_mut(&target).unwrap();

			if let NPCKind::Enemy {
				ref mut hp,
				max_hp,
				..
			} = enemy.kind {
				*hp = max_hp;
			}
			self.fights.remove(&target);
		}
        Ok(())
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
        let group_leader = self.groups.get(&group_id).unwrap().group_leader;
        let is_deleted = self.try_delete_group(group_id);
        if !is_deleted && group_leader == peer_addr {
            let group = self.groups.get_mut(&group_id).unwrap();
            info!(
                "{}({}) leadership switched to {}",
                group.name, group.id, group.players[0]
            );
            group.group_leader = group.players[0];
        }
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
        let (group_name, group_leader) = {
            let group = &self.groups.get(&group_id).unwrap();
            (group.name.clone(), group.group_leader)
        };

        if group_leader != peer_addr {
            return Err(ErrorCode::NOT_GROUP_LEADER);
        }

        let receiver_con = self.get_connection(*self.get_name_addr(receiver_name)?)?;

        if receiver_con.player.group_id.is_some() {
            return Err(ErrorCode::ALREADY_IN_GROUP);
        }

        let receiver_addr = receiver_con.addr;
        let receiver_tx = receiver_con.tx.clone();

        let invitations = self
            .invitations
            .entry(receiver_addr)
            .or_insert_with(HashMap::new);
        if invitations.get(&inviter_name) == Some(&group_id) {
            return Err(ErrorCode::ALREADY_INVITED);
        }
        invitations.insert(inviter_name.clone(), group_id);
        let _ = receiver_tx.send(Message::Event(EventType::GROUP_INVITE {
            sender: inviter_name,
            group_name: String::from(group_name),
        }));
        Ok(())
    }

    pub fn try_join_group(
        &mut self,
        peer_addr: SocketAddr,
        leader_name: String,
    ) -> Result<String, ErrorCode> {
        let group_id = *self
            .invitations
            .get(&peer_addr)
            .ok_or(ErrorCode::INVALID_COMMAND)?
            .get(&leader_name)
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

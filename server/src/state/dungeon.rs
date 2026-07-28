use crate::{
    dungeon::generation::generate_dungeon,
    structures::{dungeon::format_dungeon_id, enums::error::ErrorCode},
};

use super::*;

impl ServerInfo {
    pub fn try_create_dungeon(&mut self, peer_addr: SocketAddr) -> Result<(), ErrorCode> {
        let player = self.get_player(peer_addr)?;

        if player.location != self.world.dungeon_entrance {
            return Err(ErrorCode::FORBIDDEN_ACTION);
        }

        match self.try_create_group(peer_addr, format!("{}'s group", player.name).as_str()) {
            Ok(_) | Err(ErrorCode::ALREADY_IN_GROUP) => {}
            Err(code) => return Err(code),
        };

        let player = self.get_player(peer_addr)?;
        let Some(gid) = player.group_id else {
            return Err(ErrorCode::NOT_IN_GROUP);
        };

        if let Some(group) = self.groups.get(&gid)
            && group.group_leader != peer_addr
        {
            return Err(ErrorCode::NOT_GROUP_LEADER);
        }

        if self.dungeons.contains_key(&gid) {
            return Err(ErrorCode::DUNGEON_ALREADY_IN_PROGRESS);
        }

        let room_receivers: Vec<_> = self
            .get_room_receivers(peer_addr)?
            .iter()
            .map(|con| con.tx.clone())
            .collect();
        let player_name = player.name.clone();
        let dungeon = generate_dungeon(&self.base_world, gid);
        self.dungeons.insert(gid, dungeon);

        let player = self.get_player_mut(peer_addr)?;
        player.location = format_dungeon_id("room", gid, 0);

        for con in self.get_group_receivers(peer_addr)? {
            let _ = con.tx.send(Message::Event(EventType::DUNGEON_CREATE));
        }

        for tx in room_receivers {
            let _ = tx.send(Message::Event(EventType::ROOM_LEAVE {
                player_name: player_name.clone(),
            }));
        }

        Ok(())
    }

    pub fn try_join_dungeon(&mut self, peer_addr: SocketAddr) -> Result<(), ErrorCode> {
        let player = self.get_player(peer_addr)?;

        if player.location != self.world.dungeon_entrance {
            return Err(ErrorCode::FORBIDDEN_ACTION);
        }

        if parse_dungeon_id(&player.location).is_some() {
            return Err(ErrorCode::DUNGEON_ALREADY_IN_PROGRESS);
        };

        let Some(gid) = player.group_id else {
            return Err(ErrorCode::NOT_IN_GROUP);
        };

        let Some(_) = self.dungeons.get(&gid) else {
            return Err(ErrorCode::NO_DUNGEON_IN_PROGRESS);
        };

        let player_name = player.name.clone();

        for con in self.get_room_receivers(peer_addr)? {
            let _ = con.tx.send(Message::Event(EventType::ROOM_LEAVE {
                player_name: player_name.clone(),
            }));
        }

        let player = self.get_player_mut(peer_addr)?;
        player.location = format_dungeon_id("room", gid, 0);

        for con in self.get_room_receivers(peer_addr)? {
            let _ = con.tx.send(Message::Event(EventType::ROOM_JOIN {
                player_name: player_name.clone(),
            }));
        }

        Ok(())
    }

    pub fn close_dungeon(&mut self, gid: Uuid) {
        let entrance = self.world.dungeon_entrance.clone();

        let addrs = match self.groups.get(&gid) {
            Some(group) => group.players.clone(),
            None => Vec::new(),
        };

        let receiver_txs: Vec<_> = self
            .connections
            .values()
            .filter(|con| con.player.location == entrance)
            .map(|con| con.tx.clone())
            .collect();

        for addr in addrs {
            if let Some(con) = self.connections.get_mut(&addr) {
                con.player.location = entrance.clone();
                for tx in &receiver_txs {
                    let _ = tx.send(Message::Event(EventType::ROOM_JOIN {
                        player_name: con.player.name.clone(),
                    }));
                }
            }
        }

        self.dungeons.remove(&gid);
        info!(dungeon = %gid, "dungeon deleted");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{
        addr, connect, dg_room, group_with, populated_server, test_dungeon, test_gid,
    };

    // --- close_dungeon (la fonction elle-même) ---

    #[test]
    fn close_dungeon_removes_the_dungeon() {
        let server = populated_server();
        let mut guard = server.lock().unwrap();
        let gid = test_gid();
        guard.dungeons.insert(gid, test_dungeon());

        guard.close_dungeon(gid);

        assert!(!guard.dungeons.contains_key(&gid));
    }

    #[test]
    fn close_dungeon_relocates_members_to_spawn() {
        let server = populated_server();
        let _rxs = group_with(&server, &[(addr(1), "alice"), (addr(2), "bob")]);
        let mut guard = server.lock().unwrap();

        let gid = guard.get_player(addr(1)).unwrap().group_id.unwrap();
        guard.dungeons.insert(gid, test_dungeon());
        for a in [addr(1), addr(2)] {
            guard.get_player_mut(a).unwrap().location = dg_room(0);
        }

        guard.close_dungeon(gid);

        let spawn = guard.world.dungeon_entrance.clone();
        assert_eq!(guard.get_player(addr(1)).unwrap().location, spawn);
        assert_eq!(guard.get_player(addr(2)).unwrap().location, spawn);
    }

    #[test]
    fn close_dungeon_notifies_players_at_spawn() {
        let server = populated_server();
        let _rxs = group_with(&server, &[(addr(1), "alice"), (addr(2), "bob")]);
        let mut witness_rx = connect(&server, addr(3), "witness");
        let mut guard = server.lock().unwrap();

        // le témoin reste au spawn, les deux membres sont dans le donjon
        let spawn = guard.world.dungeon_entrance.clone();
        guard.get_player_mut(addr(3)).unwrap().location = spawn;
        let gid = guard.get_player(addr(1)).unwrap().group_id.unwrap();
        guard.dungeons.insert(gid, test_dungeon());
        for a in [addr(1), addr(2)] {
            guard.get_player_mut(a).unwrap().location = dg_room(0);
        }
        while witness_rx.try_recv().is_ok() {} // on ignore le bruit d'installation

        guard.close_dungeon(gid);
        drop(guard);

        let mut joins = 0;
        while let Ok(msg) = witness_rx.try_recv() {
            if matches!(msg, Message::Event(EventType::ROOM_JOIN { .. })) {
                joins += 1;
            }
        }
        assert_eq!(joins, 2, "le témoin doit voir arriver les 2 membres");
    }

    #[test]
    fn close_dungeon_on_missing_dungeon_is_a_noop() {
        let server = populated_server();
        let mut guard = server.lock().unwrap();
        // aucun donjon inséré : l'appel ne doit ni paniquer ni créer d'état
        guard.close_dungeon(test_gid());
        assert!(guard.dungeons.is_empty());
    }

    // --- Déclencheur 1 : suppression du groupe ---

    #[test]
    fn deleting_last_group_member_closes_open_dungeon() {
        let server = populated_server();
        let _rx = connect(&server, addr(1), "alice");
        let mut guard = server.lock().unwrap();

        guard.try_create_group(addr(1), "grp").unwrap();
        let gid = guard.get_player(addr(1)).unwrap().group_id.unwrap();
        guard.dungeons.insert(gid, test_dungeon());
        guard.get_player_mut(addr(1)).unwrap().location = dg_room(0);

        // alice quitte : le groupe devient vide -> supprimé -> donjon fermé
        guard.try_leave_group(addr(1)).unwrap();

        assert!(!guard.dungeons.contains_key(&gid));
    }
}

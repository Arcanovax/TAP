use crate::{
    dungeon::generation::generate_dungeon,
    structures::{dungeon::format_dungeon_id, enums::error::ErrorCode},
};

use super::*;

impl ServerInfo {
    pub fn try_create_dungeon(&mut self, peer_addr: SocketAddr) -> Result<(), ErrorCode> {
        let player = self.get_player(peer_addr)?;
        match self.try_create_group(peer_addr, format!("{}'s group", player.name).as_str()) {
            Ok(_) | Err(ErrorCode::ALREADY_IN_GROUP) => {}
            Err(code) => return Err(code),
        };

        let player = self.get_player(peer_addr)?;
        let Some(gid) = player.group_id else {
            return Err(ErrorCode::NOT_IN_GROUP);
        };

        if let Some(group) = self.groups.get(&gid) {
            if group.group_leader != peer_addr {
                return Err(ErrorCode::NOT_GROUP_LEADER);
            }
        }

        if let Some(_) = self.dungeons.get(&gid) {
            return Err(ErrorCode::DUNGEON_ALREADY_IN_PROGRESS);
        }

        let dungeon = generate_dungeon(&self.base_world, gid);
        self.dungeons.insert(gid, dungeon);

        let player = self.get_player_mut(peer_addr)?;
        player.location = format_dungeon_id("room", gid, 0);

        for con in self.get_group_receivers(peer_addr)? {
            let _ = con.tx.send(Message::Event(EventType::DUNGEON_CREATE));
        }

        Ok(())
    }

    pub fn try_join_dungeon(&mut self, peer_addr: SocketAddr) -> Result<(), ErrorCode> {
        let player = self.get_player(peer_addr)?;

        if let Some(_) = parse_dungeon_id(&player.location) {
            return Err(ErrorCode::DUNGEON_ALREADY_IN_PROGRESS);
        };

        let Some(gid) = player.group_id else {
            return Err(ErrorCode::NOT_IN_GROUP);
        };

        let Some(_) = self.dungeons.get(&gid) else {
            return Err(ErrorCode::NO_DUNGEON_IN_PROGRESS);
        };

        let player = self.get_player_mut(peer_addr)?;
        player.location = format_dungeon_id("room", gid, 0);

        Ok(())
    }
}

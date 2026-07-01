use std::collections::HashMap;

use uuid::Uuid;

use crate::structures::{item::Item, npc::NPC, room::Room};

#[derive(Debug)]
pub struct Dungeon {
    pub rooms: HashMap<String, Room>,
    pub npcs: HashMap<String, NPC>,
    pub items: HashMap<String, Item>,
    pub name_to_ref: HashMap<String, String>,
}

pub fn parse_dungeon_id(id: &str) -> Option<Uuid> {
    match id.split('_').collect::<Vec<_>>().as_slice() {
        [prefix, gid, _] => match Uuid::parse_str(gid) {
            Ok(uuid) if *prefix == "room.dg" => Some(uuid),
            _ => None,
        },
        _ => None,
    }
}

pub fn format_dungeon_id(gid: Uuid, n: u8) -> String {
    format!("room.dg_{gid}_{n}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn round_trip_dungeon_success() {
        let gid = Uuid::new_v4();
        let format = format_dungeon_id(gid, 2);
        let parsed_gid = parse_dungeon_id(&format);
        assert_eq!(parsed_gid, Some(gid))
    }
}

use uuid::Uuid;

pub fn parse_dungeon_id(id: &str) -> Option<Uuid> {
	match id.split('_').collect::<Vec<_>>().as_slice() {
        [prefix, gid, _] => match prefix.split_once('.') {
            Some((_, "dg")) => Uuid::parse_str(gid).ok(),
            _ => None,
        },
        _ => None,
    }
}
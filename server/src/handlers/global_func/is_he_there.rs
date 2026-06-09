use crate::structures::room::Room;

pub fn is_he_there(name: &str, player_loc: &Room) -> bool {
    if player_loc.npc.len() != 0 {
        for npc in &player_loc.npc {
            if npc == name {
                return true;
            }
        }
        return false;
    } else {
        return false;
    };
}


use crate::structures::location::Location;

pub fn is_he_there(name: &str, player_loc: &Location) -> bool {
    if let Some(npcs) = &player_loc.npc {
        for npc in npcs {
            if *npc == name {
                return true;
            }
        }
        return false;
    } else {
        return false;
    };
}
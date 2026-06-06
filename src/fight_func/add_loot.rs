use std::collections::HashMap;

use crate::structures::{enums::npc_kind::NPCKind, fight::Fight, npc::NPC, player::Player};

pub fn add_loot<'a>(fight: &mut Fight, list_players: &mut HashMap<String, Player>, enemy: &mut NPC){
    for player_id in fight.fighters.clone() {
        if let Some(player) = list_players.get_mut(&player_id) {
            if let NPCKind::Enemy { ref loot, .. } = enemy.kind {
                for loot_item in loot {
                    player.inventory.entry(loot_item.clone()).and_modify(|number| if *loot_item != "item.gold" {*number += 1} else {*number += 50}).or_insert(1);
                }
            }
        }
    }
}
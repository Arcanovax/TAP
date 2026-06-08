use std::{net::SocketAddr, sync::MutexGuard};

use crate::{state::{ServerInfo, SharedServer}, structures::enums::{attack_res::AttackRes, item_kind::ItemKind, npc_kind::NPCKind, state::State}};

pub fn execute_attack<'a>(player_id: SocketAddr, target_id: Vec<String>, world: &mut MutexGuard<'_, ServerInfo>) -> AttackRes<'a> {
    
	// let mut pre_world_mut = world.lock().unwrap();
    let world_mut = &mut *world;
    let player = world_mut.connections.get_mut(&player_id).unwrap();
    let enemy = world_mut.npcs.get_mut(&target_id[0]).unwrap();
    let fight = world_mut.fights.get_mut(&target_id[0]).unwrap();
    
    let mut curr_damages: u32 = 15;
    for id in player.inventory.keys() {
        if let Some(item) = world_mut.items.get(id) {
            if let ItemKind::Weapon { damages } = item.kind {
                if curr_damages < damages { curr_damages = damages; }
            }
        }
    }

    if let NPCKind::Enemy { ref mut hp, ref loot, ref mut defeated, .. } = enemy.kind {
        if curr_damages < *hp {
            *hp -= curr_damages;
            if fight.turn == fight.fighters.len() as u32 - 1 {
                fight.enemy_turn = true;
                fight.turn = 0;
            } else {
                fight.turn += 1;
            }
            return AttackRes::Hit(format!("{} hits for {}", player.name, curr_damages));
        } else {
            *hp = 0;
			*defeated = true;
			for pl_name in &mut fight.fighters {
				let pl = world_mut.connections.get_mut(pl_name).unwrap();
				pl.status = State::Idle;
				for loot_item in loot {
					let amount = if loot_item == "item.gold" { 50 } else { 1 };
					*pl.inventory.entry(loot_item.clone()).or_insert(1) += amount;
				}
			}

            return AttackRes::KillTarget(format!("Target killed! Loot acquired."));
        }
    }
    AttackRes::Peace("Not an enemy")
}
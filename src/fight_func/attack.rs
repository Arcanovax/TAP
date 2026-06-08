use crate::structures::{enums::{attack_res::AttackRes, item_kind::ItemKind, npc_kind::NPCKind, state::State}, world::World};

pub fn execute_attack<'a>(player_id: &str, target_id: &str, world: &mut World) -> AttackRes<'a> {
    
    let player = world.players.get_mut(player_id).unwrap();
    let enemy = world.npcs.get_mut(target_id).unwrap();
    let fight = world.fights.get_mut(target_id).unwrap();
    
    let mut curr_damages: u32 = 15;
    for id in player.inventory.keys() {
        if let Some(item) = world.items.get(id) {
            if let ItemKind::Weapon { damages } = item.kind {
                if curr_damages < damages { curr_damages = damages; }
            }
        }
    }

    if let NPCKind::Enemy { ref mut hp, ref loot, ref mut beaten, .. } = enemy.kind {
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
			*beaten = true;
			for pl_name in &mut fight.fighters {
				let mut pl = world.players.get_mut(pl_name).unwrap();
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
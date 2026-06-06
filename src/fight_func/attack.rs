use crate::structures::{enums::{attack_res::AttackRes, item_kind::ItemKind, npc_kind::NPCKind, state::State}, world::World};

// Dans un module combat_system.rs
pub fn execute_attack(player_id: &str, target_id: &str, world: &mut World) -> AttackRes<'_> {
    
    let player = world.players.get_mut(player_id).unwrap();
    let enemy = world.npcs.get_mut(target_id).unwrap();
    let fight = world.fights.get_mut(target_id).unwrap();
    
    // 1. Calcul des dégâts
    let mut curr_damages: u32 = 15;
    for id in player.inventory.keys() {
        if let Some(item) = world.items.get(id) {
            if let ItemKind::Weapon { damages } = item.kind {
                if curr_damages < damages { curr_damages = damages; }
            }
        }
    }

    // 2. Application des dégâts
    if let NPCKind::Enemy { ref mut hp, ref loot, .. } = enemy.kind {
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
            player.status = State::Idle;
            
            // 3. Application immédiate du loot puisque "player" est déjà accessible
            for loot_item in loot {
                let amount = if loot_item == "item.gold" { 50 } else { 1 };
                *player.inventory.entry(loot_item.clone()).or_insert(1) += amount;
            }
            return AttackRes::KillTarget(format!("Target killed! Loot acquired."));
        }
    }
    AttackRes::Peace("Not an enemy")
}
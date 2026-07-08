use std::time::{SystemTime, UNIX_EPOCH};

use tracing::info;

use crate::{
    protocol::{EventType, Message},
    state::ServerInfo,
    structures::enums::{item_kind::ItemKind, npc_kind::NPCKind, state::State},
};

pub fn enemy_attack(opponent_id: &str, world: &mut ServerInfo) {
    let mut target_index: usize = 0;

    let (list_fighters, opponent_kind) = {
        (
            world.fights.get_mut(opponent_id).unwrap().fighters.clone(),
            world.resolve_npc_mut(opponent_id).unwrap().kind.clone(),
        )
    };

    let nb_fighters = list_fighters.len();
    let mut target_killed = false;

    let target_name = {
        if nb_fighters == 1 {
            list_fighters[0].clone()
        } else {
            let nanos = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .subsec_nanos() as usize;
            target_index = nanos % nb_fighters;
            list_fighters[target_index].clone()
        }
    };

    let defense = {
        let mut start_defense: u32 = 0;
        let inventory = world
            .connections
            .values()
            .find(|f| f.player.name == target_name)
            .unwrap()
            .player
            .inventory
            .clone();
        for id in inventory.keys() {
            if let Some(item) = world.resolve_item(id) {
                if let ItemKind::Armor { protection } = item.kind {
                    if protection > start_defense {
                        start_defense = protection;
                    }
                }
            }
        }
        start_defense
    };

    let (target_hp, e_damages) = {
        //Astrale
        let target = &mut world
            .connections
            .values_mut()
            .find(|f| f.player.name == target_name)
            .unwrap()
            .player;

        if let NPCKind::Enemy { damages, .. } = opponent_kind {
            let damages_after_defense = damages.saturating_sub(damages * defense.min(100) / 100);
            if damages_after_defense < target.hp {
                target.hp -= damages_after_defense;
            } else {
                target_killed = true;
                target.hp = target.max_hp - 10;
                target.location = String::from("room.city_square");
                target.status = State::Idle;
            }
            (target.hp, damages_after_defense)
        } else {
            unreachable!("No enemy here!");
        }
    };

    if target_killed {
        info!(target = %target_name, damage = e_damages, "player defeated by enemy");
    } else {
        info!(target = %target_name, damage = e_damages, target_hp, "enemy attack landed");
    }

    if target_killed {
        if nb_fighters == 1 {
            world.fights.remove(opponent_id);
            if let Some(npc) = world.resolve_npc_mut(opponent_id) {
                if let NPCKind::Enemy {
                    ref mut hp, max_hp, ..
                } = npc.kind
                {
                    *hp = max_hp;
                }
            }
        } else {
            let fight = world.fights.get_mut(opponent_id).unwrap();
            fight
                .defeated_fighters
                .push(fight.fighters.remove(target_index));
        }
    }

    for fighter in list_fighters {
        if let Some(con) = world
            .connections
            .values()
            .find(|c| c.player.name == fighter)
        {
            let _ = con.tx.send(Message::Event(EventType::ENEMY_ATTACK {
                target: target_name.clone(),
                damages: e_damages,
                target_hp,
                target_killed,
            }));
        }
    }
    if let Some(fight) = world.fights.get_mut(opponent_id) {
        fight.turn = 0;
    }
}

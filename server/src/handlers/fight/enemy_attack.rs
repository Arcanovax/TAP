use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    protocol::{EventType, Message},
    state::ServerInfo,
    structures::{
        enums::{npc_kind::NPCKind, state::State},
        player::Player,
    },
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

    let (target_name, target_hp, e_damages) = {
        let target: &mut Player;
        if nb_fighters == 1 {
            target = &mut world
                .connections
                .values_mut()
                .find(|c| c.player.name == *list_fighters.get(0).unwrap())
                .unwrap()
                .player;
        } else {
            let nanos = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .subsec_nanos() as usize;
            target_index = nanos % nb_fighters;
            let target_name = list_fighters.get(target_index).unwrap();
            target = &mut world
                .connections
                .values_mut()
                .find(|c| c.player.name == *target_name)
                .unwrap()
                .player;
        }
        if let NPCKind::Enemy { damages, .. } = opponent_kind {
            if damages < target.hp {
                target.hp -= damages;
            } else {
                target_killed = true;
                target.hp = target.max_hp - 10;
                target.location = String::from("room.city_square");
                target.status = State::Idle;
            }
            (target.name.clone(), target.hp, damages)
        } else {
            unreachable!("No enemy here!");
        }
    };

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
}

use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    protocol::{EventType, Message}, state::ServerInfo, structures::{
        attack_result::Enemy_Attack, enums::{enn_att_res::EnnAttRes, npc_kind::NPCKind, state::State}, fight::Fight, npc::NPC, player::Player,
    },
};

pub fn enemy_attack(opponent_id: &str, world: &mut ServerInfo) -> Enemy_Attack {
    // let world = &mut *world_mut;
    // let fight: &mut Fight = world.fights.get_mut(opponent_id).unwrap();
    // let opponent: &NPC = world.world.npcs.get(opponent_id).unwrap();
    let mut target_index: usize = 0;
    
    let (list_fighters, opponent_kind) = {
        (world.fights.get_mut(opponent_id).unwrap().fighters.clone(), world.world.npcs.get(opponent_id).unwrap().kind.clone())
    };
    
    let nb_fighters = list_fighters.len();

    let (target_name, target_hp, e_damages) = {
        let target: &mut Player;
        if nb_fighters == 1 {
            target = &mut world
                .connections
                .get_mut(list_fighters.get(0).unwrap())
                .unwrap()
                .player;
        } else {
            let nanos = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .subsec_nanos() as usize;
            target_index = nanos % nb_fighters;
            target = &mut world
                .connections
                .get_mut(&list_fighters[target_index])
                .unwrap()
                .player;
        }
        if let NPCKind::Enemy { damages, .. } = opponent_kind {
            if damages < target.hp {
                target.hp -= damages;
            } else {
                target.hp = target.max_hp - 10;
                target.location = String::from("loc.city_square");
                target.status = State::Idle;
                if nb_fighters == 1 {
                    world.fights.remove(opponent_id);
                }
            }
            (target.name.clone(), target.hp, damages)
        } else {
            unreachable!("No enemy here!");
        }
    };
    for fighter in list_fighters {
        if let Some(con) = world.connections.get(&fighter) {
            if con.player.name != target_name {
                let _ = con.tx.send(Message::Event(EventType::ENEMY_ATTACK { target: target_name.clone(), damages: e_damages, target_hp }));
            }
        }
    }
    Enemy_Attack { damages: e_damages, target: target_name }
}


use crate::{
    handlers::{
        fight::{attack::execute_attack, enemy_attack::enemy_attack, is_it_my_turn::is_it_my_turn},
        global_func::{
            check_fight::check_fight, get_player::get_player_mut, is_he_there::is_he_there,
        },
    }, protocol::{EventType, Message}, state::SharedServer, structures::{
        attack_result::Attack_Result, enums::{
            attack_res::AttackRes, enn_att_res::EnnAttRes, error::ErrorCode, fighter_status::FighterStatus, npc_kind::NPCKind, state::State, turn_res::TurnRes,
        }, fight::Fight,
    },
};
use std::{collections::HashMap, net::SocketAddr};

mod attack;
mod enemy_attack;
mod is_it_my_turn;
#[cfg(test)]
mod tests;

pub fn fight_request(
    peer_addr: SocketAddr,
    enn_name: &Vec<String>,
    world: &SharedServer,
) -> Message {
    if enn_name.len() != 1 {
        return Message::Response {
            error: ErrorCode::INVALID_ARGS,
            data: None,
        };
    }
    let mut pre_world = world.lock().unwrap();
    let world_mut = &mut *pre_world;
    let (p_loc, p_status, p_name, p_hp) = {
        let player = match get_player_mut(&mut world_mut.connections, &peer_addr) {
            Ok(p) => p,
            Err(msg) => {
                return Message::Response {
                    error: ErrorCode::PLAYER_NOT_FOUND,
                    data: Some(serde_json::to_value(msg).unwrap()),
                }
            }
        };
        (player.location.clone(), player.status.clone(), player.name.clone(), player.hp)
    };

    let loc = match world_mut.world.rooms.get(&p_loc) {
        Some(l) => l,
        None => {
            return Message::Response {
                error: ErrorCode::ROOM_NOT_FOUND,
                data: Some(serde_json::to_value("You're nowhere. I can't find you.").unwrap()),
            }
        }
    };

    if !is_he_there(&enn_name[0], loc) {
        return Message::Response {
            error: ErrorCode::NPC_NOT_FOUND,
            data: Some(serde_json::to_value("This target isn't here.").unwrap()),
        };
    }

    let (is_defeated, target_hp) = {
        if let NPCKind::Enemy { defeated, hp, .. } = &world_mut.world.npcs[&enn_name[0]].kind {
            (*defeated, *hp)
        } else {
            return Message::Response {
                error: ErrorCode::NPC_NOT_HOSTILE,
                data: Some(serde_json::to_value("This target isn't an enemy.").unwrap()),
            };
        }
    };

    if is_defeated {
        return Message::Response {
            error: ErrorCode::DEFEATED_ENEMY,
            data: Some(serde_json::to_value("This target has already been defeated").unwrap()),
        };
    }

    match p_status {
        State::Idle => {
            if let Some(fight) = check_fight(&enn_name[0], &mut world_mut.fights) {
                for fighter in fight.fighters.clone() {
                    if let Some(con) = world_mut.connections.get(&fighter) {
                        let _ = con.tx.send(Message::Event(EventType::ENTER_FIGHT {
                            player_name: p_name.clone(), hp: p_hp
                        }));
                    }
                }
                if !fight.fighters.contains(&peer_addr) {
                    fight.fighters.push(peer_addr)
                } else {
                    // If he is in State::Idle and in the fighters list, it means he have been defeated and is not allowed to come back in fight
                    return Message::Response {
                        error: ErrorCode::DEFEATED_FIGHTER,
                        data: None
                    };
                }
            } else {
                world_mut.fights.insert(
                    enn_name[0].to_string(),
                    Fight {
                        fighters: vec![peer_addr],
                        turn: 0,
                        enemy_turn: false,
                    },
                );
            };

            if let Ok(player) = get_player_mut(&mut world_mut.connections, &peer_addr) {
                player.status = State::InFight { target_id: enn_name[0].to_string() };
            }

            let mut fighters: HashMap<String, u32> = HashMap::new();

            for fighter_addr in &world_mut.fights.get(&enn_name[0]).unwrap().fighters {
                let fighter = world_mut.connections.get(&fighter_addr).unwrap();
                fighters.insert(fighter.player.name.clone(), fighter.player.hp);
            }
            Message::Response {
                error: ErrorCode::SUCCESS,
                data: Some(
                    serde_json::to_value(Attack_Result{
                        attacker_hp: p_hp,
                        attacker_name: p_name,
                        target_hp,
                        damage: 0,
                        status: State::InFight { target_id: enn_name[0].to_string() },
                        enemy_attack: None,
                        fighters: Some(fighters)
                    })
                    .unwrap(),
                ),
            }
        }
        State::InFight { target_id: target } => {
            let turn_result = {
                let fight = check_fight(&target, &mut world_mut.fights)
                    .expect("There is no fight.");
                is_it_my_turn(peer_addr, fight)
            };

            match turn_result {
                TurnRes::MyTurn => {
                    Message::Response {
                        error: ErrorCode::SUCCESS,
                        data: Some(
                            serde_json::to_value(execute_attack(
                                peer_addr, enn_name.clone(), world_mut
                            )).unwrap())
                        }
                    // match execute_attack(peer_addr, enn_name.clone(), world_mut) {
                    //     AttackRes::Hit(message) => {
                    //         if world_mut.fights.get(&enn_name[0]).unwrap().enemy_turn {
                    //             Message::Response {
                    //                 error: ErrorCode::SUCCESS,
                    //                 data: Some(
                    //                     serde_json::to_value(Attack_Result {
                    //                         attacker_hp: player.hp.clone(),
                    //                         attacker_name: player.name.clone(),
                    //                         target_hp: *hp,
                    //                         damage: 0,
                    //                         status: FighterStatus::ENTERFIGHT,
                    //                         enemy_attack: Some(enemy_attack(&enn_name[0], world_mut))
                    //                     })
                    //                     .unwrap(),
                    //                 ),
                    //             }
                    //         } else {
                    //             Message::Response {
                    //                 error: ErrorCode::SUCCESS,
                    //                 data: Some(serde_json::to_value(Attack_Result {
                    //                         attacker_hp: player.hp.clone(),
                    //                         attacker_name: player.name.clone(),
                    //                         target_hp: *hp,
                    //                         damage: 0,
                    //                         status: FighterStatus::ENTERFIGHT,
                    //                         enemy_attack: None
                    //                     }).unwrap()),
                    //             }
                    //         }
                    //     }
                    //     AttackRes::KillTarget(msg) => Message::Response {
                    //         error: ErrorCode::SUCCESS,
                    //         data: Some(serde_json::to_value(&msg).unwrap()),
                    //     },
                    //     AttackRes::Peace(msg) => Message::Response {
                    //         error: ErrorCode::SUCCESS,
                    //         data: Some(serde_json::to_value(msg).unwrap()),
                    //     },
                    // }
                }
                TurnRes::NotMyTurn | TurnRes::EnemyTurn => Message::Response {
                    error: ErrorCode::SUCCESS,
                    data: Some(serde_json::to_value("It's not your turn!").unwrap()),
                },
            }
        }
        State::Discuss => Message::Response {
            error: ErrorCode::INVALID_COMMAND,
            data: Some(serde_json::to_value("You can't fight in your state.").unwrap()),
        },
    }
    // match get_player_mut(&mut world_mut.connections, &peer_addr) {
    //     Ok(player) => {
    //         if let Some(loc) = world_mut.world.rooms.get(&player.location) {
    //             if !is_he_there(&enn_name[0], loc) {
    //                 return Message::Response {
    //                     error: ErrorCode::NPC_NOT_FOUND,
    //                     data: Some(serde_json::to_value("This target isn't here.").unwrap()),
    //                 };
    //             }
    //             if let NPCKind::Enemy { defeated, hp, .. } = &world_mut.world.npcs[&enn_name[0]].kind {
    //                 if *defeated {
    //                     return Message::Response {
    //                         error: ErrorCode::DEFEATED_ENEMY,
    //                         data: Some(
    //                             serde_json::to_value("This target has already been defeated")
    //                                 .unwrap(),
    //                         ),
    //                     };
    //                 } else {
    //                     match &player.status {
    //                         State::Idle => {
    //                             if let Some(fight) = check_fight(&enn_name[0], &mut world_mut.fights) {
    //                                 fight.fighters.push(peer_addr);
    //                             } else {
    //                                 world_mut.fights.insert(
    //                                     enn_name[0].to_string(),
    //                                     Fight {
    //                                         fighters: vec![peer_addr],
    //                                         turn: 0,
    //                                         enemy_turn: false,
    //                                     },
    //                                 );
    //                             }
    //                             player.status = State::InFight {
    //                                 target_id: (enn_name[0].to_string()),
    //                             };
    //                             let p_name = player.name.clone();
    //                             let p_hp = player.hp;
    //                             let p_status = player.status.clone();

    //                             let fighters_list = world_mut.fights.get(&enn_name[0]).unwrap().fighters.clone();
    //                             let _ = drop(player);
    //                             for fighter in fighters_list {
    //                                 if let Some(con) = world_mut.connections.get(&fighter) {
    //                                     let _ = con.tx.send(Message::Event(EventType::ENTER_FIGHT { player_name: p_name.clone(), hp: p_hp }));
    //                                 }
    //                             }

    //                             Message::Response {
    //                                 error: ErrorCode::SUCCESS,
    //                                 data: Some(
    //                                     serde_json::to_value(Attack_Result{
    //                                         attacker_hp: p_hp,
    //                                         attacker_name: p_name,
    //                                         target_hp: *hp,
    //                                         damage: 0,
    //                                         status: p_status,
    //                                         enemy_attack: None
    //                                     })
    //                                     .unwrap(),
    //                                 ),
    //                             }
    //                         }
    //                         State::InFight { target_id: target } => {
    //                             let fight = check_fight(&target, &mut world_mut.fights)
    //                                 .expect("There is no fight.");
        
    //                             match is_it_my_turn(peer_addr, fight) {
    //                                 TurnRes::MyTurn => {
    //                                     Message::Response {
    //                                                     error: ErrorCode::SUCCESS,
    //                                                     data: Some(
    //                                                         serde_json::to_value(execute_attack(peer_addr, enn_name.clone(), world_mut)).unwrap())}
    //                                     // match execute_attack(peer_addr, enn_name.clone(), world_mut) {
    //                                     //     AttackRes::Hit(message) => {
    //                                     //         if world_mut.fights.get(&enn_name[0]).unwrap().enemy_turn {
    //                                     //             Message::Response {
    //                                     //                 error: ErrorCode::SUCCESS,
    //                                     //                 data: Some(
    //                                     //                     serde_json::to_value(Attack_Result {
    //                                     //                         attacker_hp: player.hp.clone(),
    //                                     //                         attacker_name: player.name.clone(),
    //                                     //                         target_hp: *hp,
    //                                     //                         damage: 0,
    //                                     //                         status: FighterStatus::ENTERFIGHT,
    //                                     //                         enemy_attack: Some(enemy_attack(&enn_name[0], world_mut))
    //                                     //                     })
    //                                     //                     .unwrap(),
    //                                     //                 ),
    //                                     //             }
    //                                     //         } else {
    //                                     //             Message::Response {
    //                                     //                 error: ErrorCode::SUCCESS,
    //                                     //                 data: Some(serde_json::to_value(Attack_Result {
    //                                     //                         attacker_hp: player.hp.clone(),
    //                                     //                         attacker_name: player.name.clone(),
    //                                     //                         target_hp: *hp,
    //                                     //                         damage: 0,
    //                                     //                         status: FighterStatus::ENTERFIGHT,
    //                                     //                         enemy_attack: None
    //                                     //                     }).unwrap()),
    //                                     //             }
    //                                     //         }
    //                                     //     }
    //                                     //     AttackRes::KillTarget(msg) => Message::Response {
    //                                     //         error: ErrorCode::SUCCESS,
    //                                     //         data: Some(serde_json::to_value(&msg).unwrap()),
    //                                     //     },
    //                                     //     AttackRes::Peace(msg) => Message::Response {
    //                                     //         error: ErrorCode::SUCCESS,
    //                                     //         data: Some(serde_json::to_value(msg).unwrap()),
    //                                     //     },
    //                                     // }
    //                                 }
    //                                 TurnRes::NotMyTurn | TurnRes::EnemyTurn => Message::Response {
    //                                     error: ErrorCode::SUCCESS,
    //                                     data: Some(serde_json::to_value("It's not your turn!").unwrap()),
    //                                 },
    //                             }
    //                         }
    //                         State::Discuss => Message::Response {
    //                             error: ErrorCode::INVALID_COMMAND,
    //                             data: Some(serde_json::to_value("You can't fight in your state.").unwrap()),
    //                         },
    //                     }
    //                 }
    //             } else {
    //                 return Message::Response {
    //                     error: ErrorCode::NPC_NOT_HOSTILE,
    //                     data: Some(serde_json::to_value("This target isn't an enemy.").unwrap()),
    //                 };
    //             }
    //         } else {
    //             Message::Response {
    //                 error: ErrorCode::ROOM_NOT_FOUND,
    //                 data: Some(serde_json::to_value("You're nowhere. I can't find you.").unwrap()),
    //             }
    //         }
    //     }
    //     Err(msg) => Message::Response {
    //         error: ErrorCode::PLAYER_NOT_FOUND,
    //         data: Some(serde_json::to_value(msg).unwrap()),
    //     },
    // }
}

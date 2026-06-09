use std::net::SocketAddr;

use crate::{
    error::ErrorCode::{self},
    handlers::{
        fight_func::{
            attack::execute_attack, enemy_attack::enemy_attack, is_it_my_turn::is_it_my_turn,
        },
        global_func::{
            check_fight::check_fight, get_player::get_player_mut, is_he_there::is_he_there,
        },
    },
    protocol::Message,
    state::SharedServer,
    structures::{
        enums::{
            attack_res::AttackRes, enn_att_res::EnnAttRes, npc_kind::NPCKind, state::State,
            turn_res::TurnRes,
        },
        fight::Fight,
    },
};

pub fn fight(peer_addr: SocketAddr, enn_name: Vec<String>, world: &SharedServer) -> Message {
    if enn_name.len() != 1 {
        return Message::Response {
            error: ErrorCode::INVALID_ARGS,
            data: None,
        };
    }
    let mut pre_world = world.lock().unwrap();
    let world_mut = &mut *pre_world;
    match get_player_mut(&mut world_mut.connections, &peer_addr) {
        Ok(player) => {
            if let Some(loc) = world_mut.world.rooms.get(&player.location) {
                if !is_he_there(&enn_name[0], loc) {
                    return Message::Response {
                        error: ErrorCode::NPC_NOT_FOUND,
                        data: Some(serde_json::to_string("This target isn't here.").unwrap()),
                    };
                }
                if let NPCKind::Enemy { defeated, .. } = &world_mut.world.npcs[&enn_name[0]].kind {
                    if *defeated {
                        return Message::Response {
                            error: ErrorCode::DEFEATED_ENEMY,
                            data: Some(
                                serde_json::to_string("This target has already been defeated")
                                    .unwrap(),
                            ),
                        };
                    }
                } else {
                    return Message::Response {
                        error: ErrorCode::NPC_NOT_HOSTILE,
                        data: Some(serde_json::to_string("This target isn't an enemy.").unwrap()),
                    };
                }
                match &player.status {
                    State::Idle => {
                        if let Some(fight) = check_fight(&enn_name[0], &mut world_mut.fights) {
                            fight.fighters.push(peer_addr);
                        } else {
                            world_mut.fights.insert(
                                enn_name[0].to_string(),
                                Fight {
                                    fighters: vec![peer_addr],
                                    turn: 0,
                                    enemy_turn: false,
                                },
                            );
                        }
                        player.status = State::InFight {
                            target_id: (enn_name[0].to_string()),
                        };
                        Message::Response {
                            error: ErrorCode::SUCCESS,
                            data: Some(
                                serde_json::to_string(&format!(
                                    "{} says: 'Hello there!'",
                                    player.name
                                ))
                                .unwrap(),
                            ),
                        }
                    }
                    State::InFight { target_id: target } => {
                        let fight = check_fight(&target, &mut world_mut.fights)
                            .expect("There is no fight.");

                        match is_it_my_turn(peer_addr, fight) {
                            TurnRes::MyTurn => {
                                match execute_attack(peer_addr, enn_name.clone(), world_mut) {
                                    AttackRes::Hit(message) => {
                                        if world_mut.fights.get(&enn_name[0]).unwrap().enemy_turn {
                                            match enemy_attack(&enn_name[0], world_mut) {
                                                EnnAttRes::Hit(msg)
                                                | EnnAttRes::Kill(msg)
                                                | EnnAttRes::KillAndWin(msg)
                                                | EnnAttRes::Error(msg) => Message::Response {
                                                    error: ErrorCode::SUCCESS,
                                                    data: Some(
                                                        serde_json::to_string(&format!(
                                                            "{}\n{}",
                                                            message, msg
                                                        ))
                                                        .unwrap(),
                                                    ),
                                                },
                                            }
                                        } else {
                                            Message::Response {
                                                error: ErrorCode::SUCCESS,
                                                data: Some(
                                                    serde_json::to_string(&message).unwrap(),
                                                ),
                                            }
                                        }
                                    }
                                    AttackRes::KillTarget(msg) => Message::Response {
                                        error: ErrorCode::SUCCESS,
                                        data: Some(serde_json::to_string(&msg).unwrap()),
                                    },
                                    AttackRes::Peace(msg) => Message::Response {
                                        error: ErrorCode::SUCCESS,
                                        data: Some(serde_json::to_string(msg).unwrap()),
                                    },
                                }
                            }
                            TurnRes::NotMyTurn | TurnRes::EnemyTurn => Message::Response {
                                error: ErrorCode::SUCCESS,
                                data: Some(serde_json::to_string("It's not your turn!").unwrap()),
                            },
                        }
                    }
                    State::Discuss => Message::Response {
                        error: ErrorCode::INVALID_COMMAND,
                        data: Some(
                            serde_json::to_string("You can't fight in your state.").unwrap(),
                        ),
                    },
                }
            } else {
                Message::Response {
                    error: ErrorCode::ROOM_NOT_FOUND,
                    data: Some(serde_json::to_string("You're nowhere. I can't find you.").unwrap()),
                }
            }
        }
        Err(msg) => Message::Response {
            error: ErrorCode::PLAYER_NOT_FOUND,
            data: Some(serde_json::to_string(msg).unwrap()),
        },
    }
}


use crate::{
    handlers::fight::{
        attack::execute_attack, enemy_attack::enemy_attack, is_it_my_turn::is_it_my_turn,
    },
    protocol::{Message, Payload},
    state::SharedServer,
    structures::{
        enums::{
            attack_res::AttackRes, enn_att_res::EnnAttRes, error::ErrorCode, npc_kind::NPCKind,
            state::State, turn_res::TurnRes,
        },
        fight::Fight,
        room::Room,
    },
};
use std::net::SocketAddr;

mod attack;
mod enemy_attack;
mod is_it_my_turn;
#[cfg(test)]
mod tests;

fn is_he_there(name: &str, player_loc: &Room) -> bool {
    if player_loc.npc.len() != 0 {
        for npc in &player_loc.npc {
            if npc == name {
                return true;
            }
        }
        return false;
    } else {
        return false;
    };
}

pub fn fight_request(
    peer_addr: SocketAddr,
    args: &Vec<String>,
    server_info: &SharedServer,
) -> Message {
    if args.len() != 1 {
        return Message::Response {
            error: ErrorCode::INVALID_ARGS,
            payload: Payload::Empty,
        };
    }

    let target_name = &args[0];
    let mut binding = server_info.lock().unwrap();

    let room = match binding.get_player_room(peer_addr) {
        Ok(room) => room,
        Err(code) => {
            return Message::Response {
                error: code,
                payload: Payload::Empty,
            };
        }
    };

    if !is_he_there(target_name, room) {
        return Message::Response {
            error: ErrorCode::NPC_NOT_FOUND,
            payload: Payload::Empty,
        };
    }

    if let NPCKind::Enemy { defeated, .. } = binding.world.npcs.get(target_name).unwrap().kind {
        if defeated {
            return Message::Response {
                error: ErrorCode::DEFEATED_ENEMY,
                payload: Payload::Empty,
            };
        }
    } else {
        return Message::Response {
            error: ErrorCode::NPC_NOT_HOSTILE,
            payload: Payload::Json(serde_json::to_value("This target isn't an enemy.").unwrap()),
        };
    }

    let player = match binding.get_player(peer_addr) {
        Ok(player) => player,
        Err(code) => {
            return Message::Response {
                error: code,
                payload: Payload::Empty,
            };
        }
    };

    match &player.status {
        State::Idle => {
            if let Some(fight) = binding.fights.get_mut(target_name) {
                fight.fighters.push(peer_addr);
            } else {
                binding.fights.insert(
                    target_name.to_string(),
                    Fight {
                        fighters: vec![peer_addr],
                        turn: 0,
                        enemy_turn: false,
                    },
                );
            }
            let player = binding.get_player_mut(peer_addr).unwrap();
            player.status = State::InFight {
                target_id: (target_name.to_string()),
            };
            return Message::Response {
                error: ErrorCode::SUCCESS,
                payload: Payload::Json(
                    serde_json::to_value(&format!("{} says: 'Hello there!'", player.name)).unwrap(),
                ),
            };
        }
        State::InFight { target_id: target } => {
            let target = target.clone();
            let fight = binding.fights.get_mut(&target).expect("There is no fight.");

            match is_it_my_turn(peer_addr, fight) {
                TurnRes::MyTurn => match execute_attack(peer_addr, target_name, &mut binding) {
                    AttackRes::Hit(message) => {
                        if binding.fights.get(target_name).unwrap().enemy_turn {
                            match enemy_attack(&target_name, &mut binding) {
                                EnnAttRes::Hit(msg)
                                | EnnAttRes::Kill(msg)
                                | EnnAttRes::KillAndWin(msg)
                                | EnnAttRes::Error(msg) => {
                                    return Message::Response {
                                        error: ErrorCode::SUCCESS,
                                        payload: Payload::Json(
                                            serde_json::to_value(&format!("{}\n{}", message, msg))
                                                .unwrap(),
                                        ),
                                    };
                                }
                            }
                        } else {
                            return Message::Response {
                                error: ErrorCode::SUCCESS,
                                payload: Payload::Json(serde_json::to_value(&message).unwrap()),
                            };
                        }
                    }
                    AttackRes::KillTarget(msg) => {
                        return Message::Response {
                            error: ErrorCode::SUCCESS,
                            payload: Payload::Json(serde_json::to_value(&msg).unwrap()),
                        };
                    }
                    AttackRes::Peace(msg) => {
                        return Message::Response {
                            error: ErrorCode::SUCCESS,
                            payload: Payload::Json(serde_json::to_value(msg).unwrap()),
                        };
                    }
                },
                TurnRes::NotMyTurn | TurnRes::EnemyTurn => {
                    return Message::Response {
                        error: ErrorCode::SUCCESS,
                        payload: Payload::Json(
                            serde_json::to_value("It's not your turn!").unwrap(),
                        ),
                    };
                }
            }
        }
        State::Discuss => {
            return Message::Response {
                error: ErrorCode::INVALID_COMMAND,
                payload: Payload::Json(
                    serde_json::to_value("You can't fight in your state.").unwrap(),
                ),
            };
        }
    }
}

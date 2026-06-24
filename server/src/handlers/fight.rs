use crate::{
    handlers::fight::{
        attack::execute_attack, is_it_my_turn::is_it_my_turn,
    }, protocol::{EventType, Message}, state::SharedServer, structures::{
        attack_result::Attack_Result, enums::{error::ErrorCode, npc_kind::NPCKind,
            state::State, turn_res::TurnRes,
        }, fight::Fight, room::Room,
    },
};
use std::{collections::HashMap, fs::OpenOptions, net::SocketAddr, io::Write};

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
            data: None,
        };
    }
    let mut pre_world = server_info.lock().unwrap();
    let world_mut = &mut *pre_world;
    let (p_status, p_name, p_hp) = {
        let player = match world_mut.get_player_mut(peer_addr) {
            Ok(p) => p,
            Err(msg) => {
                return Message::Response {
                    error: ErrorCode::PLAYER_NOT_FOUND,
                    data: Some(serde_json::to_value(msg).unwrap()),
                }
            }
        };
        (player.status.clone(), player.name.clone(), player.hp)
    };

    let loc = match world_mut.get_player_room(peer_addr) {
        Ok(room) => room,
        Err(code) => {
            return Message::Response {
                error: code,
                data: None,
            };
        }
    };

    if !is_he_there(&args[0], loc) {
        return Message::Response {
            error: ErrorCode::NPC_NOT_FOUND,
            data: Some(serde_json::to_value("This target isn't here.").unwrap()),
        };
    }

    let (is_defeated, target_hp) = {
        if let NPCKind::Enemy { defeated, hp, .. } = world_mut.world.npcs[&args[0]].kind {
            (defeated, hp)
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
            if let Some(fight) = world_mut.fights.get_mut(&args[0]) {
				if !fight.fighters.contains(&p_name) && !fight.defeated_fighters.contains(&p_name) {
					fight.fighters.push(p_name.clone());
					for fighter in fight.fighters.clone() {
						if let Some(con) = world_mut.connections.values().find(|c|c.player.name == fighter) {
							let _ = con.tx.send(Message::Event(EventType::ENTER_FIGHT {
								player_name: p_name.clone(), hp: p_hp
							}));
						}
					}
                } else {
                    return Message::Response {
                        error: ErrorCode::DEFEATED_FIGHTER,
                        data: None
                    };
                }
            } else {
                world_mut.fights.insert(
                    args[0].to_string(),
                    Fight {
                        fighters: vec![p_name.clone()],
                        defeated_fighters: Vec::new(),
                        turn: 0,
                        enemy_turn: false,
                    },
                );
            };

            if let Ok(player) = world_mut.get_player_mut(peer_addr) {
                player.status = State::InFight { target_id: args[0].to_string() };
            }

            let mut fighters: HashMap<String, u32> = HashMap::new();

            for fighter_name in &world_mut.fights.get(&args[0]).unwrap().fighters {
                // if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("debug_network.txt") {
                //     let _ = writeln!(file, "ko (State {:#?}) : {:#?}", world_mut.connections, fighter_name);}
                if let Some(fighter) = world_mut.connections.values().find(|c| &c.player.name == fighter_name){
                    fighters.insert(fighter.player.name.clone(), fighter.player.hp);
                }
            }
            Message::Response {
                error: ErrorCode::SUCCESS,
                data: Some(
                    serde_json::to_value(Attack_Result{
                        attacker_hp: p_hp,
                        attacker_name: p_name,
                        target_hp,
                        damage: 0,
                        status: State::InFight { target_id: args[0].to_string() },
                        fighters: Some(fighters)
                    })
                    .unwrap(),
                ),
            }
        }
        State::InFight { target_id: target } => {
            let turn_result = {
                let fight = world_mut.fights.get_mut(&target)
                    .expect("There is no fight.");
                is_it_my_turn(p_name, fight)
            };

            match turn_result {
                TurnRes::MyTurn => {
                    Message::Response {
                        error: ErrorCode::SUCCESS,
                        data: Some(
                            serde_json::to_value(execute_attack(
                                peer_addr, &args[0].clone(), world_mut
                            )).unwrap())
                        }
                }
                TurnRes::NotMyTurn | TurnRes::EnemyTurn => Message::Response {
                    error: ErrorCode::NOT_YOUR_TURN,
                    data: None,
                },
            }
        }
        State::Discuss => Message::Response {
            error: ErrorCode::INVALID_COMMAND,
            data: Some(serde_json::to_value("You can't fight in your state.").unwrap()),
        },
    }
}

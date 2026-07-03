use std::net::SocketAddr;

use crate::{
    handlers::fight::enemy_attack::enemy_attack,
    protocol::{EventType, Message, Payload},
    state::SharedServer,
    structures::enums::{error::ErrorCode, item_kind::ItemKind, state::State},
};

pub fn consume(peer_addr: SocketAddr, args: &Vec<String>, server_info: &SharedServer) -> Message {
    if args.len() != 1 {
        return Message::Response {
            error: ErrorCode::INVALID_ARGS,
            payload: Payload::Empty,
        };
    }
    let mut pre_world = server_info.lock().unwrap();
    let world_mut = &mut *pre_world;
    let (p_status, p_name) = {
        let player = match world_mut.get_player_mut(peer_addr) {
            Ok(p) => p,
            Err(msg) => {
                return Message::Response {
                    error: ErrorCode::INVALID_COMMAND,
                    payload: Payload::Json(serde_json::to_value(msg).unwrap()),
                };
            }
        };
        (player.status.clone(), player.name.clone())
    };

    let item_kind = {
        let item = world_mut.resolve_item(&args[0]);

        match item {
            Some(it) => it.kind.clone(),
            None => {
                return Message::Response {
                    error: ErrorCode::NOT_AN_ITEM,
                    payload: Payload::Empty,
                };
            }
        }
    };

    if let ItemKind::Potion { healing } = item_kind {
        let player = world_mut.get_player_mut(peer_addr);
        match player {
            Ok(pl) => {
                if let Some(i) = pl.inventory.get(&args[0]) {
                    if *i > 1 {
                        pl.inventory.insert(args[0].clone(), *i - 1);
                    } else {
                        pl.inventory.remove(&args[0]);
                    }
                } else {
                    return Message::Response {
                        error: ErrorCode::ITEM_NOT_IN_INVENTORY,
                        payload: Payload::Empty,
                    };
                }
            }
            Err(code) => {
                return Message::Response {
                    error: code,
                    payload: Payload::Empty,
                };
            }
        };

        let player = world_mut.get_player_mut(peer_addr).unwrap();
        player.hp = player.hp.saturating_add(healing).min(100);

        match p_status {
            State::InFight { target_id } => {
                let (fighters, turn) = {
                    let fight = world_mut.fights.get_mut(&target_id).unwrap();
                    fight.turn += 1;
                    (fight.fighters.clone(), fight.turn)
                };
                for fighter in &fighters {
                    if let Some(con) = world_mut
                        .connections
                        .values()
                        .find(|c| c.player.name == *fighter)
                    {
                        let _ = con.tx.send(Message::Event(EventType::HEALING {
                            player_name: p_name.clone(),
                            heal: healing,
                        }));
                    } else {
                    }
                }
                if turn == fighters.len() as u32 {
                    enemy_attack(&target_id, world_mut);
                }
            }
            _ => {}
        }
        Message::Response {
            error: ErrorCode::SUCCESS,
            payload: Payload::Empty,
        }
    } else {
        Message::Response {
            error: ErrorCode::UNUSABLE_ITEM,
            payload: Payload::Empty,
        }
    }
}


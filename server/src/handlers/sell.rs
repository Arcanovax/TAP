use crate::{
    protocol::{Message, Payload},
    state::SharedServer,
    structures::enums::{error::ErrorCode, item_kind::ItemKind, npc_kind::NPCKind},
};
use std::{collections::HashMap, net::SocketAddr};

#[cfg(test)]
mod tests;

pub(super) fn sell_request(
    args: &[String],
    server_info: &SharedServer,
    peer_addr: SocketAddr,
) -> Message {
    let mut binding = server_info.lock().unwrap();
    let (npc_ref, item_ref, amount) = match args.len() {
        2 => (args[0].clone(), args[1].clone(), 1),
        3 => (
            args[0].clone(),
            args[1].clone(),
            args[2].parse().unwrap_or(1),
        ),
        _ => {
            return Message::Response {
                error: ErrorCode::INVALID_ARGS,
                payload: Payload::Empty,
            };
        }
    };
    if amount == 0 {
        return Message::Response {
            error: ErrorCode::INVALID_ARGS,
            payload: Payload::Empty,
        };
    }

    match binding.get_player_room(peer_addr) {
        Ok(room) => {
            if !room.npc.contains(&npc_ref) {
                return Message::Response {
                    error: ErrorCode::NPC_NOT_FOUND,
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
    }

    match binding.resolve_npc(&npc_ref) {
        Some(npc) => {
            match &npc.kind {
                NPCKind::Merchant { .. } => {}
                _ => {
                    return Message::Response {
                        error: ErrorCode::INVALID_ARGS,
                        payload: Payload::Empty,
                    };
                }
            };
        }
        None => {
            return Message::Response {
                error: ErrorCode::NPC_NOT_FOUND,
                payload: Payload::Empty,
            };
        }
    }

    let price = match binding.resolve_item(&item_ref) {
        Some(item) if !matches!(item.kind, ItemKind::QuestItem) => item.price * amount,
        Some(_) => {
            return Message::Response {
                error: ErrorCode::FORBIDDEN_ACTION,
                payload: Payload::Empty,
            };
        }
        None => {
            return Message::Response {
                error: ErrorCode::ITEM_NOT_FOUND,
                payload: Payload::Empty,
            };
        }
    };

    let player = match binding.get_player_mut(peer_addr) {
        Ok(player) => player,
        Err(code) => {
            return Message::Response {
                error: code,
                payload: Payload::Empty,
            };
        }
    };

    match player.inventory.get(&item_ref) {
        Some(amt) => {
            if amount > *amt {
                return Message::Response {
                    error: ErrorCode::ITEM_NOT_IN_INVENTORY,
                    payload: Payload::Empty,
                };
            }
        }
        None => {
            return Message::Response {
                error: ErrorCode::ITEM_NOT_IN_INVENTORY,
                payload: Payload::Empty,
            };
        }
    }

    player.gold += price;
    if let Some(amt) = player.inventory.get_mut(&item_ref) {
        *amt -= amount;
        if *amt == 0 {
            player.inventory.remove(&item_ref);
        }
    }

    Message::Response {
        error: ErrorCode::SUCCESS,
        payload: Payload::Pair(HashMap::from([
            ("sold".to_string(), item_ref),
            ("amount".to_string(), amount.to_string()),
            ("gold".to_string(), price.to_string()),
        ])),
    }
}

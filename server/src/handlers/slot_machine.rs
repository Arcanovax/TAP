use crate::protocol::{Message, Payload};
use crate::state::SharedServer;
use crate::structures::enums::error::ErrorCode;
use std::collections::HashMap;
use std::net::SocketAddr;

#[cfg(test)]
mod tests;

const SLOT_MACHINE_COST: u8 = 5;

pub(super) fn slot_machine_request(
    server_info: &SharedServer,
    peer_addr: SocketAddr,
    roll: impl FnOnce() -> Option<String>,
) -> Message {
    let mut binding = server_info.lock().unwrap();
    let player = match binding.get_player_mut(peer_addr) {
        Ok(player) => player,
        Err(code) => {
            return Message::Response {
                error: code,
                payload: Payload::Empty,
            };
        }
    };

    if player.gold < SLOT_MACHINE_COST as u32 {
        return Message::Response {
            error: ErrorCode::NOT_ENOUGH_GOLD,
            payload: Payload::Empty,
        };
    }

    player.gold -= SLOT_MACHINE_COST as u32;
    match roll() {
        Some(item) => {
            *player.inventory.entry(item.clone()).or_insert(0) += 1;
            Message::Response {
                error: ErrorCode::SUCCESS,
                payload: Payload::Pair(HashMap::from([("item".to_string(), item)])),
            }
        }
        None => Message::Response {
            error: ErrorCode::GAME_LOSE,
            payload: Payload::Empty,
        },
    }
}

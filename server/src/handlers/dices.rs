use crate::{
    protocol::{Message, Payload},
    state::SharedServer,
    structures::enums::error::ErrorCode,
};
use std::{collections::HashMap, net::SocketAddr};

#[cfg(test)]
mod tests;

const DICES_COST: u8 = 5;
const DICES_MIN: u8 = 1;
const DICES_MAX: u8 = 10;

pub fn dices_request(
    server_info: &SharedServer,
    peer_addr: SocketAddr,
    args: &Vec<String>,
    mut draw: Vec<u8>,
) -> Message {
    let mut binding = server_info.lock().unwrap();
    let gambling_room = binding.world.gambling_room.clone();
    let player = match binding.get_player_mut(peer_addr) {
        Ok(player) => {
            if player.location != gambling_room {
                return Message::Response {
                    error: ErrorCode::FORBIDDEN_ACTION,
                    payload: Payload::Empty,
                };
            }
            if player.gold < DICES_COST as u32 {
                return Message::Response {
                    error: ErrorCode::NOT_ENOUGH_GOLD,
                    payload: Payload::Empty,
                };
            }
            player
        }
        Err(code) => {
            return Message::Response {
                error: code,
                payload: Payload::Empty,
            };
        }
    };

    if args.len() == 0 || args.len() > 10 {
        return Message::Response {
            error: ErrorCode::INVALID_ARGS,
            payload: Payload::Empty,
        };
    }

    let guesses: Vec<u8> = match args
        .iter()
        .map(|guess| guess.parse())
        .collect::<Result<_, _>>()
    {
        Ok(guesses) => guesses,
        Err(_) => {
            return Message::Response {
                error: ErrorCode::INVALID_ARGS,
                payload: Payload::Empty,
            };
        }
    };

    if !guesses.iter().all(|g| *g >= DICES_MIN && *g <= DICES_MAX) {
        return Message::Response {
            error: ErrorCode::INVALID_ARGS,
            payload: Payload::Empty,
        };
    }

	let draw_copy = draw.clone();
    player.gold -= DICES_COST as u32;
    let multiplier = guesses.len() as u32;

    let all_match = guesses
        .iter()
        .all(|g| match draw.iter().position(|d| d == g) {
            Some(pos) => {
                draw.swap_remove(pos);
                true
            }
            None => false,
        });

    if !all_match {
        return Message::Response {
            error: ErrorCode::GAME_LOSE,
            payload: Payload::Text(draw_copy.iter().map(|f| f.to_string()).collect::<Vec<String>>().join(" ")),
        };
    }

    let gain = multiplier * DICES_COST as u32;
    player.gold += gain;
    Message::Response {
        error: ErrorCode::SUCCESS,
        payload: Payload::Pair(HashMap::from([
			("gold".to_string(), gain.to_string()),
			("draw".to_string(), draw_copy.iter().map(|f|f.to_string()).collect::<Vec<String>>().join("//"))])),
    }
}

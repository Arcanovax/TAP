use std::net::SocketAddr;

use crate::{protocol::{Message, Payload}, state::SharedServer, structures::{enums::{error::ErrorCode, game_event::GameEvent}, handler_outcome::HandlerOutcome}};

pub fn handle_answer(peer_addr: SocketAddr, server_info: &SharedServer, args: &Vec<String>) -> HandlerOutcome {
    if args.len() != 1 {
        return Message::Response {
            error: ErrorCode::INVALID_ARGS,
            payload: Payload::Empty,
        }
        .into();
    }
	
    let binding = server_info.lock().unwrap();
    let _ = match binding.get_player(peer_addr) {
        Ok(player) => player,
        Err(code) => {
            return Message::Response {
                error: code,
                payload: Payload::Empty,
            }
            .into();
        }
    };

    HandlerOutcome {
        message: Message::Response {
            error: ErrorCode::SUCCESS,
            payload: Payload::Empty,
        },
        event: Some(GameEvent::Answer { answer: args[0].to_string() }),
    }
}
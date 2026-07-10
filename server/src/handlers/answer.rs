use std::net::SocketAddr;

use crate::{protocol::{Message, Payload}, state::SharedServer, structures::{enums::{error::ErrorCode, game_event::GameEvent}, handler_outcome::HandlerOutcome, quest::Goal}};

pub fn handle_answer(peer_addr: SocketAddr, server_info: &SharedServer, args: &Vec<String>) -> HandlerOutcome {
    if args.len() != 1 {
        return Message::Response {
            error: ErrorCode::INVALID_ARGS,
            payload: Payload::Empty,
        }
        .into();
    }
	
    let binding = server_info.lock().unwrap();
    let player = match binding.get_player(peer_addr) {
        Ok(player) => player,
        Err(code) => {
            return Message::Response {
                error: code,
                payload: Payload::Empty,
            }
            .into();
        }
    };

	let mut message = Message::Response { error: ErrorCode::NO_QUEST_AVAILABLE, payload: Payload::Empty };

	for (quest_ref, step) in &player.quests_in_progress {
		let Some(quest) = binding.world.quests.get(quest_ref) else {
			continue;
		};
		if let Goal::Answer { answer, room } = &quest.goals[*step] {
			if player.location != *room || args[0] != *answer {
				message = Message::Response { error: ErrorCode::WRONG_ANSWER_OR_PLACE_OR_BOTH, payload: Payload::Empty }
			} else {
				message = Message::Response { error: ErrorCode::SUCCESS, payload: Payload::Empty };
				break;
			}
		} else {
			message = Message::Response { error: ErrorCode::INVALID_COMMAND, payload: Payload::Empty }
		}
	}


    HandlerOutcome {
        message: message,
        event: Some(GameEvent::Answer { answer: args[0].to_string() }),
    }
}
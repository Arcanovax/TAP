use crate::{
    protocol::{Message, Payload},
    state::SharedServer,
    structures::enums::error::ErrorCode,
};

#[cfg(test)]
mod tests;

pub fn quest_info_request(server_info: &SharedServer, args: &[String]) -> Message {
    if args.len() != 1 {
        return Message::Response {
            error: ErrorCode::INVALID_ARGS,
            payload: Payload::Empty,
        };
    }

    match server_info.lock().unwrap().world.quests.get(&args[0]) {
        Some(quest) => Message::Response {
            error: ErrorCode::SUCCESS,
            payload: Payload::Json(serde_json::to_value(quest).unwrap()),
        },
        None => Message::Response {
            error: ErrorCode::NO_QUEST_AVAILABLE,
            payload: Payload::Empty,
        },
    }
}

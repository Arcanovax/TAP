use crate::{protocol::{Message, Payload}, state::SharedServer, structures::enums::error::ErrorCode};

pub fn handle_answer(server_info: &SharedServer, args: &Vec<String>) {
	if args.len() != 1 {
        return Message::Response {
            error: ErrorCode::INVALID_ARGS,
            payload: Payload::Empty,
        };
    }
	
}
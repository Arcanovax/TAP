use crate::error::ErrorCode;
use crate::protocol::{Message, MessageType};
use crate::state::ServerInfo;
use std::sync::{Arc, Mutex};

pub(super) fn who_request(request: Message, server_info: &Arc<Mutex<ServerInfo>>) -> Message {
    Message {
        message: MessageType::RESPONSE,
        data: server_info
            .lock()
            .unwrap()
            .get_number_of_players()
            .to_string(),
        error_response: ErrorCode::SUCCESS,
        error_code: ErrorCode::SUCCESS.code(),
        ..request
    }
}

use crate::error::ErrorCode;
use crate::protocol::{Message, MessageType};
use crate::state::ServerInfo;
use connect::connect_request;
use std::sync::{Arc, Mutex};

mod connect;

pub fn handle_request(request: Message, server_info: &Arc<Mutex<ServerInfo>>) -> Message {
    if request.message != MessageType::COMMAND {
        return Message::default();
    };
    match request.command_name.to_uppercase().as_str() {
        "CONNECT" => connect_request(request, server_info),
        _ => Message {
            message: MessageType::RESPONSE,
            error_response: ErrorCode::INVALID_COMMAND,
            error_code: ErrorCode::INVALID_COMMAND.code(),
            ..request
        },
    }
}

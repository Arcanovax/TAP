use crate::command::Command;
use crate::error::ErrorCode;
use crate::protocol::{Message, MessageType};
use crate::state::ServerInfo;
use connect::connect_request;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

mod connect;

pub fn handle_request(
    request: Message,
    server_info: &Arc<Mutex<ServerInfo>>,
    peer_addr: SocketAddr,
) -> Message {
    if request.message != MessageType::COMMAND {
        return Message::default();
    };
    match Command::parse(&request.command_name) {
        Some(Command::CONNECT) => connect_request(request, server_info, peer_addr),
        Some(Command::QUIT) => Message::default(),
        None => Message {
            message: MessageType::RESPONSE,
            error_response: ErrorCode::INVALID_COMMAND,
            error_code: ErrorCode::INVALID_COMMAND.code(),
            ..request
        },
    }
}

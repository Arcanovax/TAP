use crate::command::Command;
use crate::error::ErrorCode;
use crate::protocol::{Message, MessageType};
use crate::state::{ServerInfo, Tx};
use chat::chat_request;
use connect::connect_request;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use who::who_request;

mod chat;
mod connect;
mod who;

pub fn handle_request(
    request: Message,
    server_info: &Arc<Mutex<ServerInfo>>,
    peer_addr: SocketAddr,
    tx: &Tx,
) -> Message {
    if request.message != MessageType::COMMAND {
        return Message::default();
    };
    match Command::parse(&request.command_name) {
        Some(Command::CONNECT) => connect_request(request, server_info, peer_addr, tx),
        Some(Command::QUIT) => Message::default(),
        Some(Command::WHO) => who_request(request, server_info),
        Some(Command::CHAT) => chat_request(request, server_info, peer_addr),
        None => Message {
            message: MessageType::RESPONSE,
            error_response: ErrorCode::INVALID_COMMAND,
            error_code: ErrorCode::INVALID_COMMAND.code(),
            ..request
        },
    }
}

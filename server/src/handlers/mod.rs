use crate::command::Command;
use crate::error::ErrorCode;
use crate::protocol::Message;
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
    match request {
        Message::Command { name, args } => match Command::parse(&name) {
            Some(Command::CONNECT) => connect_request(args, server_info, peer_addr, tx),
            Some(Command::QUIT) => Message::default(),
            Some(Command::WHO) => who_request(server_info),
            Some(Command::CHAT) => chat_request(args, server_info, peer_addr),
            None => Message::Response {
                error: ErrorCode::INVALID_COMMAND,
                data: None,
            },
        },
        _ => Message::Response {
            error: ErrorCode::INVALID_COMMAND,
            data: None,
        },
    }
}

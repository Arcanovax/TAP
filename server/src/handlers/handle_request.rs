use crate::state::{SharedServer, Tx};
use std::net::SocketAddr;

use crate::{
    command::Command,
    error::ErrorCode,
    handlers::{
        chat::chat_request,
        connect::connect_request,
        drop::drop_request,
        fight_func::fight::fight,
        global_func::{move_to::move_to, talk_to::talk_to},
        group::group_request,
        look::look_request,
        status::status_request,
        take::take_request,
        who::who_request,
    },
    protocol::Message,
};

pub fn handle_request(
    request: Message,
    server_info: &SharedServer,
    peer_addr: SocketAddr,
    tx: &Tx,
) -> Message {
    match request {
        Message::Command { name, args } => match Command::parse(&name) {
            Some(Command::CONNECT) => connect_request(args, server_info, peer_addr, tx),
            Some(Command::QUIT) => Message::Response {
                error: ErrorCode::SUCCESS,
                data: None,
            },
            Some(Command::WHO) => who_request(server_info),
            Some(Command::CHAT) => chat_request(args, server_info, peer_addr),
            Some(Command::GROUP) => group_request(args, server_info, peer_addr),
            Some(Command::STATUS) => status_request(server_info, peer_addr),
            Some(Command::MOVE) => move_to(server_info, peer_addr, args),
            Some(Command::TALK) => talk_to(peer_addr, args, server_info),
            Some(Command::ATTACK) => fight(peer_addr, args, server_info),
            Some(Command::LOOK) => look_request(server_info, peer_addr),
            Some(Command::DROP) => drop_request(server_info, peer_addr, args),
            Some(Command::TAKE) => take_request(server_info, peer_addr, args),
            _ => Message::Response {
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

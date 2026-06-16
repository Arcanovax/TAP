use crate::{
    handlers::quest::quest_request,
    state::{SharedServer, Tx},
    structures::handler_outcome::HandlerOutcome,
};
use std::net::SocketAddr;

use crate::{
    command::Command,
    error::ErrorCode,
    handlers::{
        chat::chat_request, connect::connect_request, drop::drop_request, fight_func::fight::fight,
        group::group_request, inventory::inventory_request, look::look_request,
        movement::move_request, status::status_request, take::take_request, talk::talk_request,
        who::who_request,
    },
    protocol::Message,
};

pub fn handle_request(
    request: &Message,
    server_info: &SharedServer,
    peer_addr: SocketAddr,
    tx: &Tx,
) -> Message {
    let handler_result: HandlerOutcome = match &request {
        Message::Command { name, args } => match Command::parse(name) {
            Some(Command::CONNECT) => connect_request(args, server_info, peer_addr, tx).into(),
            Some(Command::QUIT) => Message::Response {
                error: ErrorCode::SUCCESS,
                data: Some(serde_json::to_value("OK bye").unwrap()),
            }
            .into(),
            Some(Command::WHO) => who_request(server_info).into(),
            Some(Command::CHAT) => chat_request(args, server_info, peer_addr).into(),
            Some(Command::GROUP) => group_request(args, server_info, peer_addr).into(),
            Some(Command::STATUS) => status_request(server_info, peer_addr).into(),
            Some(Command::MOVE) => move_request(server_info, peer_addr, args).into(),
            Some(Command::TALK) => talk_request(peer_addr, args, server_info),
            Some(Command::ATTACK) => fight(peer_addr, args, server_info).into(),
            Some(Command::LOOK) => look_request(server_info, peer_addr).into(),
            Some(Command::DROP) => drop_request(server_info, peer_addr, args).into(),
            Some(Command::TAKE) => take_request(server_info, peer_addr, args).into(),
            Some(Command::INVENTORY) => inventory_request(server_info, peer_addr).into(),
            Some(Command::QUEST) => quest_request(args, server_info, peer_addr).into(),
            _ => Message::Response {
                error: ErrorCode::INVALID_COMMAND,
                data: None,
            }
            .into(),
        },
        _ => Message::Response {
            error: ErrorCode::INVALID_COMMAND,
            data: None,
        }
        .into(),
    };

    server_info
        .lock()
        .unwrap()
        .advance_quests(peer_addr, handler_result.event.as_ref());

    handler_result.message
}

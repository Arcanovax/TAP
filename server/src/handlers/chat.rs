use crate::protocol::{ChatScope, EventType, Message, Payload};
use crate::state::SharedServer;
use crate::structures::enums::error::ErrorCode;
use std::net::SocketAddr;
use tracing::info;

#[cfg(test)]
mod tests;

pub(super) fn chat_request(
    args: &Vec<String>,
    server_info: &SharedServer,
    peer_addr: SocketAddr,
) -> Message {
    if !server_info.lock().unwrap().is_connected(peer_addr) {
        return Message::Response {
            error: ErrorCode::INVALID_COMMAND,
            payload: Payload::Empty,
        };
    }
    if args.len() <= 1 {
        return Message::Response {
            error: ErrorCode::INVALID_ARGS,
            payload: Payload::Empty,
        };
    }
    let scope = &args[0];
    let body = args[1..].join(" ") + "\n";
    let mut binding = server_info.lock().unwrap();

    let sender_name = match binding.get_player(peer_addr) {
        Ok(player) => player.name.clone(),
        Err(code) => {
            return Message::Response {
                error: code,
                payload: Payload::Empty,
            };
        }
    };
    let receivers = match scope.to_uppercase().as_str() {
        "GLOBAL" => binding.get_global_receivers(peer_addr),
        "GROUP" => match binding.get_group_receivers(peer_addr) {
            Ok(receivers) => receivers,
            Err(code) => {
                return Message::Response {
                    error: code,
                    payload: Payload::Empty,
                };
            }
        },
        "ROOM" => match binding.get_room_receivers(peer_addr) {
            Ok(receivers) => receivers,
            Err(code) => {
                return Message::Response {
                    error: code,
                    payload: Payload::Empty,
                };
            }
        },
        _ => {
            return Message::Response {
                error: ErrorCode::INVALID_ARGS,
                payload: Payload::Empty,
            };
        }
    };
    let chat_scope = match scope.parse::<ChatScope>() {
        Ok(scope) => scope,
        Err(code) => {
            return Message::Response {
                error: code,
                payload: Payload::Empty,
            };
        }
    };
    for con in receivers {
        let _ = con.tx.send(Message::Event(EventType::CHAT {
            body: body.clone(),
            sender: sender_name.clone(),
            scope: chat_scope.clone(),
        }));
    }
    info!("Send {} scoped chat: {}", scope.to_uppercase(), body);
    return Message::Response {
        error: ErrorCode::SUCCESS,
        payload: Payload::Empty,
    };
}

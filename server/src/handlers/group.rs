use crate::protocol::{Message, Payload};
use crate::state::SharedServer;
use crate::structures::enums::error::ErrorCode;
use std::collections::HashMap;
use std::net::SocketAddr;

#[cfg(test)]
mod tests;

pub(super) fn group_request(
    args: &[String],
    server_info: &SharedServer,
    peer_addr: SocketAddr,
) -> Message {
    if args.is_empty() {
        return Message::Response {
            error: ErrorCode::INVALID_ARGS,
            payload: Payload::Empty,
        };
    }

    match args[0].to_uppercase().as_str() {
        "CREATE" => group_create_request(server_info, peer_addr, args),
        "LEAVE" => group_leave_request(server_info, peer_addr),
        "INVITE" => group_invite_request(args, server_info, peer_addr),
        "JOIN" => group_join_request(server_info, peer_addr, args),
        "LIST" => group_list_request(server_info, peer_addr),
        _ => Message::Response {
            error: ErrorCode::INVALID_ARGS,
            payload: Payload::Empty,
        },
    }
}

fn group_create_request(
    server_info: &SharedServer,
    peer_addr: SocketAddr,
    args: &[String],
) -> Message {
    let group_name: String = if args.len() <= 1 {
        match server_info.lock().unwrap().get_player(peer_addr) {
            Ok(player) => player.name.clone() + "'s group",
            Err(code) => {
                return Message::Response {
                    error: code,
                    payload: Payload::Empty,
                };
            }
        }
    } else {
        args[1..].join(" ")
    };

    match server_info
        .lock()
        .unwrap()
        .try_create_group(peer_addr, group_name.as_str())
    {
        Ok(gid) => Message::Response {
            error: ErrorCode::SUCCESS,
            payload: Payload::Pair(HashMap::from([("group".to_string(), gid)])),
        },
        Err(code) => Message::Response {
            error: code,
            payload: Payload::Empty,
        },
    }
}

fn group_leave_request(server_info: &SharedServer, peer_addr: SocketAddr) -> Message {
    server_info
        .lock()
        .unwrap()
        .try_leave_group(peer_addr)
        .into()
}

fn group_invite_request(
    args: &[String],
    server_info: &SharedServer,
    peer_addr: SocketAddr,
) -> Message {
    if args.len() != 2 {
        return Message::Response {
            error: ErrorCode::INVALID_ARGS,
            payload: Payload::Empty,
        };
    }

    server_info
        .lock()
        .unwrap()
        .try_invite_group(args[1].clone(), peer_addr)
        .into()
}

fn group_join_request(
    server_info: &SharedServer,
    peer_addr: SocketAddr,
    args: &[String],
) -> Message {
    if args.len() <= 1 {
        return Message::Response {
            error: ErrorCode::INVALID_ARGS,
            payload: Payload::Empty,
        };
    }
    match server_info
        .lock()
        .unwrap()
        .try_join_group(peer_addr, args[1..].join(" "))
    {
        Ok(gid) => Message::Response {
            error: ErrorCode::SUCCESS,
            payload: Payload::Pair(HashMap::from([("group".to_string(), gid)])),
        },
        Err(code) => Message::Response {
            error: code,
            payload: Payload::Empty,
        },
    }
}

fn group_list_request(server_info: &SharedServer, peer_addr: SocketAddr) -> Message {
    server_info
        .lock()
        .unwrap()
        .try_get_group_list(peer_addr)
        .map(|list| serde_json::to_value(&list).unwrap())
        .into()
}

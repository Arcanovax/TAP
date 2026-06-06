use crate::error::ErrorCode;
use crate::protocol::{EventType, Message};
use crate::state::SharedServer;
use std::net::SocketAddr;

pub(super) fn group_request(
    args: Vec<String>,
    server_info: &SharedServer,
    peer_addr: SocketAddr,
) -> Message {
    if args.len() == 0 {
        return Message::Response {
            error: ErrorCode::INVALID_ARGS,
            data: None,
        };
    }

    match args[0].to_uppercase().as_str() {
        "CREATE" => group_create_request(server_info, peer_addr),
        "LEAVE" => group_leave_request(server_info, peer_addr),
        "INVITE" => group_invite_request(args, server_info, peer_addr),
        "JOIN" => group_join_request(server_info, peer_addr),
        _ => Message::Response {
            error: ErrorCode::INVALID_ARGS,
            data: None,
        },
    }
}

fn group_create_request(server_info: &SharedServer, peer_addr: SocketAddr) -> Message {
    let err = match server_info.lock().unwrap().try_create_group(peer_addr) {
        Ok(()) => ErrorCode::SUCCESS,
        Err(code) => code,
    };

    Message::Response {
        error: err,
        data: None,
    }
}

fn group_leave_request(server_info: &SharedServer, peer_addr: SocketAddr) -> Message {
    let err = match server_info.lock().unwrap().try_leave_group(peer_addr) {
        Ok(()) => ErrorCode::SUCCESS,
        Err(code) => code,
    };

    Message::Response {
        error: err,
        data: None,
    }
}

fn group_invite_request(
    args: Vec<String>,
    server_info: &SharedServer,
    peer_addr: SocketAddr,
) -> Message {
    if args.len() != 2 {
        return Message::Response {
            error: ErrorCode::INVALID_ARGS,
            data: None,
        };
    }

    let err = match server_info
        .lock()
        .unwrap()
        .try_invite_group(args[1].clone(), peer_addr)
    {
        Ok(()) => ErrorCode::SUCCESS,
        Err(code) => code,
    };

    Message::Response {
        error: err,
        data: None,
    }
}

fn group_join_request(server_info: &SharedServer, peer_addr: SocketAddr) -> Message {
    let err = match server_info.lock().unwrap().try_join_group(peer_addr) {
        Ok(()) => ErrorCode::SUCCESS,
        Err(code) => code,
    };

    Message::Response {
        error: err,
        data: None,
    }
}

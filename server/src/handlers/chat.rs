use crate::error::ErrorCode;
use crate::protocol::{EventType, Message};
use crate::state::ServerInfo;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

pub(super) fn chat_request(
    args: Vec<String>,
    server_info: &Arc<Mutex<ServerInfo>>,
    peer_addr: SocketAddr,
) -> Message {
    if !server_info.lock().unwrap().is_connected(peer_addr) {
        return Message::Response {
            error: ErrorCode::INVALID_COMMAND,
            data: None,
        };
    }
    if args.len() <= 1 {
        return Message::Response {
            error: ErrorCode::INVALID_ARGS,
            data: None,
        };
    }
    let scope = &args[0];
    let body = args[1..].join(" ") + "\n";
    let mut binding = server_info.lock().unwrap();
    let receivers = match scope.to_uppercase().as_str() {
        "GLOBAL" => binding.get_global_receivers(peer_addr),
        _ => {
            return Message::Response {
                error: ErrorCode::INVALID_ARGS,
                data: None,
            };
        }
    };
    for con in receivers {
        let _ = con.tx.send(Message::Event {
            kind: EventType::CHAT,
            data: body.clone(),
        });
    }
    return Message::Response {
        error: ErrorCode::SUCCESS,
        data: None,
    };
}

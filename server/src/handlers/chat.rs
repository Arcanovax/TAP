use crate::error::ErrorCode;
use crate::protocol::{EventType, Message, MessageType};
use crate::state::{Connection, ServerInfo};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

pub(super) fn chat_request(
    request: Message,
    server_info: &Arc<Mutex<ServerInfo>>,
    peer_addr: SocketAddr,
) -> Message {
    if !server_info.lock().unwrap().is_connected(peer_addr) {
        return Message {
            message: MessageType::RESPONSE,
            error_response: ErrorCode::INVALID_COMMAND,
            error_code: ErrorCode::INVALID_COMMAND.code(),
            ..request
        };
    }
    if request.args.len() <= 1 {
        return Message {
            message: MessageType::RESPONSE,
            error_response: ErrorCode::INVALID_ARGS,
            error_code: ErrorCode::INVALID_ARGS.code(),
            ..request
        };
    }
    let scope = &request.args[0];
    let body = request.args[1..].join(" ") + "\n";
    let mut binding = server_info.lock().unwrap();
    let receivers = match scope.to_uppercase().as_str() {
        "GLOBAL" => binding.get_global_receivers(peer_addr),
        _ => {
            return Message {
                message: MessageType::RESPONSE,
                error_response: ErrorCode::INVALID_ARGS,
                error_code: ErrorCode::INVALID_ARGS.code(),
                ..request
            };
        }
    };
    for con in receivers {
        let _ = con.tx.send(Message {
            message: MessageType::EVENT,
            command_line: String::new(),
            response_line: String::new(),
            event_line: String::new(),
            command_name: String::new(),
            args: Vec::new(),
            error_response: ErrorCode::SUCCESS,
            error_code: ErrorCode::SUCCESS.code(),
            event_type: EventType::CHAT,
            data: body.clone(),
        });
    }
    Message {
        message: MessageType::RESPONSE,
        error_response: ErrorCode::SUCCESS,
        error_code: ErrorCode::SUCCESS.code(),
        ..request
    }
}

use crate::error::ErrorCode;
use crate::protocol::{Message, MessageType};
use crate::state::ServerInfo;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tracing::info;

pub(super) fn connect_request(
    request: Message,
    server_info: &Arc<Mutex<ServerInfo>>,
    peer_addr: SocketAddr,
) -> Message {
    if request.args.len() != 1 {
        return Message {
            message: MessageType::RESPONSE,
            error_response: ErrorCode::INVALID_ARGS,
            error_code: ErrorCode::INVALID_ARGS.code(),
            ..request
        };
    }
    match server_info
        .lock()
        .unwrap()
        .try_add_player(request.args[0].to_string(), peer_addr)
    {
        Ok(()) => {
            info!("{} is connected", request.args[0]);
            Message {
                message: MessageType::RESPONSE,
                error_response: ErrorCode::NONE,
                error_code: ErrorCode::NONE.code(),
                ..request
            }
        }
        Err(code) => Message {
            message: MessageType::RESPONSE,
            error_code: code.code(),
            error_response: code,
            ..request
        },
    }
}

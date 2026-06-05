use crate::error::ErrorCode;
use crate::protocol::Message;
use crate::state::{ServerInfo, Tx};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tracing::info;

pub(super) fn connect_request(
    args: Vec<String>,
    server_info: &Arc<Mutex<ServerInfo>>,
    peer_addr: SocketAddr,
    tx: &Tx,
) -> Message {
    if args.len() != 1 {
        return Message::Response {
            error: ErrorCode::INVALID_ARGS,
            data: None,
        };
    }
    match server_info
        .lock()
        .unwrap()
        .try_add_player(args[0].to_string(), peer_addr, tx)
    {
        Ok(()) => {
            info!("{} is connected", args[0]);
            return Message::Response {
                error: ErrorCode::SUCCESS,
                data: None,
            };
        }
        Err(code) => Message::Response {
            error: code,
            data: None,
        },
    }
}

use crate::error::ErrorCode;
use crate::protocol::Message;
use crate::state::{SharedServer, Tx};
use std::net::SocketAddr;
// use tracing::info;

#[cfg(test)]
mod tests;

pub(super) fn connect_request(
    args: &Vec<String>,
    server_info: &SharedServer,
    peer_addr: SocketAddr,
    tx: &Tx,
) -> Message {
    if args.len() != 1 {
        return Message::Response {
            error: ErrorCode::INVALID_ARGS,
            data: None,
        };
    }
    //info!("{} is connected", args[0]);
    server_info
        .lock()
        .unwrap()
        .try_add_player(args[0].to_string(), peer_addr, tx)
        .into()
}

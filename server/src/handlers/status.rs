use crate::protocol::Message;
use crate::state::SharedServer;
use std::net::SocketAddr;

#[cfg(test)]
mod tests;

pub(super) fn status_request(server_info: &SharedServer, peer_addr: SocketAddr) -> Message {
    server_info
        .lock()
        .unwrap()
        .get_player(peer_addr)
        .map(|player| serde_json::to_string(&player).unwrap())
        .into()
}

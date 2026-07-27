use crate::{protocol::Message, state::SharedServer};
use std::net::SocketAddr;

#[cfg(test)]
mod tests;

pub fn inventory_request(server_info: &SharedServer, peer_addr: SocketAddr) -> Message {
    server_info
        .lock()
        .unwrap()
        .get_player(peer_addr)
        .map(|player| {
            let items: Vec<String> = player
                .inventory
                .iter()
                .flat_map(|(s, &n)| std::iter::repeat_n(s.clone(), n as usize))
                .collect();
            serde_json::to_value(items).unwrap()
        })
        .into()
}

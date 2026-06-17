use crate::{error::ErrorCode, protocol::Message, state::SharedServer};
use std::net::SocketAddr;

#[cfg(test)]
mod tests;

pub(super) fn quest_request(
    args: &Vec<String>,
    server_info: &SharedServer,
    peer_addr: SocketAddr,
) -> Message {
    if !server_info.lock().unwrap().is_connected(peer_addr) {
        return Message::Response {
            error: ErrorCode::INVALID_COMMAND,
            data: None,
        };
    }
    if args.len() == 0 {
        return Message::Response {
            error: ErrorCode::INVALID_ARGS,
            data: None,
        };
    }

    let mut binding = server_info.lock().unwrap();
    let mut npc_ref = args.join(" ");
    if let Some(reference) = binding.world.name_to_ref.get(&npc_ref.to_lowercase()) {
        npc_ref = reference.clone();
    }
    binding.try_accept_quest(peer_addr, &npc_ref).into()
}

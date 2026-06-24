use crate::{
    protocol::{EventType, Message, Payload},
    state::SharedServer,
    structures::enums::error::ErrorCode,
};
use std::{collections::HashMap, net::SocketAddr};
use tracing::info;

#[cfg(test)]
mod tests;

pub fn take_request(
    server_info: &SharedServer,
    peer_addr: SocketAddr,
    args: &Vec<String>,
) -> Message {
    let mut binding = server_info.lock().unwrap();
    if !binding.is_connected(peer_addr) {
        return Message::Response {
            error: ErrorCode::INVALID_COMMAND,
            payload: Payload::Empty,
        };
    }
    if args.len() == 0 {
        return Message::Response {
            error: ErrorCode::INVALID_ARGS,
            payload: Payload::Empty,
        };
    }

    let mut item = args.join(" ");
    if let Some(reference) = binding.world.name_to_ref.get(&item.to_lowercase()) {
        item = reference.clone();
    }
    match binding.try_take_item(peer_addr, &item) {
        Ok(item) => {
            info!("{} taken", item);
            let receivers = binding.get_room_receivers(peer_addr).unwrap();
            let player = binding.get_player(peer_addr).unwrap();
            for con in receivers {
                let _ = con.tx.send(Message::Event(EventType::ROOM_TAKE {
                    player_name: player.name.clone(),
                    item: item.clone(),
                }));
            }
            Message::Response {
                error: ErrorCode::SUCCESS,
                payload: Payload::Pair(HashMap::from([("taken".to_string(), item)])),
            }
        }
        Err(code) => Message::Response {
            error: code,
            payload: Payload::Empty,
        },
    }
}

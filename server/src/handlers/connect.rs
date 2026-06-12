use crate::error::ErrorCode;
use crate::protocol::Message;
use crate::state::{SharedServer, Tx};
// use crate::structures::player::Player;
// use crate::structures::room::Room;
use std::net::SocketAddr;
// use serde::Serialize;
use tracing::info;

// #[derive(Serialize)]
// struct LoginResponse<'a> {
// 	player: &'a Player,
// 	room: &'a Room
// }

pub(super) fn connect_request(
    args: Vec<String>,
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
	// let mut pre_world_mut = server_info.lock().unwrap();
    // let server_mut = &mut *pre_world_mut;
    match server_info
		.lock()
		.unwrap()
        .try_add_player(args[0].to_string(), peer_addr, tx)
    {
        Ok(()) => {
            info!("{} is connected", args[0]);
			// let player = server_mut.get_player(peer_addr).unwrap();
			// let room = server_mut.get_player_room(peer_addr).unwrap();
			// let datas = LoginResponse { player: player, room: room };
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

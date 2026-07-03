use crate::{
    handlers::{
        buy::buy_request,
        consume::consume,
        dungeon::dungeon_request,
        flee::flee,
        gold::gold_request,
        item::{item_request, items_request},
        npc::{npc_request, npcs_request},
        quest::quest_request,
        quests::quests_request,
        sell::sell_request,
    }, protocol::Payload, state::{SharedServer, Tx}, structures::{
        enums::{command::Command, error::ErrorCode, state::State}, handler_outcome::HandlerOutcome,
    },
};
use std::{net::SocketAddr};

use crate::{
    handlers::{
        chat::chat_request, connect::connect_request, drop::drop_request, fight::fight_request,
        group::group_request, inventory::inventory_request, look::look_request,
        movement::move_request, status::status_request, take::take_request, talk::talk_request,
        who::who_request,
    },
    protocol::Message,
};

pub fn handle_request(
    request: &Message,
    server_info: &SharedServer,
    peer_addr: SocketAddr,
    tx: &Tx,
) -> Message {
    let handler_result: HandlerOutcome = match &request {
        Message::Command { name, args } => {

			let is_in_fight = {
				if let Ok(player) = server_info.lock().unwrap().get_player_mut(peer_addr) {
					matches!(player.status, State::InFight { .. })
				} else {
					false
				}
			};

			if is_in_fight && ["TAKE", "DROP", "QUEST", "BUY","SELL", "TALK"].contains(&name.to_uppercase().as_str()) {
				Message::Response { error: ErrorCode::FORBIDDEN_ACTION, payload: Payload::Empty }.into()
			} else {
				match Command::parse(name) {
				Some(Command::CONNECT) => connect_request(args, server_info, peer_addr, tx).into(),
				Some(Command::QUIT) => Message::Response {
					error: ErrorCode::SUCCESS,
					payload: Payload::Text("bye".to_string()),
				}
				.into(),
				Some(Command::WHO) => who_request(server_info).into(),
				Some(Command::CHAT) => chat_request(args, server_info, peer_addr).into(),
				Some(Command::GROUP) => group_request(args, server_info, peer_addr).into(),
				Some(Command::STATUS) => status_request(server_info, peer_addr).into(),
				Some(Command::MOVE) => move_request(server_info, peer_addr, args).into(),
				Some(Command::TALK) => talk_request(peer_addr, args, server_info),
				Some(Command::ATTACK) => fight_request(peer_addr, args, server_info).into(),
				Some(Command::FLEE) => flee(peer_addr, args, server_info).into(),
				Some(Command::CONSUME) => consume(peer_addr, args, server_info).into(),
				Some(Command::LOOK) => look_request(server_info, peer_addr).into(),
				Some(Command::DROP) => drop_request(server_info, peer_addr, args).into(),
				Some(Command::TAKE) => take_request(server_info, peer_addr, args).into(),
				Some(Command::INVENTORY) => inventory_request(server_info, peer_addr).into(),
				Some(Command::QUEST) => quest_request(args, server_info, peer_addr).into(),
				Some(Command::NPC) => npc_request(server_info, args).into(),
				Some(Command::NPCS) => npcs_request(server_info).into(),
				Some(Command::ITEM) => item_request(server_info, args).into(),
				Some(Command::ITEMS) => items_request(server_info).into(),
				Some(Command::QUESTS) => quests_request(server_info, peer_addr).into(),
				Some(Command::BUY) => buy_request(args, server_info, peer_addr).into(),
				Some(Command::SELL) => sell_request(args, server_info, peer_addr).into(),
				Some(Command::GOLD) => gold_request(server_info, peer_addr).into(),
				Some(Command::DUNGEON) => dungeon_request(args, server_info, peer_addr).into(),
				_ => Message::Response {
					error: ErrorCode::INVALID_COMMAND,
					payload: Payload::Empty,
				}
				.into(),
			}
			}
		}
        _ => Message::Response {
            error: ErrorCode::INVALID_COMMAND,
            payload: Payload::Empty,
        }
        .into(),
    };

    server_info
        .lock()
        .unwrap()
        .advance_quests(peer_addr, handler_result.event.as_ref());

    handler_result.message
}

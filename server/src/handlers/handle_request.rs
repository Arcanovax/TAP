use crate::{
    handlers::{
        buy::buy_request,
        consume::consume,
        dices::dices_request,
        dungeon::dungeon_request,
        flee::flee,
        gold::gold_request,
        help::help_request,
        item::{item_request, items_request},
        npc::{npc_request, npcs_request},
        quest::quest_request,
        quest_info::quest_info_request,
        quests::quests_request,
        room::{room_request, rooms_request},
        sell::sell_request,
        slot_machine::slot_machine_request,
    },
    protocol::Payload,
    state::{SharedServer, Tx},
    structures::{
        enums::{command::Command, error::ErrorCode, state::State},
        handler_outcome::HandlerOutcome,
    },
};
use rand::RngExt;
use std::net::SocketAddr;

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
            let (is_in_fight, is_in_dungeon) = {
                if let Ok(player) = server_info.lock().unwrap().get_player_mut(peer_addr) {
                    (matches!(player.status, State::InFight { .. }), player.in_dungeon)
                } else {
                    (false, false)
                }
            };

			if is_in_dungeon && ["GROUP"].contains(&name.to_uppercase().as_str()) {
				if args.len() > 0 && args[0] == "LEAVE".to_string() {
					return Message::Response {
						error: ErrorCode::FORBIDDEN_ACTION,
						payload: Payload::Empty,
					}
					.into();
				}
			}

            if is_in_fight
                && ["TAKE", "DROP", "QUEST", "BUY", "SELL", "TALK", "MOVE"]
                    .contains(&name.to_uppercase().as_str())
            {
                return Message::Response {
                    error: ErrorCode::FORBIDDEN_ACTION,
                    payload: Payload::Empty,
                }
                .into();
            }
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
                Some(Command::NPCS) => npcs_request(server_info, peer_addr).into(),
                Some(Command::ITEM) => item_request(server_info, args).into(),
                Some(Command::ITEMS) => items_request(server_info, peer_addr).into(),
                Some(Command::QUESTS) => quests_request(server_info, peer_addr).into(),
                Some(Command::BUY) => buy_request(args, server_info, peer_addr).into(),
                Some(Command::SELL) => sell_request(args, server_info, peer_addr).into(),
                Some(Command::GOLD) => gold_request(server_info, peer_addr).into(),
                Some(Command::DUNGEON) => dungeon_request(args, server_info, peer_addr).into(),
                Some(Command::QUEST_INFO) => quest_info_request(server_info, args).into(),
                Some(Command::HELP) => help_request().into(),
                Some(Command::ROOM) => room_request(server_info, args).into(),
                Some(Command::ROOMS) => rooms_request(server_info, peer_addr).into(),
                Some(Command::SLOT_MACHINE) => {
                    let pool: Vec<String> = server_info
                        .lock()
                        .unwrap()
                        .world
                        .items
                        .keys()
                        .cloned()
                        .collect();
                    let roll = move || {
                        let mut rng = rand::rng();
                        if rng.random_bool(1.0 / 10.0) {
                            Some(pool[rng.random_range(0..pool.len())].clone())
                        } else {
                            None
                        }
                    };
                    slot_machine_request(server_info, peer_addr, roll).into()
                }
                Some(Command::DICES) => {
                    let mut rng = rand::rng();
                    let mut draw = Vec::new();
                    for _ in 0..10 {
                        draw.push(rng.random_range(1..=10));
                    }
                    dices_request(server_info, peer_addr, args, draw).into()
                }
                _ => Message::Response {
                    error: ErrorCode::INVALID_COMMAND,
                    payload: Payload::Empty,
                }
                .into(),
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

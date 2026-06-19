use std::{collections::{HashMap, VecDeque}, fs::OpenOptions, io::Write};

use serde_json::Value;

use crate::{enums::{actions::PendingAction, focus::Focus, states::States}, structures::{room::Room, server_event::ServerEvent, world::World}};

pub fn response_handling(world: &mut World, server: &ServerEvent) {
	if server.error == Some("SUCCESS".to_string()) {
		match world.state {
			States::Login => {
				if world.action == PendingAction::Auth{
						world.state = States::InGame;
						world.player.name = world.input.to_string();
						world.input.clear();
						let _ = world.tx_to_serv.try_send(String::from("ITEMS\n"));
						// let _ = world.tx_to_serv.try_send(String::from("LOOK\n"));
						// if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("debug_network.txt") {
						// 	let _ = writeln!(file, "ko (State {:?}) : {:#?}", world.state, res);}
						world.action = PendingAction::Items;
						// world.action = PendingAction::ClientLook;
					}
				},
				States::InGame => {
					// if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("debug_network.txt") {
					// 	let _ = writeln!(file, "ko (State {:#?}) : {:#?}", world.action, msg);}
					match &world.action {
						PendingAction::Look
						| PendingAction::ClientLook => {
							world.room = serde_json::from_value(server.data.clone().unwrap()).unwrap();
							
							if world.action == PendingAction::Look {
								let room = serde_json::from_value::<Room>(server.data.clone().unwrap_or(Value::from("Corrupted datas"))).unwrap_or(Room::new());
								world.output.push_back(format!("[Server response] {}", room.room_view));
							}
							world.action = PendingAction::None;
						},
						PendingAction::Items => {
							world.list_items = serde_json::from_value(server.data.clone().unwrap()).unwrap();
							let _ = world.tx_to_serv.try_send(String::from("NPCS\n"));
							world.action = PendingAction::Npcs;
						},
						PendingAction::Npcs => {
							world.list_npcs = serde_json::from_value(server.data.clone().unwrap()).unwrap();
							let _ = world.tx_to_serv.try_send(String::from("LOOK\n"));
							world.action = PendingAction::ClientLook;
						}
						PendingAction::Talk(name) => {
							world.room.dialogs = serde_json::from_value(server.data.clone().unwrap()).unwrap();
							if let Some(npc) = world.list_npcs.get(name) {
								world.state = States::InDiscuss(npc.name.clone());
							} else {
								world.state = States::InDiscuss(name.clone());
							}
						}
						PendingAction::SendChat(command, args) => {
							match command.to_uppercase().as_str() {
								"CHAT GLOBAL" => world.chat.global_messages.push_back(format!("[me] {}", args.clone())),
								"CHAT GROUP" => world.chat.group_messages.push_back(format!("[me] {}", args.clone())),
								"CHAT ROOM" => world.chat.room_messages.push_back(format!("[me] {}", args.clone())),
								_ => {}
							}
							if world.room.focus != Focus::CHAT {
									world.room.chat_scroll_pos.scroll_to_bottom();
								}
							world.action = PendingAction::None;
						},
						PendingAction::Move => {
							world.chat.room_messages = VecDeque::new();
							let _ = world.tx_to_serv.try_send(String::from("LOOK\n"));
							world.action = PendingAction::ClientLook;
						}
						_ => {}
				}
				if world.room.focus != Focus::OUTPUT {
					world.room.output_scroll_pos.scroll_to_bottom();
				}
			}
			_ => {}
		}
	} else {
		if world.state == States::Login {
			world.error = true;
			world.message_error = server.error.clone().unwrap_or("Unknown error".to_string());
			world.click = true;
		} else {
			match world.action {
				PendingAction::Items => {
					let _ = world.tx_to_serv.try_send(String::from("NPCS\n"));
					world.action = PendingAction::Npcs;
				},
				PendingAction::Npcs => {
					let _ = world.tx_to_serv.try_send(String::from("LOOK\n"));
					world.action = PendingAction::ClientLook;
				}
				_ => world.output.push_back(format!("[Error] {}", server.error.clone().unwrap_or("Unknown error".to_string())))
			}
			
		}
	}
}
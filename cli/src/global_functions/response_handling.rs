use std::{collections::{VecDeque}};

use crate::{enums::{actions::PendingAction, focus::Focus, item_kind::ItemKind, npc_kind::NPCKind, states::States}, structures::{attack_results::Attack_Result, room::Room, status_view::StatusView, world::World}};

pub fn response_handling(world: &mut World, answers: Vec<&str>) {
	// if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("debug_network.txt") {
	// 	let _ = writeln!(file, "ko (State {:?}) : {:#?}", world.state, answers);}
	let real_answer = answers[1..].join(" ");
	if answers[0] == "OK" {
		match world.state {
			States::ServerWait => {
				world.state = States::Login;
			}
			States::Login => {
				if world.action == PendingAction::Auth{
					world.state = States::Idle;
					world.message = String::new();
					world.player.name = world.input.to_string();
					world.input.clear();
					let _ = world.tx_to_serv.try_send(String::from("ITEMS\n"));
					world.action = PendingAction::Items;
				}
			},
			States::Idle
			| States::InFight { .. } => {
				match &world.action {
					PendingAction::Look
					| PendingAction::ClientLook => {
						world.room = serde_json::from_str(&real_answer).unwrap();
						if world.action == PendingAction::Look {
							let room = serde_json::from_str::<Room>(&real_answer).unwrap_or(Room::new());
							world.output.push_back(format!("[Server response] {}", room.room));
						}
						let _ = world.tx_to_serv.try_send(String::from("STATUS\n"));
						world.action = PendingAction::ClientStatus;
					},
					
					PendingAction::Status
					| PendingAction::ClientStatus => {
						let status: StatusView = serde_json::from_str::<StatusView>(&real_answer).unwrap();
						if world.action == PendingAction::Status {
							world.output.push_back(format!("[Server response] {}", status));
						}
						world.player.hp = status.hp;
						world.state = status.status;
						let _ = world.tx_to_serv.try_send(String::from("INVENTORY\n"));
						world.action = PendingAction::Inventory;
					},
					
					PendingAction::Items => {
						world.list_items = serde_json::from_str(&real_answer).unwrap();
						let _ = world.tx_to_serv.try_send(String::from("NPCS\n"));
						world.action = PendingAction::Npcs;
					},
					
					PendingAction::Npcs => {
						world.list_npcs = serde_json::from_str(&real_answer).unwrap();
						let _ = world.tx_to_serv.try_send(String::from("LOOK\n"));
						world.action = PendingAction::ClientLook;
					},
					
					PendingAction::Flee => {
						world.output.push_back(format!("[Server response] {}", real_answer));
						world.state = States::Idle;
						let _ = world.tx_to_serv.try_send("GOLD\n".to_string());
						world.action = PendingAction::Gold;
					},
					
					PendingAction::Gold => {
						let gold: Vec<&str> = real_answer.split("=").collect();
						world.player.gold = gold[1].parse().unwrap();
						world.action = PendingAction::None;
					},

					PendingAction::Inventory
					| PendingAction::ClientInventory => {
						let mut item: &str = "";
						let mut count = 1;
						let list_items: Vec<String> = serde_json::from_str(&real_answer).unwrap();
						for (i, line) in list_items.iter().enumerate() {
							if item != line {
								if i > 0 {
									world.player.inventory.insert(item.to_string(), count);
								}
								item = line;
								count = 1;
							} else {
								count += 1;
							}
						}
						if item != "" {
							world.player.inventory.insert(item.to_string(), count);
						}

						if world.action == PendingAction::ClientInventory {
							for (item, number) in &world.player.inventory {
								world.output.push_back(format!("{} x{}", item, number));
							}
						}
						let _ = world.tx_to_serv.try_send(String::from("GOLD\n"));
						
						world.action = PendingAction::Gold;
					},
					
					PendingAction::Talk(name) => {
						for sentence in real_answer.split("\\") {
							world.room.dialogs.push_back(sentence.to_string());
						}
						if let Some(npc) = world.list_npcs.get(name) {
							world.state = States::InDiscuss(npc.name.clone(), world.room.dialogs.pop_front().unwrap_or("".to_string()));
						} else {
							world.state = States::InDiscuss(name.clone(), world.room.dialogs.pop_front().unwrap_or("".to_string()));
						}
						world.room.npc_list_state.select(None);
					},
					
					PendingAction::Consume(item) => {
						// if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("debug_network.txt") {
						// 	let _ = writeln!(file, "real (State {:?}) : {:#?}", count, line);}
						let quantity = world.player.inventory.get(item).unwrap();
						if *quantity > 1 {
							world.player.inventory.insert(item.clone(), quantity - 1);
						} else {
							world.player.inventory.remove(item);
						}
						if world.state == States::Idle {
							world.output.push_back(format!("[server response] You have successfully used {}.", item));
						}
						let heal = {
							let item_name = world.list_items.get(&format!("item.{}", item));
							match item_name {
								Some(it) => {
									if let ItemKind::Potion { healing } = it.kind {
										healing
									} else {0 as u32}
								},
								None => 0 as u32
							}
						};
						// if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("debug_network.txt") {
						// 		let _ = writeln!(file, "real (State {:#?}) :", heal);}
						world.player.hp = world.player.hp.saturating_add(heal).min(100);
						world.action = PendingAction::None;
					},

					PendingAction::GroupCreate(name) => {
						world.group.in_group = true;
						world.output.push_back(format!("{name} group successfully created."));
						world.action = PendingAction::None;
					},

					PendingAction::GroupJoin(name) => {
						world.group.in_group = true;
						world.output.push_back(format!("{name} group successfully joined."));
						world.action = PendingAction::None;
					},

					PendingAction::GroupInvite(name) => {
						world.output.push_back(format!("Invitation successfully sended to {name}."));
						world.action = PendingAction::None;
					},

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
					PendingAction::Attack(name) => {
						let result: Attack_Result = serde_json::from_str(&real_answer).unwrap();
						world.player.hp = result.attacker_hp;
						let fight = &mut world.room.fight;
						if fight.target_name == "".to_string() {
							fight.target_name = match world.list_npcs.get(name) {Some(npc) => npc.name.clone(), None => name.clone()};
							// It works with our own server but if its not, the max_hp will be the hp at the moment we enter in fight, doesn't matter if the fight begin earlier.
							fight.target_max_hp = match world.list_npcs.get(name) {Some(npc) => if let NPCKind::Enemy { max_hp, ..} = npc.kind {max_hp} else {result.target_hp}, None => result.target_hp};
						}
						match result.fighters {
							Some(fighters) => fight.fighters = fighters,
							None => {}
						}
						fight.target_hp = result.target_hp;
						world.state = States::InFight { target_id: fight.target_name.clone() };
						world.room.focus = Focus::COMMAND;
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
			world.message_error = real_answer.to_string();
			world.click = true;
		} else {
			match &world.action {
				PendingAction::Items => {
					let _ = world.tx_to_serv.try_send(String::from("NPCS\n"));
					world.action = PendingAction::Npcs;
				},
				PendingAction::Npcs => {
					let _ = world.tx_to_serv.try_send(String::from("LOOK\n"));
					world.action = PendingAction::ClientLook;
				}
				PendingAction::SendChat(command, args) => {
					world.output.push_back(format!("> {command} {args}"));
					world.output.push_back(format!("[Error] {}", real_answer));
					world.action = PendingAction::None;
				}
				_ => {
					// world.output.push_back(format!("[Error] coucou"));
					world.output.push_back(format!("[Error] {}", real_answer));
					world.action = PendingAction::None;
				}

			}
			
		}
	}
}
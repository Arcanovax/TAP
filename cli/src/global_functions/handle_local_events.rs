use std::{fs::OpenOptions, io::Write};

use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{
    enums::{
        actions::PendingAction, channels::Channels, focus::Focus, item_kind::ItemKind,
        npc_kind::NPCKind, states::States,
    },
    global_functions::find_action::find_action,
    structures::world::World,
};

pub fn handle_global_events(key: KeyEvent, world: &mut World) {
    if world.room.focus == Focus::COMMAND
        && ![KeyCode::Tab, KeyCode::Enter, KeyCode::Up, KeyCode::Down].contains(&key.code)
    {
        world.room.text_area.input(key);
        world.index_command = 0;
    } else {
        match key.code {
            KeyCode::Down => match world.room.focus {
                Focus::CHAT => world.room.chat_scroll_pos.scroll_down(),
                Focus::DESCR => world.room.descr_scroll_pos.scroll_down(),
                Focus::OUTPUT => world.room.output_scroll_pos.scroll_down(),
                Focus::NPC => world.room.npc_list_state.select_next(),
                Focus::INVENTORY => world.room.inventory_list_state.select_next(),
                Focus::EXITS => world.room.exits_list_state.select_next(),
                Focus::COMMAND => {
                    if world.index_command > 0 {
                        world.index_command = world.index_command.saturating_sub(1);
                        world.room.text_area.clear();
                        if world.index_command > 0 {
                            if let Some(command) = world.old_command.get(world.index_command - 1) {
                                world.room.text_area.insert_str(command);
                            }
                        }
                    }
                }
                Focus::BAG => world.room.bag_state.select_next(),
                _ => {}
            },
            KeyCode::Up => {
                // if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("debug_network.txt") {
                // 		let _ = writeln!(file, "ko (State {:?}) : {:#?}", world.old_command, world.index_command);}
                match world.room.focus {
                    Focus::CHAT => world.room.chat_scroll_pos.scroll_up(),
                    Focus::DESCR => world.room.descr_scroll_pos.scroll_up(),
                    Focus::OUTPUT => world.room.output_scroll_pos.scroll_up(),
                    Focus::NPC => world.room.npc_list_state.select_previous(),
                    Focus::INVENTORY => world.room.inventory_list_state.select_previous(),
                    Focus::EXITS => world.room.exits_list_state.select_previous(),
                    Focus::COMMAND => {
                        if world.index_command < world.old_command.len() {
                            world.index_command += 1;
                            world.room.text_area.clear();
                            if let Some(command) = world.old_command.get(world.index_command - 1) {
                                world.room.text_area.insert_str(command);
                            }
                        }
                    }
                    Focus::BAG => world.room.bag_state.select_previous(),
                    _ => {}
                }
            }
            KeyCode::Left => match world.room.focus {
                Focus::CHAT => {
                    world.chat.channel = match world.chat.channel {
                        Channels::GLOBAL => Channels::GROUP,
                        Channels::GROUP => Channels::ROOM,
                        Channels::ROOM => Channels::GLOBAL,
                    }
                }
                _ => {}
            },
            KeyCode::Right => match world.room.focus {
                Focus::CHAT => {
                    world.chat.channel = match world.chat.channel {
                        Channels::GLOBAL => Channels::ROOM,
                        Channels::GROUP => Channels::GLOBAL,
                        Channels::ROOM => Channels::GROUP,
                    }
                }
                _ => {}
            },
            KeyCode::Tab => {
                let current_index = Focus::iterator(&world.state)
                    .position(|f| f == &world.room.focus)
                    .unwrap_or(0);

                let next_index = (current_index + 1) % Focus::iterator(&world.state).len();
                world.room.focus = Focus::iterator(&world.state)
                    .nth(next_index)
                    .unwrap()
                    .clone();
                world.room.exits_list_state.select(None);
                world.room.inventory_list_state.select(None);
                world.room.npc_list_state.select(None);
                match world.room.focus {
                    Focus::EXITS => world.room.exits_list_state.select_first(),
                    Focus::INVENTORY => world.room.inventory_list_state.select_first(),
                    Focus::NPC => world.room.npc_list_state.select_first(),
                    _ => {}
                }
            }
            KeyCode::Enter => match world.room.focus {
                Focus::COMMAND => {
                    if !world.room.text_area.is_empty() {
                        let command = world.room.text_area.lines().join("");
                        world.index_command = 0;
                        world.old_command.push_front(command.clone());
                        let split_command: Vec<&str> = command.split(" ").collect();
                        let _ = world.tx_to_serv.try_send(split_command.join(" ") + "\n");

                        if !["CHAT"].contains(&split_command[0].to_uppercase().as_str()) {
                            world
                                .output
                                .push_back(format!("\n> {}", split_command.join(" ")));
                            world.room.output_scroll_pos.scroll_to_bottom();
                        }

                        find_action(split_command, world);
                        world.room.text_area.clear();
                    }
                }
                Focus::EXITS => {
                    if let Some(index) = world.room.exits_list_state.selected_mut() {
                        if let Some(dir) = world.room.room.exits.keys().nth(*index) {
                            world
                                .output
                                .push_back(format!("\n> {}", format!("MOVE {}\n", dir)));
                            world.room.output_scroll_pos.scroll_to_bottom();
                            let _ = world.tx_to_serv.try_send(format!("MOVE {}\n", dir));
                            world.action = PendingAction::Move;
                        } else {
                        }
                    }
                }
                Focus::NPC => {
                    if let Some(index) = world.room.npc_list_state.selected_mut() {
                        if let Some(selected_npc) = world.room.npcs.get(*index) {
                            if let Some(npc) = world.list_npcs.get(selected_npc) {
                                match &npc.kind {
                                    NPCKind::Enemy { .. } => {
                                        world.output.push_back(format!(
                                            "\n> {}",
                                            format!("attack {}\n", selected_npc)
                                        ));
                                        world.room.output_scroll_pos.scroll_to_bottom();
                                        let _ = world
                                            .tx_to_serv
                                            .try_send(format!("attack {}\n", selected_npc));
                                        world.action = PendingAction::Attack(selected_npc.clone());
                                    }
                                    NPCKind::Citizen => {
                                        world.output.push_back(format!(
                                            "\n> {}",
                                            format!("TALK {}\n", selected_npc)
                                        ));
                                        world.room.output_scroll_pos.scroll_to_bottom();
                                        let _ = world
                                            .tx_to_serv
                                            .try_send(format!("TALK {}\n", selected_npc));
                                        world.action = PendingAction::Talk(selected_npc.clone());
                                    }
                                    NPCKind::Merchant { inventory } => {
                                        world.room.focus = Focus::SELL;
                                        world.room.sell_list_state.select_first();
                                        world.state =
                                            States::Trade(inventory.clone(), selected_npc.clone());
                                    }
                                }
                            }
                        }
                        world.room.npc_list_state.select(None);
                    }
                }
                Focus::INVENTORY | Focus::BAG => {
                    if world.room.focus == Focus::BAG && world.room.bag.len() == 0 {
                        world.room.fight.bag = false;
                        world.room.focus = Focus::COMMAND;
                    } else {
                        if world.room.focus == Focus::INVENTORY && world.player.inventory.len() == 0
                        {
                            world.output.push_back(
                                "\nAre you really trying to use... nothing?".to_string(),
                            );
                        } else {
                            let item_name = {
                                if world.room.focus == Focus::INVENTORY {
                                    let index = world.room.inventory_list_state.selected().unwrap();
                                    world.player.inventory.keys().nth(index)
                                } else {
                                    let index = world.room.bag_state.selected().unwrap();
                                    world.room.fight.bag = false;
                                    world.room.bag.iter().nth(index)
                                }
                            };
                            match item_name {
                                Some(item) => {
                                    if let Some(item_obj) = world.list_items.get(item) {
                                        match item_obj.kind {
                                            ItemKind::Potion { .. } => {
                                                world.output.push_back(format!(
                                                    "\n> {}",
                                                    format!("CONSUME {}\n", item)
                                                ));
                                                let _ = world
                                                    .tx_to_serv
                                                    .try_send(format!("CONSUME {}\n", item));
                                                world.action = PendingAction::Consume(item.clone());
                                            }
                                            _ => {
                                                world.output.push_back(format!(
                                                    "\n> {}",
                                                    format!("DROP {}\n", item)
                                                ));
                                                let _ = world
                                                    .tx_to_serv
                                                    .try_send(format!("DROP {}\n", item));
                                                world.action = PendingAction::Drop(item.clone());
                                            }
                                        }
                                    }
                                    world.room.focus = Focus::COMMAND;
                                    world.room.output_scroll_pos.scroll_to_bottom();
                                }
                                None => {
                                    world.output.push_back(
                                        "An error occurs, impossible to find your selected object."
                                            .to_string(),
                                    );
                                }
                            }
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }
}

use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    enums::{
        actions::PendingAction, channels::Channels, focus::Focus, item_kind::ItemKind,
        npc_kind::NPCKind, states::States,
    },
    global_functions::{check_text_areas::check_text_areas, find_action::find_action},
    structures::world::World,
};

pub fn handle_global_events(key: KeyEvent, world: &mut World) {
    if !check_text_areas(world, key) {
        match key.code {
            KeyCode::Down => match world.room.focus {
                Focus::CHAT => world.room.chat_scroll_pos.scroll_down(),
                Focus::DESCR => world.room.descr_scroll_pos.scroll_down(),
                Focus::OUTPUT => world.room.output_scroll_pos.scroll_down(),
                Focus::NPC => world.room.npc_list_state.select_next(),
                Focus::INVENTORY => world.room.inventory_list_state.select_next(),
                Focus::EXITS => world.room.exits_list_state.select_next(),
                Focus::QUESTS => world.room.quests_list_state.select_next(),
                Focus::CHOICE(..) => world.room.choice_list_state.select_next(),
                Focus::COMMAND => {
                    if world.index_command > 0 {
                        world.index_command = world.index_command.saturating_sub(1);
                        world.room.text_area.clear();
                        if world.index_command > 0
                            && let Some(command) = world.old_command.get(world.index_command - 1) {
                                world.room.text_area.insert_str(command);
                            }
                    }
                }
                Focus::BAG => world.room.bag_state.select_next(),
                _ => {}
            },
            KeyCode::F(number) => {
                match number {
                    1 => world.room.focus = Focus::COMMAND,
                    2 => world.room.focus = Focus::CHATTEXT,
                    7 => world.chat.channel = Channels::GLOBAL,
                    8 => world.chat.channel = Channels::ROOM,
                    9 => world.chat.channel = Channels::GROUP,
                    _ => match world.state {
                        States::InFight { .. } => {}
                        _ => match number {
                            3 => world.room.focus = Focus::NPC,
                            4 => world.room.focus = Focus::INVENTORY,
                            5 => world.room.focus = Focus::QUESTS,
                            6 => world.room.focus = Focus::EXITS,
                            _ => {}
                        },
                    },
                }
                world.room.exits_list_state.select(None);
                world.room.inventory_list_state.select(None);
                world.room.npc_list_state.select(None);
                world.room.quests_list_state.select(None);
                match world.room.focus {
                    Focus::EXITS => world.room.exits_list_state.select_first(),
                    Focus::INVENTORY => world.room.inventory_list_state.select_first(),
                    Focus::NPC => world.room.npc_list_state.select_first(),
                    Focus::QUESTS => world.room.quests_list_state.select_first(),
                    _ => {}
                }
            }
            KeyCode::Up => match world.room.focus {
                Focus::CHAT => world.room.chat_scroll_pos.scroll_up(),
                Focus::DESCR => world.room.descr_scroll_pos.scroll_up(),
                Focus::OUTPUT => world.room.output_scroll_pos.scroll_up(),
                Focus::NPC => world.room.npc_list_state.select_previous(),
                Focus::INVENTORY => world.room.inventory_list_state.select_previous(),
                Focus::EXITS => world.room.exits_list_state.select_previous(),
                Focus::QUESTS => world.room.quests_list_state.select_previous(),
                Focus::CHOICE(..) => world.room.choice_list_state.select_previous(),
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
            },
            KeyCode::Left => if world.room.focus == Focus::CHAT {
                world.chat.channel = match world.chat.channel {
                    Channels::GLOBAL => Channels::GROUP,
                    Channels::GROUP => Channels::ROOM,
                    Channels::ROOM => Channels::GLOBAL,
                }
            },
            KeyCode::Right => if world.room.focus == Focus::CHAT {
                world.chat.channel = match world.chat.channel {
                    Channels::GLOBAL => Channels::ROOM,
                    Channels::GROUP => Channels::GLOBAL,
                    Channels::ROOM => Channels::GROUP,
                }
            },
            KeyCode::Tab | KeyCode::BackTab => {
                let current_index = Focus::iterator(&world.state)
                    .position(|f| f == &world.room.focus)
                    .unwrap_or(0);

                let final_index = {
                    if key.modifiers == KeyModifiers::SHIFT {
                        if current_index == 0 {
                            Focus::iterator(&world.state).len() - 1
                        } else {
                            current_index.saturating_sub(1)
                        }
                    } else {
                        current_index + 1
                    }
                };
                let next_index = (final_index) % Focus::iterator(&world.state).len();
                world.room.focus = Focus::iterator(&world.state)
                    .nth(next_index)
                    .unwrap()
                    .clone();
                world.room.exits_list_state.select(None);
                world.room.inventory_list_state.select(None);
                world.room.npc_list_state.select(None);
                world.room.quests_list_state.select(None);
                match world.room.focus {
                    Focus::EXITS => world.room.exits_list_state.select_first(),
                    Focus::INVENTORY => world.room.inventory_list_state.select_first(),
                    Focus::NPC => world.room.npc_list_state.select_first(),
                    Focus::QUESTS => world.room.quests_list_state.select_first(),
                    _ => {}
                }
            }
            KeyCode::Enter => match &world.room.focus {
                Focus::COMMAND => {
                    if !world.room.text_area.is_empty() {
                        let command = world.room.text_area.lines().join(" ").trim().to_string();
                        world.index_command = 0;
                        world.old_command.push_front(command.clone());
                        let split_command: Vec<&str> = command.split(" ").collect();
                        let _ = world.tx_to_serv.try_send(split_command.join(" ") + "\n");

                        if command.to_lowercase() == "quit" {
                            world.quit = true;
                        }

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
                Focus::CHATTEXT => {
                    if !world.room.chat_text_area.is_empty() {
                        let message = world.room.chat_text_area.lines().join(" ");
                        let scope = match world.chat.channel {
                            Channels::GLOBAL => "GLOBAL",
                            Channels::ROOM => "ROOM",
                            Channels::GROUP => "GROUP",
                        };
                        let _ = world
                            .tx_to_serv
                            .try_send(format!("CHAT {scope} {}\n", message));
                        world.action = PendingAction::SendChat(format!("CHAT {}", scope), message);
                        world.room.chat_text_area.clear();
                    }
                }
                Focus::EXITS => {
                    if let Some(index) = world.room.exits_list_state.selected_mut()
                        && let Some(dir) = world.room.room.exits.keys().nth(*index) {
                            world
                                .output
                                .push_back(format!("\n> {}", format!("MOVE {}\n", dir)));
                            world.room.output_scroll_pos.scroll_to_bottom();
                            let _ = world.tx_to_serv.try_send(format!("MOVE {}\n", dir));
                            world.action = PendingAction::Move;
                        }
                }
                Focus::CHOICE(second, selected_npc, inventory) => {
                    if let Some(index) = world.room.choice_list_state.selected() {
                        match index {
                            0 => {
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
                            1 => match second.as_str() {
                                "Attack" => {
                                    world.output.push_back(format!(
                                        "\n> {}",
                                        format!("ATTACK {}\n", selected_npc)
                                    ));
                                    world.room.output_scroll_pos.scroll_to_bottom();
                                    let _ = world
                                        .tx_to_serv
                                        .try_send(format!("ATTACK {}\n", selected_npc));
                                    world.action = PendingAction::Attack(selected_npc.clone());
                                }
                                "Shop" => {
                                    world.room.sell_list_state.select_first();
                                    world.state =
                                        States::Trade(inventory.clone(), selected_npc.clone());
                                    world.room.focus = Focus::SELL;
                                }
                                _ => {}
                            },
                            _ => {}
                        }
                    }
                }
                Focus::NPC => {
                    if let Some(index) = world.room.npc_list_state.selected() {
                        if let Some(selected_npc) = world.room.npcs.get(index)
                            && let Some(npc) = world.list_npcs.get(selected_npc) {
                                match &npc.kind {
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
                                    NPCKind::Enemy { .. } => {
                                        world.room.focus = Focus::CHOICE(
                                            "Attack".to_string(),
                                            selected_npc.clone(),
                                            Vec::new(),
                                        );
                                        world.room.choice_list_state.select_first();
                                    }
                                    NPCKind::Merchant { inventory } => {
                                        world.room.focus = Focus::CHOICE(
                                            "Shop".to_string(),
                                            selected_npc.clone(),
                                            inventory.clone(),
                                        );
                                        world.room.choice_list_state.select_first();
                                    }
                                }
                            }
                        world.room.npc_list_state.select(None);
                    }
                }
                Focus::INVENTORY | Focus::BAG => {
                    if world.room.focus == Focus::BAG && world.room.bag.is_empty() {
                        world.room.fight.bag = false;
                        world.room.focus = Focus::COMMAND;
                    } else {
                        if world.room.focus == Focus::INVENTORY && world.player.inventory.is_empty()
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
                                    world.room.bag.get(index)
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
                                    if world.room.focus == Focus::BAG {
                                        world.room.focus = Focus::COMMAND;
                                    }
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

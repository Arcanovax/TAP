use std::{collections::VecDeque, fmt::format, fs::OpenOptions, io::Write};

use crate::{
    enums::{
        actions::PendingAction, focus::Focus, item_kind::ItemKind, npc_kind::NPCKind,
        states::States,
    }, global_functions::check_goals::check_goals, structures::{
        attack_results::AttackResult, npc::NPC, quest::Quest, quest_view::{QuestView, QuestsView}, room::{Room, RoomPayload}, status_view::StatusView, world::World,
    },
};

pub fn response_handling(world: &mut World, answers: Vec<&str>) {
    // if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("debug_network.txt") {
    // 	let _ = writeln!(file, "ko (State {:?}) : {:#?}", world.state, answers);}
    let real_answer = answers[1..].join(" ");
    if answers[0] == "OK" {
        match &world.state {
            States::ServerWait => {
                world.state = States::Login;
            }
            States::Login => {
                if world.action == PendingAction::Auth {
                    world.state = States::Idle;
                    world.message = String::new();
                    world.player.name = world.input.to_string();
                    world.input.clear();
                    let _ = world.tx_to_serv.try_send(String::from("ITEMS\n"));
                    world.action = PendingAction::Items;
                }
            }
            States::Idle | States::InFight { .. } => {
                match &world.action {
                    PendingAction::Look | PendingAction::ClientLook | PendingAction::MoveLook => {
                        let payload: RoomPayload = serde_json::from_str(&real_answer).unwrap();
                        if world.action == PendingAction::Look {
                            world.room.apply_update(payload);
                            world
                                .output
                                .push_back(format!("[Server response]\n{}", world.room));
                            world.action = PendingAction::None;
                        } else {
                            world.room = Room::new();
                            world.room.apply_update(payload);
                            if world.action == PendingAction::ClientLook {
                                let _ = world.tx_to_serv.try_send(String::from("STATUS\n"));
                                world.action = PendingAction::ClientStatus;
                            } else {
                                world.action = PendingAction::None;
                            }
                        }
                    }

                    PendingAction::Quest => {
                        let view_quest: QuestView = serde_json::from_str(&real_answer).unwrap();
                        world.output.push_back(format!(
                            "[Server Response] You accept this quest:\n {:#?}",
                            view_quest
                        ));
                        let _ = world.tx_to_serv.try_send("QUESTS\n".to_string());
                        world.action = PendingAction::Quests;
                    }

                    PendingAction::QuestInfo(id) => {
						let mut new_quest: Quest = serde_json::from_str(&real_answer).unwrap();
						check_goals(world.list_npcs.clone(), &mut new_quest);
                        world
                            .player
                            .quests
                            .insert(id.clone(), new_quest);
                        let quests_number = world.player.quests.len();
                        if quests_number == world.player.quests_views.len() {
                            if world.rooms.len() == 0 {
                                let _ = world.tx_to_serv.try_send("ROOMS\n".to_string());
                                world.action = PendingAction::Rooms;
                            } else {
                                world.action = PendingAction::None;
                            }
                        } else {
                            let _ = world.tx_to_serv.try_send(format!(
                                "QUEST_INFO {}\n",
                                world.player.quests_views[quests_number].quest_id
                            ));
                        }
                    }

                    PendingAction::Rooms => {
                        world.rooms = serde_json::from_str(&real_answer).unwrap();
                        world.action = PendingAction::None;
                    }

                    PendingAction::Drop(item) => {
                        if let Some(item_obj) = world.list_items.get(item) {
                            world.output.push_back(format!(
                                "You successfully droped one {}",
                                item_obj.name
                            ));
                        } else {
                            world
                                .output
                                .push_back(format!("You successfully droped one {}", item));
                        }
                        world
                            .player
                            .inventory
                            .entry(item.to_string())
                            .and_modify(|f| *f -= 1);
                        if *world.player.inventory.get(&item.to_string()).unwrap_or(&0) < 1 {
                            world.player.inventory.remove(&item.to_string());
                        }
                    }

                    PendingAction::Take(item) => {
                        if let Some(item_obj) = world.list_items.get(item) {
                            world.output.push_back(format!(
                                "You successfully retrieved one {}",
                                item_obj.name
                            ));
                        } else {
                            world
                                .output
                                .push_back(format!("You successfully retrieved one {}", item));
                        }
                        let quantity = world.player.inventory.entry(item.to_string()).or_insert(0);
                        *quantity += 1;
                    }

                    PendingAction::Status | PendingAction::ClientStatus => {
                        let status: StatusView =
                            serde_json::from_str::<StatusView>(&real_answer).unwrap();
                        world.player.hp = status.hp;
                        world.state = status.status.clone();
                        if world.action == PendingAction::Status {
                            world
                                .output
                                .push_back(format!("[Server response] {}", status));
                            world.action = PendingAction::None;
                        } else {
                            let _ = world.tx_to_serv.try_send(String::from("INVENTORY\n"));
                            world.action = PendingAction::ClientInventory;
                        }
                    }

                    PendingAction::Items => {
                        world.list_items = serde_json::from_str(&real_answer).unwrap();
                        let _ = world.tx_to_serv.try_send(String::from("NPCS\n"));
                        world.action = PendingAction::Npcs;
                    }

                    PendingAction::Npcs => {
                        world.list_npcs = serde_json::from_str(&real_answer).unwrap();
                        let _ = world.tx_to_serv.try_send(String::from("LOOK\n"));
                        world.action = PendingAction::ClientLook;
                    }

                    PendingAction::Flee => {
                        world
                            .output
                            .push_back(format!("[Server response] {}", real_answer));
                        world.state = States::Idle;
                        let _ = world.tx_to_serv.try_send("GOLD\n".to_string());
                        world.action = PendingAction::Gold;
                    }

                    PendingAction::Gold => {
                        let gold: Vec<&str> = real_answer.split("=").collect();
                        world.player.gold = gold[1].parse().unwrap();
                        let _ = world.tx_to_serv.try_send("QUESTS\n".to_string());
                        world.action = PendingAction::Quests;
                    }

                    PendingAction::Quests => {
                        // if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("debug_network.txt") {
                        // 	let _ = writeln!(file, "ok (State {:#?}) :", real_answer);}
                        let quests_list: Vec<QuestsView> =
                            serde_json::from_str(&real_answer).unwrap();
                        if quests_list.len() > 0 && (quests_list.len() != world.player.quests.len())
                        {
							// world.player.quests = HashMap::new();
                            let id = &quests_list[0].quest_id.clone();
                            world.player.quests_views = quests_list;
                            let _ = world.tx_to_serv.try_send(format!("QUEST_INFO {}\n", id));
                            world.action = PendingAction::QuestInfo(id.clone());
                        } else {
							if world.rooms.len() == 0 {
                                let _ = world.tx_to_serv.try_send("ROOMS\n".to_string());
                                world.action = PendingAction::Rooms;
                            } else {
                                world.action = PendingAction::None;
                            }
                        }
                    }

                    PendingAction::Inventory | PendingAction::ClientInventory => {
                        let list_items: Vec<String> = serde_json::from_str(&real_answer).unwrap();

                        world.player.inventory.clear();

                        for item in list_items {
                            *world.player.inventory.entry(item).or_insert(0) += 1;
                        }

                        if world.action == PendingAction::Inventory {
                            world.output.push_back("In your inventory: ".to_string());
                            for (item, number) in &world.player.inventory {
                                world.output.push_back(format!("- {} x{}", item, number));
                            }
                            if world.player.inventory.len() == 0 {
                                world.output.push_back("Nothing".to_string());
                            }
                        }
                        let _ = world.tx_to_serv.try_send(String::from("GOLD\n"));
                        world.action = PendingAction::Gold;
                    }

                    PendingAction::Talk(name) => {
                        for sentence in real_answer.split("\\") {
                            world.room.dialogs.push_back(sentence.to_string());
                        }
                        if let Some(npc) = world.list_npcs.get(name) {
                            world.state = States::InDiscuss(
                                npc.name.clone(),
                                world.room.dialogs.pop_front().unwrap_or("".to_string()),
                            );
                        } else {
                            world.state = States::InDiscuss(
                                name.clone(),
                                world.room.dialogs.pop_front().unwrap_or("".to_string()),
                            );
                        }
                        world.room.npc_list_state.select(None);
                    }

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
                            world.output.push_back(format!(
                                "[server response] You have successfully used {}.",
                                item
                            ));
                        }
                        let heal = {
                            let item_name = world.list_items.get(item);
                            match item_name {
                                Some(it) => {
                                    if let ItemKind::Potion { healing } = it.kind {
                                        healing
                                    } else {
                                        0 as u32
                                    }
                                }
                                None => 0 as u32,
                            }
                        };
                        world.player.hp = world.player.hp.saturating_add(heal).min(100);
                        world.action = PendingAction::None;
                    }

                    PendingAction::GroupCreate(name) => {
                        world.group.in_group = true;
                        world
                            .output
                            .push_back(format!("{name} group successfully created."));
                        world.action = PendingAction::None;
                    }

                    PendingAction::Who => {
                        let numbers: Vec<&str> = real_answer.split("=").collect();
                        let number: u32 = numbers[1].parse().unwrap();
                        let be = if number > 1 {
                            "are".to_string()
                        } else {
                            "is".to_string()
                        };
                        let plural = if number > 1 {
                            "players".to_string()
                        } else {
                            "player".to_string()
                        };
                        world.output.push_back(format!(
                            "Currently, there {be} {number} {plural} connected."
                        ));
                    }

                    PendingAction::Npc => {
                        let npc_view: NPC = serde_json::from_str(&real_answer).unwrap();
                        world.output.push_back(format!("{npc_view}"));
                    }

                    PendingAction::GroupJoin(name) => {
                        world.group.in_group = true;
                        world
                            .output
                            .push_back(format!("{name} group successfully joined."));
                        world.action = PendingAction::None;
                    }

                    PendingAction::GroupInvite(name) => {
                        world
                            .output
                            .push_back(format!("Invitation successfully sended to {name}."));
                        world.action = PendingAction::None;
                    }

                    PendingAction::SendChat(command, args) => {
                        match command.to_uppercase().as_str() {
                            "CHAT GLOBAL" => world
                                .chat
                                .global_messages
                                .push_back(format!("[me] {}", args.clone())),
                            "CHAT GROUP" => world
                                .chat
                                .group_messages
                                .push_back(format!("[me] {}", args.clone())),
                            "CHAT ROOM" => world
                                .chat
                                .room_messages
                                .push_back(format!("[me] {}", args.clone())),
                            _ => {}
                        }
                        if world.room.focus != Focus::CHAT {
                            world.room.chat_scroll_pos.scroll_to_bottom();
                        }
                        world.action = PendingAction::None;
                    }

                    PendingAction::Move => {
                        world.chat.room_messages = VecDeque::new();
                        let _ = world.tx_to_serv.try_send(String::from("LOOK\n"));
                        world.action = PendingAction::MoveLook;
                    }
                    PendingAction::Attack(name) => {
                        let result: AttackResult = serde_json::from_str(&real_answer).unwrap();
                        world.player.hp = result.attacker_hp;
                        let fight = &mut world.room.fight;
                        if fight.target_name == "".to_string() {
                            fight.target_name = match world.list_npcs.get(name) {
                                Some(npc) => npc.name.clone(),
                                None => name.clone(),
                            };
                            // It works with our own server but if its not, the max_hp will be the hp at the moment we enter in fight, doesn't matter if the fight begin earlier.
                            fight.target_max_hp = match world.list_npcs.get(name) {
                                Some(npc) => {
                                    if let NPCKind::Enemy { max_hp, .. } = npc.kind {
                                        max_hp
                                    } else {
                                        result.target_hp
                                    }
                                }
                                None => result.target_hp,
                            };
                        }
                        match result.fighters {
                            Some(fighters) => fight.fighters = fighters,
                            None => {}
                        }
                        fight.target_hp = result.target_hp;
                        world.state = States::InFight {
                            target_id: fight.target_name.clone(),
                        };
                        world.room.focus = Focus::COMMAND;
                        if result.damage > 0 {
                            let (damages, enn_hp) = (result.damage, result.target_hp);
                            world.output.push_back(format!(
								"[FIGHT] You dealt {} damages to the enemy. {} has {} HP remaining.",
								damages, world.room.fight.target_name, enn_hp
							));
                        }
                    }
                    _ => {}
                }
                if world.room.focus != Focus::OUTPUT {
                    world.room.output_scroll_pos.scroll_to_bottom();
                }
            }
            States::Trade(..) => {
                match &world.action {
                    PendingAction::Buy(item_name) => {
                        let answers: Vec<&str> = real_answer.split_whitespace().collect();
                        let mut amount = 0;
                        let mut cost = 0;
                        let mut item_id = String::from("");
                        for elem in answers.iter() {
                            let elems: Vec<&str> = elem.split("=").collect();
                            match elems[0] {
                                "amount" => amount = elems[1].parse::<u32>().unwrap(),
                                "gold" => cost = elems[1].parse::<u32>().unwrap(),
                                "bought" => item_id = elems[1].to_string(),
                                _ => {}
                            }
                        }
                        *world.player.inventory.entry(item_id).or_insert(0) += amount.clone();
                        world.player.gold -= cost;
                        world.output.push_back(format!("[Server Response] You successfully bought {} {}. It costs you {} golds!", amount, item_name, cost));
                        world.room.output_scroll_pos.scroll_to_bottom();
                    }
                    PendingAction::Sell(item_name) => {
                        let answers: Vec<&str> = real_answer.split_whitespace().collect();
                        let mut item_id = String::from("");
                        let mut amount = 0;
                        let mut cost = 0;
                        for elem in answers.iter() {
                            let elems: Vec<&str> = elem.split("=").collect();
                            if let Ok(mut file) = OpenOptions::new()
                                .create(true)
                                .append(true)
                                .open("debug_network.txt")
                            {
                                let _ =
                                    writeln!(file, "real (State {:#?}) :{:#?}", elems, real_answer);
                            }
                            match elems[0] {
                                "amount" => amount = elems[1].parse::<u32>().unwrap(),
                                "gold" => cost = elems[1].parse::<u32>().unwrap(),
                                "sold" => item_id = elems[1].to_string(),
                                _ => {}
                            }
                        }
                        world
                            .player
                            .inventory
                            .entry(item_id.clone())
                            .and_modify(|f| *f -= amount);
                        if *world.player.inventory.get(&item_id).unwrap_or(&0) < 1 {
                            world.player.inventory.remove(&item_id);
                        }
                        world.player.gold += cost;
                        world.output.push_back(format!("[Server Response] You successfully sold {} {}. That earned you {} golds!", amount, item_name, cost));
                        world.room.output_scroll_pos.scroll_to_bottom();
                    }
                    _ => {}
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
                }
                PendingAction::Npcs => {
                    let _ = world.tx_to_serv.try_send(String::from("LOOK\n"));
                    world.action = PendingAction::ClientLook;
                }
                PendingAction::SendChat(command, args) => {
                    world.output.push_back(format!(""));
                    world.output.push_back(format!("> {command} {args}"));
                    world.output.push_back(format!("[Error] {}", real_answer));
                    world.action = PendingAction::None;
                }
                _ => {
                    world
                        .output
                        .push_back(format!("[Error] {:#?}, {}", world.action, real_answer));
                    world.action = PendingAction::None;
                }
            }
        }
    }
}

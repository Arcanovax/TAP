use crate::{
    enums::{actions::PendingAction, focus::Focus, goals::Goal, states::States},
    structures::{
        fight::Fight, group::Invitation, quest_finish::FinishedQuest, quest_udate::UpdateView,
        world::World,
    },
};

pub fn event_handling(world: &mut World, answer: Vec<&str>) {
    if answer.len() > 2 && answer[2] == "CHAT" {
        let channel = match answer[1] {
            "ROOM" => Some(&mut world.chat.room_messages),
            "GLOBAL" => Some(&mut world.chat.global_messages),
            "GROUP" => Some(&mut world.chat.group_messages),
            _ => None,
        };
        let text: String = format!("[{}] {}", answer[3], answer[4..].join(" "));
        if let Some(messages) = channel {
            messages.push_back(text);

            if world.room.focus != Focus::Chat {
                world.room.chat_scroll_pos.scroll_to_bottom();
            }
        }
    } else {
        match answer[1] {
            "ROOM" => {
                let name = answer[4..].join(" ");
                match answer[3] {
                    "ENTER" => world
                        .output
                        .push_back(format!("[Server info] {} walks into the room.", name)),
                    "LEAVE" => world
                        .output
                        .push_back(format!("[Server info] {} leave the room.", name)),
                    _ => {}
                }
            }
            "GROUP" => match answer[2] {
                "JOIN" => {
                    let player_name = answer[3];
                    world
                        .chat
                        .group_messages
                        .push_back(format!("{player_name} join the group."));
                }
                "INVITE" => world.group.invitation.push(Invitation {
                    sender: answer[3].to_string(),
                }),
                "LEAVE" => {
                    let leaver = answer[3];
                    world
                        .chat
                        .group_messages
                        .push_back(format!("{leaver} leave the group."));
                }
                _ => {}
            },
            "DUNGEON" => {
                match answer[2] {
                    "END" => {
                        world.output.push_back("[Server] Congratulation! This dungeon is cleared!".to_string());
                        world.dungeon = false;
                        let _ =world.tx_to_serv.try_send(String::from("LOOK\n"));
                        world.action = PendingAction::ClientLook;
                    },
                    "CREATE" => {},
                    _ => {}
                }
            }
            "STATS" => {}
            "FIGHT" => {
                match answer[2] {
                    "ENTER" => {
                        world.output.push_back("".to_string());
                        world
                            .output
                            .push_back(format!("[FIGHT] {} says: 'Hello there!'.", answer[3]));
                        world.room.fight.fighters.insert(
                            answer[3].to_string(),
                            answer[4].parse::<u32>().unwrap_or(100),
                        );
                    }
                    "LEAVE" => {
                        world.output.push_back("".to_string());
                        world
                            .output
                            .push_back(format!("[FIGHT] {} leave the fight. Coward!!", answer[3]));
                        world.room.fight.fighters.remove(answer[3]);
                    }
                    "ENEMY" => {
                        let (target, target_hp, damages, target_killed) =
                            (answer[3], answer[4], answer[5], answer[6]);
                        world.output.push_back("".to_string());
                        if target_killed.to_lowercase() == "true" {
                            world.output.push_back(format!(
                                "[FIGHT] {} dealt {} damages to {}. {} is dead. What a shame!",
                                world.room.fight.target_name, damages, target, target
                            ));
                            if target == world.player.name {
                                world.state = States::Idle;
                                world.room.fight = Fight::new();
                            } else {
                                world.room.fight.fighters.remove(target);
                            }
                        } else {
                            world.output.push_back(format!(
                                "[FIGHT] {} dealt {} damages to {}. {} has {} HP remaining.",
                                world.room.fight.target_name, damages, target, target, target_hp
                            ));
                            world
                                .room
                                .fight
                                .fighters
                                .insert(target.to_string(), target_hp.parse::<u32>().unwrap());
                        }
                        if target == world.player.name {
                            world.player.hp = target_hp.parse::<u32>().unwrap();
                        }
                    }
                    "ATTACK" => {
                        let (player_name, damages, enn_hp, loot) =
                            (answer[3], answer[4], answer[5], answer[6]);
                        world.output.push_back("".to_string());
                        world.output.push_back(format!(
                            "[FIGHT] {} dealt {} damages to the enemy. {} has {} HP remaining.",
                            player_name, damages, world.room.fight.target_name, enn_hp
                        ));
                        if let Ok(hp_enn) = enn_hp.parse::<u32>() {
                            world.room.fight.target_hp = hp_enn;
                            if hp_enn == 0 {
                                let loot_split: Vec<String> = loot
                                    .split("//")
                                    .map(|f| {
                                        format!(
                                            "- {} x{}",
                                            f,
                                            if f == "item.gold" { 50 } else { 1 }
                                        )
                                    })
                                    .collect();
                                let loot_final = loot_split.join("\n");
                                world.state = States::Idle;
                                world.room.fight = Fight::new();
                                world.output.push_back(format!(
                                    "Congratulation! The enemy is defeated! You earned :\n{}",
                                    loot_final
                                ));
                                let _ = world.tx_to_serv.try_send(String::from("INVENTORY\n"));
                                world.action = PendingAction::ClientInventory;
                            }
                        } else {
                            world.state = States::Idle;
                            world.room.fight = Fight::new();
                            world.output.push_back("An error occurs with enemy HP so I decided to evacuate you immediately.".to_string());
                        }
                    }
                    "HEALING" => {
                        let (player_name, heal) = (answer[3], answer[4]);
                        world.output.push_back("".to_string());
                        world.output.push_back(format!(
                            "[FIGHT] {} healed himself for {} HP.",
                            player_name, heal
                        ));

                        let quantity = world
                            .room
                            .fight
                            .fighters
                            .entry(player_name.to_string())
                            .or_insert(0);
                        *quantity += heal.parse::<u32>().unwrap();

                        if player_name == world.player.name {
                            world.player.hp += heal.parse::<u32>().unwrap();
                        }
                    }
                    _ => {}
                }
                world.room.output_scroll_pos.scroll_to_bottom();
            }
            "QUEST" => match answer[2] {
                "UPDATE" => {
                    let update: UpdateView = serde_json::from_str(answer[3]).unwrap();
                    let quest_name = {
                        if let Some(item_obj) = world.player.quests.get_mut(&update.quest) {
                            item_obj.finished_goals += 1;
                            item_obj.name.clone()
                        } else {
                            update.quest
                        }
                    };
                    world.output.push_back(format!(
                        "Congratulation! You validate the goal '{}' of the {} quest.",
                        update.previous_goal, quest_name
                    ));
                    if let Goal::Retrieve { item, amount, .. } = update.previous_goal {
                        world
                            .player
                            .inventory
                            .entry(item)
                            .and_modify(|f| *f -= amount);
                        world.player.inventory.retain(|_, quantity| *quantity > 0);
                    }
                }
                "FINISH" => {
                    let finish: FinishedQuest = serde_json::from_str(answer[3]).unwrap();
                    let quest_name = {
                        if let Some(quest_obj) = world.player.quests.get_mut(&finish.quest) {
                            quest_obj.completed = true;
                            quest_obj.name.clone()
                        } else {
                            finish.quest.clone()
                        }
                    };
                    world.output.push_back(format!(
                        "Unbelievable! You've completed the quest {} and earned {}.",
                        quest_name, finish.reward
                    ));
                    if finish.reward == "item.gold" {
                        world.player.gold += 50;
                    } else {
                        *world.player.inventory.entry(finish.reward).or_insert(0) += 1;
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }
}

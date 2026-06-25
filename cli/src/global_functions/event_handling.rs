
use std::{fs::OpenOptions, io::Write};

use crate::{enums::{focus::Focus, states::States}, structures::{fight::Fight, group::Invitation, world::World}};

pub fn event_handling(world: &mut World, answer: Vec<&str>) {
	// if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("debug_draw.txt") {
	// 	let _ = writeln!(file, "RECU (State {}) : {:#?}", answer[1], answer);}
	if answer[2] == "CHAT" {
		let channel = match answer[1] {
			"ROOM" => Some(&mut world.chat.room_messages),
			"GLOBAL" => Some(&mut world.chat.global_messages),
			"GROUP" => Some(&mut world.chat.group_messages),
			_ => None
		};
			let text: String = format!("[{}] {}", answer[3], answer[4..].join(" "));
			if let Some(messages) = channel {
				messages.push_back(text);
				
				if world.room.focus != Focus::CHAT {
					world.room.chat_scroll_pos.scroll_to_bottom();
				}
			}
	}
	match answer[1] {
		"ROOM" => {}
		"GROUP" => {
			match answer[2] {
				"JOIN" => {
					let player_name = answer[3];
					world.chat.group_messages.push_back(format!("{player_name} join the group."));
				},
				"INVITE" => world.group.invitation.push(Invitation{sender: answer[3].to_string()}),
				"LEAVE" => {
					let leaver = answer[3];
					world.chat.group_messages.push_back(format!("{leaver} leave the group."));
				},
				_ => {}
			}
		}
		"STATS" => {},
		"FIGHT" => {
			match answer[2] {
				"ENTER" => {
					world.output.push_back("".to_string());
					world.output.push_back(format!("{} says: 'Hello there!'.", answer[3]));
					world.room.fight.fighters.insert(answer[3].to_string(), answer[4].parse::<u32>().unwrap_or(100));
					world.room.output_scroll_pos.scroll_to_bottom();
				}
				"LEAVE" => {
					world.output.push_back("".to_string());
					world.output.push_back(format!(
						"{} leave the fight. Coward!!", answer[3]
					));
					world.room.fight.fighters.remove(answer[3]);
				}
				"ENEMY" => {
					let ( target, target_hp, damages, target_killed ) = (answer[3], answer[4], answer[5], answer[6]);
					world.output.push_back("".to_string());
					if target_killed.to_lowercase() == "true" {
						world.output.push_back(format!(
							"{} dealt {} damages to {}. {} is dead. What a shame!",
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
							"{} dealt {} damages to {}. {} has {} HP remaining.",
							world.room.fight.target_name, damages, target, target, target_hp
						));
						world.room.fight.fighters.insert(target.to_string(), target_hp.parse::<u32>().unwrap());
					}
					if target == world.player.name {
						world.player.hp = target_hp.parse::<u32>().unwrap();
					}
					world.room.output_scroll_pos.scroll_to_bottom();
				}
				"ATTACK" => {
					let ( player_name, damages, enn_hp) = ( answer[3], answer[4],answer[5] );
					world.output.push_back("".to_string());
					world.output.push_back(format!(
						"{} dealt {} damages to the enemy. {} has {} HP remaining.",
						player_name, damages, world.room.fight.target_name, enn_hp
					));
					world.room.fight.target_hp = enn_hp.parse::<u32>().unwrap();
					world.room.output_scroll_pos.scroll_to_bottom();
				}
				_ => {}
			}
		},
		"QUEST" => {},
		_ => {}
	}
}
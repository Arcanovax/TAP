use serde::Deserialize;
use crate::*;

pub async fn handle_events(game: &mut Game, answer: Vec<&str>){
	if answer[2] == "CHAT" {
		let channel = match answer[1] {
			"ROOM" => &mut game.chat.room_messages,
			"GLOBAL" => &mut game.chat.global_messages,
			"GROUP" => &mut game.chat.group_messages,
			_ => return
		};
		let text: String = format!("[{}] {}", answer[3], answer[4..].join(" "));
		channel.push(text);
	}

	if answer[1] == "STATS" {
		if let Some(val_str) = answer[2].strip_prefix("players=") {
			if let Ok(val) = val_str.parse::<i32>() {
				game.nb_players = val;
			}
		}
	}


	if answer[1] == "GROUP"{
		match answer[2] {
			"INVITE" => {
				game.group.invitation = Some(Invitation{sender: answer[3..].join(" ")})
			}
			"JOIN" => {
				game.group.grouplist.push(answer[3..].join(" "));
			}
			"LEAVE" => {
				game.group.grouplist.retain(|x| x != &answer[3..].join(" "));
			}
			_ => return
		}
	}

	if answer[1] == "QUEST"{
		match answer[2] {
			"UPDATE" => {
				match serde_json::from_str::<QuestUpdateEvent>(answer[3..].join(" ").as_str()) {
					Ok(quest_upt) => {

						for quest in &mut game.quests.all{
							if quest.quest_id == quest_upt.quest{
								quest.goal = Some(quest_upt.goal.clone());
								if let Some((pos, all)) = quest.progress.split_once('/') {
								let i: usize = pos.parse().unwrap_or(0) + 1;
								quest.progress = format!("{}/{}", i, all);
								}
								

							}
						}
					}
					Err(e) => {
						println!("EVT QUEST error parsing: {}", e);
					}
				}
			}
			"FINISH" => {
				match serde_json::from_str::<QuestFinishEvent>(answer[3..].join(" ").as_str()) {
					Ok(quest_rm) => {
						game.quests.all.retain(|quest| quest.quest_id != quest_rm.quest);
						game.player.inventory.is_load = false;
        			}
					Err(e) => {
						println!("EVT QUEST error parsing: {}", e);
					}
				}
			}
			_ => return
		}
	}

	if answer[1] == "FIGHT"{
		match answer[2] {
			"ENTER" => {
				if let Some(ref mut fight) = game.active_fight{
					if let Ok(life_val) = answer[4].parse::<i32>() {
						fight.players.insert(answer[3].to_string(), life_val);
						let text: String = format!("{} join the fight", answer[3].to_string());
						fight.chat.push(text);
					}
				}
			}
			"LEAVE" => {
				if let Some(ref mut fight) = game.active_fight{
						fight.players.remove(answer[3]);
						fight.chat.push(format!("{} left the fight", answer[3].to_string()));
					}
			}
			"ATTACK" => {
				if let Some(ref mut fight) = game.active_fight{
					if let Ok(new_life) = answer[5].parse::<i32>() {
						fight.enemy_hp = new_life;
						fight.chat.push(format!("{} attack and deal {} damage", answer[3], answer[4]));
					}
				}
			}
			"ENEMY" => {
				let Some(ref mut state) = game.player.state else { return };
				if let Some(ref mut fight) = game.active_fight{
					if answer[6] == "false"{
						if let Ok(damage) = answer[5].parse::<i32>() {
							if answer[3] == game.player.name{
								state.hp -= damage;
							}
							if let Some(player_hp) = fight.players.get_mut(answer[3]) {
                			*player_hp -= damage;
            				}
							let text: String = format!("{} attack {} and deal {} damage", fight.enemy.name, answer[3], damage);
							fight.chat.push(text);
						}

					}
					else{
						if answer[3] == game.player.name{
							if let Some(npc) = game.loaded_npcs.get_mut(&fight.enemy.id){
								npc.npc_talk = Some(NpcTalk{
									texts: vec!["You lost".to_string()],
									text_i: 0,
								});
							}
							game.active_fight = None;
							game.player.state = None;
						}
						else {
							fight.players.remove(answer[3]);
							fight.chat.push(format!("{} killed {}",fight.enemy.name, answer[3]));
						}


					}

				}
			}
			"HEALING" => {
				let Some(ref mut state) = game.player.state else { return };
				if let Some(ref mut fight) = game.active_fight{
					if let Ok(heal) = answer[4].parse::<i32>() {
						if answer[3] == game.player.name{
							state.hp = (state.hp + heal).min(100);
						}
						if let Some(player_hp) = fight.players.get_mut(answer[3]) {
							*player_hp = (player_hp.clone() + heal).min(100);
						}
						let text: String = format!("{} healed {}HP", answer[3], heal);
						fight.chat.push(text);
						}
				}
			}
			_ => return
		}
	}

	if answer[1] == "ROOM"{
		match answer[2] {
			"PRESENCE" => {
				let name: String =answer[4..].join(" ");
				if answer[3] == "ENTER"{
					if let Some(ref mut map_data) = game.map_data{
						map_data.players.push(name);
						// game.map_data = None
					}
				}
				else if answer[3] == "LEAVE"{
					if let Some(ref mut map_data) = game.map_data{
						if name != game.player.name{
							map_data.players.retain(|x| x != &name);
						}
					}
				}
			}
			"TAKE" => {
				if let Some(ref mut map_data) = game.map_data {
					if let Some(pos) = map_data.items.iter().position(|x| x == &answer[4..].join(" ")) {
						map_data.items.remove(pos);
					}
				}
			}
			"DROP" => {
				if let Some(ref mut map_data) = game.map_data {
					map_data.items.push(answer[4..].join(" "));
				}
			}
			_ => return
		}

	}


	// 	// if let Some(quest_finish) = server_event.quest_finish {
	// 	// 	game.quests.retain(|quest| quest.name != quest_finish.quest_name);
	// 	// }

}





#[derive(Deserialize, Debug)]
pub struct QuestUpdateEvent {
	pub quest: String,
	pub goal: Goal,
	pub previous_goal: Goal
}


#[derive(Deserialize, Debug)]
pub struct QuestFinishEvent {
	pub quest: String,
	pub reward: String
}

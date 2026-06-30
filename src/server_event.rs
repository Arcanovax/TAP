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

	if answer[1] == "ROOM"{
		println!("PRESENCEEEEEE");
		match answer[2] {
			"PRESENCE" => {
				let name: String =answer[4..].join(" ");
				if answer[3] == "ENTER"{
					if let Some(ref mut map_data) = game.map_data{
						map_data.players.push(name);
						game.map_data = None
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

			_ => return
		}

	}


	// 	// if let Some(quest_upt) = server_event.quest_update {
	// 	// 	for quest in &mut game.quests{
	// 	// 		if quest.name == quest_upt.quest_name{
	// 	// 			quest.goals = vec![quest_upt.goal];
	// 	// 			break;
	// 	// 		}
	// 	// 	}
	// 	// }
	// 	// if let Some(quest_finish) = server_event.quest_finish {
	// 	// 	game.quests.retain(|quest| quest.name != quest_finish.quest_name);
	// 	// }

	// 	if let Some(leave) = server_event.room_leave {
	// 		if let Some(ref mut map_data) = game.map_data{
	// 			if leave.player_name != game.player.name{
	// 				map_data.players.retain(|x| x != &leave.player_name);
	// 			}
	// 		}
	// 	}
	// 	if let Some(join) = server_event.room_join {
	// 		if let Some(ref mut map_data) = game.map_data{
	// 			map_data.players.push(join.player_name);
	// 			game.map_data = None
	// 		}
	// 	}
	// 	if let Some(players) = server_event.players {
	// 		game.nb_players = players.players;
	// 	}
	// 	if let Some(take) = server_event.take {
	// 		if let Some(ref mut map_data) = game.map_data {
	// 			if let Some(pos) = map_data.items.iter().position(|x| x == &take.item) {
	// 				map_data.items.remove(pos);
	// 			}
	// 		}
	// 	}
	// 	if let Some(drop) = server_event.drop {
	// 		if let Some(ref mut map_data) = game.map_data {
	// 			map_data.items.push(drop.item);
	// 		}
	// 	}
	// }
}


#[derive(Deserialize, Debug)]
pub struct ServerEvent {
    #[serde(rename = "INVITE")]
    pub invite: Option<InviteData>,
	#[serde(rename = "GROUP_JOIN")]
    pub join: Option<GroupEvent>,
	#[serde(rename = "GROUP_LEAVE")]
    pub leave: Option<GroupEvent>,
	#[serde(rename = "ROOM_LEAVE")]
	pub room_leave: Option<PlayersEvent>,
	#[serde(rename = "ROOM_JOIN")]
	pub room_join: Option<PlayersEvent>,
	#[serde(rename = "PLAYERS")]
	pub players: Option<Players>,
	#[serde(rename = "QUEST_UPDATE")]
	pub quest_update: Option<QuestUpdateEvent>,
	#[serde(rename = "QUEST_FINISH")]
	pub quest_finish: Option<QuestFinishEvent>,

	#[serde(rename = "TAKE")]
    pub take: Option<ItemEvent>,
	#[serde(rename = "DROP")]
    pub drop: Option<ItemEvent>,

	#[serde(rename = "CHAT")]
    pub chat: Option<ChatData>,
  	pub data: Option<serde_json::Value>,
	pub error: Option<String>
}


#[derive(Deserialize, Debug)]
pub struct PlayersEvent {
	pub player_name: String,
}

#[derive(Deserialize, Debug)]
pub struct Players {
	pub players: i32,
}


#[derive(Deserialize, Debug)]
pub struct QuestUpdateEvent {
	pub quest_name: String,
	pub goal: Goal
}

#[derive(Deserialize, Debug)]
pub struct QuestFinishEvent {
	pub quest_name: String
}



#[derive(Deserialize, Debug)]
pub struct ItemEvent {
	pub player_name: String,
	pub item: String
}

#[derive(Deserialize, Debug)]
pub struct GroupEvent {
	pub player_name: String,
}



#[derive(Deserialize, Debug)]
pub struct ChatData {
	pub body: String,
    pub sender: String,
    pub scope: String,
}

#[derive(Deserialize, Debug)]
pub struct InviteData {
    pub sender: String,
    pub group_name: String,
}

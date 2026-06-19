use serde::Deserialize;
use crate::*;

pub async fn handle_events(game: &mut Game, server_event: ServerEvent){
	if let Some(invite) = server_event.invite {
		game.group.invitation = Some(Invitation{sender: invite.sender, group_name: invite.group_name})
	}
	if let Some(new) = server_event.join {
		game.group.grouplist.push(new.player_name);
	}
	if let Some(leaver) = server_event.leave {
		game.group.grouplist.retain(|x| x != &leaver.player_name);
	}
	if let Some(leave) = server_event.room_leave {
		if let Some(ref mut map_data) = game.map_data{
			if leave.player_name != game.player.name{
				map_data.players.retain(|x| x != &leave.player_name);
			}
		}
	}
	if let Some(join) = server_event.room_join {
		if let Some(ref mut map_data) = game.map_data{
			map_data.players.push(join.player_name);
			game.map_data = None
		}
	}
	if let Some(players) = server_event.players {
		game.nb_players = players.players;
	}
	if let Some(take) = server_event.take {
		if let Some(ref mut map_data) = game.map_data {
			if let Some(pos) = map_data.items.iter().position(|x| x == &take.item) {
				map_data.items.remove(pos);
			}
		}
	}
	if let Some(drop) = server_event.drop {
		if let Some(ref mut map_data) = game.map_data {
			map_data.items.push(drop.item);
		}
	}
	if let Some(msg) = server_event.chat {
		let channel = match msg.scope.as_str(){
			"ROOM" => &mut game.chat.room_messages,
			"GLOBAL" => &mut game.chat.global_messages,
			"GROUP" => &mut game.chat.group_messages,
			_ => return
			};
		let text: String = format!("[{}] {}\n",msg.sender,msg.body);
		channel.push(text);
	}
}


#[derive(Deserialize, Debug)]
pub struct ServerEvent {
    #[serde(rename = "type")]
    pub event_type: String,

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
	pub quest: Option<Quest>,

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
	pub item: String
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

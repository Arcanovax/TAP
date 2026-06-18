use serde::Deserialize;

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

use serde::Deserialize;

use crate::structures::{chat_data::ChatData, invite_data::InviteData};

#[derive(Deserialize, Debug)]
pub struct ServerEvent {
    #[serde(rename = "type")]
    pub event_type: String,

    #[serde(rename = "INVITE")]
    pub invite: Option<InviteData>,
	#[serde(rename = "CHAT")]
    pub chat: Option<ChatData>,
    pub data: Option<String>
}
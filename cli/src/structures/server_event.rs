use serde::Deserialize;
use serde_json::Value;

use crate::structures::{attack::Attack, chat_data::ChatData, enn_attack::EnnAttack, enter_fight::EnterFight, invite_data::InviteData};

#[derive(Deserialize, Debug)]
pub struct ServerEvent {
    #[serde(rename = "type")]
    pub event_type: String,

    #[serde(rename = "INVITE")]
    pub invite: Option<InviteData>,
    #[serde(rename = "ENTER_FIGHT")]
    pub enter: Option<EnterFight>,
    #[serde(rename = "ATTACK")]
    pub attack: Option<Attack>,
    #[serde(rename = "ENEMY_ATTACK")]
    pub enn_attack: Option<EnnAttack>,
	#[serde(rename = "GROUP_JOIN")]
    pub join: Option<String>,
	#[serde(rename = "GROUP_LEAVE")]
    pub leave: Option<String>,
	#[serde(rename = "CHAT")]
    pub chat: Option<ChatData>,
    pub data: Option<Value>,
	pub error: Option<String>

}
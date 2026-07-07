use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(tag = "status", rename_all = "lowercase")]
pub enum QuestStatus {
    Completed,
    Active { progress: String },
}

#[derive(Serialize, Debug, Deserialize)]
pub struct QuestsView {
    pub quest_id: String,
	#[serde(flatten)]
    pub status: QuestStatus,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct QuestView {
    pub quest_id: String,
    pub description: String,
    pub reward: String,
    pub status: String,
}

impl Display for QuestsView {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Id: {}\nStatus: {:#?}", self.quest_id, self.status)
	}
}
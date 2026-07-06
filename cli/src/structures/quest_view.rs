use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
pub struct QuestsView {
    pub quest_id: String,
    pub status: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct QuestView {
    pub quest_id: String,
    pub description: String,
    pub reward: String,
    pub status: String,
}

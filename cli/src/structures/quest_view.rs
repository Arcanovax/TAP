use serde::{Deserialize, Serialize};

// #[derive(Serialize, Debug, Deserialize)]
// // #[serde(tag = "status", rename_all = "lowercase")]
// pub enum Status {
//     Completed,
//     Active { progress: String },
// }

#[derive(Serialize, Debug, Deserialize)]
pub struct QuestView {
    quest_id: String,
    description: String,
    reward: String,
    status: String,
}
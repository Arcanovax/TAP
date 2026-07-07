use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct FinishedQuest {
	pub quest: String,
	pub reward: String
}
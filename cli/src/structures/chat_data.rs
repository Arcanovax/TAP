use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ChatData {
	pub body: String,
    pub sender: String,
    pub scope: String,
}
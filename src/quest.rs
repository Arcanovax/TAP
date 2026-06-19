use crate::*;

#[derive(Deserialize, Debug)]
pub struct Quest{
    pub npc_id: String,
	pub id: String,
	pub name: String,
    pub description: String,
    pub reward: String,
    pub goals: Vec<Goal>,
}

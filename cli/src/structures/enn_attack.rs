use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct EnnAttack {
	pub target: String,
    pub damages: u32,
    pub target_hp: u32,
	pub target_killed: bool
}
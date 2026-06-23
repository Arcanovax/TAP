use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Attack {
    pub player_name: String,
    pub damages: u32,
    pub enemy_hp: u32 
}
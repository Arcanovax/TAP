use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct EnterFight {
    pub player_name: String,
    pub hp: u32,
}
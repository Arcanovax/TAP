use std::collections::HashMap;

use crate::structures::enums::{fighter_status::FighterStatus, state::State};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Enemy_Attack {
    pub damages: u32,
    pub target: String
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Attack_Result {
    pub attacker_hp: u32,
    pub attacker_name: String,
    pub target_hp: u32,
    pub damage: u32,
    pub status: State,
    pub enemy_attack: Option<Enemy_Attack>,
    pub fighters: Option<HashMap<String, u32>>
}
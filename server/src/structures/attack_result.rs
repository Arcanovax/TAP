use std::collections::HashMap;

use crate::structures::enums::{state::State};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AttackResult {
    pub attacker_hp: u32,
    pub attacker_name: String,
    pub target_hp: u32,
    pub damage: u32,
    pub status: State,
    pub fighters: Option<HashMap<String, u32>>
}
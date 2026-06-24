use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::enums::states::States;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Attack_Result {
    pub attacker_hp: u32,
    pub attacker_name: String,
    pub target_hp: u32,
    pub damage: u32,
    pub status: States,
    pub fighters: Option<HashMap<String, u32>>
}
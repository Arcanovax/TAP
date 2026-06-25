use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum NPCKind {
    Merchant,
    Enemy {
        hp: u32,
        max_hp: u32,
        defeated: bool,
    },
    Citizen,
}
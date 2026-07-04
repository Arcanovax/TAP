use std::fmt::Display;

use crate::enums::states::States;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct StatusView {
    pub hp: u32,
    pub max_hp: u32,
    pub status: States,
}

impl Display for StatusView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "hp: {}\nhp_max: {}\nStatus: {}",
            self.hp, self.max_hp, self.status
        )
    }
}

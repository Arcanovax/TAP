use serde::{Deserialize, Serialize};

use crate::enums::goals::Goal;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Quest {
    pub name: String,
    pub description: String,
    pub reward: String,
    pub goals: Vec<Goal>,
}
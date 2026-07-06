use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::enums::goals::Goal;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Quest {
    pub name: String,
    pub description: String,
    pub reward: String,
    pub goals: Vec<Goal>,
}

impl Display for Quest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut list_goals: Vec<String> = Vec::new();
        for goal in &self.goals {
            list_goals.push(format!("- {}", goal));
        }
        write!(
            f,
            "{}\n Reward: {}\n Goals:\n{}",
            self.name,
            self.reward,
            list_goals.join("\n")
        )
    }
}

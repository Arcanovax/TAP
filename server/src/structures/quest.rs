use serde::{Deserialize, Serialize};

use crate::command::Command;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub enum Goal {
    Collect { item: String, amount: u32 },
    Talk { dialog: String },
}

impl Goal {
    pub fn command(&self) -> Command {
        match self {
            Goal::Collect { .. } => Command::TAKE,
            Goal::Talk { .. } => Command::TALK,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Quest {
    pub name: String,
    pub description: String,
    pub reward: String,
    pub goals: Vec<Goal>,
}

impl Quest {
    pub fn references(&self) -> Vec<&str> {
        let mut refs = Vec::new();
        for goal in &self.goals {
            match goal {
                Goal::Collect { item, .. } => refs.push(item.as_str()),
                Goal::Talk { dialog } => refs.push(dialog.as_str()),
            }
        }
        refs
    }
}

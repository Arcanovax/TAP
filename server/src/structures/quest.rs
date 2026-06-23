use serde::{Deserialize, Serialize};

use crate::structures::{enums::game_event::GameEvent, player::Player};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub enum Goal {
    Collect { item: String, amount: u32 },
    Talk { dialog: String },
}

impl Goal {
    pub(crate) fn is_satisfied(&self, player: &Player, event: Option<&GameEvent>) -> bool {
        match self {
            Goal::Collect { item, amount } => {
                player.inventory.get(item).copied().unwrap_or(0) >= *amount
            }
            Goal::Talk { dialog } => {
                matches!(event, Some(GameEvent::Talked { dialog: d }) if d == dialog)
            }
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
        let mut refs: Vec<&str> = vec![&self.reward];
        for goal in &self.goals {
            match goal {
                Goal::Collect { item, .. } => refs.push(item.as_str()),
                Goal::Talk { dialog } => refs.push(dialog.as_str()),
            }
        }
        refs
    }
}

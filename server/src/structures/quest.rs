use serde::{Deserialize, Serialize};

use crate::structures::{enums::game_event::GameEvent, player::Player};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub enum Goal {
    Collect {
        item: String,
        amount: u32,
    },
    Talk {
        dialog: String,
    },
    Retrieve {
        item: String,
        amount: u32,
        dialog: String,
    },
    Answer {
        answer: String,
        room: String,
    },
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
            Goal::Retrieve {
                item,
                amount,
                dialog,
            } => {
                player.inventory.get(item).copied().unwrap_or(0) >= *amount
                    && matches!(event, Some(GameEvent::Talked { dialog: d }) if d == dialog)
            }
            Goal::Answer { answer, room } => {
                *room == player.location
                    && matches!(event, Some(GameEvent::Answer { answer: a }) if a.to_lowercase() == answer.to_lowercase())
            }
        }
    }
}

impl From<Goal> for String {
    fn from(val: Goal) -> Self {
        match val {
            Goal::Collect { item, amount } => format!("Collect {} {}", amount, item),
            Goal::Answer { .. } => "Find the answer of his riddle".to_string(),
            Goal::Talk { dialog } => {
                let mut splitted = dialog.splitn(3, ".");
                let npc = splitted.next().unwrap().to_owned() + "." + splitted.next().unwrap();
                format!("Talk to {}", npc)
            }
            Goal::Retrieve {
                item,
                amount,
                dialog,
            } => {
                let mut splitted = dialog.splitn(3, ".");
                let npc = splitted.next().unwrap().to_owned() + "." + splitted.next().unwrap();
                format!("Gave {} {} to {}", amount, item, npc)
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
                Goal::Retrieve { item, dialog, .. } => {
                    refs.extend([item.as_str(), dialog.as_str()])
                }
                Goal::Answer { .. } => {}
            }
        }
        refs
    }
}

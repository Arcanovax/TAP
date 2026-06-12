use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Goal {
    Collect { item: String, amount: u16 },
    Talk { dialog: String },
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Quest {
    name: String,
    description: String,
    reward: String,
    goals: Vec<Goal>,
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

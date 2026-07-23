use ratatui::{
    style::Color,
    text::{Line, Text},
};
use serde::{Deserialize, Serialize};

use crate::enums::goals::Goal;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct Quest {
    pub name: String,
    pub description: String,
    pub reward: String,
    pub goals: Vec<Goal>,
    #[serde(skip)]
    pub finished_goals: usize,
    #[serde(skip)]
    pub completed: bool,
}

impl Quest {
    pub fn to_text<'a>(&'a self, name: String) -> Text<'a> {
        let mut list_goals: Vec<Line> = vec![
            Line::from(self.name.clone()),
            Line::from(format!("Reward: {} ({})", name, self.reward)),
        ];
        for (i, goal) in self.goals.iter().enumerate() {
            let color = if i < self.finished_goals || self.completed {
                Color::Green
            } else {
                Color::White
            };

            list_goals.push(Line::from(format!("- {}", goal)).style(color));
        }
        Text::from(list_goals)
    }
}

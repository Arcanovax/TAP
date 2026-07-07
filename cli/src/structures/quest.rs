
use ratatui::{style::Color, text::{Line, Text}};
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
	pub completed: bool
}

impl<'a> From<&'a Quest> for Text<'a> {
    fn from(quest: &'a Quest) -> Self {
		
        let mut list_goals: Vec<Line> = vec![
			Line::from(quest.name.clone()),
			Line::from(format!("Reward: {}", quest.reward))
		];
        for (i, goal) in quest.goals.iter().enumerate() {
			let color = if i < quest.finished_goals || quest.completed {
				Color::Green
			} else {
				Color::White
			};

            list_goals.push(Line::from(format!("- {}", goal)).style(color));
        }
        Text::from(list_goals)
	}
}

use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Hash, Eq, Clone)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::East => write!(f, "East"),
            Direction::West => write!(f, "West"),
            Direction::North => write!(f, "North"),
            Direction::South => write!(f, "South"),
        }
    }
}

#[derive(Serialize, Debug, Deserialize)]
pub struct RoomsView {
    pub id: String,
    pub name: String,
    pub description: String,
    pub exits: HashMap<Direction, String>,
}

impl Display for RoomsView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n\n{}", self.name, self.description)
    }
}

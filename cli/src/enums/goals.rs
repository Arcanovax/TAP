use std::fmt::Display;

use serde::{Deserialize, Serialize};

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

impl Display for Goal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Goal::Collect { item, amount } => writeln!(f, "Collect {} {}", amount, item),
            Goal::Talk { dialog } => {
                writeln!(f, "Talk to {}", dialog)
            }
            Goal::Retrieve {
                item,
                amount,
                dialog,
            } => writeln!(f, "Gave {} {} to {}", amount, item, dialog),
            Goal::Answer { room, .. } => {
                write!(f, "Answer to his riddle at {}.", room)
            }
        }
    }
}

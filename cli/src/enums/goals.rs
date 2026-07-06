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
}

impl Display for Goal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Goal::Collect { item, amount } => write!(f, "Collect {} {}\n", amount, item),
            Goal::Talk { dialog } => write!(f, "Talk to {}\n", dialog),
            Goal::Retrieve {
                item,
                amount,
                dialog,
            } => write!(f, "Gave {} {} to {}\n", amount, item, dialog),
        }
    }
}

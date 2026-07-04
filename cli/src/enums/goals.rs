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
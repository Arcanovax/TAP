use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Quest {
    name: String,
    description: String,
    reward: String,
}

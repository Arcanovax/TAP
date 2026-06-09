use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Exit {
    North { toward: String },
    South { toward: String },
    East { toward: String },
    West { toward: String },
}


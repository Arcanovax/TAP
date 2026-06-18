use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::structures::enums::error::ErrorCode;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Exit {
    North { toward: String },
    South { toward: String },
    East { toward: String },
    West { toward: String },
}

impl FromStr for Exit {
    type Err = ErrorCode;
    fn from_str(s: &str) -> Result<Self, ErrorCode> {
        match s.to_uppercase().as_str() {
            "NORTH" => Ok(Self::North {
                toward: String::new(),
            }),
            "SOUTH" => Ok(Self::South {
                toward: String::new(),
            }),
            "EAST" => Ok(Self::East {
                toward: String::new(),
            }),
            "WEST" => Ok(Self::West {
                toward: String::new(),
            }),
            _ => Err(ErrorCode::INVALID_ARGS),
        }
    }
}

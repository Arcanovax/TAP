use serde::{Deserialize, Serialize};
use std::str::FromStr;

use crate::structures::enums::error::ErrorCode;

#[derive(Debug, PartialEq, Serialize, Deserialize, Hash, Eq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl FromStr for Direction {
    type Err = ErrorCode;
    fn from_str(s: &str) -> Result<Self, ErrorCode> {
        match s.to_uppercase().as_str() {
            "NORTH" => Ok(Self::North),
            "SOUTH" => Ok(Self::South),
            "EAST" => Ok(Self::East),
            "WEST" => Ok(Self::West),
            _ => Err(ErrorCode::INVALID_ARGS),
        }
    }
}

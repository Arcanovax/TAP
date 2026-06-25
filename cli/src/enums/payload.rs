use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, PartialEq, Eq, Deserialize, Clone)]
pub enum Payload {
    Empty,
    Text(String),
    Pair(HashMap<String, String>),
    Json(serde_json::Value),
}
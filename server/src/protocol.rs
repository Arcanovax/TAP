use crate::error::ErrorCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub enum EventType {
    CHAT,
    INVITE,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Message {
    Command {
        name: String,
        args: Vec<String>,
    },
    Response {
        error: ErrorCode,
        data: Option<String>,
    },
    Event {
        kind: EventType,
        data: String,
    },
}

impl Message {
    pub fn parse(str: String) -> Result<Self, serde_json::Error> {
        serde_json::from_str(&str)
    }

    pub fn to_str(&self) -> String {
        serde_json::to_string(self).unwrap_or_default() + "\n"
    }
}
//
// impl Default for Message {
//     fn default() -> Self {
//         Message::Response {
//             error: ErrorCode::INVALID_COMMAND,
//             data: None,
//         }
//     }
// }

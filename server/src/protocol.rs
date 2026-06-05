use crate::error::ErrorCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub enum EventType {
    CHAT,
    NONE,
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
// pub struct Message {
//     pub message: MessageType,
//     pub command_line: String,
//     pub command_name: String,
//     pub args: Vec<String>,
//     pub error_response: ErrorCode,
//     pub error_code: u16,
//     pub event_type: EventType,
//     pub data: Option<String>,
// }

impl Message {
    pub fn parse(str: String) -> Self {
        serde_json::from_str(&str).unwrap_or_else(|_| Message::default())
    }

    pub fn to_str(&self) -> String {
        serde_json::to_string(self).unwrap_or_default() + "\n"
    }
}

impl Default for Message {
    fn default() -> Self {
        Message::Response {
            error: ErrorCode::INVALID_COMMAND,
            data: None,
        }
    }
}

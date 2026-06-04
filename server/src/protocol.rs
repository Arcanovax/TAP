use crate::error::ErrorCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub enum MessageType {
    COMMAND,
    RESPONSE,
    EVENT,
}

#[derive(Serialize, Deserialize)]
pub enum EventType {
    NONE,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub message: MessageType,
    pub command_line: String,
    pub response_line: String,
    pub event_line: String,
    pub command_name: String,
    pub args: Vec<String>,
    pub error_response: ErrorCode,
    pub error_code: u16,
    pub event_type: EventType,
    pub event_data: String,
}

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
        Message {
            message: MessageType::COMMAND,
            command_line: String::new(),
            response_line: String::new(),
            event_line: String::new(),
            command_name: String::new(),
            args: Vec::new(),
            error_response: ErrorCode::NONE,
            error_code: ErrorCode::NONE.code(),
            event_type: EventType::NONE,
            event_data: String::new(),
        }
    }
}

use std::str::FromStr;

use crate::{error::ErrorCode, structures::quest::Goal};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum ChatScope {
    GLOBAL,
    GROUP,
    ROOM,
}

impl FromStr for ChatScope {
    type Err = ErrorCode;
    fn from_str(s: &str) -> Result<Self, ErrorCode> {
        match s.to_uppercase().as_str() {
            "GLOBAL" => Ok(ChatScope::GLOBAL),
            "GROUP" => Ok(ChatScope::GROUP),
            "ROOM" => Ok(ChatScope::ROOM),
            _ => Err(ErrorCode::INVALID_ARGS),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub enum EventType {
    CHAT {
        body: String,
        sender: String,
        scope: ChatScope,
    },
    INVITE {
        sender: String,
        group_name: String,
    },
    QUEST_UPDATE {
        quest_name: String,
        goal: Goal,
    },
    QUEST_FINISH {
        quest_name: String,
    },
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
    Event(EventType),
}

impl Message {
    pub fn parse(str: String) -> Result<Self, serde_json::Error> {
        serde_json::from_str(&str)
    }

    pub fn to_str(&self) -> String {
        serde_json::to_string(self).unwrap_or_default() + "\n"
    }
}

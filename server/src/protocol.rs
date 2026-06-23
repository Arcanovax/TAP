use std::str::FromStr;

use crate::structures::{enums::error::ErrorCode, quest::Goal};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
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
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
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
        reward: String,
    },
    GROUP_LEAVE {
        player_name: String,
    },
    GROUP_JOIN {
        player_name: String,
    },
    ROOM_LEAVE {
        player_name: String,
    },
    ROOM_JOIN {
        player_name: String,
    },
    PLAYERS {
        players: usize,
    },
    TAKE {
        player_name: String,
        item: String,
    },
    DROP {
        player_name: String,
        item: String,
    },
    ENTER_FIGHT {
        player_name: String,
        hp: u32
    },
    ATTACK {
        player_name: String,
        damages: u32,
        enemy_hp: u32
    },
    ENEMY_ATTACK {
        target: String,
        damages: u32,
        target_hp: u32
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum Message {
    Command {
        name: String,
        args: Vec<String>,
    },
    Response {
        error: ErrorCode,
        data: Option<serde_json::Value>,
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

impl From<Result<(), ErrorCode>> for Message {
    fn from(result: Result<(), ErrorCode>) -> Self {
        let code = result.err().unwrap_or(ErrorCode::SUCCESS);
        Message::Response {
            error: code,
            data: None,
        }
    }
}

impl From<Result<Value, ErrorCode>> for Message {
    fn from(result: Result<Value, ErrorCode>) -> Self {
        match result {
            Ok(data) => {
                return Message::Response {
                    error: ErrorCode::SUCCESS,
                    data: Some(data),
                };
            }
            Err(code) => {
                return Message::Response {
                    error: code,
                    data: None,
                };
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_ok_unit_gives_succes_without_data() {
        let msg: Message = Ok::<(), ErrorCode>(()).into();
        assert_eq!(
            msg,
            Message::Response {
                error: ErrorCode::SUCCESS,
                data: None
            }
        )
    }

    #[test]
    fn from_ok_unit_gives_succes_with_data() {
        let msg: Message = Ok::<Value, ErrorCode>(Value::String("Hello Test".to_string())).into();
        assert_eq!(
            msg,
            Message::Response {
                error: ErrorCode::SUCCESS,
                data: Some(Value::String("Hello Test".to_string()))
            }
        )
    }
}

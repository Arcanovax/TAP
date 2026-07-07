use std::{collections::HashMap, str::FromStr};

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
    GROUP_INVITE {
        sender: String,
        group_name: String,
    },
    QUEST_UPDATE {
        quest_id: String,
        goal: Goal,
        previous_goal: Goal,
    },
    QUEST_FINISH {
        quest_id: String,
        reward: String,
    },
    GROUP_LEAVE {
        player_name: String,
    },
    GROUP_JOIN {
        player_name: String,
    },
    DUNGEON_CREATE,
    ROOM_LEAVE {
        player_name: String,
    },
    ROOM_JOIN {
        player_name: String,
    },
    STATS_PLAYERS {
        players: usize,
    },
    ROOM_TAKE {
        player_name: String,
        item: String,
    },
    ROOM_DROP {
        player_name: String,
        item: String,
    },
    FIGHT_LEAVE {
        player_name: String,
    },
    ENTER_FIGHT {
        player_name: String,
        hp: u32,
    },
    HEALING {
        player_name: String,
        heal: u32,
    },
    ATTACK {
        player_name: String,
        damages: u32,
        enemy_hp: u32,
        loot: Vec<String>,
    },
    ENEMY_ATTACK {
        target: String,
        damages: u32,
        target_hp: u32,
        target_killed: bool,
    },
    SERVER_RESET,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub enum Payload {
    Empty,
    Text(String),
    Pair(HashMap<String, String>),
    Json(serde_json::Value),
}

impl Payload {
    fn to_str(&self) -> String {
        match self {
            Payload::Empty => String::new(),
            Payload::Pair(hashmap) => {
                let mut str = String::new();

                for (i, (key, value)) in hashmap.iter().enumerate() {
                    if i != 0 {
                        str += " ";
                    }
                    str += format!("{}={}", key, value).as_str();
                }
                str
            }
            Payload::Text(str) => str.to_string(),
            Payload::Json(json) => json.to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Message {
    Command { name: String, args: Vec<String> },
    Response { error: ErrorCode, payload: Payload },
    Event(EventType),
}

impl Message {
    pub fn to_str(&self) -> String {
        match self {
            Message::Response { error, payload } => match error {
                ErrorCode::SUCCESS => match payload.to_str() {
                    rendered if rendered.is_empty() => "OK\n".to_string(),
                    rendered => format!("OK {rendered}\n"),
                },
                _ => format!("ERR {} {}\n", error.code(), error.name()),
            },
            Message::Command { .. } => String::new(),
            Message::Event(event) => match event {
                EventType::ROOM_JOIN { player_name } => {
                    format!("EVT ROOM PRESENCE ENTER {player_name}\n")
                }
                EventType::ROOM_LEAVE { player_name } => {
                    format!("EVT ROOM PRESENCE LEAVE {player_name}\n")
                }
                EventType::GROUP_JOIN { player_name } => {
                    format!("EVT GROUP JOIN {player_name}\n")
                }
                EventType::DUNGEON_CREATE => format!("EVT DUNGEON CREATE\n"),
                EventType::GROUP_LEAVE { player_name } => {
                    format!("EVT GROUP LEAVE {player_name}\n")
                }
                EventType::GROUP_INVITE { sender, .. } => {
                    format!("EVT GROUP INVITE {sender}\n")
                }
                EventType::CHAT {
                    scope,
                    sender,
                    body,
                } => {
                    format!("EVT {scope:?} CHAT {sender} {body}\n")
                }
                EventType::STATS_PLAYERS { players } => {
                    format!("EVT STATS players={players}\n")
                }
                EventType::ROOM_TAKE { player_name, item } => {
                    format!("EVT ROOM TAKE {player_name} {item}\n")
                }
                EventType::FIGHT_LEAVE { player_name } => {
                    format!("EVT FIGHT LEAVE {player_name}\n")
                }
                EventType::ENTER_FIGHT { player_name, hp } => {
                    format!("EVT FIGHT ENTER {player_name} {hp}\n")
                }
                EventType::HEALING { player_name, heal } => {
                    format!("EVT FIGHT HEALING {player_name} {heal}\n")
                }
                EventType::ENEMY_ATTACK {
                    target,
                    target_hp,
                    target_killed,
                    damages,
                } => {
                    format!("EVT FIGHT ENEMY {target} {target_hp} {damages} {target_killed}\n")
                }
                EventType::ATTACK {
                    player_name,
                    damages,
                    enemy_hp,
                    loot,
                } => {
                    let loot_list = loot.join("//");
                    format!("EVT FIGHT ATTACK {player_name} {damages} {enemy_hp} {loot_list}\n")
                }
                EventType::ROOM_DROP { player_name, item } => {
                    format!("EVT ROOM DROP {player_name} {item}\n")
                }
                EventType::SERVER_RESET => format!("EVT SERVER RESET\n"),
                EventType::QUEST_UPDATE {
                    quest_id,
                    goal,
                    previous_goal,
                } => {
                    let data = serde_json::json!({ "quest": quest_id, "goal": goal , "previous_goal": previous_goal});
                    format!("EVT QUEST UPDATE {data}\n")
                }
                EventType::QUEST_FINISH { quest_id, reward } => {
                    let data = serde_json::json!({ "quest": quest_id, "reward": reward });
                    format!("EVT QUEST FINISH {data}\n")
                }
            },
        }
    }
}

impl From<Result<(), ErrorCode>> for Message {
    fn from(result: Result<(), ErrorCode>) -> Self {
        let code = result.err().unwrap_or(ErrorCode::SUCCESS);
        Message::Response {
            error: code,
            payload: Payload::Empty,
        }
    }
}

impl From<Result<Value, ErrorCode>> for Message {
    fn from(result: Result<Value, ErrorCode>) -> Self {
        match result {
            Ok(data) => {
                return Message::Response {
                    error: ErrorCode::SUCCESS,
                    payload: Payload::Json(data),
                };
            }
            Err(code) => {
                return Message::Response {
                    error: code,
                    payload: Payload::Empty,
                };
            }
        }
    }
}

impl From<Result<String, ErrorCode>> for Message {
    fn from(result: Result<String, ErrorCode>) -> Self {
        match result {
            Ok(data) => {
                return Message::Response {
                    error: ErrorCode::SUCCESS,
                    payload: Payload::Text(data),
                };
            }
            Err(code) => {
                return Message::Response {
                    error: code,
                    payload: Payload::Empty,
                };
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn response(error: ErrorCode, payload: Payload) -> String {
        Message::Response { error, payload }.to_str()
    }

    fn pair(key: &str, value: &str) -> Payload {
        Payload::Pair(HashMap::from([(key.to_string(), value.to_string())]))
    }

    // --- From: la conversion de Result reste valable ---

    #[test]
    fn from_ok_unit_gives_success_empty() {
        let msg: Message = Ok::<(), ErrorCode>(()).into();
        assert_eq!(
            msg,
            Message::Response {
                error: ErrorCode::SUCCESS,
                payload: Payload::Empty
            }
        )
    }

    // --- Réponses : framing texte ---

    #[test]
    fn empty_success_renders_bare_ok() {
        assert_eq!(response(ErrorCode::SUCCESS, Payload::Empty), "OK\n");
    }

    #[test]
    fn text_success_renders_inline() {
        assert_eq!(
            response(ErrorCode::SUCCESS, Payload::Text("connected".to_string())),
            "OK connected\n"
        );
    }

    #[test]
    fn pair_success_renders_key_value() {
        assert_eq!(
            response(ErrorCode::SUCCESS, pair("room", "loc.square")),
            "OK room=loc.square\n"
        );
    }

    #[test]
    fn json_success_renders_inline_json() {
        assert_eq!(
            response(
                ErrorCode::SUCCESS,
                Payload::Json(serde_json::json!({"hp": 100}))
            ),
            "OK {\"hp\":100}\n"
        );
    }

    #[test]
    fn error_renders_code_and_symbolic_name() {
        assert_eq!(
            response(ErrorCode::NAME_IN_USE, Payload::Empty),
            "ERR 201 NAME_IN_USE\n"
        );
    }

    // --- Events : EVT <category> <type> <data> ---

    fn event(evt: EventType) -> String {
        Message::Event(evt).to_str()
    }

    #[test]
    fn room_join_renders_presence_enter() {
        assert_eq!(
            event(EventType::ROOM_JOIN {
                player_name: "alice".to_string()
            }),
            "EVT ROOM PRESENCE ENTER alice\n"
        );
    }

    #[test]
    fn room_leave_renders_presence_leave() {
        assert_eq!(
            event(EventType::ROOM_LEAVE {
                player_name: "alice".to_string()
            }),
            "EVT ROOM PRESENCE LEAVE alice\n"
        );
    }

    #[test]
    fn group_join_renders() {
        assert_eq!(
            event(EventType::GROUP_JOIN {
                player_name: "alice".to_string()
            }),
            "EVT GROUP JOIN alice\n"
        );
    }

    #[test]
    fn group_leave_renders() {
        assert_eq!(
            event(EventType::GROUP_LEAVE {
                player_name: "alice".to_string()
            }),
            "EVT GROUP LEAVE alice\n"
        );
    }

    #[test]
    fn group_invite_renders_leader_only() {
        // RFC pur : group_name n'est pas émis sur le wire.
        assert_eq!(
            event(EventType::GROUP_INVITE {
                sender: "alice".to_string(),
                group_name: "alice's group".to_string(),
            }),
            "EVT GROUP INVITE alice\n"
        );
    }

    #[test]
    fn chat_room_scope_renders() {
        assert_eq!(
            event(EventType::CHAT {
                scope: ChatScope::ROOM,
                sender: "alice".to_string(),
                body: "hello world".to_string(),
            }),
            "EVT ROOM CHAT alice hello world\n"
        );
    }

    #[test]
    fn chat_global_scope_renders() {
        assert_eq!(
            event(EventType::CHAT {
                scope: ChatScope::GLOBAL,
                sender: "alice".to_string(),
                body: "hi".to_string(),
            }),
            "EVT GLOBAL CHAT alice hi\n"
        );
    }

    #[test]
    fn stats_players_renders() {
        assert_eq!(
            event(EventType::STATS_PLAYERS { players: 2 }),
            "EVT STATS players=2\n"
        );
    }

    #[test]
    fn room_take_renders() {
        assert_eq!(
            event(EventType::ROOM_TAKE {
                player_name: "alice".to_string(),
                item: "sword".to_string(),
            }),
            "EVT ROOM TAKE alice sword\n"
        );
    }

    #[test]
    fn room_drop_renders() {
        assert_eq!(
            event(EventType::ROOM_DROP {
                player_name: "alice".to_string(),
                item: "sword".to_string(),
            }),
            "EVT ROOM DROP alice sword\n"
        );
    }
}

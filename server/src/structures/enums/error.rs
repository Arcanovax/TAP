use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum ErrorCode {
    NAME_IN_USE,
    NO_EXIT,
    NOT_IN_GROUP,
    NOT_GROUP_LEADER,
    ALREADY_IN_GROUP,
    ITEM_NOT_FOUND,
    ITEM_NOT_IN_INVENTORY,
    NPC_NOT_FOUND,
    NPC_NOT_HOSTILE,
    ROOM_NOT_FOUND,
    DEFEATED_ENEMY,
    NOT_YOUR_TURN,
    DEFEATED_FIGHTER,
    NO_QUEST_AVAILABLE,
    CONNECTION_FAILED,
    SEND_FAILED,
    INVALID_ARGS,
    INVALID_COMMAND,
    ALREADY_CONNECTED,
    NO_DIALOG,
    SUCCESS,
    DISCONNECTION_FAIL,
    ALREADY_INVITED,
    NOT_ENOUGH_GOLD,
    GAME_LOSE,
}

impl ErrorCode {
    pub fn code(&self) -> u16 {
        match self {
            ErrorCode::NAME_IN_USE => 201,
            ErrorCode::NO_EXIT => 301,
            ErrorCode::NO_DIALOG => 302,
            ErrorCode::NOT_IN_GROUP => 401,
            ErrorCode::ALREADY_IN_GROUP => 402,
            ErrorCode::NOT_GROUP_LEADER => 403,
            ErrorCode::ALREADY_INVITED => 407,
            ErrorCode::ITEM_NOT_FOUND
            | ErrorCode::ROOM_NOT_FOUND
            | ErrorCode::DEFEATED_ENEMY
            | ErrorCode::NOT_YOUR_TURN
            | ErrorCode::DEFEATED_FIGHTER
            | ErrorCode::ITEM_NOT_IN_INVENTORY
            | ErrorCode::NPC_NOT_FOUND => 404,
            ErrorCode::NPC_NOT_HOSTILE => 405,
            ErrorCode::NO_QUEST_AVAILABLE => 406,
            ErrorCode::NOT_ENOUGH_GOLD => 408,
            ErrorCode::GAME_LOSE => 409,
            ErrorCode::CONNECTION_FAILED => 900,
            ErrorCode::SEND_FAILED => 901,
            ErrorCode::INVALID_ARGS => 902,
            ErrorCode::INVALID_COMMAND => 903,
            ErrorCode::ALREADY_CONNECTED => 904,
            ErrorCode::DISCONNECTION_FAIL => 905,
            ErrorCode::SUCCESS => 0,
        }
    }

    pub fn name(&self) -> String {
        format!("{:?}", self)
    }
}

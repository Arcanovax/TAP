use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum ErrorCode {
    NAME_IN_USE,
    NO_EXIT,
    NOT_IN_GROUP,
    ALREADY_IN_GROUP,
    ITEM_NOT_FOUND,
    ITEM_NOT_IN_INVENTORY,
    NPC_NOT_FOUND,
    NPC_NOT_HOSTILE,
    NO_QUEST_AVAILABLE,
    CONNECTION_FAILED,
    SEND_FAILED,
    INVALID_ARGS,
    INVALID_COMMAND,
    ALREADY_CONNECTED,
    SUCCESS,
}

impl ErrorCode {
    pub fn code(&self) -> u16 {
        match self {
            ErrorCode::NAME_IN_USE => 201,
            ErrorCode::NO_EXIT => 301,
            ErrorCode::NOT_IN_GROUP => 401,
            ErrorCode::ALREADY_IN_GROUP => 402,
            ErrorCode::ITEM_NOT_FOUND
            | ErrorCode::ITEM_NOT_IN_INVENTORY
            | ErrorCode::NPC_NOT_FOUND => 404,
            ErrorCode::NPC_NOT_HOSTILE => 405,
            ErrorCode::NO_QUEST_AVAILABLE => 406,
            ErrorCode::CONNECTION_FAILED => 900,
            ErrorCode::SEND_FAILED => 901,
            ErrorCode::INVALID_ARGS => 902,
            ErrorCode::INVALID_COMMAND => 903,
            ErrorCode::ALREADY_CONNECTED => 904,
            ErrorCode::SUCCESS => 0,
        }
    }
}

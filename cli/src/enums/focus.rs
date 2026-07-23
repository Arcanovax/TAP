use std::slice::Iter;

use serde::Deserialize;

use crate::enums::states::States;

#[derive(Debug, Default, Deserialize, PartialEq, Clone)]
pub enum Focus {
    #[default]
    COMMAND,
    OUTPUT,
    CHATTEXT,
    SELL,
    BUY,
    CHAT,
    DESCR,
    NPC,
    INVENTORY,
    QUESTS,
    EXITS,
    BAG,
    CHOICE(String, String, Vec<String>),
}

impl Focus {
    pub fn iterator(state: &States) -> Iter<'static, Focus> {
        match state {
            States::InFight { .. } => {
                static FOCUS: [Focus; 4] =
                    [Focus::COMMAND, Focus::OUTPUT, Focus::CHAT, Focus::CHATTEXT];
                FOCUS.iter()
            }
            _ => {
                static FOCUS: [Focus; 9] = [
                    Focus::COMMAND,
                    Focus::OUTPUT,
                    Focus::CHAT,
                    Focus::CHATTEXT,
                    Focus::DESCR,
                    Focus::NPC,
                    Focus::INVENTORY,
                    Focus::QUESTS,
                    Focus::EXITS,
                ];
                FOCUS.iter()
            }
        }
    }
}

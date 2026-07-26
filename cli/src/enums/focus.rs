use std::slice::Iter;

use serde::Deserialize;

use crate::enums::states::States;

#[derive(Debug, Default, Deserialize, PartialEq, Clone)]
pub enum Focus {
    #[default]
    Command,
    Output,
    ChatText,
    Sell,
    Buy,
    Chat,
    Descr,
    Npc,
    Inventory,
    Quests,
    Exits,
    Bag,
    Choice(String, String, Vec<String>),
}

impl Focus {
    pub fn iterator(state: &States) -> Iter<'static, Focus> {
        match state {
            States::InFight { .. } => {
                static FOCUS: [Focus; 4] =
                    [Focus::Command, Focus::Output, Focus::Chat, Focus::ChatText];
                FOCUS.iter()
            }
            _ => {
                static FOCUS: [Focus; 9] = [
                    Focus::Command,
                    Focus::Output,
                    Focus::Chat,
                    Focus::ChatText,
                    Focus::Descr,
                    Focus::Npc,
                    Focus::Inventory,
                    Focus::Quests,
                    Focus::Exits,
                ];
                FOCUS.iter()
            }
        }
    }
}

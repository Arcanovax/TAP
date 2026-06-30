use std::slice::Iter;

use serde::Deserialize;

use crate::enums::states::States;

#[derive(Debug, Default, Deserialize, PartialEq, Clone)]
pub enum Focus {
    #[default]
	COMMAND,
    OUTPUT,
    CHAT,
    DESCR,
	NPC,
	INVENTORY,
	EXITS
}

impl Focus {
	pub fn iterator(state: &States) -> Iter<'static, Focus> {
		match state {
			States::InFight { .. } => {
				static FOCUS: [Focus; 3] = [Focus::COMMAND, Focus::OUTPUT, Focus::CHAT];
				FOCUS.iter()
			}
			_ => {
				static FOCUS: [Focus; 7] = [Focus::COMMAND, Focus::OUTPUT, Focus::CHAT, Focus::DESCR, Focus::NPC, Focus::INVENTORY, Focus::EXITS];
				FOCUS.iter()
			}
		}
	}
}
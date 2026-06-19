use std::slice::Iter;

use serde::Deserialize;

#[derive(Debug, Default, Deserialize, PartialEq, Clone)]
pub enum Focus {
    #[default]
	COMMAND,
    OUTPUT,
    CHAT,
    DESCR,
	NPC,
	INVENTORY,
	EXITS,
	DISCUSS
}

impl Focus {
	pub fn iterator() -> Iter<'static, Focus> {
		static FOCUS: [Focus; 7] = [Focus::COMMAND, Focus::OUTPUT, Focus::CHAT, Focus::DESCR, Focus::NPC, Focus::INVENTORY, Focus::EXITS];
		FOCUS.iter()
	}
}
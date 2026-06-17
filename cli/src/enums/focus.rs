use serde::Deserialize;

#[derive(Debug, Default, Deserialize, PartialEq, Clone)]
pub enum Focus {
    #[default]
	COMMAND,
    CHAT,
	NPC,
	INVENTORY,
	EXITS,
    DESCR,
    OUTPUT,
    NONE
}
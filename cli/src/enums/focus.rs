use serde::Deserialize;

#[derive(Debug, Default, Deserialize, PartialEq, Clone)]
pub enum Focus {
    #[default]
    CHAT,
	COMMAND,
	NPC,
	INVENTORY,
	EXITS,
    DESCR,
    OUTPUT,
    NONE
}
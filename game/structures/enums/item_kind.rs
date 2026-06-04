use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ItemKind {
	Weapon { damages: u32 },
	Potion { healing: u32 },
}
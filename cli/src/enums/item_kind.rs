use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum ItemKind {
    Weapon { damages: u32 },
    Armor { protection: u32 },
    Potion { healing: u32 },
    Miscellaneous,
}

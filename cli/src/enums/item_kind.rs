use std::fmt::{Display, write};

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum ItemKind {
    Weapon { damages: u32 },
    Armor { protection: u32 },
    Potion { healing: u32 },
    Miscellaneous,
    QuestItem,
}

impl Display for ItemKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ItemKind::Armor { protection } => write!(f, "Armor: +{}\n", protection),
            ItemKind::Miscellaneous => write!(f, "Miscellaneous\n"),
            ItemKind::QuestItem => write!(f, "QuestItem\n"),
            ItemKind::Potion { healing } => write!(f, "Potion: +{}HP\n", healing),
            ItemKind::Weapon { damages } => write!(f, "Weapon: +{} damages\n", damages),
        }
    }
}

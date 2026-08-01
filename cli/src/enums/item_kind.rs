use std::fmt::Display;

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
            ItemKind::Armor { protection } => writeln!(f, "Armor: +{}", protection),
            ItemKind::Miscellaneous => writeln!(f, "Miscellaneous"),
            ItemKind::QuestItem => writeln!(f, "QuestItem"),
            ItemKind::Potion { healing } => writeln!(f, "Potion: +{}HP", healing),
            ItemKind::Weapon { damages } => writeln!(f, "Weapon: +{} damages", damages),
        }
    }
}

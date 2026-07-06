use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::enums::item_kind::ItemKind;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Item {
    pub name: String,
    pub price: u32,
    pub kind: ItemKind,
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\nValue: {}\nKind:\n{}",
            self.name, self.price, self.kind
        )
    }
}

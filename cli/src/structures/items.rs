use serde::{Deserialize, Serialize};

use crate::enums::item_kind::ItemKind;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Item {
    pub name: String,
    pub price: u32,
    pub kind: ItemKind,
}
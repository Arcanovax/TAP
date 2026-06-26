use crate::structures::enums::item_kind::ItemKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Item {
    pub name: String,
    pub price: u32,
    pub kind: ItemKind,
}

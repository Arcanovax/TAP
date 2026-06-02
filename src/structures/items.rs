use crate::structures::enums::item_kind::ItemKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Items {
	name: String,
	price: u32,
	number: u32,
	kind: ItemKind
}
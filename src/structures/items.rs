use crate::structures::enums::item_kind::ItemKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Items {
	id: String,
	name: String,
	price: u32,
	kind: ItemKind
}
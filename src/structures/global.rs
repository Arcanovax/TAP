use crate::structures::enums::item_kind::ItemKind;
use crate::structures::location::Location;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Global {
	pub locations: HashMap<String, Location>,
}
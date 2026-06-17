use serde::Deserialize;

use crate::{enums::exits::Exits};

#[derive(Deserialize, Debug)]
pub struct RoomView {
    pub id: String,
    pub name: String,
    pub description: String,
    pub exits: Vec<Exits>,
}

impl RoomView {
    pub fn new() -> Self {
        RoomView {
            id: "".to_string(),
            name: "".to_string(),
            description: "".to_string(),
            exits: Vec::new(),
        }
    }
}
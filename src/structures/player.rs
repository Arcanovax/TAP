use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;
use uuid::Uuid;
pub type Tx = UnboundedSender<Message>;

use std::collections::HashMap;

use crate::{protocol::Message, structures::enums::state::State};

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
	pub hp: u32,
	pub max_hp: u32,
	pub location: String,
	pub status: State,
	pub inventory: HashMap<String, u32>,
	pub available_quests: Vec<String>,
    pub group_id: Option<Uuid>,
}

impl Player {
    pub fn new(name: String) -> Self {
        Player {
            name,
            hp: 100,
            max_hp: 100,
            location: String::from("loc.city_square"),
            status: State::Idle,
            inventory: HashMap::new(),
            available_quests: Vec::new(),
            group_id: None,
        }
    }
}

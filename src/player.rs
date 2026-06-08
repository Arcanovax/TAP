use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;
// use std::collections::HashMap;
use uuid::Uuid;
pub type Tx = UnboundedSender<Message>;

use std::{collections::HashMap, net::SocketAddr};

use crate::{protocol::Message, structures::enums::state::State};

// #[derive(Debug, PartialEq, Serialize, Deserialize)]
// pub enum State {
//     InFight { target_id: String },
//     Idle,
//     Respawn,
//     Discuss,
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
	pub hp: u32,
	pub max_hp: u32,
	pub location: String,
	pub status: State,
	pub inventory: HashMap<String, u32>,
	pub available_quests: Vec<String>,
	pub addr: SocketAddr,
    pub tx: Tx,
    pub group_id: Option<Uuid>,
}

impl Player {
    pub fn new(name: String, addr: SocketAddr, tx: Tx) -> Self {
        Player {
            name,
			addr,
    		tx,
            hp: 10,
            max_hp: 10,
            location: String::from("place"),
            status: State::Idle,
            inventory: HashMap::new(),
            available_quests: Vec::new(),
            group_id: None,
        }
    }
}

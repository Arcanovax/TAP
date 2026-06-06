use serde::{Deserialize, Serialize};
// use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum State {
    InFight { target_id: String },
    Idle,
    Respawn,
    Discuss,
}

#[derive(Debug)]
pub struct Player {
    pub id: Uuid,
    pub name: String,
    //     pub hp: u32,
    //     pub max_hp: u32,
    //     pub location: String,
    //     pub status: State,
    //     pub inventory: HashMap<String, u32>,
    //     pub available_quests: Vec<String>,
    pub group_id: Option<Uuid>,
}

impl Player {
    pub fn new(name: String) -> Self {
        Player {
            id: Uuid::new_v4(),
            name,
            // hp: 10,
            // max_hp: 10,
            // location: String::from("place"),
            // status: State::Idle,
            // inventory: HashMap::new(),
            // available_quests: Vec::new(),
            group_id: None,
        }
    }
}

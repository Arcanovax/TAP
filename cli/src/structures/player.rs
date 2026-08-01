use std::collections::HashMap;

use serde::Deserialize;

use crate::structures::{quest::Quest, quest_view::QuestsView};

#[derive(Deserialize, Debug)]
pub struct Player {
    pub name: String,
    pub hp: u32,
    pub max_hp: u32,
    pub inventory: HashMap<String, u32>,
    pub gold: u32,
    pub quests: HashMap<String, Quest>,
    pub quests_views: Vec<QuestsView>,
}

impl Player {
    pub fn new() -> Self {
        Player {
            name: "".to_string(),
            hp: 100,
            max_hp: 100,
            inventory: HashMap::new(),
            gold: 0,
            quests: HashMap::new(),
            quests_views: Vec::new(),
        }
    }
}

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use std::collections::{HashMap, HashSet};

use crate::structures::dungeon::parse_dungeon_id;
use crate::structures::enums::state::State;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Player {
    pub name: String,
    pub hp: u32,
    pub max_hp: u32,
    pub location: String,
    pub status: State,
    pub inventory: HashMap<String, u32>,
    pub available_quests: Vec<String>,
    pub group_id: Option<Uuid>,
    pub finished_quest: HashSet<String>,
    pub quests_in_progress: HashMap<String, usize>,
    pub gold: u32,
}

impl Player {
    pub fn new(name: String, spawn_point: String) -> Self {
        Player {
            name,
            hp: 100,
            max_hp: 100,
            location: spawn_point,
            status: State::Idle,
            inventory: HashMap::new(),
            available_quests: Vec::new(),
            group_id: None,
            finished_quest: HashSet::new(),
            quests_in_progress: HashMap::new(),
            gold: 50,
        }
    }

    /// Le joueur est en donjon si et seulement si sa `location` est un id de donjon.
    /// Source de vérité unique : pas de flag à synchroniser.
    pub fn in_dungeon(&self) -> bool {
        parse_dungeon_id(&self.location).is_some()
    }
}

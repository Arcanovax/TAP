use std::collections::HashMap;

use ratatui::layout::Rect;

#[derive(Default, Debug)]
pub struct Fight {
    pub target_name: String,
    pub target_hp: u32,
    pub target_max_hp: u32,
    pub fighters: HashMap<String, u32>,
    pub buttons: HashMap<String, Rect>,
    pub bag: bool,
}

impl Fight {
    pub fn new() -> Self {
        Fight {
            target_name: "".to_string(),
            target_hp: 0,
            target_max_hp: 0,
            fighters: HashMap::new(),
            buttons: HashMap::new(),
            bag: false,
        }
    }
}

use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct Fight {
    pub target_name: String,
    pub target_hp: u32,
    pub fighters: HashMap<String, u32>
}

impl Fight {
    pub fn new() -> Self {
        Fight { target_name: "".to_string(), target_hp: 0, fighters: HashMap::new() }
    }
}
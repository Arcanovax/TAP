use serde::Deserialize;

use crate::enums::goals::Goal;

#[derive(Deserialize, Debug)]
pub struct UpdateView {
    pub quest: String,
    _goal: Goal,
    pub previous_goal: Goal,
}

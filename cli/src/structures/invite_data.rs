use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct InviteData {
    pub sender: String,
    pub group_name: String,
}
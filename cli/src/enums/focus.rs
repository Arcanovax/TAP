use serde::Deserialize;

#[derive(Debug, Default, Deserialize, PartialEq, Clone)]
pub enum Focus {
    #[default]
    CHAT,
    DESCR,
    OUTPUT,
    NONE
}
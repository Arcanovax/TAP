use std::{fmt::Display, time::Instant};

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub enum States {
    Login,
    ServerWait,
    ServerError(String),
    Idle,
    Trade(Vec<String>),
    InFight { target_id: String },
    InDiscuss(String, String),
    Quit(u32, Box<States>, #[serde(skip)] Option<Instant>),
}

impl Display for States {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = match self {
            States::Login => "Login",
            States::ServerWait => "ServerWait",
            States::ServerError(..) => "ServerError",
            States::Idle => "Idle",
            States::Trade(..) => "Trade",
            States::InFight { .. } => "InFight",
            States::InDiscuss(..) => "InDiscuss",
            States::Quit(..) => "Quit",
        };
        write!(f, "{status}")
    }
}

use std::fmt::Display;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct CommandHelp {
    pub command: String,
    pub description: String,
}

impl Display for CommandHelp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}: {}", self.command, self.description)
    }
}

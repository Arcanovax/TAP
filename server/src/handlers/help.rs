use crate::{
    protocol::{Message, Payload},
    structures::enums::{command::Command, error::ErrorCode},
};
use serde::Serialize;
use strum::IntoEnumIterator;

#[cfg(test)]
mod tests;

#[derive(Serialize)]
struct CommandHelp {
    command: String,
    description: String,
}

pub(super) fn help_request() -> Message {
    let commands: Vec<CommandHelp> = Command::iter()
        .map(|command| CommandHelp {
            command: format!("{command:?}"),
            description: command.description().to_string(),
        })
        .collect();

    Message::Response {
        error: ErrorCode::SUCCESS,
        payload: Payload::Json(serde_json::to_value(commands).unwrap()),
    }
}

use crate::protocol::{Message, Payload};
use crate::state::SharedServer;
use crate::structures::enums::error::ErrorCode;
use std::collections::HashMap;
use tracing::info;

#[cfg(test)]
mod tests;

pub(super) fn who_request(server_info: &SharedServer) -> Message {
    info!("Requested number of player");
    Message::Response {
        error: ErrorCode::SUCCESS,
        payload: Payload::Pair(HashMap::from([(
            "players".to_string(),
            server_info
                .lock()
                .unwrap()
                .get_number_of_players()
                .to_string(),
        )])),
    }
}

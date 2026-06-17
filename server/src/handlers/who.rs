use serde_json::json;

use crate::protocol::Message;
use crate::state::SharedServer;
use crate::structures::enums::error::ErrorCode;

#[cfg(test)]
mod tests;

pub(super) fn who_request(server_info: &SharedServer) -> Message {
    Message::Response {
        error: ErrorCode::SUCCESS,
        data: Some(json!({ "players": server_info.lock().unwrap().get_number_of_players() })),
    }
}

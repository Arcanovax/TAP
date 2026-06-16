use serde_json::json;

use crate::error::ErrorCode;
use crate::protocol::Message;
use crate::state::SharedServer;

#[cfg(test)]
mod tests;

pub(super) fn who_request(server_info: &SharedServer) -> Message {
    Message::Response {
        error: ErrorCode::SUCCESS,
        data: Some(json!({ "players": server_info.lock().unwrap().get_number_of_players() })),
    }
}

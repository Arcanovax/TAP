use crate::error::ErrorCode;
use crate::protocol::Message;
use crate::state::SharedServer;

#[cfg(test)]
mod tests;

pub(super) fn who_request(server_info: &SharedServer) -> Message {
    Message::Response {
        error: ErrorCode::SUCCESS,
        data: Some(
            serde_json::to_value(server_info.lock().unwrap().get_number_of_players()).unwrap(),
        ),
    }
}

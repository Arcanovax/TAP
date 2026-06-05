use crate::error::ErrorCode;
use crate::protocol::Message;
use crate::state::ServerInfo;
use std::sync::{Arc, Mutex};

pub(super) fn who_request(server_info: &Arc<Mutex<ServerInfo>>) -> Message {
    Message::Response {
        error: ErrorCode::SUCCESS,
        data: Some(
            server_info
                .lock()
                .unwrap()
                .get_number_of_players()
                .to_string(),
        ),
    }
}

use crate::error::ErrorCode;
use crate::protocol::Message;
use crate::state::SharedServer;
use std::net::SocketAddr;

pub(super) fn status_request(server_info: &SharedServer, peer_addr: SocketAddr) -> Message {
    match server_info.lock().unwrap().get_player(peer_addr) {
        Ok(player) => Message::Response {
            error: ErrorCode::SUCCESS,
            data: Some(serde_json::to_string(player).unwrap()),
        },
        Err(code) => Message::Response {
            error: code,
            data: None,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{addr, assert_success_contains, connect, err, test_server};

    #[test]
    fn status_without_connection_returns_invalid_command() {
        let server = test_server();
        assert_eq!(
            status_request(&server, addr(1)),
            err(ErrorCode::INVALID_COMMAND)
        );
    }

    #[test]
    fn status_connected_returns_player_data() {
        let server = test_server();
        connect(&server, addr(1), "alice");
        let result = status_request(&server, addr(1));
        assert_success_contains(&result, "\"name\":\"alice\"");
    }
}

use crate::error::ErrorCode;
use crate::protocol::Message;
use crate::state::SharedServer;

pub(super) fn who_request(server_info: &SharedServer) -> Message {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{addr, connect, ok_data, test_server};

    #[test]
    fn who_without_players_returns_zero() {
        let server = test_server();
        assert_eq!(who_request(&server), ok_data("0"));
    }

    #[test]
    fn who_counts_connected_players() {
        let server = test_server();
        connect(&server, addr(1), "alice");
        connect(&server, addr(2), "bob");
        assert_eq!(who_request(&server), ok_data("2"));
    }
}

use crate::{error::ErrorCode, protocol::Message, state::SharedServer};
use std::net::SocketAddr;

pub fn inventory_request(server_info: &SharedServer, peer_addr: SocketAddr) -> Message {
    match server_info.lock().unwrap().get_player(peer_addr) {
        Ok(player) => {
            return Message::Response {
                error: ErrorCode::SUCCESS,
                data: Some(serde_json::to_string(&player.inventory).unwrap()),
            };
        }
        Err(code) => {
            return Message::Response {
                error: code,
                data: None,
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{addr, connect, err, ok_data, test_server};

    #[test]
    fn inventory_without_connection_returns_invalid_command() {
        let server = test_server();
        assert_eq!(
            inventory_request(&server, addr(1)),
            err(ErrorCode::INVALID_COMMAND)
        );
    }

    #[test]
    fn inventory_when_empty_returns_empty_map() {
        let server = test_server();
        connect(&server, addr(1), "alice");
        assert_eq!(inventory_request(&server, addr(1)), ok_data("{}"));
    }
}

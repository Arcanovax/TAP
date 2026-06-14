use crate::{error::ErrorCode, protocol::Message, state::SharedServer};
use std::net::SocketAddr;

pub fn take_request(
    server_info: &SharedServer,
    peer_addr: SocketAddr,
    args: &Vec<String>,
) -> Message {
    let mut binding = server_info.lock().unwrap();
    if !binding.is_connected(peer_addr) {
        return Message::Response {
            error: ErrorCode::INVALID_COMMAND,
            data: None,
        };
    }
    if args.len() == 0 {
        return Message::Response {
            error: ErrorCode::INVALID_ARGS,
            data: None,
        };
    }

    let mut taken: Vec<String> = Vec::new();
    for item in args {
        if let Ok(item) = binding.try_take_item(peer_addr, item) {
            taken.push(item);
        }
    }
    Message::Response {
        error: ErrorCode::SUCCESS,
        data: Some(serde_json::to_string(&taken).unwrap()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{addr, connect, err, ok_data, populated_server};

    #[test]
    fn take_without_connection_returns_invalid_command() {
        let server = populated_server();
        let result = take_request(&server, addr(1), &vec!["sword".to_string()]);
        assert_eq!(result, err(ErrorCode::INVALID_COMMAND));
    }

    #[test]
    fn take_without_args_returns_invalid_args() {
        let server = populated_server();
        connect(&server, addr(1), "alice");
        let result = take_request(&server, addr(1), &vec![]);
        assert_eq!(result, err(ErrorCode::INVALID_ARGS));
    }

    #[test]
    fn take_existing_item_moves_it_to_inventory() {
        let server = populated_server();
        connect(&server, addr(1), "alice");
        let result = take_request(&server, addr(1), &vec!["sword".to_string()]);
        assert_eq!(result, ok_data(r#"["sword"]"#));
        // l'item est bien passé dans l'inventaire
        let guard = server.lock().unwrap();
        let player = guard.get_player(addr(1)).unwrap();
        assert_eq!(player.inventory.get("sword"), Some(&1));
    }

    #[test]
    fn take_unknown_item_returns_empty_success() {
        let server = populated_server();
        connect(&server, addr(1), "alice");
        let result = take_request(&server, addr(1), &vec!["shield".to_string()]);
        assert_eq!(result, ok_data("[]"));
    }
}

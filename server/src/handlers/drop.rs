use crate::{error::ErrorCode, protocol::Message, state::SharedServer};
use std::net::SocketAddr;

pub fn drop_request(
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
    let mut dropped: Vec<String> = Vec::new();
    for item in args {
        if let Ok(item) = binding.try_drop_item(peer_addr, item) {
            dropped.push(item);
        }
    }
    Message::Response {
        error: ErrorCode::SUCCESS,
        data: Some(serde_json::to_string(&dropped).unwrap()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handlers::take::take_request;
    use crate::test_utils::{addr, connect, err, ok_data, populated_server};

    #[test]
    fn drop_without_connection_returns_invalid_command() {
        let server = populated_server();
        let result = drop_request(&server, addr(1), &vec!["sword".to_string()]);
        assert_eq!(result, err(ErrorCode::INVALID_COMMAND));
    }

    #[test]
    fn drop_without_args_returns_invalid_args() {
        let server = populated_server();
        connect(&server, addr(1), "alice");
        let result = drop_request(&server, addr(1), &vec![]);
        assert_eq!(result, err(ErrorCode::INVALID_ARGS));
    }

    #[test]
    fn drop_item_in_inventory_returns_it() {
        let server = populated_server();
        connect(&server, addr(1), "alice");
        // d'abord ramasser pour avoir l'item en inventaire
        take_request(&server, addr(1), &vec!["sword".to_string()]);

        let result = drop_request(&server, addr(1), &vec!["sword".to_string()]);
        assert_eq!(result, ok_data(r#"["sword"]"#));
        // l'item n'est plus dans l'inventaire
        let guard = server.lock().unwrap();
        assert!(
            guard
                .get_player(addr(1))
                .unwrap()
                .inventory
                .get("sword")
                .is_none()
        );
    }

    #[test]
    fn drop_item_not_in_inventory_returns_empty_success() {
        let server = populated_server();
        connect(&server, addr(1), "alice");
        let result = drop_request(&server, addr(1), &vec!["sword".to_string()]);
        assert_eq!(result, ok_data("[]"));
    }
}

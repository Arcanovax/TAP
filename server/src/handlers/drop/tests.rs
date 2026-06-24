use super::*;
use crate::handlers::take::take_request;
use crate::test_utils::{addr, connect, err, ok_pair, populated_server};

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
    assert_eq!(result, ok_pair("dropped", "sword"));
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
fn drop_item_not_in_inventory_returns_item_not_in_inventory() {
    let server = populated_server();
    connect(&server, addr(1), "alice");
    let result = drop_request(&server, addr(1), &vec!["sword".to_string()]);
    assert_eq!(result, err(ErrorCode::ITEM_NOT_IN_INVENTORY));
}

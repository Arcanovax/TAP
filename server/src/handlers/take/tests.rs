use super::*;
use crate::test_utils::{addr, connect, err, ok_pair, populated_server};

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
    assert_eq!(result, ok_pair(&[("taken", "sword")]));
    // l'item est bien passé dans l'inventaire
    let guard = server.lock().unwrap();
    let player = guard.get_player(addr(1)).unwrap();
    assert_eq!(player.inventory.get("sword"), Some(&1));
}

#[test]
fn take_unknown_item_returns_item_not_found() {
    let server = populated_server();
    connect(&server, addr(1), "alice");
    let result = take_request(&server, addr(1), &vec!["shield".to_string()]);
    assert_eq!(result, err(ErrorCode::ITEM_NOT_FOUND));
}

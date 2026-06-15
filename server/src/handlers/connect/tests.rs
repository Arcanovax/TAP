use super::*;
use crate::test_utils::{addr, connect, err, populated_server, test_server, tx_rx};

#[test]
fn connect_without_args_returns_invalid_args() {
    let server = test_server();
    let (tx, _rx) = tx_rx();
    let result = connect_request(&vec![], &server, addr(1), &tx);
    assert_eq!(result, err(ErrorCode::INVALID_ARGS));
}

#[test]
fn connect_with_too_many_args_returns_invalid_args() {
    let server = test_server();
    let (tx, _rx) = tx_rx();
    let result = connect_request(
        &vec!["alice".to_string(), "bob".to_string()],
        &server,
        addr(1),
        &tx,
    );
    assert_eq!(result, err(ErrorCode::INVALID_ARGS));
}

#[test]
fn connect_valid_returns_success_and_registers_player() {
    let server = test_server();
    let (tx, _rx) = tx_rx();
    let result = connect_request(&vec!["alice".to_string()], &server, addr(1), &tx);
    assert_eq!(result, err(ErrorCode::SUCCESS));
    assert!(server.lock().unwrap().is_connected(addr(1)));
}

#[test]
fn connect_duplicate_name_returns_name_in_use() {
    let server = test_server();
    connect(&server, addr(1), "alice");
    let (tx, _rx) = tx_rx();
    // même nom, adresse différente
    let result = connect_request(&vec!["alice".to_string()], &server, addr(2), &tx);
    assert_eq!(result, err(ErrorCode::NAME_IN_USE));
}

#[test]
fn connect_same_addr_twice_returns_already_connected() {
    let server = test_server();
    connect(&server, addr(1), "alice");
    let (tx, _rx) = tx_rx();
    // même adresse, autre nom
    let result = connect_request(&vec!["bob".to_string()], &server, addr(1), &tx);
    assert_eq!(result, err(ErrorCode::ALREADY_CONNECTED));
}

#[test]
fn connect_notifies_arrival_room_join() {
    let server = populated_server();
    let mut alice_rx = connect(&server, addr(1), "alice");
    let (tx, _rx) = tx_rx();
    connect_request(&vec!["bob".to_string()], &server, addr(2), &tx);

    let event = alice_rx
        .try_recv()
        .expect("Alice should have had receive an event");
    assert_eq!(
        event,
        Message::Event(EventType::ROOM_JOIN {
            player_name: String::from("bob")
        })
    );
}

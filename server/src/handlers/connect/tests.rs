use super::*;
use crate::test_utils::{addr, connect, err, ok_text, populated_server, test_server, tx_rx};

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
    assert_eq!(result, ok_text("connected"));
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

#[test]
fn connect_broadcasts_player_count_to_others() {
    let server = populated_server();
    let mut alice_rx = connect(&server, addr(1), "alice");
    let (tx, _rx) = tx_rx();
    connect_request(&vec!["bob".to_string()], &server, addr(2), &tx);

    // alice reçoit ROOM_JOIN puis PLAYERS : on vérifie la présence de l'event PLAYERS
    let events: Vec<_> = std::iter::from_fn(|| alice_rx.try_recv().ok()).collect();
    assert!(
        events.contains(&Message::Event(EventType::STATS_PLAYERS { players: 2 })),
        "alice devrait recevoir PLAYERS {{ players: 2 }}, reçu: {events:?}"
    );
}

#[test]
fn connect_does_not_send_player_count_to_self() {
    let server = test_server();
    connect(&server, addr(1), "alice");
    let (tx, mut bob_rx) = tx_rx();
    connect_request(&vec!["bob".to_string()], &server, addr(2), &tx);

    // le joueur qui se connecte est exclu des global receivers
    let events: Vec<_> = std::iter::from_fn(|| bob_rx.try_recv().ok()).collect();
    assert!(
        !events
            .iter()
            .any(|e| matches!(e, Message::Event(EventType::STATS_PLAYERS { .. }))),
        "bob ne devrait pas recevoir son propre event PLAYERS, reçu: {events:?}"
    );
}

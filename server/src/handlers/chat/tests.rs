use super::*;
use crate::test_utils::{addr, connect, err, test_server};

#[test]
fn chat_without_connection_returns_invalid_command() {
    let server = test_server();
    let result = chat_request(
        &vec!["GLOBAL".to_string(), "hi".to_string()],
        &server,
        addr(1),
    );
    assert_eq!(result, err(ErrorCode::INVALID_COMMAND));
}

#[test]
fn chat_without_body_returns_invalid_args() {
    let server = test_server();
    connect(&server, addr(1), "alice");
    let result = chat_request(&vec!["GLOBAL".to_string()], &server, addr(1));
    assert_eq!(result, err(ErrorCode::INVALID_ARGS));
}

#[test]
fn chat_with_unknown_scope_returns_invalid_args() {
    let server = test_server();
    connect(&server, addr(1), "alice");
    let result = chat_request(
        &vec!["WHISPER".to_string(), "hi".to_string()],
        &server,
        addr(1),
    );
    assert_eq!(result, err(ErrorCode::INVALID_ARGS));
}

#[test]
fn chat_group_scope_without_group_returns_not_in_group() {
    let server = test_server();
    connect(&server, addr(1), "alice");
    let result = chat_request(
        &vec!["GROUP".to_string(), "hi".to_string()],
        &server,
        addr(1),
    );
    assert_eq!(result, err(ErrorCode::NOT_IN_GROUP));
}

#[test]
fn chat_global_broadcasts_to_others_not_sender() {
    let server = test_server();
    let mut rx_alice = connect(&server, addr(1), "alice");
    let mut rx_bob = connect(&server, addr(2), "bob");

    let result = chat_request(
        &vec!["GLOBAL".to_string(), "hello world".to_string()],
        &server,
        addr(1),
    );

    assert_eq!(result, err(ErrorCode::SUCCESS));
    assert_eq!(
        rx_bob.try_recv().expect("bob should receive the chat"),
        Message::Event(EventType::CHAT {
            body: "hello world\n".to_string(),
            sender: "alice".to_string(),
            scope: ChatScope::GLOBAL,
        })
    );
    assert!(
        rx_alice.try_recv().is_err(),
        "the sender must not receive its own chat"
    );
}

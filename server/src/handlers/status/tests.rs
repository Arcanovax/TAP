use super::*;
use crate::test_utils::{addr, connect, err, ok_data, test_server};

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
    assert_eq!(
        result,
        ok_data(r#"{"hp":100,"max_hp":100,"status":"Idle"}"#)
    );
}

use super::*;
use crate::test_utils::{addr, assert_success_contains, connect, err, test_server};

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
    assert_success_contains(&result, "\"name\":\"alice\"");
}

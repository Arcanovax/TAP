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

use super::*;
use crate::test_utils::{addr, connect, ok_pair, test_server};

#[test]
fn who_without_players_returns_zero() {
    let server = test_server();
    assert_eq!(who_request(&server), ok_pair("players", "0"));
}

#[test]
fn who_counts_connected_players() {
    let server = test_server();
    connect(&server, addr(1), "alice");
    connect(&server, addr(2), "bob");
    assert_eq!(who_request(&server), ok_pair("players", "2"));
}

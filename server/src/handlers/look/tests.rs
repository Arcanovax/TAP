use super::*;
use crate::test_utils::{
    addr, assert_success_contains, connect, err, populated_server, test_server,
};

#[test]
fn look_without_connection_returns_invalid_command() {
    let server = populated_server();
    assert_eq!(
        look_request(&server, addr(1)),
        err(ErrorCode::INVALID_COMMAND)
    );
}

#[test]
fn look_in_existing_room_returns_room_and_players() {
    let server = populated_server();
    connect(&server, addr(1), "alice");
    let result = look_request(&server, addr(1));
    assert_success_contains(&result, "room.city_square");
    assert_success_contains(&result, "alice");
}

#[test]
fn look_when_room_missing_returns_player_not_found() {
    let server = test_server(); // monde vide -> "room.city_square" n'existe pas
    connect(&server, addr(1), "alice");
    assert_eq!(
        look_request(&server, addr(1)),
        err(ErrorCode::ROOM_NOT_FOUND)
    );
}

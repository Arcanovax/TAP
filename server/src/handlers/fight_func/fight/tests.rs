use super::*;
use crate::test_utils::{
    addr, assert_success_contains, connect, populated_server, response_error, test_server,
};

#[test]
fn fight_with_wrong_args_returns_invalid_args() {
    let server = populated_server();
    connect(&server, addr(1), "alice");
    let result = fight(addr(1), &vec![], &server);
    assert_eq!(response_error(&result), &ErrorCode::INVALID_ARGS);
}

#[test]
fn fight_without_connection_returns_player_not_found() {
    let server = populated_server();
    let result = fight(addr(1), &vec!["goblin".to_string()], &server);
    assert_eq!(response_error(&result), &ErrorCode::PLAYER_NOT_FOUND);
}

#[test]
fn fight_when_room_missing_returns_room_not_found() {
    let server = test_server(); // monde vide
    connect(&server, addr(1), "alice");
    let result = fight(addr(1), &vec!["goblin".to_string()], &server);
    assert_eq!(response_error(&result), &ErrorCode::ROOM_NOT_FOUND);
}

#[test]
fn fight_target_not_in_room_returns_npc_not_found() {
    let server = populated_server();
    connect(&server, addr(1), "alice");
    let result = fight(addr(1), &vec!["dragon".to_string()], &server);
    assert_eq!(response_error(&result), &ErrorCode::NPC_NOT_FOUND);
}

#[test]
fn fight_non_hostile_target_returns_npc_not_hostile() {
    let server = populated_server();
    connect(&server, addr(1), "alice");
    // "guard" est un Citizen présent dans la room
    let result = fight(addr(1), &vec!["guard".to_string()], &server);
    assert_eq!(response_error(&result), &ErrorCode::NPC_NOT_HOSTILE);
}

#[test]
fn fight_engages_enemy_and_sets_in_fight_state() {
    let server = populated_server();
    connect(&server, addr(1), "alice");
    let result = fight(addr(1), &vec!["goblin".to_string()], &server);

    assert_eq!(response_error(&result), &ErrorCode::SUCCESS);
    assert_success_contains(&result, "Hello there!");

    let guard = server.lock().unwrap();
    let player = guard.get_player(addr(1)).unwrap();
    assert!(matches!(player.status, State::InFight { .. }));
}

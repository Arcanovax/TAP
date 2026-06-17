use super::*;
use crate::test_utils::{addr, assert_success_contains, connect, err, populated_server};

#[test]
fn quest_without_connection_returns_invalid_command() {
    let server = populated_server();
    let result = quest_request(&vec!["guard".to_string()], &server, addr(1));
    assert_eq!(result, err(ErrorCode::INVALID_COMMAND));
}

#[test]
fn quest_with_wrong_args_returns_invalid_args() {
    let server = populated_server();
    connect(&server, addr(1), "alice");
    assert_eq!(
        quest_request(&vec![], &server, addr(1)),
        err(ErrorCode::INVALID_ARGS)
    );
    assert_eq!(
        quest_request(&vec!["a".to_string(), "b".to_string()], &server, addr(1)),
        err(ErrorCode::NPC_NOT_FOUND)
    );
}

#[test]
fn quest_with_unknown_npc_returns_npc_not_found() {
    let server = populated_server();
    connect(&server, addr(1), "alice");
    let result = quest_request(&vec!["nobody".to_string()], &server, addr(1));
    assert_eq!(result, err(ErrorCode::NPC_NOT_FOUND));
}

#[test]
fn quest_from_npc_without_quest_returns_no_quest_available() {
    let server = populated_server();
    connect(&server, addr(1), "alice");
    let result = quest_request(&vec!["villager".to_string()], &server, addr(1));
    assert_eq!(result, err(ErrorCode::NO_QUEST_AVAILABLE));
}

#[test]
fn quest_accept_returns_quest_data() {
    let server = populated_server();
    connect(&server, addr(1), "alice");
    let result = quest_request(&vec!["guard".to_string()], &server, addr(1));
    assert_success_contains(&result, "quest.fetch");
}

#[test]
fn quest_accept_twice_returns_no_quest_available() {
    let server = populated_server();
    connect(&server, addr(1), "alice");
    quest_request(&vec!["guard".to_string()], &server, addr(1));
    let result = quest_request(&vec!["guard".to_string()], &server, addr(1));
    assert_eq!(result, err(ErrorCode::NO_QUEST_AVAILABLE));
}

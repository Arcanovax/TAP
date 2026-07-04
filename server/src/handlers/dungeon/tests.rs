use super::*;
use crate::{
    protocol::EventType,
    test_utils::{addr, connect, err, group_with, response_error, test_server},
};

// --- DISPATCH ---

#[test]
fn dungeon_without_subcommand_returns_invalid_args() {
    let server = test_server();
    let result = dungeon_request(&vec![], &server, addr(20001));
    assert_eq!(result, err(ErrorCode::INVALID_ARGS));
}

#[test]
fn dungeon_unknown_subcommand_returns_invalid_args() {
    let server = test_server();
    let result = dungeon_request(&vec!["FOO".to_string()], &server, addr(20001));
    assert_eq!(result, err(ErrorCode::INVALID_ARGS));
}

// --- CREATE ---

#[test]
fn dungeon_create_without_connected_returns_invalid_command() {
    let server = test_server();
    let result = dungeon_create_request(&server, addr(20001));
    assert_eq!(result, err(ErrorCode::INVALID_COMMAND));
}

#[test]
fn dungeon_create_without_group_returns_success() {
    let server = test_server();
    let a = addr(20001);
    connect(&server, a, "hero");
    let result = dungeon_create_request(&server, a);
    assert_eq!(response_error(&result), &ErrorCode::SUCCESS);
}

#[test]
fn dungeon_create_moves_creator_into_dungeon() {
    let server = test_server();
    let a = addr(20001);
    connect(&server, a, "hero");
    dungeon_create_request(&server, a);
    let loc = server
        .lock()
        .unwrap()
        .get_player(a)
        .unwrap()
        .location
        .clone();
    assert!(
        loc.contains(".dg_"),
        "creator should be inside a dungeon room, got {loc:?}"
    );
}

#[test]
fn dungeon_create_when_already_in_dungeon_returns_already_in_progress() {
    let server = test_server();
    let a = addr(20001);
    connect(&server, a, "hero");
    dungeon_create_request(&server, a);
    let result = dungeon_create_request(&server, a);
    assert_eq!(result, err(ErrorCode::DUNGEON_ALREADY_IN_PROGRESS));
}

#[test]
fn dungeon_create_when_not_group_leader_returns_not_group_leader() {
    let server = test_server();
    let members = vec![(addr(20001), "leader"), (addr(20002), "member")];
    group_with(&server, &members);
    let result = dungeon_create_request(&server, members[1].0);
    assert_eq!(result, err(ErrorCode::NOT_GROUP_LEADER));
}

#[test]
fn dungeon_create_when_group_already_has_dungeon_returns_already_in_progress() {
    let server = test_server();
    let members = vec![(addr(20001), "leader"), (addr(20002), "member")];
    group_with(&server, &members);
    dungeon_create_request(&server, members[0].0);
    let result = dungeon_create_request(&server, members[0].0);
    assert_eq!(result, err(ErrorCode::DUNGEON_ALREADY_IN_PROGRESS));
}

#[test]
fn dungeon_create_notifies_group_members() {
    let server = test_server();
    let members = vec![(addr(20001), "leader"), (addr(20002), "member")];
    let mut rxs = group_with(&server, &members);
    let result = dungeon_create_request(&server, members[0].0);
    assert_eq!(response_error(&result), &ErrorCode::SUCCESS);
    assert_eq!(
        rxs[1]
            .try_recv()
            .expect("member should be notified of the dungeon creation"),
        Message::Event(EventType::DUNGEON_CREATE)
    );
}

// --- JOIN ---

#[test]
fn dungeon_join_without_connected_returns_invalid_command() {
    let server = test_server();
    let result = dungeon_join_request(&server, addr(20001));
    assert_eq!(result, err(ErrorCode::INVALID_COMMAND));
}

#[test]
fn dungeon_join_without_group_returns_not_in_group() {
    let server = test_server();
    let a = addr(20001);
    connect(&server, a, "hero");
    let result = dungeon_join_request(&server, a);
    assert_eq!(result, err(ErrorCode::NOT_IN_GROUP));
}

#[test]
fn dungeon_join_without_dungeon_returns_no_dungeon_in_progress() {
    let server = test_server();
    let members = vec![(addr(20001), "leader"), (addr(20002), "member")];
    group_with(&server, &members);
    let result = dungeon_join_request(&server, members[1].0);
    assert_eq!(result, err(ErrorCode::NO_DUNGEON_IN_PROGRESS));
}

#[test]
fn dungeon_join_moves_member_into_dungeon() {
    let server = test_server();
    let members = vec![(addr(20001), "leader"), (addr(20002), "member")];
    group_with(&server, &members);
    dungeon_create_request(&server, members[0].0);
    let result = dungeon_join_request(&server, members[1].0);
    assert_eq!(response_error(&result), &ErrorCode::SUCCESS);
    let loc = server
        .lock()
        .unwrap()
        .get_player(members[1].0)
        .unwrap()
        .location
        .clone();
    assert!(
        loc.contains(".dg_"),
        "member should be inside the dungeon, got {loc:?}"
    );
}

#[test]
fn dungeon_join_when_already_in_dungeon_returns_already_in_progress() {
    let server = test_server();
    let members = vec![(addr(20001), "leader"), (addr(20002), "member")];
    group_with(&server, &members);
    dungeon_create_request(&server, members[0].0);
    dungeon_join_request(&server, members[1].0);
    let result = dungeon_join_request(&server, members[1].0);
    assert_eq!(result, err(ErrorCode::DUNGEON_ALREADY_IN_PROGRESS));
}

use super::*;
use crate::{
    protocol::EventType,
    test_utils::{addr, assert_success_contains, connect, err, group_with, test_server},
};

// GROUP CREATE TESTS

#[test]
fn group_create_without_connected_return_invalid_command() {
    let server = test_server();
    let addr = addr(10101);
    let result = group_create_request(&server, addr, &vec![]);
    assert_eq!(result, err(ErrorCode::INVALID_COMMAND));
}

#[test]
fn group_create_with_connected_return_success() {
    let server = test_server();
    let addr = addr(10101);
    connect(&server, addr, "test_user");
    let result = group_create_request(&server, addr, &vec![]);
    assert_success_contains(&result, "group");
}

#[test]
fn group_create_when_in_group_return_already_in_group() {
    let server = test_server();
    let addr = addr(10101);
    connect(&server, addr, "test_user");
    group_create_request(&server, addr, &vec![]);
    let result = group_create_request(&server, addr, &vec![]);
    assert_eq!(result, err(ErrorCode::ALREADY_IN_GROUP));
}

#[test]
fn group_create_with_args_return_success() {
    let server = test_server();
    let addr = addr(10101);
    connect(&server, addr, "test_user");
    let result = group_create_request(
        &server,
        addr,
        &vec![
            "test".to_string(),
            "custom".to_string(),
            "group".to_string(),
        ],
    );
    assert_success_contains(&result, "group");
}

// GROUP LEAVE TESTS

#[test]
fn group_leave_without_connected_return_invalid_command() {
    let server = test_server();
    let addr = addr(10101);
    group_create_request(&server, addr, &vec![]);
    let result = group_leave_request(&server, addr);
    assert_eq!(result, err(ErrorCode::INVALID_COMMAND));
}

#[test]
fn group_leave_without_group_return_not_in_group() {
    let server = test_server();
    let addr = addr(10101);
    connect(&server, addr, "test_user");
    let result = group_leave_request(&server, addr);
    assert_eq!(result, err(ErrorCode::NOT_IN_GROUP));
}

#[test]
fn group_leave_with_group_return_success() {
    let server = test_server();
    let addr = addr(10101);
    connect(&server, addr, "test_user");
    group_create_request(&server, addr, &vec![]);
    let result = group_leave_request(&server, addr);
    assert_eq!(result, err(ErrorCode::SUCCESS));
}

#[test]
fn group_leave_event_receive_leave_event() {
    let server = &test_server();
    let members = vec![(addr(10101), "p1"), (addr(10102), "p2")];
    let mut rxs = group_with(server, &members);
    let result = group_leave_request(server, members[0].0);
    assert_eq!(result, err(ErrorCode::SUCCESS));
    assert!(rxs[0].try_recv().is_err());
    assert_eq!(
        rxs[1].try_recv().expect("leave event not received"),
        Message::Event(EventType::GROUP_LEAVE {
            player_name: "p1".to_string()
        })
    );
}

// GROUP INVITE TESTS

#[test]
fn group_invite_when_not_in_group_returns_not_in_group() {
    let server = test_server();
    connect(&server, addr(1), "alice");
    let result = group_invite_request(
        &vec!["INVITE".to_string(), "bob".to_string()],
        &server,
        addr(1),
    );
    assert_eq!(result, err(ErrorCode::NOT_IN_GROUP));
}

#[test]
fn group_invite_with_wrong_args_returns_invalid_args() {
    let server = test_server();
    connect(&server, addr(1), "alice");
    group_create_request(&server, addr(1), &vec![]);
    let result = group_invite_request(&vec!["INVITE".to_string()], &server, addr(1));
    assert_eq!(result, err(ErrorCode::INVALID_ARGS));
}

#[test]
fn group_invite_self_returns_invalid_args() {
    let server = test_server();
    connect(&server, addr(1), "alice");
    group_create_request(&server, addr(1), &vec![]);
    let result = group_invite_request(
        &vec!["INVITE".to_string(), "alice".to_string()],
        &server,
        addr(1),
    );
    assert_eq!(result, err(ErrorCode::INVALID_ARGS));
}

#[test]
fn group_invite_notifies_target() {
    let server = test_server();
    connect(&server, addr(1), "alice");
    group_create_request(&server, addr(1), &vec![]);
    let mut rx_bob = connect(&server, addr(2), "bob");

    let result = group_invite_request(
        &vec!["INVITE".to_string(), "bob".to_string()],
        &server,
        addr(1),
    );

    assert_eq!(result, err(ErrorCode::SUCCESS));
    assert_eq!(
        rx_bob
            .try_recv()
            .expect("bob should receive the invitation"),
        Message::Event(EventType::INVITE {
            sender: "alice".to_string(),
            group_name: "alice's group".to_string(),
        })
    );
}

// GROUP JOIN TESTS

#[test]
fn group_join_without_invitation_returns_invalid_command() {
    let server = test_server();
    connect(&server, addr(1), "alice");
    let result = group_join_request(&server, addr(1));
    assert_eq!(result, err(ErrorCode::INVALID_COMMAND));
}

#[test]
fn group_join_notifies_existing_members() {
    let server = test_server();
    let mut rx_alice = connect(&server, addr(1), "alice");
    group_create_request(&server, addr(1), &vec![]);
    let _rx_bob = connect(&server, addr(2), "bob");
    group_invite_request(
        &vec!["INVITE".to_string(), "bob".to_string()],
        &server,
        addr(1),
    );

    let result = group_join_request(&server, addr(2));

    assert_success_contains(&result, "group");
    assert_eq!(
        rx_alice
            .try_recv()
            .expect("alice should be notified of the join"),
        Message::Event(EventType::GROUP_JOIN {
            player_name: "bob".to_string(),
        })
    );
}

// GROUP LIST TESTS

#[test]
fn group_list_when_not_in_group_returns_not_in_group() {
    let server = test_server();
    connect(&server, addr(1), "alice");
    let result = group_list_request(&server, addr(1));
    assert_eq!(result, err(ErrorCode::NOT_IN_GROUP));
}

#[test]
fn group_list_returns_members() {
    let server = test_server();
    group_with(&server, &[(addr(1), "alice"), (addr(2), "bob")]);
    let result = group_list_request(&server, addr(1));
    assert_success_contains(&result, "alice");
    assert_success_contains(&result, "bob");
}

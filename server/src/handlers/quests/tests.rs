use super::*;
use crate::test_utils::{addr, assert_success_contains, connect, err, ok_data, populated_server};

#[test]
fn quests_without_connection_returns_invalid_command() {
    let server = populated_server();
    assert_eq!(
        quests_request(&server, addr(1)),
        err(ErrorCode::INVALID_COMMAND)
    );
}

#[test]
fn quests_when_none_returns_empty_array() {
    let server = populated_server();
    connect(&server, addr(1), "alice");
    assert_eq!(quests_request(&server, addr(1)), ok_data("[]"));
}

#[test]
fn quests_in_progress_are_listed_as_active_with_progress() {
    let server = populated_server();
    connect(&server, addr(1), "alice");
    {
        let mut binding = server.lock().unwrap();
        let player = binding.get_player_mut(addr(1)).unwrap();
        player
            .quests_in_progress
            .insert("quest.fetch".to_string(), 0);
    }

    let result = quests_request(&server, addr(1));
    assert_success_contains(&result, "quest.fetch");
    assert_success_contains(&result, "\"status\":\"active\"");
    // quest.fetch a 1 goal, step 0 -> "0/1"
    assert_success_contains(&result, "\"progress\":\"0/1\"");
}

#[test]
fn finished_quests_are_listed_as_completed() {
    let server = populated_server();
    connect(&server, addr(1), "alice");
    {
        let mut binding = server.lock().unwrap();
        let player = binding.get_player_mut(addr(1)).unwrap();
        player.finished_quest.insert("quest.fetch".to_string());
    }

    let result = quests_request(&server, addr(1));
    assert_success_contains(&result, "quest.fetch");
    assert_success_contains(&result, "\"status\":\"completed\"");
}

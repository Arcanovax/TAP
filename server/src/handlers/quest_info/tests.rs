use super::*;
use crate::test_utils::{assert_success_contains, err, populated_server};

#[test]
fn quest_info_with_wrong_args_returns_invalid_args() {
    let server = populated_server();
    // 0 argument
    assert_eq!(
        quest_info_request(&server, &[]),
        err(ErrorCode::INVALID_ARGS)
    );
    // 2 arguments
    assert_eq!(
        quest_info_request(
            &server,
            &["quest.fetch".to_string(), "extra".to_string()],
        ),
        err(ErrorCode::INVALID_ARGS)
    );
}

#[test]
fn quest_info_unknown_quest_returns_no_quest_available() {
    let server = populated_server();
    assert_eq!(
        quest_info_request(&server, &["quest.unknown".to_string()]),
        err(ErrorCode::NO_QUEST_AVAILABLE)
    );
}

#[test]
fn quest_info_returns_the_quest_details() {
    let server = populated_server();
    let result = quest_info_request(&server, &["quest.fetch".to_string()]);
    // cf. test_world : quest.fetch = « Fetch a sword », reward « gold »,
    // goal Collect 1 sword. Le JSON sérialisé doit porter ces informations.
    assert_success_contains(&result, "quest.fetch");
    assert_success_contains(&result, "Fetch a sword");
    assert_success_contains(&result, "Collect");
}

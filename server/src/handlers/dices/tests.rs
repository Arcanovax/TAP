use super::*;
use crate::test_utils::{addr, connect, err, give_gold, test_server};

#[test]
fn dices_not_connected_returns_invalid_command() {
    let server = test_server();

    assert_eq!(
        dices_request(
            &server,
            addr(1),
            &vec!["1".to_string()],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0].to_vec()
        ),
        err(ErrorCode::INVALID_COMMAND)
    );
}

#[test]
fn dices_no_gold_returns_not_enough_gold() {
    let server = test_server();
    connect(&server, addr(1), "alice");
    give_gold(&server, addr(1), 0);

    assert_eq!(
        dices_request(
            &server,
            addr(1),
            &vec!["1".to_string()],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0].to_vec()
        ),
        err(ErrorCode::NOT_ENOUGH_GOLD)
    );
}

#[test]
fn dices_invalid_number_args_returns_invalid_args() {
    let server = test_server();
    connect(&server, addr(1), "alice");
    give_gold(&server, addr(1), 10);

    assert_eq!(
        dices_request(
            &server,
            addr(1),
            &vec![],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0].to_vec()
        ),
        err(ErrorCode::INVALID_ARGS)
    );
    assert_eq!(
        dices_request(
            &server,
            addr(1),
            &vec![
                "1".to_string(),
                "2".to_string(),
                "3".to_string(),
                "4".to_string(),
                "5".to_string(),
                "6".to_string(),
                "7".to_string(),
                "8".to_string(),
                "9".to_string(),
                "10".to_string(),
                "11".to_string(),
            ],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0].to_vec()
        ),
        err(ErrorCode::INVALID_ARGS)
    );
}

#[test]
fn dices_invalid_arg_type_returns_invalid_args() {
    let server = test_server();
    connect(&server, addr(1), "alice");
    give_gold(&server, addr(1), 10);

    assert_eq!(
        dices_request(
            &server,
            addr(1),
            &vec!["test".to_string()],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0].to_vec()
        ),
        err(ErrorCode::INVALID_ARGS)
    );
}

#[test]
fn dices_invalid_arg_range_returns_invalid_args() {
    let server = test_server();
    connect(&server, addr(1), "alice");
    give_gold(&server, addr(1), 10);

    assert_eq!(
        dices_request(
            &server,
            addr(1),
            &vec!["0".to_string()],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0].to_vec()
        ),
        err(ErrorCode::INVALID_ARGS)
    );
    assert_eq!(
        dices_request(
            &server,
            addr(1),
            &vec!["11".to_string()],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0].to_vec()
        ),
        err(ErrorCode::INVALID_ARGS)
    );
}

#[test]
fn dices_with_losing_roll_returns_game_loose() {
    let server = test_server();
    connect(&server, addr(1), "alice");
    give_gold(&server, addr(1), 10);

    assert_eq!(
        dices_request(
            &server,
            addr(1),
            &vec![
                "1".to_string(),
                "2".to_string(),
                "3".to_string(),
                "4".to_string(),
                "5".to_string(),
                "6".to_string(),
                "7".to_string(),
                "8".to_string(),
                "9".to_string(),
                "10".to_string(),
            ],
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1].to_vec()
        ),
        Message::Response {
            error: ErrorCode::GAME_LOSE,
            payload: Payload::Text("1 1 1 1 1 1 1 1 1 1".to_string()),
        }
    );
}

#[test]
fn dices_with_winning_roll_returns_success_with_gains() {
    let server = test_server();
    connect(&server, addr(1), "alice");
    give_gold(&server, addr(1), 10);

    assert_eq!(
        dices_request(
            &server,
            addr(1),
            &vec![
                "1".to_string(),
                "2".to_string(),
                "2".to_string(),
                "2".to_string(),
            ],
            [1, 1, 1, 2, 2, 1, 1, 1, 1, 2].to_vec()
        ),
        Message::Response {
        error: ErrorCode::SUCCESS,
        payload: Payload::Pair(HashMap::from([
			("gold".to_string(), "20".to_string()),
			("draw".to_string(), "1//1//1//2//2//1//1//1//1//2".to_string())])),
    	}
    );
}

#[test]
fn dices_with_bad_roll_returns_game_lose() {
    let server = test_server();
    connect(&server, addr(1), "alice");
    give_gold(&server, addr(1), 10);

    assert_eq!(
        dices_request(
            &server,
            addr(1),
            &vec![
                "1".to_string(),
                "2".to_string(),
                "2".to_string(),
                "2".to_string(),
            ],
            [1, 1, 1, 2, 2, 1, 1, 1, 1, 1].to_vec()
        ),
        Message::Response {
            error: ErrorCode::GAME_LOSE,
            payload: Payload::Text("1 1 1 2 2 1 1 1 1 1".to_string()),
        }
    );
}

#[test]
fn dices_with_wrong_location_returns_forbidden_action() {
    let server = test_server();
    connect(&server, addr(1), "alice");
    server
        .lock()
        .unwrap()
        .get_player_mut(addr(1))
        .unwrap()
        .location = "room.not_gambling_room".to_string();

    assert_eq!(
        dices_request(
            &server,
            addr(1),
            &vec![
                "1".to_string(),
                "2".to_string(),
                "2".to_string(),
                "2".to_string(),
            ],
            [1, 1, 1, 2, 2, 1, 1, 1, 1, 2].to_vec()
        ),
        err(ErrorCode::FORBIDDEN_ACTION)
    );
}

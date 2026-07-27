//! Tests de `slot_machine_request` (extension gambling : `SLOT_MACHINE`).
//!
//! L'aléatoire est injecté via la closure `roll` (`None` = perdu,
//! `Some(item)` = gagné), ce qui rend chaque cas déterministe.
//! Le handler ne dépend pas du monde : un `test_server()` (monde vide) suffit.

use super::*;
use crate::structures::enums::error::ErrorCode;
use crate::test_utils::{addr, connect, err, give_gold, ok_pair, test_server};

#[test]
fn slot_machine_without_connection_returns_invalid_command() {
    let server = test_server();
    // roll ne doit même pas être consulté quand le joueur n'est pas connecté.
    assert_eq!(
        slot_machine_request(&server, addr(1), || Some("item.cherry".to_string())),
        err(ErrorCode::INVALID_COMMAND)
    );
}

#[test]
fn slot_machine_with_insufficient_gold_returns_not_enough_gold() {
    let server = test_server();
    connect(&server, addr(1), "alice");
    give_gold(&server, addr(1), 4); // < 5

    // Même avec un tirage gagnant, le manque d'or doit court-circuiter.
    assert_eq!(
        slot_machine_request(&server, addr(1), || Some("item.cherry".to_string())),
        err(ErrorCode::NOT_ENOUGH_GOLD)
    );
}

#[test]
fn slot_machine_with_losing_roll_returns_game_lose() {
    let server = test_server();
    connect(&server, addr(1), "alice");
    give_gold(&server, addr(1), 10);

    assert_eq!(
        slot_machine_request(&server, addr(1), || None),
        err(ErrorCode::GAME_LOSE)
    );
}

#[test]
fn slot_machine_with_winning_roll_returns_success_with_item() {
    let server = test_server();
    connect(&server, addr(1), "alice");
    give_gold(&server, addr(1), 5); // borne basse : 5 suffit (condition = gold < 5)

    assert_eq!(
        slot_machine_request(&server, addr(1), || Some("item.cherry".to_string())),
        ok_pair(&[("item", "item.cherry")])
    );
}

#[test]
fn slot_machine_with_wrong_location_returns_forbidden_action() {
    let server = test_server();
    connect(&server, addr(1), "alice");
    server
        .lock()
        .unwrap()
        .get_player_mut(addr(1))
        .unwrap()
        .location = "room.not_gambling_room".to_string();

    assert_eq!(
        slot_machine_request(&server, addr(1), || None),
        err(ErrorCode::FORBIDDEN_ACTION)
    );
}

//! Tests de `buy_request`.
//!
//! Contrat encodé (les implémentations sont encore des stubs : ces tests sont la
//! spec à satisfaire — voir aussi `sell/tests.rs`) :
//! - Grammaire : `BUY <marchand> <item...> [amount]`. `<marchand>` = nom ou id
//!   (résolu via `name_to_ref`). `amount` = dernier arg s'il parse en `u32` >= 1,
//!   sinon 1 ; le reste (entre marchand et amount) forme le nom d'item.
//! - Le marchand doit être présent dans la pièce du joueur.
//! - Transaction « joueur seul » : seuls `player.gold` et `player.inventory`
//!   changent (le marchand est un puits/source infini).
//! - Précédence : pas connecté -> INVALID_COMMAND ; marchand ou item manquant /
//!   amount nul -> INVALID_ARGS ; marchand absent de la pièce -> NPC_NOT_FOUND ;
//!   item hors catalogue du marchand -> ITEM_NOT_FOUND ; or insuffisant ->
//!   NOT_ENOUGH_GOLD.

use super::*;
use crate::test_utils::{addr, connect, err, give_gold, ok_pair, populated_server};

// --- Cas d'erreur ---

#[test]
fn buy_without_connection_returns_invalid_command() {
    let server = populated_server();
    let result = buy_request(
        &vec!["merchant".to_string(), "sword".to_string()],
        &server,
        addr(1),
    );
    assert_eq!(result, err(ErrorCode::INVALID_COMMAND));
}

#[test]
fn buy_without_args_returns_invalid_args() {
    let server = populated_server();
    connect(&server, addr(1), "alice");
    let result = buy_request(&vec![], &server, addr(1));
    assert_eq!(result, err(ErrorCode::INVALID_ARGS));
}

#[test]
fn buy_with_merchant_but_no_item_returns_invalid_args() {
    let server = populated_server();
    connect(&server, addr(1), "alice");
    let result = buy_request(&vec!["merchant".to_string()], &server, addr(1));
    assert_eq!(result, err(ErrorCode::INVALID_ARGS));
}

#[test]
fn buy_with_zero_amount_returns_invalid_args() {
    let server = populated_server();
    connect(&server, addr(1), "alice");
    let result = buy_request(
        &vec!["merchant".to_string(), "sword".to_string(), "0".to_string()],
        &server,
        addr(1),
    );
    assert_eq!(result, err(ErrorCode::INVALID_ARGS));
}

#[test]
fn buy_without_merchant_in_room_returns_npc_not_found() {
    let server = populated_server();
    connect(&server, addr(1), "alice");
    // room.market ne contient aucun NPC marchand
    server
        .lock()
        .unwrap()
        .get_player_mut(addr(1))
        .unwrap()
        .location = "room.market".to_string();
    let result = buy_request(
        &vec!["merchant".to_string(), "sword".to_string()],
        &server,
        addr(1),
    );
    assert_eq!(result, err(ErrorCode::NPC_NOT_FOUND));
}

#[test]
fn buy_item_not_sold_returns_item_not_found() {
    let server = populated_server();
    connect(&server, addr(1), "alice");
    give_gold(&server, addr(1), 100);
    let result = buy_request(
        &vec!["merchant".to_string(), "shield".to_string()],
        &server,
        addr(1),
    );
    assert_eq!(result, err(ErrorCode::ITEM_NOT_FOUND));
}

#[test]
fn buy_with_insufficient_gold_returns_not_enough_gold() {
    let server = populated_server();
    connect(&server, addr(1), "alice");
    give_gold(&server, addr(1), 5); // sword vaut 10
    let result = buy_request(
        &vec!["merchant".to_string(), "sword".to_string()],
        &server,
        addr(1),
    );
    assert_eq!(result, err(ErrorCode::NOT_ENOUGH_GOLD));

    // l'or n'a pas bougé, rien n'est entré dans l'inventaire
    let guard = server.lock().unwrap();
    let player = guard.get_player(addr(1)).unwrap();
    assert_eq!(player.gold, 5);
    assert_eq!(player.inventory.get("sword"), None);
}

// --- Cas nominaux ---

#[test]
fn buy_existing_item_adds_to_inventory_and_debits_gold() {
    let server = populated_server();
    connect(&server, addr(1), "alice");
    give_gold(&server, addr(1), 100);
    let result = buy_request(
        &vec!["merchant".to_string(), "sword".to_string()],
        &server,
        addr(1),
    );
    assert_eq!(result, ok_pair(&[("bought", "sword"), ("amount", "1"), ("gold", "10")]));

    let guard = server.lock().unwrap();
    let player = guard.get_player(addr(1)).unwrap();
    assert_eq!(player.gold, 90); // 100 - 10
    assert_eq!(player.inventory.get("sword"), Some(&1));
}

#[test]
fn buy_with_amount_debits_and_stacks() {
    let server = populated_server();
    connect(&server, addr(1), "alice");
    give_gold(&server, addr(1), 100);
    let result = buy_request(
        &vec!["merchant".to_string(), "sword".to_string(), "3".to_string()],
        &server,
        addr(1),
    );
    assert_eq!(result, ok_pair(&[("bought", "sword"), ("amount", "3"), ("gold", "30")]));

    let guard = server.lock().unwrap();
    let player = guard.get_player(addr(1)).unwrap();
    assert_eq!(player.gold, 70); // 100 - 3 * 10
    assert_eq!(player.inventory.get("sword"), Some(&3));
}

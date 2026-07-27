//! Tests de `sell_request`.
//!
//! Contrat encodé (implémentation encore stub : ces tests sont la spec à
//! satisfaire — voir aussi `buy/tests.rs`) :
//! - Grammaire : `SELL <marchand> <item...> [amount]`. `<marchand>` = nom ou id
//!   (résolu via `name_to_ref`). `amount` = dernier arg s'il parse en `u32` >= 1,
//!   sinon 1 ; le reste (entre marchand et amount) forme le nom d'item.
//! - Le marchand doit être présent dans la pièce du joueur.
//! - Transaction « joueur seul » : seuls `player.gold` et `player.inventory`
//!   changent. Le prix de revente = `Item.price` (pas de décote).
//! - Précédence : pas connecté -> INVALID_COMMAND ; marchand ou item manquant /
//!   amount nul -> INVALID_ARGS ; marchand absent de la pièce -> NPC_NOT_FOUND ;
//!   item absent de l'inventaire ou quantité insuffisante -> ITEM_NOT_IN_INVENTORY.

use super::*;
use crate::test_utils::{addr, connect, err, give_item, ok_pair, populated_server};

// --- Cas d'erreur ---

#[test]
fn sell_without_connection_returns_invalid_command() {
    let server = populated_server();
    let result = sell_request(
        &["merchant".to_string(), "sword".to_string()],
        &server,
        addr(1),
    );
    assert_eq!(result, err(ErrorCode::INVALID_COMMAND));
}

#[test]
fn sell_without_args_returns_invalid_args() {
    let server = populated_server();
    connect(&server, addr(1), "alice");
    let result = sell_request(&[], &server, addr(1));
    assert_eq!(result, err(ErrorCode::INVALID_ARGS));
}

#[test]
fn sell_with_merchant_but_no_item_returns_invalid_args() {
    let server = populated_server();
    connect(&server, addr(1), "alice");
    let result = sell_request(&["merchant".to_string()], &server, addr(1));
    assert_eq!(result, err(ErrorCode::INVALID_ARGS));
}

#[test]
fn sell_with_zero_amount_returns_invalid_args() {
    let server = populated_server();
    connect(&server, addr(1), "alice");
    give_item(&server, addr(1), "sword", 1);
    let result = sell_request(
        &["merchant".to_string(), "sword".to_string(), "0".to_string()],
        &server,
        addr(1),
    );
    assert_eq!(result, err(ErrorCode::INVALID_ARGS));
}

#[test]
fn sell_without_merchant_in_room_returns_npc_not_found() {
    let server = populated_server();
    connect(&server, addr(1), "alice");
    give_item(&server, addr(1), "sword", 1);
    // room.market ne contient aucun NPC marchand
    server
        .lock()
        .unwrap()
        .get_player_mut(addr(1))
        .unwrap()
        .location = "room.market".to_string();
    let result = sell_request(
        &["merchant".to_string(), "sword".to_string()],
        &server,
        addr(1),
    );
    assert_eq!(result, err(ErrorCode::NPC_NOT_FOUND));
}

#[test]
fn sell_to_non_merchant_returns_invalid_args() {
    // "guard" est bien présent dans city_square mais c'est un Citizen, pas un Merchant.
    // Mirroir du comportement de buy (cf. buy.rs : NPC présent mais pas marchand -> INVALID_ARGS).
    let server = populated_server();
    connect(&server, addr(1), "alice");
    give_item(&server, addr(1), "sword", 1);
    let result = sell_request(
        &["guard".to_string(), "sword".to_string()],
        &server,
        addr(1),
    );
    assert_eq!(result, err(ErrorCode::INVALID_ARGS));
}

#[test]
fn sell_item_not_in_inventory_returns_item_not_in_inventory() {
    let server = populated_server();
    connect(&server, addr(1), "alice");
    let result = sell_request(
        &["merchant".to_string(), "sword".to_string()],
        &server,
        addr(1),
    );
    assert_eq!(result, err(ErrorCode::ITEM_NOT_IN_INVENTORY));
}

#[test]
fn sell_more_than_owned_returns_item_not_in_inventory() {
    let server = populated_server();
    connect(&server, addr(1), "alice");
    give_item(&server, addr(1), "sword", 1);
    let result = sell_request(
        &["merchant".to_string(), "sword".to_string(), "2".to_string()],
        &server,
        addr(1),
    );
    assert_eq!(result, err(ErrorCode::ITEM_NOT_IN_INVENTORY));
}

// --- Cas nominaux ---

#[test]
fn sell_existing_item_removes_it_and_credits_gold() {
    let server = populated_server();
    connect(&server, addr(1), "alice");
    give_item(&server, addr(1), "sword", 1);
    let result = sell_request(
        &["merchant".to_string(), "sword".to_string()],
        &server,
        addr(1),
    );
    assert_eq!(
        result,
        ok_pair(&[("sold", "sword"), ("gold", "10"), ("amount", "1")])
    );

    let guard = server.lock().unwrap();
    let player = guard.get_player(addr(1)).unwrap();
    assert_eq!(player.gold, 60); // 50 + 10
    // dernier exemplaire vendu -> la clé disparaît de l'inventaire
    assert_eq!(player.inventory.get("sword"), None);
}

#[test]
fn sell_with_amount_credits_and_decrements() {
    let server = populated_server();
    connect(&server, addr(1), "alice");
    give_item(&server, addr(1), "sword", 3);
    let result = sell_request(
        &["merchant".to_string(), "sword".to_string(), "2".to_string()],
        &server,
        addr(1),
    );
    assert_eq!(
        result,
        ok_pair(&[("gold", "20"), ("sold", "sword"), ("amount", "2"),])
    );

    let guard = server.lock().unwrap();
    let player = guard.get_player(addr(1)).unwrap();
    assert_eq!(player.gold, 70); // 50 + 2 * 10
    assert_eq!(player.inventory.get("sword"), Some(&1)); // 3 - 2
}

#[test]
fn sell_with_quest_item_returns_forbidden_action() {
    let server = populated_server();
    connect(&server, addr(1), "alice");
    give_item(&server, addr(1), "quest_item", 1);
    let result = sell_request(
        &["merchant".to_string(), "quest_item".to_string()],
        &server,
        addr(1),
    );

    assert_eq!(result, err(ErrorCode::FORBIDDEN_ACTION));
}

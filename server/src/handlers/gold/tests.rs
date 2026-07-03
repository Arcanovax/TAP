//! Tests de `gold_request` (extension économie : `GOLD -> OK gold=<n>`).
//!
//! Le solde est exposé via une commande d'extension dédiée (Pair `gold=…`),
//! et non dans STATUS/INVENTORY qui restent des réponses RFC-pures.
//! `gold` ne dépend pas du monde : un `test_server()` (monde vide) suffit.

use super::*;
use crate::test_utils::{addr, connect, err, give_gold, ok_pair, test_server};

#[test]
fn gold_without_connection_returns_invalid_command() {
    let server = test_server();
    assert_eq!(
        gold_request(&server, addr(1)),
        err(ErrorCode::INVALID_COMMAND)
    );
}

#[test]
fn gold_defaults_to_zero_for_new_player() {
    let server = test_server();
    connect(&server, addr(1), "alice");
    assert_eq!(gold_request(&server, addr(1)), ok_pair(&[("gold", "50")]));
}

#[test]
fn gold_reflects_player_balance() {
    let server = test_server();
    connect(&server, addr(1), "alice");
    give_gold(&server, addr(1), 42);
    assert_eq!(gold_request(&server, addr(1)), ok_pair(&[("gold", "42")]));
}

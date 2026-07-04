use super::*;
use crate::test_utils::{
    addr, assert_success_contains, connect, connect_in_dungeon, dg_npc, dungeon_server,
    populated_server, response_error, test_server,
};

#[test]
fn fight_with_wrong_args_returns_invalid_args() {
    let server = populated_server();
    connect(&server, addr(1), "alice");
    let result = fight_request(addr(1), &vec![], &server);
    assert_eq!(response_error(&result), &ErrorCode::INVALID_ARGS);
}

#[test]
fn fight_without_connection_returns_player_not_found() {
    let server = populated_server();
    let result = fight_request(addr(1), &vec!["goblin".to_string()], &server);
    assert_eq!(response_error(&result), &ErrorCode::INVALID_COMMAND);
}

#[test]
fn fight_when_room_missing_returns_room_not_found() {
    let server = test_server(); // monde vide
    connect(&server, addr(1), "alice");
    let result = fight_request(addr(1), &vec!["goblin".to_string()], &server);
    assert_eq!(response_error(&result), &ErrorCode::ROOM_NOT_FOUND);
}

#[test]
fn fight_target_not_in_room_returns_npc_not_found() {
    let server = populated_server();
    connect(&server, addr(1), "alice");
    let result = fight_request(addr(1), &vec!["dragon".to_string()], &server);
    assert_eq!(response_error(&result), &ErrorCode::NPC_NOT_FOUND);
}

#[test]
fn fight_non_hostile_target_returns_npc_not_hostile() {
    let server = populated_server();
    connect(&server, addr(1), "alice");
    // "guard" est un Citizen présent dans la room
    let result = fight_request(addr(1), &vec!["guard".to_string()], &server);
    assert_eq!(response_error(&result), &ErrorCode::NPC_NOT_HOSTILE);
}

#[test]
fn fight_engages_enemy_and_sets_in_fight_state() {
    let server = populated_server();
    connect(&server, addr(1), "alice");
    let result = fight_request(addr(1), &vec!["goblin".to_string()], &server);

    assert_eq!(response_error(&result), &ErrorCode::SUCCESS);

    let guard = server.lock().unwrap();
    let player = guard.get_player(addr(1)).unwrap();
    assert!(matches!(player.status, State::InFight { .. }));
}

// --- Variante « donjon » : cible absente de la salle de donjon.

#[test]
fn fight_target_not_in_dungeon_room_returns_npc_not_found() {
    let server = dungeon_server();
    connect_in_dungeon(&server, addr(1), "alice");
    let result = fight_request(addr(1), &vec!["dragon".to_string()], &server);
    assert_eq!(response_error(&result), &ErrorCode::NPC_NOT_FOUND);
}

// --- Test ROUGE : révèle un vrai bug.
// Un joueur devrait pouvoir engager l'ennemi de sa salle de donjon (par son id).
// `is_he_there` passe (l'id est dans la salle), mais `fight_request` fait ensuite
// `world.npcs[&args[0]]` en dur (state.rs -> world uniquement) et PANIQUE, car l'id
// de donjon n'existe pas dans `world.npcs`. Correctif attendu : passer par `resolve_npc`.
#[test]
fn fight_dungeon_enemy_by_id_engages() {
    let server = dungeon_server();
    connect_in_dungeon(&server, addr(1), "alice");
    let result = fight_request(addr(1), &vec![dg_npc(0)], &server);
    assert_eq!(response_error(&result), &ErrorCode::SUCCESS);
}

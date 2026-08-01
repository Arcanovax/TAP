use super::*;
use crate::test_utils::{
    addr, connect, connect_in_dungeon, dg_item, dg_room, dungeon_server, err, ok_pair,
    populated_server,
};

#[test]
fn take_without_connection_returns_invalid_command() {
    let server = populated_server();
    let result = take_request(&server, addr(1), &["sword".to_string()]);
    assert_eq!(result, err(ErrorCode::INVALID_COMMAND));
}

#[test]
fn take_without_args_returns_invalid_args() {
    let server = populated_server();
    connect(&server, addr(1), "alice");
    let result = take_request(&server, addr(1), &[]);
    assert_eq!(result, err(ErrorCode::INVALID_ARGS));
}

#[test]
fn take_existing_item_moves_it_to_inventory() {
    let server = populated_server();
    connect(&server, addr(1), "alice");
    let result = take_request(&server, addr(1), &["sword".to_string()]);
    assert_eq!(result, ok_pair(&[("taken", "sword")]));
    // l'item est bien passé dans l'inventaire
    let guard = server.lock().unwrap();
    let player = guard.get_player(addr(1)).unwrap();
    assert_eq!(player.inventory.get("sword"), Some(&1));
}

#[test]
fn take_unknown_item_returns_item_not_found() {
    let server = populated_server();
    connect(&server, addr(1), "alice");
    let result = take_request(&server, addr(1), &["shield".to_string()]);
    assert_eq!(result, err(ErrorCode::ITEM_NOT_FOUND));
}

// --- Variantes « donjon » : le joueur est dans une salle de donjon.
// Les entités sont référencées par leur id de donjon (`dg_item`), le nom ne
// routant pas en donjon (aucun `name_to_ref` côté donjon).

#[test]
fn take_existing_item_in_dungeon_moves_it_to_inventory() {
    let server = dungeon_server();
    connect_in_dungeon(&server, addr(1), "alice");
    let result = take_request(&server, addr(1), &[dg_item(0)]);
    assert_eq!(result, ok_pair(&[("taken", dg_item(0).as_str())]));
    let guard = server.lock().unwrap();
    let player = guard.get_player(addr(1)).unwrap();
    assert_eq!(player.inventory.get(&dg_item(0)), Some(&1));
}

#[test]
fn take_unknown_item_in_dungeon_returns_item_not_found() {
    let server = dungeon_server();
    connect_in_dungeon(&server, addr(1), "alice");
    // dg_item(9) n'est pas dans la salle d'entrée.
    let result = take_request(&server, addr(1), &[dg_item(9)]);
    assert_eq!(result, err(ErrorCode::ITEM_NOT_FOUND));
}

#[test]
fn take_removes_item_from_dungeon_room() {
    let server = dungeon_server();
    connect_in_dungeon(&server, addr(1), "alice");
    take_request(&server, addr(1), &[dg_item(0)]);
    // l'item a bien quitté la salle du donjon
    let guard = server.lock().unwrap();
    let room = guard.resolve_room(&dg_room(0)).unwrap();
    assert!(room.items.iter().all(|owned| owned.item != dg_item(0)));
}

#[test]
fn take_in_empty_dungeon_room_returns_item_not_found() {
    let server = dungeon_server();
    connect_in_dungeon(&server, addr(1), "alice");
    // dg_room(1) est vide : on y place le joueur puis on tente de ramasser l'item de l'entrée
    server
        .lock()
        .unwrap()
        .get_player_mut(addr(1))
        .unwrap()
        .location = dg_room(1);
    let result = take_request(&server, addr(1), &[dg_item(0)]);
    assert_eq!(result, err(ErrorCode::ITEM_NOT_FOUND));
}

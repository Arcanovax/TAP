use super::*;
use crate::handlers::take::take_request;
use crate::test_utils::{
    addr, connect, connect_in_dungeon, dg_item, dg_room, dungeon_server, err, ok_pair,
    populated_server,
};

#[test]
fn drop_without_connection_returns_invalid_command() {
    let server = populated_server();
    let result = drop_request(&server, addr(1), &["sword".to_string()]);
    assert_eq!(result, err(ErrorCode::INVALID_COMMAND));
}

#[test]
fn drop_without_args_returns_invalid_args() {
    let server = populated_server();
    connect(&server, addr(1), "alice");
    let result = drop_request(&server, addr(1), &[]);
    assert_eq!(result, err(ErrorCode::INVALID_ARGS));
}

#[test]
fn drop_item_in_inventory_returns_it() {
    let server = populated_server();
    connect(&server, addr(1), "alice");
    // d'abord ramasser pour avoir l'item en inventaire
    take_request(&server, addr(1), &["sword".to_string()]);

    let result = drop_request(&server, addr(1), &["sword".to_string()]);
    assert_eq!(result, ok_pair(&[("dropped", "sword")]));
    // l'item n'est plus dans l'inventaire
    let guard = server.lock().unwrap();
    assert!(
        !guard
            .get_player(addr(1))
            .unwrap()
            .inventory
            .contains_key("sword")
    );
}

#[test]
fn drop_item_not_in_inventory_returns_item_not_in_inventory() {
    let server = populated_server();
    connect(&server, addr(1), "alice");
    let result = drop_request(&server, addr(1), &["sword".to_string()]);
    assert_eq!(result, err(ErrorCode::ITEM_NOT_IN_INVENTORY));
}

// --- Variante « donjon » : entités référencées par id de donjon.

#[test]
fn drop_item_in_dungeon_returns_it() {
    let server = dungeon_server();
    connect_in_dungeon(&server, addr(1), "alice");
    // ramasser d'abord pour avoir l'item en inventaire
    take_request(&server, addr(1), &[dg_item(0)]);

    let result = drop_request(&server, addr(1), &[dg_item(0)]);
    assert_eq!(result, ok_pair(&[("dropped", dg_item(0).as_str())]));
    let guard = server.lock().unwrap();
    assert!(
        !guard
            .get_player(addr(1))
            .unwrap()
            .inventory
            .contains_key(&dg_item(0))
    );
}

#[test]
fn drop_puts_item_back_into_dungeon_room() {
    let server = dungeon_server();
    connect_in_dungeon(&server, addr(1), "alice");
    take_request(&server, addr(1), &[dg_item(0)]);
    drop_request(&server, addr(1), &[dg_item(0)]);
    // l'item est bien redéposé dans la salle du donjon
    let guard = server.lock().unwrap();
    let room = guard.resolve_room(&dg_room(0)).unwrap();
    assert!(room.items.iter().any(|owned| owned.item == dg_item(0)));
}

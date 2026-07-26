use super::*;
use crate::{
    structures::{enums::item_kind::ItemKind, item::Item},
    test_utils::{
        addr, assert_success_contains, connect, connect_in_dungeon, dg_item, dungeon_server,
        populated_server,
    },
};

#[test]
fn item_right_name_returns_success() {
    let server = populated_server();
    let res = item_request(&server, &[String::from("sword")]);
    assert_eq!(
        res,
        Message::Response {
            error: ErrorCode::SUCCESS,
            payload: Payload::Json(
                serde_json::to_value(Item {
                    name: String::from("sword"),
                    price: 10,
                    kind: ItemKind::Miscellaneous
                })
                .unwrap()
            )
        }
    )
}

#[test]
fn npc_wrong_name_returns_npc_not_found() {
    let server = populated_server();
    let res = item_request(&server, &[String::from("no_item")]);
    assert_eq!(
        res,
        Message::Response {
            error: ErrorCode::ITEM_NOT_FOUND,
            payload: Payload::Empty
        }
    )
}

// --- Liste ITEMS : contenu du donjon si le joueur y est, sinon celui du world.
// (Suppose la signature `items_request(&server, peer_addr)`.)

#[test]
fn items_outside_dungeon_returns_world_items() {
    let server = populated_server();
    connect(&server, addr(1), "alice");
    let res = items_request(&server, addr(1));
    // "sword" est la clé d'item du world
    assert_success_contains(&res, "sword");
}

#[test]
fn items_in_dungeon_returns_dungeon_items() {
    let server = dungeon_server();
    connect_in_dungeon(&server, addr(1), "alice");
    let res = items_request(&server, addr(1));
    assert_success_contains(&res, &dg_item(0));
}

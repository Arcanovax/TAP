use super::*;
use crate::{
    structures::{enums::item_kind::ItemKind, item::Item},
    test_utils::{dg_item, dungeon_server, populated_server},
};

#[test]
fn item_right_name_returns_success() {
    let server = populated_server();
    let res = item_request(&server, &vec![String::from("sword")]);
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
    let res = item_request(&server, &vec![String::from("no_item")]);
    assert_eq!(
        res,
        Message::Response {
            error: ErrorCode::ITEM_NOT_FOUND,
            payload: Payload::Empty
        }
    )
}

// --- Variante « donjon » : item référencé par son id de donjon.

#[test]
fn item_in_dungeon_by_id_returns_success() {
    let server = dungeon_server();
    let res = item_request(&server, &vec![dg_item(0)]);
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

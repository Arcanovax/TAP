use super::*;
use crate::test_utils::{dg_npc, dungeon_server, populated_server};

#[test]
fn npc_right_name_returns_success() {
    let server = populated_server();
    let res = npc_request(&server, &vec![String::from("guard")]);
    assert_eq!(
        res,
        Message::Response {
            error: ErrorCode::SUCCESS,
            payload: Payload::Json(
                serde_json::to_value(NPCView {
                    name: "guard",
                    kind: NPCKindView::Citizen,
                    has_quest: true
                })
                .unwrap()
            )
        }
    )
}

#[test]
fn npc_wrong_name_returns_npc_not_found() {
    let server = populated_server();
    let res = npc_request(&server, &vec![String::from("nobody")]);
    assert_eq!(
        res,
        Message::Response {
            error: ErrorCode::NPC_NOT_FOUND,
            payload: Payload::Empty
        }
    )
}

// --- Variante « donjon » : npc (ennemi) référencé par son id de donjon.

#[test]
fn npc_in_dungeon_by_id_returns_success() {
    let server = dungeon_server();
    let res = npc_request(&server, &vec![dg_npc(0)]);
    assert_eq!(
        res,
        Message::Response {
            error: ErrorCode::SUCCESS,
            payload: Payload::Json(
                serde_json::to_value(NPCView {
                    name: "goblin",
                    kind: NPCKindView::Enemy {
                        hp: 30,
                        max_hp: 30,
                        defeated: false
                    },
                    has_quest: false
                })
                .unwrap()
            )
        }
    )
}

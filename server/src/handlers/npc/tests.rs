use super::*;
use crate::test_utils::{
    addr, assert_success_contains, connect, connect_in_dungeon, dg_npc, dungeon_server,
    populated_server,
};

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

// --- Liste NPCS : contenu du donjon si le joueur y est, sinon celui du world.
// (Suppose la signature `npcs_request(&server, peer_addr)`.)

#[test]
fn npcs_outside_dungeon_returns_world_npcs() {
    let server = populated_server();
    connect(&server, addr(1), "alice");
    let res = npcs_request(&server, addr(1));
    // "guard" n'existe que dans le world (le donjon ne contient que "goblin")
    assert_success_contains(&res, "guard");
}

#[test]
fn npcs_in_dungeon_returns_dungeon_npcs() {
    let server = dungeon_server();
    connect_in_dungeon(&server, addr(1), "alice");
    let res = npcs_request(&server, addr(1));
    assert_success_contains(&res, &dg_npc(0));
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

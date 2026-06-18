use super::*;
use crate::test_utils::populated_server;

#[test]
fn npc_right_name_returns_success() {
    let server = populated_server();
    let res = npc_request(&server, &vec![String::from("guard")]);
    assert_eq!(
        res,
        Message::Response {
            error: ErrorCode::SUCCESS,
            data: Some(
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
            data: None
        }
    )
}

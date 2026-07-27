use super::*;
use crate::structures::enums::error::ErrorCode;
use crate::test_utils::{
    addr, assert_success_contains, connect, connect_in_dungeon, dg_room, dungeon_server, err,
    populated_server,
};

// --- ROOM : détail d'une room par id (équivalent de npc_request) ---

#[test]
fn room_without_args_returns_invalid_args() {
    let server = populated_server();
    let res = room_request(&server, &[]);
    assert_eq!(res, err(ErrorCode::INVALID_ARGS));
}

#[test]
fn room_right_id_returns_success() {
    let server = populated_server();
    let res = room_request(&server, &[String::from("room.city_square")]);
    assert_success_contains(&res, "room.city_square");
}

#[test]
fn room_wrong_id_returns_room_not_found() {
    let server = populated_server();
    let res = room_request(&server, &[String::from("room.nowhere")]);
    assert_eq!(res, err(ErrorCode::ROOM_NOT_FOUND));
}

#[test]
fn room_in_dungeon_by_id_returns_success() {
    let server = dungeon_server();
    let res = room_request(&server, &[dg_room(0)]);
    assert_success_contains(&res, &dg_room(0));
}

// --- ROOMS : liste des rooms (contenu du donjon si le joueur y est, sinon le world) ---

#[test]
fn rooms_outside_dungeon_returns_world_rooms() {
    let server = populated_server();
    connect(&server, addr(1), "alice");
    let res = rooms_request(&server, addr(1));
    // "room.market" n'existe que dans le world (le donjon a ses propres rooms)
    assert_success_contains(&res, "room.market");
}

#[test]
fn rooms_in_dungeon_returns_dungeon_rooms() {
    let server = dungeon_server();
    connect_in_dungeon(&server, addr(1), "alice");
    let res = rooms_request(&server, addr(1));
    assert_success_contains(&res, &dg_room(0));
}

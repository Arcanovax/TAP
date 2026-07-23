use super::*;
use crate::test_utils::{
    addr, connect, connect_in_dungeon, dg_room, dungeon_server, err, ok_pair, populated_server,
    test_server,
};

#[test]
fn move_without_connection_returns_invalid_command() {
    let server = populated_server();
    let result = move_request(&server, addr(1), &vec!["North".to_string()]);
    assert_eq!(result, err(ErrorCode::INVALID_COMMAND));
}

#[test]
fn move_without_args_returns_invalid_args() {
    let server = populated_server();
    connect(&server, addr(1), "alice");
    let result = move_request(&server, addr(1), &vec![]);
    assert_eq!(result, err(ErrorCode::INVALID_ARGS));
}

#[test]
fn move_with_too_many_args_returns_invalid_args() {
    let server = populated_server();
    connect(&server, addr(1), "alice");
    let result = move_request(
        &server,
        addr(1),
        &vec!["North".to_string(), "South".to_string()],
    );
    assert_eq!(result, err(ErrorCode::INVALID_ARGS));
}

#[test]
fn move_with_invalid_direction_returns_invalid_args() {
    let server = populated_server();
    connect(&server, addr(1), "alice");
    let result = move_request(&server, addr(1), &vec!["Up".to_string()]);
    assert_eq!(result, err(ErrorCode::INVALID_ARGS));
}

#[test]
fn move_when_room_not_found_returns_room_not_found() {
    // test_server() a un monde vide : la location par défaut du joueur n'existe pas.
    let server = test_server();
    connect(&server, addr(1), "alice");
    let result = move_request(&server, addr(1), &vec!["North".to_string()]);
    assert_eq!(result, err(ErrorCode::ROOM_NOT_FOUND));
}

#[test]
fn move_without_matching_exit_returns_no_exit() {
    // city_square n'a qu'une sortie Nord : aller au Sud n'a pas de sortie.
    let server = populated_server();
    connect(&server, addr(1), "alice");
    let result = move_request(&server, addr(1), &vec!["South".to_string()]);
    assert_eq!(result, err(ErrorCode::NO_EXIT));
}

#[test]
fn move_through_valid_exit_succeeds_and_updates_location() {
    let server = populated_server();
    connect(&server, addr(1), "alice");
    let result = move_request(&server, addr(1), &vec!["North".to_string()]);
    assert_eq!(result, ok_pair(&[("room", "room.market")]));

    // la location du joueur a bien été mise à jour
    let guard = server.lock().unwrap();
    let player = guard.get_player(addr(1)).unwrap();
    assert_eq!(player.location, "room.market");
}

#[test]
fn move_direction_is_case_insensitive() {
    // FromStr<Exit> normalise en majuscules : "north" doit fonctionner.
    let server = populated_server();
    connect(&server, addr(1), "alice");
    let result = move_request(&server, addr(1), &vec!["north".to_string()]);
    assert_eq!(result, ok_pair(&[("room", "room.market")]));
}

#[test]
fn consecutive_moves_follow_exits_from_new_room() {
    let server = populated_server();
    connect(&server, addr(1), "alice");

    // city_square --North--> market
    let first = move_request(&server, addr(1), &vec!["North".to_string()]);
    assert_eq!(first, ok_pair(&[("room", "room.market")]));

    // depuis market : --South--> city_square (prouve qu'on repart de la nouvelle salle)
    let second = move_request(&server, addr(1), &vec!["South".to_string()]);
    assert_eq!(second, ok_pair(&[("room", "room.city_square")]));

    let guard = server.lock().unwrap();
    assert_eq!(
        guard.get_player(addr(1)).unwrap().location,
        "room.city_square"
    );
}

#[test]
fn move_notifies_departure_room_with_room_leave() {
    let server = populated_server();
    connect(&server, addr(1), "alice");
    let mut bob_rx = connect(&server, addr(2), "bob"); // bob reste dans city_square

    move_request(&server, addr(1), &vec!["North".to_string()]);

    let event = bob_rx
        .try_recv()
        .expect("bob aurait dû être notifié du départ d'alice");
    assert_eq!(
        event,
        Message::Event(EventType::ROOM_LEAVE {
            player_name: "alice".to_string(),
        })
    );
}

#[test]
fn move_notifies_arrival_room_with_room_join() {
    let server = populated_server();
    connect(&server, addr(1), "alice");
    let mut charlie_rx = connect(&server, addr(2), "charlie");

    // charlie va d'abord dans market, puis on ignore les events de SON déplacement
    move_request(&server, addr(2), &vec!["North".to_string()]);
    while charlie_rx.try_recv().is_ok() {}

    // alice arrive à son tour dans market
    move_request(&server, addr(1), &vec!["North".to_string()]);

    let event = charlie_rx
        .try_recv()
        .expect("charlie aurait dû être notifié de l'arrivée d'alice");
    assert_eq!(
        event,
        Message::Event(EventType::ROOM_JOIN {
            player_name: "alice".to_string(),
        })
    );
}

#[test]
fn move_does_not_notify_the_mover() {
    let server = populated_server();
    let mut alice_rx = connect(&server, addr(1), "alice");
    let _bob_rx = connect(&server, addr(2), "bob"); // témoin dans city_square

    move_request(&server, addr(1), &vec!["North".to_string()]);

    // bob est notifié (ROOM_LEAVE), mais alice ne se notifie pas elle-même
    assert!(
        alice_rx.try_recv().is_err(),
        "alice ne devrait recevoir aucun event pour son propre déplacement"
    );
}

// ---------------------------------------------------------------------------
// Variantes « donjon » : le joueur se déplace entre les salles du donjon.
// Entrée dg_room(0) --Nord--> dg_room(1) (et retour --Sud-->).
// ---------------------------------------------------------------------------

#[test]
fn move_without_matching_exit_in_dungeon_returns_no_exit() {
    // dg_room(0) n'a qu'une sortie Nord : le Sud n'a pas de sortie.
    let server = dungeon_server();
    connect_in_dungeon(&server, addr(1), "alice");
    let result = move_request(&server, addr(1), &vec!["South".to_string()]);
    assert_eq!(result, err(ErrorCode::NO_EXIT));
}

#[test]
fn move_through_valid_exit_in_dungeon_succeeds_and_updates_location() {
    let server = dungeon_server();
    connect_in_dungeon(&server, addr(1), "alice");
    let result = move_request(&server, addr(1), &vec!["North".to_string()]);
    assert_eq!(result, ok_pair(&[("room", dg_room(1).as_str())]));

    let guard = server.lock().unwrap();
    assert_eq!(guard.get_player(addr(1)).unwrap().location, dg_room(1));
}

#[test]
fn consecutive_moves_in_dungeon_follow_exits_from_new_room() {
    let server = dungeon_server();
    connect_in_dungeon(&server, addr(1), "alice");

    // entrée --Nord--> dg_room(1)
    let first = move_request(&server, addr(1), &vec!["North".to_string()]);
    assert_eq!(first, ok_pair(&[("room", dg_room(1).as_str())]));

    // depuis dg_room(1) : --Sud--> entrée
    let second = move_request(&server, addr(1), &vec!["South".to_string()]);
    assert_eq!(second, ok_pair(&[("room", dg_room(0).as_str())]));

    let guard = server.lock().unwrap();
    assert_eq!(guard.get_player(addr(1)).unwrap().location, dg_room(0));
}

#[test]
fn move_in_dungeon_notifies_departure_room_with_room_leave() {
    let server = dungeon_server();
    connect_in_dungeon(&server, addr(1), "alice");
    let mut bob_rx = connect_in_dungeon(&server, addr(2), "bob"); // bob reste dans l'entrée

    move_request(&server, addr(1), &vec!["North".to_string()]);

    let event = bob_rx
        .try_recv()
        .expect("bob aurait dû être notifié du départ d'alice");
    assert_eq!(
        event,
        Message::Event(EventType::ROOM_LEAVE {
            player_name: "alice".to_string(),
        })
    );
}

#[test]
fn move_in_dungeon_notifies_arrival_room_with_room_join() {
    let server = dungeon_server();
    connect_in_dungeon(&server, addr(1), "alice");
    let mut charlie_rx = connect_in_dungeon(&server, addr(2), "charlie");

    // charlie va d'abord dans dg_room(1), puis on ignore les events de SON déplacement
    move_request(&server, addr(2), &vec!["North".to_string()]);
    while charlie_rx.try_recv().is_ok() {}

    // alice arrive à son tour dans dg_room(1)
    move_request(&server, addr(1), &vec!["North".to_string()]);

    let event = charlie_rx
        .try_recv()
        .expect("charlie aurait dû être notifié de l'arrivée d'alice");
    assert_eq!(
        event,
        Message::Event(EventType::ROOM_JOIN {
            player_name: "alice".to_string(),
        })
    );
}

#[test]
fn move_in_dungeon_does_not_notify_the_mover() {
    let server = dungeon_server();
    let mut alice_rx = connect_in_dungeon(&server, addr(1), "alice");
    let _bob_rx = connect_in_dungeon(&server, addr(2), "bob"); // témoin dans l'entrée

    move_request(&server, addr(1), &vec!["North".to_string()]);

    assert!(
        alice_rx.try_recv().is_err(),
        "alice ne devrait recevoir aucun event pour son propre déplacement"
    );
}

#[test]
fn move_when_dungeon_missing_returns_room_not_found() {
    // Le joueur est dans une salle de donjon dont le donjon n'existe pas (ex: expiré).
    let server = populated_server();
    connect(&server, addr(1), "alice");
    server
        .lock()
        .unwrap()
        .get_player_mut(addr(1))
        .unwrap()
        .location = dg_room(0);
    let result = move_request(&server, addr(1), &vec!["North".to_string()]);
    assert_eq!(result, err(ErrorCode::ROOM_NOT_FOUND));
}

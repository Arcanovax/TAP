use super::*;

#[test]
fn new_room_is_empty_but_named() {
    let room = Room::new("room.hub");
    assert_eq!(room.name, "room.hub");
    assert!(room.exits.is_empty());
    assert!(room.npc.is_empty());
    assert!(room.items.is_empty());
    assert!(room.references().is_empty());
}

#[test]
fn references_collects_npcs_items_and_exit_targets() {
    let mut room = Room::new("room.hub");
    room.npc.push("npc.guard".to_string());
    room.items.push("item.sword".to_string().into());
    room.exits
        .insert(Direction::North, "room.market".to_string());

    let refs = room.references();
    assert!(refs.contains(&"npc.guard"));
    assert!(refs.contains(&"item.sword"));
    assert!(refs.contains(&"room.market"));
    assert_eq!(refs.len(), 3);
}

#[test]
fn references_are_empty_without_links() {
    let mut room = Room::new("room.hub");
    room.description = "a quiet place".to_string();
    assert!(room.references().is_empty());
}

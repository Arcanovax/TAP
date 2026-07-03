use crate::structures::{
    dungeon::{Dungeon, format_dungeon_id},
    enums::exits::Direction,
    game::World,
    room::Room,
};
use rand::{RngExt, seq::IteratorRandom};
use std::{collections::HashMap, ops::Add};
use uuid::Uuid;

const MIN_ROOM: u8 = 3;
const MAX_ROOM: u8 = 6;

const TO_NORTH: Coord = Coord { x: 0, y: -1 };
const TO_SOUTH: Coord = Coord { x: 0, y: 1 };
const TO_EAST: Coord = Coord { x: 1, y: 0 };
const TO_WEST: Coord = Coord { x: -1, y: 0 };

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Coord {
    x: i8,
    y: i8,
}

impl Add for Coord {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl From<(i8, i8)> for Coord {
    fn from(value: (i8, i8)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

pub fn generate_dungeon(world: &World, gid: Uuid) -> Dungeon {
    // TODO: Change spawn_room to dungeon entrance
    let rooms = generate_rooms(world.spawn_room.clone(), gid);
    // TODO: Peupler les rooms

    Dungeon {
        rooms,
        npcs: HashMap::new(),
        items: HashMap::new(),
        name_to_ref: HashMap::new(),
    }
}

fn generate_rooms(return_room: String, gid: Uuid) -> HashMap<String, Room> {
    let mut room_grid: HashMap<Coord, String> = HashMap::new();
    let mut rooms: HashMap<String, Room> = HashMap::new();

    let id = format_dungeon_id("room", gid, 0);
    room_grid.insert((0, 0).into(), id.clone());
    let mut start_room = Room::new(id.as_str());
    start_room.exits.insert(Direction::West, return_room);
    rooms.insert(id, start_room);

    let n = rand::rng().random_range(MIN_ROOM..MAX_ROOM);
    let mut i = 1;
    while i < n {
        let Some(coord) = room_grid.keys().choose(&mut rand::rng()) else {
            continue;
        };

        let Some(dir) = [
            (Direction::North, TO_NORTH),
            (Direction::South, TO_SOUTH),
            (Direction::West, TO_WEST),
            (Direction::East, TO_EAST),
        ]
        .into_iter()
        .filter(|(_, delta)| !room_grid.contains_key(&(*coord + *delta)))
        .map(|(dir, _)| dir)
        .choose(&mut rand::rng()) else {
            continue;
        };

        let new_id = format_dungeon_id("room", gid, i);
        let new_room = Room::new(&new_id);

        let new_coord = match dir {
            Direction::North => *coord + TO_NORTH,
            Direction::South => *coord + TO_SOUTH,
            Direction::East => *coord + TO_EAST,
            Direction::West => *coord + TO_WEST,
        };

        room_grid.insert(new_coord, new_id.clone());
        rooms.insert(new_id, new_room);

        i += 1;
    }

    link_all_rooms(room_grid, &mut rooms);
    rooms
}

fn link_all_rooms(room_grid: HashMap<Coord, String>, rooms: &mut HashMap<String, Room>) {
    for (coord, room_id) in &room_grid {
        if let Some(neighboor) = room_grid.get(&(*coord + TO_NORTH)) {
            rooms
                .get_mut(room_id)
                .unwrap()
                .exits
                .insert(Direction::North, neighboor.to_string());
            rooms
                .get_mut(neighboor)
                .unwrap()
                .exits
                .insert(Direction::South, room_id.to_string());
        }
        if let Some(neighboor) = room_grid.get(&(*coord + TO_SOUTH)) {
            rooms
                .get_mut(room_id)
                .unwrap()
                .exits
                .insert(Direction::South, neighboor.to_string());
            rooms
                .get_mut(neighboor)
                .unwrap()
                .exits
                .insert(Direction::North, room_id.to_string());
        }
        if let Some(neighboor) = room_grid.get(&(*coord + TO_WEST)) {
            rooms
                .get_mut(room_id)
                .unwrap()
                .exits
                .insert(Direction::West, neighboor.to_string());
            rooms
                .get_mut(neighboor)
                .unwrap()
                .exits
                .insert(Direction::East, room_id.to_string());
        }
        if let Some(neighboor) = room_grid.get(&(*coord + TO_EAST)) {
            rooms
                .get_mut(room_id)
                .unwrap()
                .exits
                .insert(Direction::East, neighboor.to_string());
            rooms
                .get_mut(neighboor)
                .unwrap()
                .exits
                .insert(Direction::West, room_id.to_string());
        }
    }
}

use crate::structures::{
    dungeon::{Dungeon, format_dungeon_id},
    enums::exits::Direction,
    game::World,
    room::Room,
};
use rand::RngExt;
use std::{collections::HashMap, ops::Add};
use uuid::Uuid;

const MIN_ROOM: u8 = 3;
const MAX_ROOM: u8 = 6;

const TO_NORTH: Coord = Coord { x: 0, y: -1 };
const TO_SOUTH: Coord = Coord { x: 0, y: 1 };
const TO_EAST: Coord = Coord { x: 1, y: 0 };
const TO_WEST: Coord = Coord { x: -1, y: 0 };

#[derive(PartialEq, Eq, Hash)]
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
    let rooms = generate_rooms(world.spawn_room.clone(), gid);

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
    let mut start_room = Room::new("Dungeon_0");
    start_room.exits.insert(Direction::West, return_room);
    rooms.insert(id, start_room);

    let n = rand::rng().random_range(MIN_ROOM..MAX_ROOM);
    let mut i = 1;
    while i < n {
        //Get random room
        i += 1;
    }

    rooms
}

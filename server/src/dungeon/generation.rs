use crate::structures::{
    dungeon::{Dungeon, format_dungeon_id},
    enums::{exits::Direction, npc_kind::NPCKind},
    game::World,
    room::Room,
};
use rand::{RngExt, seq::IteratorRandom};
use std::{collections::HashMap, ops::Add};
use uuid::Uuid;

const MIN_ROOM: u8 = 3;
const MAX_ROOM: u8 = 6;

const MIN_ITEM: u8 = 1;
const MAX_ITEM: u8 = 2;

const MIN_ENNEMY: u8 = 1;
const MAX_ENNEMY: u8 = 3;

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

pub fn generate_dungeon(base_world: &World, gid: Uuid) -> Dungeon {
    let rooms = generate_rooms(base_world, base_world.dungeon_entrance.clone(), gid);

    let mut dungeon = Dungeon {
        rooms,
        npcs: HashMap::new(),
        items: HashMap::new(),
    };

    populate_rooms(base_world, &mut dungeon, gid);

    dungeon
}

fn populate_rooms(world: &World, dungeon: &mut Dungeon, gid: Uuid) {
    let ennemy_pool = world
        .npcs
        .iter()
        .filter(|(_, npc)| matches!(npc.kind, NPCKind::Enemy { .. }))
        .map(|(_, npc)| npc);

    let mut ennemy_index = 0;
    let mut item_index = 0;

    for room in dungeon.rooms.values_mut() {
        let nb_ennemy = rand::rng().random_range(MIN_ENNEMY..=MAX_ENNEMY);
        let nb_item = rand::rng().random_range(MIN_ITEM..=MAX_ITEM);

        let mut i = 0;
        while i < nb_ennemy {
            let id = format_dungeon_id("npc", gid, ennemy_index + i);
            let Some(ennemy) = ennemy_pool.clone().choose(&mut rand::rng()) else {
                break;
            };
            room.npc.push(id.clone());
            dungeon.npcs.insert(id, ennemy.clone());
            i += 1;
        }
        ennemy_index += i;

        let mut i = 0;
        while i < nb_item {
            let id = format_dungeon_id("item", gid, item_index + i);
            let Some(item) = world
                .items
                .iter()
                .map(|(_, item)| item)
                .clone()
                .choose(&mut rand::rng())
            else {
                break;
            };
            room.items.push(id.clone().into());
            dungeon.items.insert(id, item.clone());
            i += 1;
        }
        item_index += i;
    }
}

fn generate_rooms(world: &World, return_room: String, gid: Uuid) -> HashMap<String, Room> {
	let mut rooms_pool: Vec<&Room> = world
        .rooms
        .iter()
        .map(|(_, room)| room)
		.collect();

    let mut room_grid: HashMap<Coord, String> = HashMap::new();
    let mut rooms: HashMap<String, Room> = HashMap::new();

	let mut chosen_room = rooms_pool.remove(rand::rng().random_range(0..rooms_pool.len()));
    let id = format_dungeon_id("room", gid, 0);
    room_grid.insert((0, 0).into(), id.clone());
    room_grid.insert((-1, 0).into(), return_room.clone());
    let mut start_room = Room::new(&format!("(Dungeon) {}", &chosen_room.name));
	start_room.description = chosen_room.description.clone();
    start_room
        .exits
        .insert(Direction::West, return_room.clone());
    rooms.insert(id, start_room);

    let n = rand::rng().random_range(MIN_ROOM..=MAX_ROOM);
    let mut i = 1;
    while i < n {
        let Some(coord) = room_grid.keys().choose(&mut rand::rng()) else {
            continue;
        };

        if let Some(room_id) = room_grid.get(&coord) {
            if *room_id == return_room {
                continue;
            }
        }

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

		chosen_room = rooms_pool.remove(rand::rng().random_range(0..rooms_pool.len()));
        let new_id = format_dungeon_id("room", gid, i);
        let mut new_room = Room::new(&format!("(Dungeon) {}", &chosen_room.name));
		new_room.description = chosen_room.description.clone();

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

    room_grid.remove(&(-1, 0).into());
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

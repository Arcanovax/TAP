use std::vec;

use crate::*;

#[derive(Clone, PartialEq, Debug)]
pub struct Dungeon {
	pub players: HashMap<String, i32>,
	pub err_join: bool,
	pub rooms: HashMap <String,RoomData>,
	pub walls: Vec<Texture2D>,
	pub walls_loaded: bool

}

impl Dungeon {
	pub fn new() -> Self {
		Self {
			players:HashMap::new(),
			err_join: false,
			rooms: HashMap::new(),
			walls: Vec::new(),
			walls_loaded: false
			}
	}
}

pub async fn get_dungeon_map(dungeon: &mut Dungeon, data: &RoomData) -> Room {
	let map = load_texture("assets/map/dungeon/floor.png").await.unwrap();

	if !dungeon.walls_loaded{
		dungeon.walls = Vec::new();
		if data.exits.contains_key(&Direction::North) {
			dungeon.walls.push(load_texture("assets/map/dungeon/top_on.png").await.unwrap());
		} else {
			dungeon.walls.push(load_texture("assets/map/dungeon/top_off.png").await.unwrap());
		}

		if data.exits.contains_key(&Direction::South) {
			dungeon.walls.push(load_texture("assets/map/dungeon/bottom_on.png").await.unwrap());
		} else {
			dungeon.walls.push(load_texture("assets/map/dungeon/bottom_off.png").await.unwrap());
		}
		if data.exits.contains_key(&Direction::East) {
			dungeon.walls.push(load_texture("assets/map/dungeon/right_on.png").await.unwrap());
		} else {
			dungeon.walls.push(load_texture("assets/map/dungeon/right_off.png").await.unwrap());
		}
		if data.exits.contains_key(&Direction::West) {
			dungeon.walls.push(load_texture("assets/map/dungeon/left_on.png").await.unwrap());
		} else {
			dungeon.walls.push(load_texture("assets/map/dungeon/left_off.png").await.unwrap());
		}
		dungeon.walls_loaded = true;
	}


	Room { id: data.id.clone(), colliders:
		[
				[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
				[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
				[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
				[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
				[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
				[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
				[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
				[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
				[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
				[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
				[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
				[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
				[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
				[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
				[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],

			], first_layer: map, second_layer: None, spawns: HashMap::from([
				(Spawn::Center, vec2(50.0, 150.0)),
				(Spawn::West, vec2(10.0, 150.0)),
				(Spawn::East, vec2(380.0, 150.0)),
				(Spawn::South, vec2(200.0, 190.0)),
				(Spawn::North, vec2(200.0, 70.0)),])
			}
}


pub fn draw_dungeon_wall(game: &mut Game){
	if let Some(dungeon) = &mut game.dungeon{
		for wall_text in dungeon.walls.clone(){
			wall_text.set_filter(FilterMode::Nearest);
			let map_params = DrawTextureParams {
			dest_size: Some(vec2(wall_text.width(), wall_text.height())),
			..Default::default()
		};
		draw_texture_ex(
			&wall_text,
			0.0,
			0.0,
			WHITE,
			map_params,
		);
		}
	}
}

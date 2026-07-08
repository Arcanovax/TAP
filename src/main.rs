mod utils;
mod rooms;
mod chat;
mod menu;
mod inventory;
mod start;
mod group;
mod player;
mod items;
mod npc;
mod server_event;
mod response;
mod camera;
mod fight;
mod quest;
mod dungeon;

use fight::*;
use player::*;
use items::*;
use server_event::*;
use camera::*;
use response::*;
use quest::*;
use dungeon::*;

use utils::*;
use npc::*;
use std::collections::HashMap;

use macroquad::prelude::*;
use rooms::*;
use chat::Chat;
use menu::*;
use inventory::*;
use std::sync::mpsc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use start::*;
use group::*;
use crate::chat::handle_chat;
use serde::Deserialize;








#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub enum Spawn {
	None,
    North,
    South,
    East,
    West,
	Center
}



struct Player {
    x: f32,
    y: f32,
    line: i32,
	row: i32,
    is_mooving: bool,
	speed: f32,
	inventory: Inventory,
    name: String,
	new_spawn:Spawn,
	state: Option<PlayerState>,
	gold: Option<i32>
}

#[derive(Deserialize, Debug, Clone)]
struct PlayerState{
	hp: i32,
	max_hp: i32,
	status: Status
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum Status {
	InFight {target_id: String},
    Idle,
	Discuss
}


struct Skin {
    texture: Texture2D,
    name: String,
}

struct Game {
	pub focus: InputFocus,
    pub menu: Menu,
    pub player: Player,
    pub chat: Chat,
    pub skin: Texture2D,
	pub tx_to_serv: tokio::sync::mpsc::Sender<String>,
    pub is_auth: bool,
    pub rx_from_serv: std::sync::mpsc::Receiver<String>,
	pub group: Group,
	pub pending_action: PendingAction,
	pub map_data: Option<LookData>,
	pub loaded_items:HashMap<String, Item>,
	pub loaded_npcs: HashMap<String,Npc>,
	pub nb_players: i32,
	pub config: GameConfig,
	pub active_fight: Option<Fight>,
	pub quests: Quests,
	pub npc_shop: NpcShop,
	pub mouse: Vec2,
	pub dungeon: Option<Dungeon>
}


struct GameConfig {
	sprite_width: f32,
    sprite_height: f32,
    tile_size: f32,
   	camera:Camera2D
}

async fn network_task(tx: mpsc::Sender<String>, mut rx: tokio::sync::mpsc::Receiver<String>) {
    let stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();
    let (mut reader, mut writer) = stream.into_split();


	let read_task = tokio::spawn(async move {
        let mut buf = [0u8; 1024];
        let mut accumulator = String::new();
        loop {
            let n = reader.read(&mut buf).await.unwrap();
            if n == 0 { break; }
            accumulator.push_str(&String::from_utf8_lossy(&buf[..n]));

            while let Some(pos) = accumulator.find('\n') {
                let line = accumulator[..pos].trim().to_string();
                if !line.is_empty() {
                    tx.send(line).ok();
                }
                accumulator = accumulator[pos + 1..].to_string();
            }
        }
    });

    let write_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            writer.write_all(msg.as_bytes()).await.unwrap();
        }
    });

    let _ = tokio::join!(read_task, write_task);
}


fn config() -> Conf {
    Conf {
        window_title: "TAP".to_owned(),
        window_width: 1280,
        window_height: 720,
		fullscreen: false,
        ..Default::default()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputFocus {
    Game,
    Chat,
    GroupMenu,
}

#[macroquad::main(config)]
async fn main() {

	let (tx_to_game, rx_from_serv) = mpsc::channel::<String>();
	let (tx_to_serv, rx_from_game) = tokio::sync::mpsc::channel::<String>(32);
	std::thread::spawn(move || {
			tokio::runtime::Runtime::new()
				.unwrap()
				.block_on(network_task(tx_to_game, rx_from_game));
		});
	let texture = load_texture("assets/player_skin.png").await.unwrap();
    texture.set_filter(FilterMode::Nearest);
    let mut game: Game = Game{
		focus: InputFocus::Game,
        chat: Chat::new(),
        menu: Menu::new(),
        player: Player {
            x: 0.0,
            y: 0.0,
            line: 0,
            row: 0,
            is_mooving: false,
            speed: 1.8,
			inventory: Inventory::new(),
            name:"".to_string(),
			new_spawn: Spawn::Center,
			state: None,
			gold: None
        },
        skin:texture,
		tx_to_serv: tx_to_serv,
        rx_from_serv: rx_from_serv,
        is_auth: false,
		group: Group::new(),
		pending_action: PendingAction::None,
		map_data: None,
		loaded_items: HashMap::new(),
		loaded_npcs: HashMap::new(),
		nb_players: 0,
		config: GameConfig{
			sprite_width: 16.0,
			sprite_height: 32.0,
			tile_size: 16.0,
			camera: Camera2D::default()
		},
		active_fight: None,
		quests: Quests { all: Vec::new(), is_load: false},
		npc_shop: NpcShop { is_active: false, buy_info: None, sell_info: None},
		mouse: Vec2::new(0.0, 0.0),
		dungeon: None
    };



	let rooms: std::collections::HashMap<String, rooms::Room> = get_rooms().await;


	let floor: Texture2D = load_texture("assets/map/fightmap.png").await.unwrap();



    loop {
		while let Ok(msg) = game.rx_from_serv.try_recv() {
			println!("Send: {:?}", game.pending_action);
            println!("GET: {}", msg);
			let parts: Vec<&str> = msg.split_whitespace().collect();
			if parts.is_empty() { return; }
			let answer = parts[1..].join(" ");
			let state: &str = parts[0];

			match state {
				"OK" | "ERR" => handle_response(&mut game, answer.as_str(), state).await,
				"EVT" => handle_events(&mut game, parts).await,
				_ => {}
			}

		}
		game.mouse = vec2(mouse_position().0, mouse_position().1);
		if !game.is_auth{
					handle_starter(&mut game);
					next_frame().await
				}
		else {
			if game.nb_players == 0 && game.pending_action == PendingAction::None{
				game.tx_to_serv.try_send("WHO \n".to_string()).ok();
				game.pending_action = PendingAction::Who;
			}
			else{

			if let Some(ref mut dungeon) = game.dungeon{

				if dungeon.err_join{
					game.tx_to_serv.try_send("DUNGEON JOIN \n".to_string()).ok();
					game.pending_action = PendingAction::DungeonJoin;
				}
				else if dungeon.rooms.is_empty(){
					game.tx_to_serv.try_send("ROOMS \n".to_string()).ok();
					game.pending_action = PendingAction::Rooms;
				}

			}


			if game.map_data.is_none() && game.pending_action == PendingAction::None {
				game.tx_to_serv.try_send("LOOK \n".to_string()).ok();
				game.pending_action = PendingAction::Look;
			}

			else if game.loaded_items.is_empty() && game.pending_action == PendingAction::None{
				game.tx_to_serv.try_send("ITEMS \n".to_string()).ok();
				game.pending_action = PendingAction::Items;
			}

			else if game.loaded_npcs.is_empty() && game.pending_action == PendingAction::None{
				game.tx_to_serv.try_send("NPCS \n".to_string()).ok();
				game.pending_action = PendingAction::Npcs;
			}

			else if game.player.state.is_none() && game.pending_action == PendingAction::None{
				game.tx_to_serv.try_send("STATUS \n".to_string()).ok();
				game.pending_action = PendingAction::Status;
			}

			else if !game.quests.is_load && game.pending_action == PendingAction::None{
				game.tx_to_serv.try_send("QUESTS \n".to_string()).ok();
				game.pending_action = PendingAction::Quests;
			}

			else if game.player.gold.is_none() && game.pending_action == PendingAction::None{
				game.tx_to_serv.try_send("GOLD \n".to_string()).ok();
				game.pending_action = PendingAction::Gold;
			}

			else{
				if let Some(state) = game.player.state.clone() {
					if state.status != Status::Idle{
						handle_fight(&mut game, &floor);

						if is_key_pressed(KeyCode::C){
							break;
						}
					}
					else{

					if !game.player.inventory.is_load && game.pending_action == PendingAction::None{
						game.tx_to_serv.try_send("INVENTORY \n".to_string()).ok();
						game.pending_action = PendingAction::Inventory;
					}
					if is_key_pressed(KeyCode::C) && game.focus == InputFocus::Game{
						break;
					}

					if let Some(map_data) = game.map_data.clone() {
						if game.dungeon.is_some() {
							let mut room_data = None;
							if let Some(dungeon) = &game.dungeon {
								if !dungeon.rooms.is_empty() {
									if let Some(rd) = dungeon.rooms.get(&map_data.room.id) {
										room_data = Some(rd.clone());
									} else {
										game.dungeon = None;
									}
								}
							}
							if let Some(room_data) = room_data {
								if let Some(dungeon) = &game.dungeon {

								}
								let map: Room = get_dungeon_map(&mut game, &room_data).await;
								handle_game(&mut game, &map, map_data);
							}
							else {
							let map = match rooms.get(&map_data.room.id) {
								Some(room_data) => room_data,
								None => &get_empty_room(),
							};
							handle_game(&mut game, map, map_data);
						}
						} else {
							let map = match rooms.get(&map_data.room.id) {
								Some(room_data) => room_data,
								None => &get_empty_room(),
							};
							handle_game(&mut game, map, map_data);
						}
					}

				}

			}
		}
		next_frame().await
		}
    }
}


fn handle_game(game: &mut Game, map: &Room, map_data: LookData){
	if game.player.new_spawn != Spawn::None && game.player.new_spawn != Spawn::Center{
			let spawn: Vec2 = map.spawns
				.get(&game.player.new_spawn)
				.or_else(|| map.spawns.get(&Spawn::Center))
				.copied()
				.unwrap_or(Vec2::ZERO);
			game.player.x = spawn.x;
			game.player.y = spawn.y;
			game.player.new_spawn = Spawn::None;
			game.tx_to_serv.try_send("LOOK\n".to_string()).ok();
			game.pending_action = PendingAction::Look;
		}


		else if game.player.new_spawn != Spawn::None{
			let spawn: Vec2 = map.spawns[&game.player.new_spawn];
			game.player.x = spawn.x;
			game.player.y = spawn.y;
			game.player.new_spawn = Spawn::None;
		}


		let floor: Texture2D = map.first_layer.clone();
		let builds: Option<Texture2D> = map.second_layer.clone();
		let map_obstacles = map.colliders;
		if let Some(builds_texture) = builds.as_ref() {
			builds_texture.set_filter(FilterMode::Nearest);
		}
		floor.set_filter(FilterMode::Nearest);

		clear_background(BLACK);

		camera_handler(game);

		if game.focus == InputFocus::Game{
			player_handler(game, &map_obstacles);
		}

		let current_skin_texture = game.skin.clone();
		let sprite_width: f32 = game.config.sprite_width;
		let sprite_height: f32 = game.config.sprite_height;
		let source_x: f32 = game.player.row as f32 * sprite_width;
		let source_y: f32 = game.player.line as f32 * sprite_height;




		let map_params = DrawTextureParams {
			dest_size: Some(vec2(floor.width(), floor.height())),
			..Default::default()
		};



		draw_texture_ex(
			&floor,
			0.0,
			0.0,
			WHITE,
			map_params,
		);

		draw_dungeon_wall(game);

		for player_name in map_data.players.iter(){
			let cut_sheet = DrawTextureParams {
			source: Some(Rect::new(0.0, 0.0, sprite_width, sprite_height - 1.0)),
			dest_size: Some(vec2(sprite_width, sprite_height - 1.0)),
			..Default::default()
			};
			if player_name.as_ref() == game.player.name{
				continue;
			}
			if let Some(coords) = map.spawns.get(&Spawn::Center) {
			draw_texture_ex(
				&current_skin_texture,
				coords.x,
				coords.y,
				WHITE,
				cut_sheet.clone());

			let screen_pos = world_to_screen_pos(*coords);
			let sprite_rect = world_to_screen_pos(vec2(sprite_width, sprite_height));
			set_default_camera();
			let rect_width = 80.0;
			let rect_height = 15.0;
			let rect = Rect::new(screen_pos.x + (sprite_rect.x / 2.0) - (rect_width / 2.0), screen_pos.y-rect_height, rect_width, rect_height);
			draw_text_center(rect, player_name, 20);
			camera_handler(game);
			}
		}


		let mut npc_slots: Vec<Vec2> = find_npc_spawns(&map.colliders, game.config.tile_size);
		let texture_param = DrawTextureParams {
			dest_size: Some(vec2(sprite_width, sprite_height)),
			..Default::default()
		};

		let activation_distance = 20.0;
		let mut active_npc: Option<(Vec2, Npc)> = None;

		for npc_id in map_data.npcs.iter() {
			if let Some(place) = npc_slots.pop() {
				let npc: Option<Npc> = game.loaded_npcs.get(npc_id).cloned();
				if let Some(npc) = npc {

					let npc_texture: Texture2D = npc.clone().texture;

					let distance = place.distance(vec2(game.player.x, game.player.y));
					if distance < activation_distance {
						active_npc = Some((place, npc));
					}

					draw_texture_ex(
						&npc_texture,
						place.x,
						place.y,
						WHITE,
						texture_param.clone()
					);

					npc_texture.set_filter(FilterMode::Nearest);
				}

			} else {
				break;
			}
		}


		let cut_sheet = DrawTextureParams {
			source: Some(Rect::new(source_x, source_y + 1.0, sprite_width, sprite_height - 1.0)),
			dest_size: Some(vec2(sprite_width, sprite_height - 1.0)),
			..Default::default()
		};
		draw_texture_ex(
			&current_skin_texture,
			game.player.x.round(), game.player.y.round(),
			WHITE,
			cut_sheet
		);

		if let Some((place, npc)) = active_npc {
			handle_npc_interactions(game, place, npc);
		}
		else{
			game.npc_shop.is_active = false;
		}


		if let Some(builds_texture) = builds.as_ref() {
			let builds_params = DrawTextureParams {
				dest_size: Some(vec2(builds_texture.width(), builds_texture.height())),
				..Default::default()
			};

			draw_texture_ex(
				builds_texture,
				0.0,
				0.0,
				WHITE,
				builds_params,
			);
		}


		set_default_camera();
		draw_player_info(game);


		display_quests(game);



		if game.focus == InputFocus::Game {
			while get_char_pressed().is_some() {}
		}


		handle_menu(game);

		handle_inv(game);
		handle_chat(game);
		handle_group(game);
		if let Some((rect, item)) = game.player.inventory.active_item_info.clone(){
			draw_item_info(rect, &item);
		};
		game.player.inventory.active_item_info = None;
		draw_menu(game);

	}
}

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

use fight::*;
use macroquad::input::KeyCode::S;
use player::*;
use items::*;
use server_event::*;
use camera::*;
use response::*;
use quest::*;

use serde_json::Value;
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






#[derive(Deserialize, Debug, Clone)]
pub enum Exit {
    North { toward: String },
    South { toward: String },
    East { toward: String },
    West { toward: String },
}




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
    spritesheet_index: usize,
	inventory: Inventory,
    name: String,
	new_spawn:Spawn,
	state: Option<PlayerState>
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
    pub skins: Vec<Skin>,
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
	pub quests: Vec<Quest>
}


struct GameConfig {
	sprite_width: f32,
    sprite_height: f32,
    tile_size: f32,
   	camera:Camera2D
}

impl Game {
    pub async fn load_skins(&mut self, skin_data: Vec<(&str, &str)>){
        for (path, name) in skin_data {
            let texture = load_texture(path).await.unwrap();
            texture.set_filter(FilterMode::Nearest);
            self.skins.push(Skin {
                texture: texture,
                name: name.to_string(),
            });
    }
    }
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
            spritesheet_index: 0,
			inventory: Inventory::new(),
            name:"".to_string(),
			new_spawn: Spawn::Center,
			state: None
        },
        skins: Vec::new(),
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
		quests: Vec::new()
    };



	let rooms: std::collections::HashMap<String, rooms::Room> = get_rooms().await;

    let skin_data: Vec<(&str, &str)> = vec![
        ("assets/skins/alex.png", "Alex"),
        ("assets/skins/kent.png", "Kent"),
        ("assets/skins/pierre.png", "Pierre"),
        ("assets/skins/shane.png", "Shane"),
    ];
    game.load_skins(skin_data).await;

	let floor: Texture2D = load_texture("assets/map/fightmap.png").await.unwrap();



    loop {
		while let Ok(msg) = game.rx_from_serv.try_recv() {
			println!("Send: {:?}", game.pending_action);
            println!("GET: {}", msg);
			if let Ok(server_event) = serde_json::from_str::<ServerEvent>(&msg) {
				if server_event.event_type == "Event" {
					handle_events(&mut game, server_event).await;
				}
				else if server_event.event_type == "Response"{
					handle_response(&mut game, server_event, msg).await;
				}
			}
			game.pending_action = PendingAction::None;

		}

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

			else{
				if let Some(mut state) = game.player.state.clone() {
					if state.status != Status::Idle{
						handle_fight(&mut game, &floor);
						if is_key_pressed(KeyCode::C){
							break;
					}
					}
					else{


				if game.map_data.is_none() && game.pending_action == PendingAction::None {
					game.tx_to_serv.try_send("LOOK \n".to_string()).ok();
					game.pending_action = PendingAction::Look;
				}

				if let Some(map_data) = game.map_data.clone() {
					let map = match rooms.get(&map_data.room.id) {
						Some(room_data) => room_data,
						None => {
							continue;
						}

					};



					if game.player.new_spawn != Spawn::None && game.player.new_spawn != Spawn::Center{
						let spawn: Vec2 = map.spawns[&game.player.new_spawn];
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

					camera_handler(&mut game);

					if game.focus == InputFocus::Game{
						player_handler(&mut game, &map_obstacles);
					}

					let current_skin_texture = game.skins[game.player.spritesheet_index as usize].texture.clone();
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
						set_default_camera();
						draw_text(player_name, screen_pos.x, screen_pos.y, 20.0, WHITE);
						camera_handler(&mut game);
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
						handle_npc_interactions(&mut game, place, npc);
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
					let info: Rect = Rect::new(10.0, 10.0, 350.0, 120.0);
					draw_rectangle(info.x, info.y, info.w, info.h, Color::new(0.0, 0.0, 0.0, 0.5));
					

					let frame = Rect::new(info.x+10.0, info.y+10.0, 100.0, 100.0);
					draw_rectangle(frame.x, frame.y, frame.w, frame.h,BLACK);
					
					let cut_sheet_head = DrawTextureParams {
						source: Some(Rect::new(0.0, 0.0, sprite_width, 20.0)),
						dest_size: Some(vec2(75.0, 100.0 )),
						..Default::default()
					};
					draw_texture_ex(
						&current_skin_texture,
					 	frame.x+12.5, frame.y,
						WHITE,
						cut_sheet_head
					);
					draw_rectangle_lines(frame.x, frame.y, frame.w, frame.h, 10.0, Color::new(0.53, 0.31, 0.16, 1.0));


					draw_text(&game.player.name, frame.x + frame.w + 5.0, frame.y + 30.0, 40.0, WHITE);
					let lifebar = Rect::new(frame.x + frame.w + 5.0, frame.y + 40.0, 200.0, 25.0);
					draw_rectangle(lifebar.x, lifebar.y, lifebar.w, lifebar.h,BLACK);
					let hp_ratio: f32 = state.hp as f32 / state.max_hp as f32;
					draw_rectangle(lifebar.x, lifebar.y+2.5, lifebar.w * hp_ratio, 20.0,RED);
					let hp_info = format!("{}/{}",state.hp,state.max_hp);
					draw_text_center(lifebar, &hp_info, 20);
					draw_rectangle_lines(lifebar.x, lifebar.y, lifebar.w, lifebar.h, 5.0, GRAY);

					// let text_player = format!("Total players: {}",game.nb_players.clone());
					// draw_text(map_data.room.name.clone(), 5.0, 30.0, 30.0, WHITE);
					// draw_text(text_player, 5.0, 70.0, 30.0, WHITE);

					// if let Some(state) = game.player.state.as_ref(){
					// 	let rect_info: Rect =get_rect_right(vec2(100.0, 60.0), 0.0);
					// 	draw_rectangle(rect_info.x, rect_info.y, rect_info.w, rect_info.h, BLACK);
					// 
					// 	draw_text_bottom(rect_info, &hp_info.to_string(), 30, 0.0);
					// }


					
					
					display_quests(&mut game);
	
					

					if game.focus == InputFocus::Game {
						while get_char_pressed().is_some() {}
					}
					if is_key_pressed(KeyCode::C) && game.focus == InputFocus::Game{
						break;
					}

					handle_menu(&mut game);

					handle_inv(&mut game);
					handle_chat(&mut game);
					handle_group(&mut game);
					draw_menu(&mut game);
				}

					}

				}

			}
		}
		next_frame().await
		}
    }
}



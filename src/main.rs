mod utils;
mod rooms;
mod chat;
mod menu;
mod inventory;
mod start;
mod group;
mod player;

use player::*;
use utils::*;
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


#[derive(Deserialize, Debug)]
struct ServerEvent {
    #[serde(rename = "type")]
    event_type: String,

    #[serde(rename = "INVITE")]
    invite: Option<InviteData>,
	#[serde(rename = "CHAT")]
    chat: Option<ChatData>,
    data: Option<String>,
	error: Option<String>

}

#[derive(Deserialize, Debug)]
struct InviteData {
    sender: String,
    group_name: String,
}


#[derive(Deserialize, Debug)]
struct ChatData {
	body: String,
    sender: String,
    scope: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MapData {
    pub name: String,
    pub exits: Vec<Exit>,
    pub description: String,
    pub npc: Vec<String>,
    pub items: Vec<String>,
	pub players: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StatusData {
    pub name: String,
    pub hp: usize,
    pub max_hp: usize,
    pub location: String,
    pub status: String,
	pub inventory: HashMap<String, u32>,
	pub available_quests: Vec<String>,
	pub group_id: Option<String>
}

#[derive(Deserialize, Debug, Clone)]
pub enum Exit {
    North { toward: String },
    South { toward: String },
    East { toward: String },
    West { toward: String },
}

#[derive(Debug, Clone, PartialEq)]
pub enum PendingAction {
    None,
    GroupList,
	Auth,
	GroupCreate(String),
	GroupJoin(String),
	GroupInvite(String),
	SendChat(String, String),
	Look,
	Status,
	Move(Spawn)
}




#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub enum Spawn {
	None,
    North,
    South,
    East,
    West,
}




fn camera_handler(camera: &mut Camera2D, tile_size: f32) {
	let map_w = 25.0 * tile_size;
	let map_h = 14.0 * tile_size;

	camera.target = vec2(map_w / 2.0, map_h / 2.0);

	let scale_x = screen_width() / map_w;
	let scale_y = screen_height() / map_h;
	let scale = scale_x.min(scale_y);
	camera.zoom = vec2(scale * 2.0 / screen_width(), scale * 2.0 / screen_height());

	set_camera(camera);
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
    pub map_id: String,
	pub tx_to_serv: tokio::sync::mpsc::Sender<String>,
    pub is_auth: bool,
    pub rx_from_serv: std::sync::mpsc::Receiver<String>,
	pub group: Group,
	pub pending_action: PendingAction,
	pub map_data: Option<MapData>,
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
        loop {
            let n = reader.read(&mut buf).await.unwrap();
            if n == 0 { break; }
            tx.send(String::from_utf8_lossy(&buf[..n]).to_string()).ok();
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
            x: 200.0,
            y: 130.0,
            line: 0,
            row: 0,
            is_mooving: false,
            speed: 1.8,
            spritesheet_index: 0,
			inventory: Inventory::new(),
            name:"".to_string(),
			new_spawn: Spawn::None
        },
        skins: Vec::new(),
        map_id: String::new(),
		tx_to_serv: tx_to_serv,
        rx_from_serv: rx_from_serv,
        is_auth: false,
		group: Group::new(),
		pending_action: PendingAction::None,
		map_data: None
    };



	let rooms: std::collections::HashMap<String, rooms::Room> = get_rooms().await;


    let skin_data: Vec<(&str, &str)> = vec![
        ("assets/skins/alex.png", "Alex"),
        ("assets/skins/kent.png", "Kent"),
        ("assets/skins/pierre.png", "Pierre"),
        ("assets/skins/shane.png", "Shane"),
    ];
    game.load_skins(skin_data).await;

	let sprite_width: f32 = 16.0;
    let sprite_height: f32 = 32.0;


    let tile_size: f32 = 16.0;


    let mut camera = Camera2D::default();



    loop {
		while let Ok(msg) = game.rx_from_serv.try_recv() {
            println!("GET: {}", msg);
			if let Ok(server_event) = serde_json::from_str::<ServerEvent>(&msg) {
				if server_event.event_type == "Event" {
					if let Some(invite) = server_event.invite {
						game.group.invitation = Some(Invitation{sender: invite.sender, group_name: invite.group_name})
					}
					if let Some(msg) = server_event.chat {
						let channel = match msg.scope.as_str(){
						"ROOM" => &mut game.chat.room_messages,
						"GLOBAL" => &mut game.chat.global_messages,
						"GROUP" => &mut game.chat.group_messages,
						_ => continue
						};
						let text: String = format!("[{}]{}\n",msg.sender,msg.body);
						channel.push(text);
					}
       			}
				else if server_event.event_type == "Response"{
					match game.pending_action {
					PendingAction::GroupList => {

						if msg.contains("SUCCESS"){
							if let Some(data) = server_event.data{
								game.group.list = data}
						} else if msg.contains("NOT_IN_GROUP") {
							game.group.in_group = false;
						}
					}
					PendingAction::Auth => {
						if msg.contains("SUCCESS") {
							game.is_auth = true
						} else if msg.contains("NAME_IN_USE") {
							println!("already use")
						}
					}
					PendingAction::GroupCreate(name) => {
						if msg.contains("SUCCESS") {
							game.group.in_group = true;
							game.group.name = name;
						} else{
							println!("Failed group create");
						}
					}
					PendingAction::GroupJoin(name) => {
						if msg.contains("SUCCESS") {
							game.group.in_group = true;
							game.group.name = name;
						} else{
							println!("Failed join");
						}
					}
					PendingAction::GroupInvite(name) => {
						if msg.contains("SUCCESS") {
							let rp: String = format!("{} invited", name);
							game.group.invite_info = Some(InviteInfo{
								state: rp,
								color: GREEN,
								time: get_time()
							});
						} else{
							let rp: String = format!("{} is offline", name);
							game.group.invite_info = Some(InviteInfo{
								state: rp,
								color: RED,
								time: get_time()
							});
						}
					}
					PendingAction::SendChat(channel, text) => {
						let channel = match channel.as_str(){
							"Room" => &mut game.chat.room_messages,
							"Global" => &mut game.chat.global_messages,
							"Group" => &mut game.chat.group_messages,
							_ => {
									game.pending_action = PendingAction::None;
									continue;
								}
							};
						if msg.contains("SUCCESS") {
							channel.push(text);
						} else{
							if let Some(error) = server_event.error{
								let rp: String = format!("[Error] {}", error);
								channel.push(rp);
							}
						}
					}
					PendingAction::Look => {
						if let Some(data_str) = &server_event.data {
							match serde_json::from_str::<MapData>(data_str) {
								Ok(parsed_map_data) => {
									game.map_data = Some(parsed_map_data);
								}
								Err(e) => {
								eprintln!("LOOK error: {}", e);
								}
							}

						}
					}
					PendingAction::Status => {
						if let Some(data_str) = &server_event.data {
							match serde_json::from_str::<StatusData>(data_str) {
								Ok(parsed_map_data) => {
									game.map_id = parsed_map_data.location;
								}
								Err(e) => {
								eprintln!("STATUS error: {}", e);
								}
							}

						}
					}
					PendingAction::Move(new_spawn) => {
						if msg.contains("SUCCESS") && game.player.new_spawn == Spawn::None{
							if let Some(data) = server_event.data{
								println!("{}", data)}

								game.player.new_spawn = new_spawn;
								game.map_id = String::new();
						} else{
							println!("Failed MOVE");
						}
					}
					_ => {
						game.pending_action = PendingAction::None;
					}
				}
    				game.pending_action = PendingAction::None;
				}
    		}
        }

		if !game.is_auth{
					handle_starter(&mut game);
					next_frame().await

				}
		else {

		if game.map_id.is_empty() && game.pending_action == PendingAction::None {
			game.tx_to_serv.try_send("STATUS\n".to_string()).ok();
			game.pending_action = PendingAction::Status;
		}
		else {



		let map = match rooms.get(&game.map_id) {
			Some(room_data) => room_data,
			None => {
				continue;
			}

		};

		if game.player.new_spawn != Spawn::None{
			let spawn: Vec2 = map.spawns[&game.player.new_spawn];
			game.player.x = spawn.x;
			game.player.y = spawn.y;
			game.player.new_spawn = Spawn::None;
			game.tx_to_serv.try_send("LOOK\n".to_string()).ok();
			game.pending_action = PendingAction::Look;

		}
		let floor: Texture2D = map.first_layer.clone();
		let builds: Option<Texture2D> = map.second_layer.clone();
		let map_obstacles = map.colliders;
		if let Some(builds_texture) = builds.as_ref() {
		builds_texture.set_filter(FilterMode::Nearest);
		}
		floor.set_filter(FilterMode::Nearest);

        clear_background(BLACK);

		camera_handler(&mut camera, tile_size);

		if game.focus == InputFocus::Game{
			player_handler(&mut game, &map_obstacles, tile_size, sprite_width, sprite_height);
		}

        let current_skin = &game.skins[game.player.spritesheet_index as usize];
        let source_x: f32 = game.player.row as f32 * sprite_width;
        let source_y: f32 = game.player.line as f32 * sprite_height;


        let cut_sheet = DrawTextureParams {
            source: Some(Rect::new(source_x, source_y + 1.0, sprite_width, sprite_height - 1.0)),
            dest_size: Some(vec2(sprite_width, sprite_height - 1.0)),
            ..Default::default()
        };

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



        draw_texture_ex(
            &current_skin.texture,
			game.player.x.round(), game.player.y.round(),
            WHITE,
            cut_sheet
        );

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

		if game.focus == InputFocus::Game {
			while get_char_pressed().is_some() {}
		}
		if is_key_pressed(KeyCode::C) && game.focus == InputFocus::Game{
            break;
        }




		// if is_key_pressed(KeyCode::F) && game.focus == InputFocus::Game {
		// 	game.group.is_active = true;
    	// }

		// if is_key_pressed(KeyCode::Enter) && game.focus == InputFocus::Game {
		// 	game.chat.is_active = true;
		// }

        handle_menu(&mut game);

		handle_inv(&mut game);
		handle_chat(&mut game);
		handle_group(&mut game);
        draw_menu(&mut game);
		if let Some(map_data) = game.map_data.clone() {
        	draw_text(map_data.name, 5.0, 30.0, 60.0, WHITE);
		}

		}
        next_frame().await
    }}
}


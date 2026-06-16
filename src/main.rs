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

use player::*;
use items::*;
use serde_json::error::Category::Data;
use utils::*;
use npc::*;
use std::collections::HashMap;
use std::fmt::Alignment::Center;
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
	#[serde(rename = "GROUP_JOIN")]
    join: Option<GroupEvent>,
	#[serde(rename = "GROUP_LEAVE")]
    leave: Option<GroupEvent>,
	#[serde(rename = "ROOM_LEAVE")]
	room_leave: Option<PlayersEvent>,
	#[serde(rename = "ROOM_JOIN")]
	room_join: Option<PlayersEvent>,
	#[serde(rename = "PLAYERS")]
	players: Option<Players>,
	#[serde(rename = "CHAT")]
    chat: Option<ChatData>,
    data: Option<serde_json::Value>,
	error: Option<String>

}

#[derive(Deserialize, Debug)]
struct InviteData {
    sender: String,
    group_name: String,
}

#[derive(Deserialize, Debug)]
struct PlayersEvent {
	player_name: String,
}

#[derive(Deserialize, Debug)]
struct Players {
	players: i32,
}


#[derive(Deserialize, Debug)]
struct GroupEvent {
	player_name: String,
}

#[derive(Deserialize, Debug)]
struct ChatData {
	body: String,
    sender: String,
    scope: String,
}

#[derive(Deserialize, Debug)]
struct MoveData {
	room: String,
}




#[derive(Deserialize, Debug, Clone)]
pub struct LookData {
    pub room: RoomData,
	pub players: Vec<String>,
	pub items: Vec<String>,
    pub npcs: Vec<String>,
}


#[derive(Deserialize, Debug, Clone)]
pub struct RoomData {
    pub id: String,
    pub name: String,
	pub description: String,
    pub exits: Vec<Exit>,

}



#[derive(Deserialize, Debug, Clone)]
pub struct StatusData {
    pub hp: i32,
    pub max_hp: i32,
    pub status: String,
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
	GroupJoin(String, String),
	GroupInvite(String),
	GroupLeave,
	SendChat(String, String),
	Look,
	Command(String, String),
	Status,
	Move(Spawn),
	Take,
	Drop,
	Who
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

fn world_to_screen_pos(world_pos: Vec2) -> Vec2 {
    let map_w = 25.0 * 16.0;
    let map_h = 14.0 * 16.0;

    let scale_x = screen_width() / map_w;
    let scale_y = screen_height() / map_h;
    let scale = scale_x.min(scale_y);

    let rendered_w = map_w * scale;
    let rendered_h = map_h * scale;
    let offset_x = (screen_width() - rendered_w) / 2.0;
    let offset_y = (screen_height() - rendered_h) / 2.0;

    return vec2(
        offset_x + world_pos.x * scale,
        offset_y + world_pos.y * scale,
    )
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
	hp: i32,
	max_hp: i32,
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
	pub items:HashMap<String, Item>,
	pub nb_players: i32
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
			hp: 0,
			max_hp: 0
        },
        skins: Vec::new(),
		tx_to_serv: tx_to_serv,
        rx_from_serv: rx_from_serv,
        is_auth: false,
		group: Group::new(),
		pending_action: PendingAction::None,
		map_data: None,
		items: get_items().await,
		nb_players:0
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
			println!("Send: {:?}", game.pending_action);
            println!("GET: {} //", msg);
			if let Ok(server_event) = serde_json::from_str::<ServerEvent>(&msg) {
				if server_event.event_type == "Event" {
					if let Some(invite) = server_event.invite {
						game.group.invitation = Some(Invitation{sender: invite.sender, group_name: invite.group_name})
					}
					if let Some(new) = server_event.join {
						game.group.grouplist.push(new.player_name);
					}
					if let Some(leaver) = server_event.leave {
						game.group.grouplist.retain(|x| x != &leaver.player_name);
					}
					if let Some(leave) = server_event.room_leave {
						if let Some(ref mut map_data) = game.map_data{
							if leave.player_name != game.player.name{
								map_data.players.retain(|x| x != &leave.player_name);
							}
						}
					}
					if let Some(join) = server_event.room_join {
						if let Some(ref mut map_data) = game.map_data{
							map_data.players.push(join.player_name);
							game.map_data = None
						}
					}
					if let Some(players) = server_event.players {
						game.nb_players = players.players;
					}
					if let Some(msg) = server_event.chat {
						let channel = match msg.scope.as_str(){
							"ROOM" => &mut game.chat.room_messages,
							"GLOBAL" => &mut game.chat.global_messages,
							"GROUP" => &mut game.chat.group_messages,
							_ => continue
							};
						let text: String = format!("[{}] {}\n",msg.sender,msg.body);
						channel.push(text);
					}
       			}
				else if server_event.event_type == "Response"{
					match game.pending_action {
					PendingAction::GroupList => {
						if msg.contains("SUCCESS"){
							if let Some(data_val) = server_event.data {
								let data_str = data_val.to_string();
								match serde_json::from_str::<Vec<String>>(&data_str) {
									Ok(players) => {
										game.group.grouplist = players;
									}
									Err(e) => {
										println!("Error JSON: {}", e);
									}
								}
							}
							else if msg.contains("NOT_IN_GROUP") {
								game.group.in_group = false;
							}
						}
					}
					PendingAction::Auth => {
						if msg.contains("SUCCESS") {
							game.is_auth = true;
						} else if msg.contains("NAME_IN_USE") {
							println!("already use")
						}
					}
					PendingAction::GroupCreate(group_name) => {
						if msg.contains("SUCCESS") {
							game.group.in_group = true;
							if group_name.is_empty(){
								let name: String = format!("{}'s Group ",game.player.name.clone());
								game.group.name = name;}
							else{
								let name: String = format!("Group {}",group_name);
								game.group.name = name;
							}
						}
						else{
							println!("Failed group create");
						}
					}
					PendingAction::GroupJoin(sender, group_name) => {
						if msg.contains("SUCCESS") {
							game.group.in_group = true;
							if sender == group_name{
								let name: String = format!("{}'s Group ",sender);
								game.group.name = name;}
							else{
								let name: String = format!("{}",group_name);
								game.group.name = name;
							}
						} else{
							println!("Failed join");
						}
					}
					PendingAction::GroupLeave => {
						if msg.contains("SUCCESS") {
							game.group.in_group = false;
							game.group.name = String::new();
							game.group.grouplist = Vec::new();
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
					PendingAction::Command(channel, cmd) => {
						let channel = match channel.as_str(){
							"Room" => &mut game.chat.room_messages,
							"Global" => &mut game.chat.global_messages,
							"Group" => &mut game.chat.group_messages,
							_ => {
									game.pending_action = PendingAction::None;
									continue;
								}
							};
						channel.push(cmd);
						if msg.contains("SUCCESS") {
							if let Some(data) = server_event.data{
									let rp: String = format!("[Server] {}", data);
									channel.push(rp);
								}
							}
						else {
							if let Some(error) = server_event.error{
									let rp: String = format!("[Server] {}", error);
									channel.push(rp);
								}
							}
						}
					PendingAction::Look => {
						if let Some(data_val) = &server_event.data {
							let data_str = data_val.to_string();
							match serde_json::from_str::<LookData>(&data_str) {
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
						if let Some(data_val) = &server_event.data {
							let data_str = data_val.to_string();
							match serde_json::from_str::<StatusData>(&data_str) {
								Ok(parsed_map_data) => {
									game.player.hp = parsed_map_data.hp;
									game.player.max_hp = parsed_map_data.max_hp;
								}
								Err(e) => {
								eprintln!("STATUS error: {}", e);
								}
							}

						}
					}
					PendingAction::Who => {
						if let Some(data_val) = &server_event.data {
							let data_str = data_val.to_string();
							match serde_json::from_str::<Players>(&data_str) {
								Ok(players) => {
									game.nb_players =players.players;
								}
								Err(e) => {
								eprintln!("STATUS error: {}", e);
								}
							}

						}
					}
					PendingAction::Move(new_spawn) => {

						if msg.contains("SUCCESS") && game.player.new_spawn == Spawn::None{
							if let Some(data_val) = server_event.data{
								let data_str = data_val.to_string();
								match serde_json::from_str::<MoveData>(&data_str) {
									Ok(parsed_map_data) => {
										game.player.new_spawn = new_spawn;
										if let Some(ref mut mapdata) = game.map_data{

											mapdata.room.id = parsed_map_data.room;
										}
									}
									Err(e) => {
										eprintln!("MOVE error: {}", e);
									}
								}
							}
						}
					}

					PendingAction::Take => {
						if let Some(data_val) = server_event.data{
							let data_str = data_val.to_string();
							if !data_str.is_empty(){
								match serde_json::from_str::<Vec<String>>(&data_str) {
									Ok(items) => {
										if let Some(ref mut map_data) = game.map_data {
											for item_id in &items {
												map_data.items.retain(|x| x != item_id);
												println!("Take {}", item_id);
												game.player.inventory.data.insert(item_id.to_string(), 1);
											}
										}
									}
									Err(e) => {
										println!("Error JSON: {}", e);
									}
								}

							}
						}

					}
					PendingAction::Drop => {
						if let Some(data_val) = server_event.data{
							let data_str = data_val.to_string();
							if !data_str.is_empty(){
								match serde_json::from_str::<Vec<String>>(&data_str) {
									Ok(items) => {
										if let Some(ref mut map_data) = game.map_data {
											for item_id in &items {
												game.player.inventory.data.remove(item_id);
												println!("Drop {}", item_id);
												map_data.items.push(item_id.to_string());
											}
										}
									}
									Err(e) => {
										println!("Error JSON: {}", e);
									}
								}
							}
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
			if game.nb_players == 0 && game.pending_action == PendingAction::None{
				game.tx_to_serv.try_send("WHO \n".to_string()).ok();
				game.pending_action = PendingAction::Who;
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

				camera_handler(&mut camera, tile_size);

				if game.focus == InputFocus::Game{
					player_handler(&mut game, &map_obstacles, tile_size, sprite_width, sprite_height);
				}

				let current_skin_texture = game.skins[game.player.spritesheet_index as usize].texture.clone();
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
					camera_handler(&mut camera, tile_size);
					}

					let npc_places: Vec<Vec2> = find_npc_spawns(&map.colliders, tile_size);

					for (i,npc_id) in map_data.npcs.iter().enumerate(){
						if npc_places.len() >= i{
							continue;
						}
						// println!("{}", npc_id);
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


				let text_player = format!("Total players: {}",game.nb_players.clone());
				draw_text(map_data.room.name.clone(), 5.0, 30.0, 60.0, WHITE);
				draw_text(text_player, 5.0, 70.0, 60.0, WHITE);

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
		next_frame().await
		}
    }
}



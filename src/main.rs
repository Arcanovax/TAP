use macroquad::prelude::*;
mod rooms;
mod chat;
mod menu;

use rooms::get_rooms;
use chat::update_chat;
use chat::Chat;
use chat::draw_chat;
use menu::*;


fn player_handler(player: &mut Player, map: &[[i32; 25]; 15], tile_size: f32, sprite_width: f32, sprite_height: f32) {
	let mut direction = Vec2::ZERO;
	let animation_speed: f64 = 0.15;
	let mut add_x: f32 = 0.0;
    let mut add_y: f32 = 0.0;
	player.is_mooving = false;

	if is_key_down(KeyCode::D) {
        direction.x += 1.0;
        player.line = 1;
        player.is_mooving = true;
    }
    if is_key_down(KeyCode::A) {
        direction.x -= 1.0;
        player.line = 3;
        player.is_mooving = true;
    }
    if is_key_down(KeyCode::S) {
        direction.y += 1.0;
        player.line = 0;
        player.is_mooving = true;
    }
    if is_key_down(KeyCode::W) {
        direction.y -= 1.0;
        player.line = 2;
        player.is_mooving = true;
    }

	if player.is_mooving && direction != Vec2::ZERO{
		let velocity: Vec2 = direction.normalize_or_zero() * (player.speed * get_frame_time() * 60.0);
		add_x = velocity.x;
        add_y = velocity.y;

        player.row = ((get_time() / animation_speed) as i32).abs() % 4
    }
	else {
		player.row = 0
	}

	let player_w = sprite_width;
	let player_h = sprite_height;
	let hitbox_w = player_w * 0.5;
	let hitbox_h = player_h * 0.7;
	let hitbox_offset_x = (player_w - hitbox_w) * 0.5;
	let hitbox_offset_y = player_h - hitbox_h;

	if add_x != 0.0 {
		let next_rect = Rect::new(
			player.x + add_x + hitbox_offset_x,
			player.y + hitbox_offset_y,
			hitbox_w,
			hitbox_h,
		);
		if !rect_collides_map(next_rect, map, tile_size) {
			player.x += add_x;
		}
	}
	if add_y != 0.0 {
		let next_rect = Rect::new(
			player.x + hitbox_offset_x,
			player.y + add_y + hitbox_offset_y,
			hitbox_w,
			hitbox_h,
		);
		if !rect_collides_map(next_rect, map, tile_size) {
			player.y += add_y;
		}
	}




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
    spritesheet_index: usize
}

fn rect_collides_map(rect: Rect, map: &[[i32; 25]; 15], tile_size: f32) -> bool {
	let origin = vec2(0.0, 0.0);
    let local_rect = Rect::new(rect.x - origin.x, rect.y - origin.y, rect.w, rect.h);
    let left = (local_rect.x / tile_size).floor() as i32;
    let right = ((local_rect.x + local_rect.w - 0.001) / tile_size).floor() as i32;
    let top = (local_rect.y / tile_size).floor() as i32;
    let bottom = ((local_rect.y + local_rect.h - 0.001) / tile_size).floor() as i32;

    for ty in top..=bottom {
        for tx in left..=right {
            if ty < 0 || tx < 0 || ty as usize >= map.len() || tx as usize >= map[0].len() {
                continue;
            }

            if map[ty as usize][tx as usize] == 1 {

                let tile_hitbox = Rect::new(
                    tx as f32 * tile_size,
                    ty as f32 * tile_size + tile_size * 0.4,
                    tile_size,
                    tile_size * 0.4,
                );

                if local_rect.overlaps(&tile_hitbox) {
                    return true;
                }
            }
        }
    }

    false
}



fn config() -> Conf {
    Conf {
        window_title: "TAP".to_owned(),
        window_width: 1280,
        window_height: 720,
		// window_resizable: false,

		fullscreen: false,
        ..Default::default()
    }
}

struct Skin {
    texture: Texture2D,
    name: String,
}

struct Game {
    pub menu: Menu,
    pub player: Player,
    pub chat: Chat,
    pub skins: Vec<Skin>,
    pub room_name: String 
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



#[macroquad::main(config)]
async fn main() {

    let mut game: Game = Game{
        chat: Chat::new(),
        menu: Menu::new(),
        player: Player {
            x: 150.0,
            y: 150.0,
            line: 0,
            row: 0,
            is_mooving: false,
            speed: 0.8,
            spritesheet_index: 0
        },
        skins: Vec::new(),
        room_name: "place".to_string()
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

        let map: &rooms::Room = rooms.get(&game.room_name).unwrap();
        let floor: Texture2D = map.first_layer.clone();
        let builds: Option<Texture2D> = map.second_layer.clone();
        let map_obstacles = map.colliders;

        if let Some(builds_texture) = builds.as_ref() {
            builds_texture.set_filter(FilterMode::Nearest);
        }
        floor.set_filter(FilterMode::Nearest);

        clear_background(BLACK);
		if is_key_pressed(KeyCode::C) && !game.chat.is_active{
            break;
        }



		camera_handler(&mut camera, tile_size);

		if !game.chat.is_active && !game.menu.is_active{
			player_handler(&mut game.player, &map_obstacles, tile_size, sprite_width, sprite_height);
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
		update_menu(&mut game.menu, game.chat.is_active);
        update_chat(&mut game.chat, game.menu.is_active);

		draw_text(map.name.clone(), 5.0, 30.0, 60.0, WHITE);
		draw_chat(&mut game.chat);
		draw_menu(&mut game);


        next_frame().await
    }
}

use macroquad::prelude::*;
mod rooms;

use rooms::get_rooms;

fn player_handler(player: &mut Player, map: &[[i32; 25]; 15], tile_size: f32, sprite_width: f32, sprite_height: f32) {
	let mut direction = Vec2::ZERO;
	let animation_speed: f64 = 0.15;
	let mut add_x: f32 = 0.0;
    let mut add_y: f32 = 0.0;
	player.is_mooving = false;

	if is_key_down(KeyCode::D) {
        direction.x += player.speed;
        player.line = 1;
        player.is_mooving = true;
    }
    if is_key_down(KeyCode::A) {
        direction.x -= player.speed;
        player.line = 3;
        player.is_mooving = true;
    }
    if is_key_down(KeyCode::S) {
        direction.y += player.speed;
        player.line = 0;
        player.is_mooving = true;
    }
    if is_key_down(KeyCode::W) {
        direction.y -= player.speed;
        player.line = 2;
        player.is_mooving = true;
    }

	if player.is_mooving {
		let velocity: Vec2 = direction.normalize() * player.speed;
		add_x = velocity.x;
        add_y = velocity.y;

        player.row = ((get_time() / animation_speed) as i32) % 4
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
	speed: f32
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
        ..Default::default()
    }
}

#[macroquad::main(config)]
async fn main() {

    let mut player = Player {
        x: 150.0,
        y: 150.0,
        line: 0,
		row: 0,
        is_mooving: false,
		speed: 0.075
    };

	let rooms: std::collections::HashMap<String, rooms::Room> = get_rooms().await;
    let map: &rooms::Room = rooms.get("place").unwrap();

	let spritesheet = load_texture("assets/skins/alex.png").await.unwrap();
    let floor: Texture2D = map.first_layer.clone();
    let builds = map.second_layer.clone();

    if let Some(builds_texture) = builds.as_ref() {
        builds_texture.set_filter(FilterMode::Nearest);
    }
	spritesheet.set_filter(FilterMode::Nearest);
    floor.set_filter(FilterMode::Nearest);
	let sprite_width: f32 = 16.0;
    let sprite_height: f32 = 32.0;


    let tile_size: f32 = 16.0;


    let mut camera = Camera2D::default();



    let map_obstacles = map.colliders;

    loop {
        clear_background(BLACK);

		player_handler(&mut player, &map_obstacles, tile_size, sprite_width, sprite_height);

		camera_handler(&mut camera, tile_size);


		if is_key_down(KeyCode::Escape) {
            break;
        }

        let source_x: f32 = player.row as f32 * sprite_width;
        let source_y: f32 = player.line as f32 * sprite_height;


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
            &spritesheet,
			player.x.round(), player.y.round(),
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
		draw_text(map.name.clone(), 5.0, 5.0, 10.0, WHITE);
        next_frame().await
    }
}

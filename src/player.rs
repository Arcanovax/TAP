use crate::*;

pub fn player_handler(game: &mut Game, map: &[[i32; 25]; 15]) {
	let player = &mut game.player;

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

	let tile_size: f32 = game.config.tile_size;
	let player_w: f32 = game.config.sprite_width;
	let player_h: f32 = game.config.sprite_height;
	let hitbox_w: f32 = player_w * 0.5;
	let hitbox_h: f32 = player_h * 0.7;
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
	let current_rect = Rect::new(
			player.x + hitbox_offset_x,
			player.y + hitbox_offset_y,
			hitbox_w,
			hitbox_h);
	let current_tile: i32 = get_current_tile(current_rect, map, tile_size);
	handle_move(game, current_tile);
}


fn handle_move(game: &mut Game, current_tile: i32){
	if game.pending_action != PendingAction::None {
        return;
    }

    if game.player.new_spawn != Spawn::None {
        return;
    }

	if game.player.x >= 390.0{
		game.tx_to_serv.try_send("MOVE East\n".to_string()).ok();
		game.pending_action = PendingAction::Move(Spawn::West);
	}
	if game.player.x <= 0.0{
		game.tx_to_serv.try_send("MOVE West\n".to_string()).ok();
		game.pending_action = PendingAction::Move(Spawn::East);
	}
	if game.player.y >= 200.0{
		game.tx_to_serv.try_send("MOVE South\n".to_string()).ok();
		game.pending_action = PendingAction::Move(Spawn::North);
	}
	if game.player.y <= -10.0 || current_tile == 2{
		game.tx_to_serv.try_send("MOVE North\n".to_string()).ok();
		game.pending_action = PendingAction::Move(Spawn::South);
	}
	if current_tile == 7{
		game.tx_to_serv.try_send("DUNGEON JOIN\n".to_string()).ok();
		game.pending_action = PendingAction::DungeonJoin;
	}
	if current_tile == 4{
		game.gambling.slot.is_active = true;
	}
	else if current_tile == 0{
		game.gambling.slot.is_active = false;
	}
}


fn get_current_tile(rect: Rect, map: &[[i32; 25]; 15], tile_size: f32) -> i32 {
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
            return map[ty as usize][tx as usize]
        }
    }
	return 0;
}

pub fn draw_player_info(game: &mut Game){
	if let Some(state) = game.player.state.clone() {
	let info: Rect = Rect::new(10.0, 10.0, 350.0, 120.0);
	draw_rectangle(info.x, info.y, info.w, info.h, Color::new(0.0, 0.0, 0.0, 0.5));


	let frame = Rect::new(info.x+10.0, info.y+10.0, 100.0, 100.0);
	draw_rectangle(frame.x, frame.y, frame.w, frame.h,BLACK);

	let cut_sheet_head = DrawTextureParams {
		source: Some(Rect::new(0.0, 0.0, game.config.sprite_width, 20.0)),
		dest_size: Some(vec2(75.0, 100.0 )),
		..Default::default()
	};
	draw_texture_ex(
		&game.skin.clone(),
		frame.x+12.5, frame.y,
		WHITE,
		cut_sheet_head
	);
	draw_rectangle_lines(frame.x, frame.y, frame.w, frame.h, 10.0, Color::new(0.53, 0.31, 0.16, 1.0));


	draw_text(&game.player.name, frame.x + frame.w + 5.0, frame.y + 30.0, 40.0, WHITE);
	if let Some(gold) = game.player.gold{
		let text_gold = format!("Gold: {}", gold);
		draw_text(&text_gold, frame.x + frame.w + 5.0, frame.y + 60.0, 30.0, YELLOW);
	}

	let lifebar = Rect::new(frame.x + frame.w + 5.0, frame.y + 70.0, 200.0, 25.0);
	draw_rectangle(lifebar.x, lifebar.y, lifebar.w, lifebar.h,BLACK);
	let hp_ratio: f32 = state.hp as f32 / state.max_hp as f32;
	draw_rectangle(lifebar.x, lifebar.y+2.5, lifebar.w * hp_ratio, 20.0,RED);
	let hp_info = format!("{}/{}",state.hp,state.max_hp);
	draw_text_center(lifebar, &hp_info, 20);
	draw_rectangle_lines(lifebar.x, lifebar.y, lifebar.w, lifebar.h, 5.0, GRAY);
}
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

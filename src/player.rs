use crate::*;

pub fn player_handler(game: &mut Game, map: &[[i32; 25]; 15], tile_size: f32, sprite_width: f32, sprite_height: f32) {
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

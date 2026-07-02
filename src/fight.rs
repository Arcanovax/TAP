use crate::*;
const ENEMY_POS: Vec2 = vec2(275.0,125.0);
const PLAYER_POS: Vec2 = vec2(85.0,140.0);

pub struct Fight{
	pub enemy: Npc,
	pub players: HashMap<String, i32>,

}

fn get_npc(game: &Game) -> Option<Npc> {
    let state = game.player.state.as_ref()?;

	let target_id = match &state.status {
		Status::InFight { target_id } => target_id,
		_ => return None,
	};

    return game.loaded_npcs.get(target_id).cloned()
}

pub fn draw_enemy_info(game: &mut Game, npc: Npc){

	let mut info: Rect = get_rect_right(vec2(350.0, 120.0), 10.0);
	info.x -= 10.0;
	draw_rectangle(info.x, info.y, info.w, info.h, Color::new(0.0, 0.0, 0.0, 0.5));


	let frame = Rect::new(info.x+10.0, info.y+10.0, 100.0, 100.0);
	draw_rectangle(frame.x, frame.y, frame.w, frame.h,BLACK);

	let cut_sheet_head = DrawTextureParams {
		source: Some(Rect::new(0.0, 0.0, game.config.sprite_width, 20.0)),
		dest_size: Some(vec2(75.0, 100.0 )),
		..Default::default()
	};
	draw_texture_ex(
		&npc.texture,
		frame.x+12.5, frame.y,
		WHITE,
		cut_sheet_head
	);
	draw_rectangle_lines(frame.x, frame.y, frame.w, frame.h, 10.0, Color::new(0.53, 0.31, 0.16, 1.0));


	draw_text(&npc.name, frame.x + frame.w + 5.0, frame.y + 30.0, 40.0, WHITE);



}

pub fn handle_fight(game: &mut Game, floor: &Texture2D) {




	camera_handler(game);

	let (enemy, players) = match &game.active_fight {
        Some(fight) => (fight.enemy.clone(), fight.players.clone()),
        None => return,
    };

	let Some(fight) = &game.active_fight else { return };

    floor.set_filter(FilterMode::Nearest);
    draw_texture_ex(
        floor,
        0.0,
        0.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(floor.width(), floor.height())),
            ..Default::default()
        },
    );

    let sprite_size = vec2(game.config.sprite_width * 1.5, game.config.sprite_height * 1.5);
	let sprite_width: f32 = game.config.sprite_width;
	let sprite_height: f32 = game.config.sprite_height;
    draw_texture_ex(
        &enemy.texture,
        ENEMY_POS.x,
        ENEMY_POS.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(sprite_size),
            ..Default::default()
        },
    );

	let players_len = fight.players.len();
	let spacing = 40.0;

	let total_width = if players_len > 1 { (players_len - 1) as f32 * spacing } else { 0.0 };
	let start_x = PLAYER_POS.x - (total_width / 2.0);
	for (i, (player,life)) in players.iter().enumerate(){
		let player_pos = &vec2(start_x + (i as f32 * spacing) ,PLAYER_POS.y);
		camera_handler(game);
		draw_texture_ex(
			&game.skins[game.player.spritesheet_index as usize].texture.clone(),
			player_pos.x,player_pos.y,
			WHITE,
			DrawTextureParams {
				source: Some(Rect::new(0.0, 0.0, sprite_width, sprite_height - 1.0)),
				dest_size: Some(vec2(sprite_width, sprite_height - 1.0)),
				..Default::default()},
    	);
		let screen_pos = world_to_screen_pos(*player_pos);
		let sprite_rect = world_to_screen_pos(vec2(sprite_width, sprite_height));
		set_default_camera();
		let rect_width = 80.0;
		let rect = Rect::new(screen_pos.x + (sprite_rect.x / 2.0) - (rect_width / 2.0), screen_pos.y-30.0, rect_width, 20.0);
		draw_text_center(rect, player, 30);
		let rect = Rect::new(screen_pos.x + (sprite_rect.x / 2.0) - (rect_width / 2.0), screen_pos.y-10.0, rect_width, 20.0);
		let hp_text = format!("{}HP", life);
		draw_text_center(rect, hp_text.as_str(), 20);
	}

	if let Some(state) = game.player.state.clone() {
		set_default_camera();
		draw_player_info(game, state);
		draw_enemy_info(game, enemy)
	}
}


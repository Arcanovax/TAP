use crate::*;
const ENEMY_POS: Vec2 = vec2(275.0,125.0);
const PLAYER_POS: Vec2 = vec2(85.0,140.0);

pub struct Fight{
	pub enemy: Npc,
	pub players: HashMap<String, i32>,
	pub enemy_hp: i32,
	pub chat: Vec<String>,
	pub consume_is_act: bool
}

pub fn draw_enemy_info(game: &mut Game, npc: Npc){
	let Some(fight) = &game.active_fight else { return };
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
	let lifebar = Rect::new(frame.x + frame.w + 5.0, frame.y + 70.0, 200.0, 25.0);
	draw_rectangle(lifebar.x, lifebar.y, lifebar.w, lifebar.h,BLACK);
	let hp_ratio: f32 = fight.enemy_hp as f32 / 2000 as f32;
	draw_rectangle(lifebar.x, lifebar.y+2.5, lifebar.w * hp_ratio, 20.0,RED);
	let hp_info = format!("{}/{}",fight.enemy_hp,2000);
	draw_text_center(lifebar, &hp_info, 20);
	draw_rectangle_lines(lifebar.x, lifebar.y, lifebar.w, lifebar.h, 5.0, GRAY);


}

pub fn handle_fight(game: &mut Game, floor: &Texture2D) {

	camera_handler(game);

	let (enemy, players, chat) = match &game.active_fight {
		Some(fight) => (fight.enemy.clone(), fight.players.clone(), fight.chat.clone()),
		None => return,
	};


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

	let players_len = players.len();
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
		draw_player_info(game, state.clone());
		draw_enemy_info(game, enemy.clone());


		let rect = get_center_rect_x(vec2(1100.0, 75.0), screen_height()-85.0);
		draw_rectangle(rect.x, rect.y, rect.w, rect.h, Color::new(0.0, 0.0, 0.0, 0.5));
		let btn_size = vec2(300.0, 55.0);
		let spacing = 30.0;
		let mouse = mouse_position();

		let total_width = (3.0 as f32 * btn_size.x) + ((3 - 1) as f32 * spacing);

		let rect_center_x = rect.x + (rect.w / 2.0);
		let start_x = rect_center_x - (total_width / 2.0);
		let pos_y = rect.y + (rect.h / 2.0) - (btn_size.y / 2.0);

		let pos_attack = vec2(start_x, pos_y);
		let btn_attack = Rect::new(pos_attack.x, pos_attack.y, btn_size.x, btn_size.y);
		if get_button(btn_attack, "Attack", 25, WHITE, mouse) {
			let rq: String = format!("ATTACK {}\n",enemy.id);
			game.tx_to_serv.try_send(rq).ok();
			game.pending_action = PendingAction::Attack(enemy.id.clone());
		}
		let Some(ref mut fight) = game.active_fight else { return };
		let btn_skill = Rect::new(start_x + (1.0 * (btn_size.x + spacing)), pos_y, btn_size.x, btn_size.y);
		if get_button(btn_skill, "Consume", 25, WHITE, mouse) {

			fight.consume_is_act = !fight.consume_is_act
		}

		let btn_flee = Rect::new(start_x + (2.0 * (btn_size.x + spacing)), pos_y, btn_size.x, btn_size.y);
		if get_button(btn_flee, "Flee", 25, WHITE, mouse) {
			let rq: String = format!("FLEE {}\n",enemy.id);
			game.tx_to_serv.try_send(rq).ok();
			game.pending_action = PendingAction::Flee;
		}

		let chat_rect = get_center_rect_x(vec2(400.0, 250.0), 125.0);
		draw_rectangle(chat_rect.x, chat_rect.y, chat_rect.w, chat_rect.h, Color::new(0.0, 0.0, 0.0, 0.75));
		for (i, msg) in chat.iter().enumerate(){
			draw_text(msg, chat_rect.x + 10.0 , chat_rect.y + (20 + (i* 20))as f32, 20.0, WHITE);
		}

		if fight.consume_is_act {
			handle_consume(game);
		}

	}
}

fn handle_consume(game: &mut Game){
	let consume_rect = get_center_rect_x(vec2(200.0, 200.0), 405.0);
	draw_rectangle(consume_rect.x, consume_rect.y, consume_rect.w, consume_rect.h, Color::new(0.0, 0.0, 0.0, 0.75));

	let inventory_data = game.player.inventory.data.clone();
	let mut y_index = 0;
	for (item_id, amount) in inventory_data.iter(){
		if let Some(item) = game.loaded_items.get(item_id).cloned() {
			if let ItemKind::Potion { .. } = item.kind {
				let item_rect = Rect::new(consume_rect.x, consume_rect.y + 50.0 * (y_index as f32), 50.0, 50.0);
				if get_item_slot_inv(item_rect, game, &item, amount, game.mouse) {
					let rq: String = format!("CONSUME {}\n", item.id);
					game.tx_to_serv.try_send(rq).ok();
					game.pending_action = PendingAction::Consume(item.id);
				}
				y_index += 1;
			}
		}
	}
}

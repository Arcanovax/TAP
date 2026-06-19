use crate::*;
const ENEMY_POS: Vec2 = vec2(275.0,125.0);
const PLAYER_POS: Vec2 = vec2(85.0,125.0);

pub struct Fight{
	enemy: Npc,
	players: Vec<String>
}

fn get_npc(game: &Game) -> Option<Npc> {
    let state = game.player.state.as_ref()?;

	let target_id = match &state.status {
		Status::InFight { target_id } => target_id,
		_ => return None,
	};

    return game.loaded_npcs.get(target_id).cloned()
}


pub fn handle_fight(game: &mut Game, floor: &Texture2D) {

    if game.active_fight.is_none() {
        if let Some(npc) = get_npc(game) {
            game.active_fight = Some(Fight {
                enemy: npc,
                players: vec![game.player.name.clone()],
            });
        }
    }



	camera_handler(game); 


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
        &fight.enemy.texture,
        ENEMY_POS.x,
        ENEMY_POS.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(sprite_size),
            ..Default::default()
        },
    );
	draw_texture_ex(
        &game.skins[game.player.spritesheet_index as usize].texture.clone(),
        PLAYER_POS.x,
        PLAYER_POS.y,
        WHITE,
        DrawTextureParams {
			source: Some(Rect::new(0.0, 0.0, sprite_width, sprite_height - 1.0)),
			dest_size: Some(vec2(sprite_width, sprite_height - 1.0)),
			..Default::default()
		},
    );
}


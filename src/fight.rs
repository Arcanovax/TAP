use crate::*;

pub async fn handle_fight(game: &mut Game){
	let npc: Option<Npc> = game.loaded_npcs.get(game.).cloned();
	if let Some(npc) = npc {

	}

	let floor: Texture2D = load_texture("assets/map/fightmap.png").await.unwrap();
	let texture_param = DrawTextureParams {
						dest_size: Some(vec2(game.config.sprite_width, game.config.sprite_height)),
						..Default::default()
			};

	draw_texture_ex(
		&npc_texture,
		150.0,
		100.0,
		WHITE,
		texture_param.clone()
	);
	floor.set_filter(FilterMode::Nearest);

	camera_handler(game);
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

}



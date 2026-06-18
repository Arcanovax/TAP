use crate::*;
const ENEMY_POS: Vec2 = vec2(275.0,125.0);
const PLAYER_POS: Vec2 = vec2(85.0,125.0);

pub struct Fight{
	ennemy: Npc,
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

pub async fn handle_fight(game: &mut Game){
	

	let npc = get_npc(game).expect("No npc");
	if game.active_fight.contains_key(&npc.id){
		game.active_fight.get(&npc.id).expect("").clone().players.push(game.player.name);
		
	}
	else {
		let new_fight: Fight = Fight { ennemy: npc, players: vec![game.player.name.clone()] };
		game.active_fight.insert(npc.id, new_fight);
	}

	let floor: Texture2D = load_texture("assets/map/fightmap.png").await.unwrap();
	let texture_sprite = DrawTextureParams {
						dest_size: Some(vec2(game.config.sprite_width * 1.5, game.config.sprite_height* 1.5)),
						..Default::default()
			};



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

	draw_texture_ex(
		&npc.texture,
		ENEMY_POS.x,
		ENEMY_POS.y,
		WHITE,
		texture_sprite.clone()
	);
	floor.set_filter(FilterMode::Nearest);

}



use std::collections::HashMap;
use crate::*;
use macroquad::prelude::*;


#[derive(Clone, PartialEq)]
pub struct Npc {
    id: String,
    pub texture: Texture2D,
}

pub fn get_npc_from_id(npcs: HashMap<String, Npc>, npc_id:  &String) -> Npc{
	let id: &str = npc_id.strip_prefix("npc.").unwrap();
	if let Some(npc) = npcs.get(id) {
		return npc.clone();
	}
	return Npc { id: String::new(), texture: Texture2D::empty()}
}



pub async fn get_npcs() -> HashMap<String, Npc> {
    let mut npcs: HashMap<String, Npc> = HashMap::new();

	let guard = Npc {
        texture: load_texture("assets/npc/city_gard.png").await.unwrap(),
        id: "city_gard".to_string(),
    };
    npcs.insert(guard.id.clone(), guard);
    return npcs;
}

pub fn find_npc_spawns(colliders: &[[i32; 25]; 15], tile_size: f32) -> Vec<Vec2> {
    let mut positions = Vec::new();

    for (row, line) in colliders.iter().enumerate() {
        for (col, &cell) in line.iter().enumerate() {
            if cell == 8 {
                positions.push(vec2(
                    col as f32 * tile_size,
                    row as f32 * tile_size,
                ));
            }
        }
    }
    return positions;
}
